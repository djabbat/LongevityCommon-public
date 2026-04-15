use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db,
    models::{ErrorResponse, RoundResult},
    orchestrator,
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub status: String,
    pub message: String,
}

/// GET /api/rounds
///
/// List all completed rounds in ascending order.
pub async fn list_rounds(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RoundResult>>, (StatusCode, Json<ErrorResponse>)> {
    match db::list_rounds(&state.pool).await {
        Ok(rounds) => Ok(Json(rounds)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    }
}

/// GET /api/rounds/:round_id
///
/// Fetch metadata for a specific round by its UUID.
pub async fn get_round(
    State(state): State<Arc<AppState>>,
    Path(round_id): Path<Uuid>,
) -> Result<Json<RoundResult>, (StatusCode, Json<ErrorResponse>)> {
    match db::get_round(&state.pool, round_id).await {
        Ok(Some(round)) => Ok(Json(round)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(format!("Round {round_id} not found"))),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    }
}

/// POST /api/rounds/trigger
///
/// Admin endpoint — manually force an aggregation of all pending updates.
/// Returns 200 with `status: "triggered"` if aggregation ran,
/// or 200 with `status: "no_updates"` if there was nothing to aggregate.
pub async fn trigger_round(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TriggerResponse>, (StatusCode, Json<ErrorResponse>)> {
    match orchestrator::force_aggregate(Arc::clone(&state)).await {
        Ok(true) => Ok(Json(TriggerResponse {
            status: "triggered".to_string(),
            message: "Aggregation completed successfully".to_string(),
        })),
        Ok(false) => Ok(Json(TriggerResponse {
            status: "no_updates".to_string(),
            message: "No pending updates to aggregate".to_string(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("Aggregation failed: {e}"))),
        )),
    }
}
