use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::AuthUser,
    models::post::{CreatePostRequest, FeedQuery, PostWithAuthor, ReactRequest, ReactionCounts},
    services::{doi_validator, feed_ranker},
    AppState,
};

pub async fn get_feed(
    State(state): State<AppState>,
    Query(query): Query<FeedQuery>,
) -> Result<Json<Vec<PostWithAuthor>>, (StatusCode, String)> {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = page * page_size;

    let rows = sqlx::query!(
        r#"SELECT
            p.id, p.author_id, p.type as post_type, p.content,
            p.doi, p.doi_verified, p.code_url, p.data_url,
            p.score, p.rank_penalty, p.parent_id, p.study_id,
            p.created_at, p.edited_at,
            u.username as author_username,
            u.degree_verified as author_degree_verified,
            COALESCE(SUM(CASE WHEN r.type = 'support' THEN 1 ELSE 0 END), 0) as support,
            COALESCE(SUM(CASE WHEN r.type = 'replicate' THEN 1 ELSE 0 END), 0) as replicate,
            COALESCE(SUM(CASE WHEN r.type = 'challenge' THEN 1 ELSE 0 END), 0) as challenge,
            COALESCE(SUM(CASE WHEN r.type = 'cite' THEN 1 ELSE 0 END), 0) as cite
           FROM posts p
           JOIN users u ON u.id = p.author_id
           LEFT JOIN post_reactions r ON r.post_id = p.id
           WHERE p.deleted_at IS NULL
             AND p.parent_id IS NULL
             AND ($1::text IS NULL OR p.type = $1)
           GROUP BY p.id, u.username, u.degree_verified
           ORDER BY p.score DESC, p.created_at DESC
           LIMIT $2 OFFSET $3"#,
        query.post_type,
        page_size,
        offset,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let posts = rows
        .into_iter()
        .map(|r| PostWithAuthor {
            post: crate::models::post::Post {
                id: r.id,
                author_id: r.author_id,
                r#type: r.post_type,
                content: r.content,
                doi: r.doi,
                doi_verified: r.doi_verified,
                code_url: r.code_url,
                data_url: r.data_url,
                score: r.score,
                rank_penalty: r.rank_penalty,
                parent_id: r.parent_id,
                study_id: r.study_id,
                created_at: r.created_at,
                edited_at: r.edited_at,
            },
            author_username: r.author_username,
            author_degree_verified: r.author_degree_verified,
            reactions: ReactionCounts {
                support: r.support.unwrap_or(0),
                replicate: r.replicate.unwrap_or(0),
                challenge: r.challenge.unwrap_or(0),
                cite: r.cite.unwrap_or(0),
            },
        })
        .collect();

    Ok(Json(posts))
}

pub async fn create_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    req.validate()
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

    let valid_types = ["ze_log", "science_thread", "study_invite", "debate"];
    if !valid_types.contains(&req.post_type.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid post type".into()));
    }

    // XSS: strip all HTML tags from content
    let content = strip_html_tags(&req.content);

    let post_id = Uuid::new_v4();
    let rank_penalty = 0.0_f64;

    sqlx::query!(
        r#"INSERT INTO posts (id, author_id, type, content, doi, doi_verified, code_url, data_url, rank_penalty, parent_id, study_id)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
        post_id,
        auth_user.id,
        req.post_type,
        content,
        req.doi,
        false, // doi_verified starts false; background task updates it
        req.code_url,
        req.data_url,
        rank_penalty,
        req.parent_id,
        req.study_id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Non-blocking DOI verification: spawn background task
    if let Some(doi) = req.doi.clone() {
        let db = state.db.clone();
        let crossref_url = state.config.crossref_base_url.clone();
        tokio::spawn(async move {
            let verified = doi_validator::verify_doi(&doi, &crossref_url).await;
            let penalty: f64 = if verified { 0.0 } else { doi_validator::FAKE_DOI_PENALTY };
            if !verified {
                tracing::warn!(post_id = %post_id, doi = %doi, "DOI verification failed, applying penalty");
            }
            let _ = sqlx::query!(
                "UPDATE posts SET doi_verified = $1, rank_penalty = rank_penalty + $2 WHERE id = $3",
                verified,
                penalty,
                post_id,
            )
            .execute(&db)
            .await;
        });
    }

    Ok(Json(serde_json::json!({
        "id": post_id,
        "doi_verified": false,
        "message": "Post created. DOI verification running in background."
    })))
}

pub async fn react_to_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
    Json(req): Json<ReactRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let valid_types = ["support", "replicate", "challenge", "cite"];
    if !valid_types.contains(&req.reaction_type.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid reaction type".into()));
    }

    sqlx::query!(
        r#"INSERT INTO post_reactions (id, post_id, user_id, type)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (post_id, user_id, type) DO NOTHING"#,
        Uuid::new_v4(),
        post_id,
        auth_user.id,
        req.reaction_type,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Recalculate score
    update_post_score(&state, post_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let rows = sqlx::query!(
        "UPDATE posts SET deleted_at = NOW() WHERE id = $1 AND author_id = $2",
        post_id,
        auth_user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .rows_affected();

    if rows == 0 {
        return Err((StatusCode::NOT_FOUND, "Post not found or unauthorized".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}

/// Strip HTML tags to prevent XSS in post content.
/// Uses regex-lite to remove all <tag> patterns.
fn strip_html_tags(input: &str) -> String {
    use regex_lite::Regex;
    static HTML_TAG_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = HTML_TAG_RE.get_or_init(|| Regex::new(r"<[^>]*>").unwrap());
    re.replace_all(input, "").into_owned()
}

async fn update_post_score(
    state: &AppState,
    post_id: Uuid,
) -> Result<(), (StatusCode, String)> {
    let row = sqlx::query!(
        r#"SELECT
            p.doi_verified, p.code_url, p.data_url, p.rank_penalty, p.created_at,
            COALESCE(SUM(CASE WHEN r.type = 'support' THEN 1 ELSE 0 END), 0) as support,
            COALESCE(SUM(CASE WHEN r.type = 'replicate' THEN 1 ELSE 0 END), 0) as replicate,
            COALESCE(SUM(CASE WHEN r.type = 'challenge' THEN 1 ELSE 0 END), 0) as challenge,
            COALESCE(SUM(CASE WHEN r.type = 'cite' THEN 1 ELSE 0 END), 0) as cite
           FROM posts p
           LEFT JOIN post_reactions r ON r.post_id = p.id
           WHERE p.id = $1
           GROUP BY p.id"#,
        post_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let score = feed_ranker::compute_score(&feed_ranker::PostScoreInput {
        reactions_support: row.support.unwrap_or(0),
        reactions_replicate: row.replicate.unwrap_or(0),
        reactions_challenge: row.challenge.unwrap_or(0),
        reactions_cite: row.cite.unwrap_or(0),
        doi_verified: row.doi_verified,
        has_code_url: row.code_url.is_some(),
        has_data_url: row.data_url.is_some(),
        created_at: row.created_at,
        rank_penalty: row.rank_penalty,
    });

    sqlx::query!("UPDATE posts SET score = $1 WHERE id = $2", score, post_id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(())
}
