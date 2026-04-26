use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser,
    models::user::{PublicUser, UpdateProfileRequest, User},
    AppState,
};

pub async fn get_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, email_verified, birth_year, country_code,
                  orcid_id, degree_verified, is_pro, fclc_node_id, fclc_node_active,
                  consent_given, consent_at, created_at
           FROM users WHERE id = $1 AND deleted_at IS NULL"#,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "User not found".into()))?;

    // Latest Ze·Profile (public view — only estimate + trend, no raw data)
    let latest = sqlx::query!(
        r#"SELECT bio_age_est, bio_age_ci_low, bio_age_ci_high, chi_ze_combined, ci_stability, recorded_at
           FROM ze_samples
           WHERE user_id = $1 AND is_verified = true
           ORDER BY recorded_at DESC LIMIT 1"#,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten();

    Ok(Json(serde_json::json!({
        "user": PublicUser::from(user),
        "ze_profile": latest.map(|s| serde_json::json!({
            "bio_age_est": s.bio_age_est,
            "bio_age_ci_low": s.bio_age_ci_low,
            "bio_age_ci_high": s.bio_age_ci_high,
            "chi_ze_combined": s.chi_ze_combined,
            "ci_stability": s.ci_stability,
            "last_sample_at": s.recorded_at,
        }))
    })))
}

/// Public profile lookup by username — used by /u/:username page
pub async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = sqlx::query!(
        r#"SELECT id, username, degree_verified, is_pro, fclc_node_active, country_code, created_at
           FROM users WHERE username = $1 AND deleted_at IS NULL"#,
        username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "User not found".into()))?;

    let ze_profile = sqlx::query!(
        r#"SELECT bio_age_est, bio_age_ci_low, bio_age_ci_high, chi_ze_combined, ci_stability, recorded_at
           FROM ze_samples
           WHERE user_id = $1 AND is_verified = true
           ORDER BY recorded_at DESC LIMIT 1"#,
        user.id
    )
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten()
    .map(|s| serde_json::json!({
        "bio_age_est": s.bio_age_est,
        "bio_age_ci_low": s.bio_age_ci_low,
        "bio_age_ci_high": s.bio_age_ci_high,
        "chi_ze_combined": s.chi_ze_combined,
        "ci_stability": s.ci_stability,
        "last_sample_at": s.recorded_at,
    }));

    Ok(Json(serde_json::json!({
        "id": user.id,
        "username": user.username,
        "degree_verified": user.degree_verified,
        "is_pro": user.is_pro,
        "fclc_node_active": user.fclc_node_active,
        "country_code": user.country_code,
        "created_at": user.created_at,
        "ze_profile": ze_profile,
    })))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        r#"UPDATE users SET
            username = COALESCE($1, username),
            birth_year = COALESCE($2, birth_year),
            country_code = COALESCE($3, country_code),
            orcid_id = COALESCE($4, orcid_id),
            fclc_node_id = COALESCE($5, fclc_node_id),
            fclc_node_active = CASE WHEN $5 IS NOT NULL THEN true ELSE fclc_node_active END
           WHERE id = $6"#,
        req.username,
        req.birth_year,
        req.country_code,
        req.orcid_id,
        req.fclc_node_id,
        auth_user.id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_account(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    // GDPR soft delete — marks user and cascades via DB triggers
    sqlx::query!(
        "UPDATE users SET deleted_at = NOW(), email = 'deleted_' || id || '@longevitycommon.deleted' WHERE id = $1",
        auth_user.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
