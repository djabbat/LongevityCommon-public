use axum::{extract::State, http::StatusCode, Extension, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::AuthUser,
    models::ze_guide::{ZeGuideAskRequest, ZeGuideResponse, ZE_GUIDE_DISCLAIMER},
    services::ai_guide::{self, ConversationTurn},
    AppState,
};

pub async fn ask_ze_guide(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<ZeGuideAskRequest>,
) -> Result<Json<ZeGuideResponse>, (StatusCode, String)> {
    req.validate()
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

    let session_id = req.session_id.unwrap_or_else(Uuid::new_v4);

    // Load up to 6 previous turns from this session for context
    let history_rows = sqlx::query!(
        r#"SELECT prompt, response FROM ze_guide_logs
           WHERE user_id = $1 AND session_id = $2
           ORDER BY created_at ASC LIMIT 6"#,
        auth_user.id,
        session_id,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let history: Vec<ConversationTurn> = history_rows
        .into_iter()
        .map(|r| ConversationTurn { prompt: r.prompt, response: r.response })
        .collect();

    let result = ai_guide::ask(
        &req.prompt,
        &history,
        &state.config.deepseek_api_key,
        &state.config.deepseek_base_url,
        &state.config.ollama_base_url,
        &state.config.ollama_model,
    )
    .await;

    // Log every interaction — required for legal protection
    sqlx::query!(
        r#"INSERT INTO ze_guide_logs
           (id, user_id, session_id, prompt, response, model_used, cited_dois, cited_files, disclaimer_sent, latency_ms)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, $9)"#,
        Uuid::new_v4(),
        auth_user.id,
        session_id,
        req.prompt,
        result.response,
        result.model_used,
        &result.cited_dois,
        &result.cited_files,
        result.latency_ms,
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to log Ze·Guide interaction: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(ZeGuideResponse {
        session_id,
        disclaimer: ZE_GUIDE_DISCLAIMER.into(),
        response: result.response,
        cited_dois: result.cited_dois,
        cited_files: result.cited_files,
        model_used: result.model_used,
    }))
}

pub async fn get_ze_guide_history(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    let logs = sqlx::query!(
        r#"SELECT session_id, prompt, response, model_used, cited_dois, created_at
           FROM ze_guide_logs
           WHERE user_id = $1
           ORDER BY created_at DESC
           LIMIT 50"#,
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = logs
        .into_iter()
        .map(|l| serde_json::json!({
            "session_id": l.session_id,
            "prompt": l.prompt,
            "response": l.response,
            "model_used": l.model_used,
            "cited_dois": l.cited_dois,
            "created_at": l.created_at,
        }))
        .collect();

    Ok(Json(result))
}
