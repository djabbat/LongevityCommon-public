use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::{middleware::auth::AuthUser, AppState};

pub async fn get_stats(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Only allow admin users (is_pro = true as MVP proxy; replace with explicit is_admin column in v2)
    let user = sqlx::query!(
        "SELECT is_pro FROM users WHERE id = $1 AND deleted_at IS NULL",
        auth_user.id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "User not found".into()))?;

    if !user.is_pro {
        return Err((StatusCode::FORBIDDEN, "Admin access required".into()));
    }

    let user_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .unwrap_or(0);

    let active_30d = sqlx::query_scalar!(
        "SELECT COUNT(DISTINCT user_id) FROM ze_samples WHERE recorded_at > NOW() - INTERVAL '30 days'"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .unwrap_or(0);

    let sample_count = sqlx::query_scalar!("SELECT COUNT(*) FROM ze_samples WHERE is_verified = true")
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .unwrap_or(0);

    let post_count = sqlx::query_scalar!("SELECT COUNT(*) FROM posts WHERE deleted_at IS NULL")
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .unwrap_or(0);

    let study_count = sqlx::query_scalar!("SELECT COUNT(*) FROM studies WHERE status = 'active'")
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .unwrap_or(0);

    let fclc_node_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE fclc_node_active = true AND deleted_at IS NULL"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "users_total": user_count,
        "users_active_30d": active_30d,
        "ze_samples_verified": sample_count,
        "posts_total": post_count,
        "studies_active": study_count,
        "fclc_nodes_active": fclc_node_count,
    })))
}
