use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{models::GlobalModelResponse, state::AppState};

/// GET /api/model/current
///
/// Return the current global model weights and the round they were produced in.
pub async fn current_model(
    State(state): State<Arc<AppState>>,
) -> Json<GlobalModelResponse> {
    let weights = state.global_model.read().await.clone();
    let round = *state.current_round.read().await;

    Json(GlobalModelResponse {
        weights,
        round,
        version: format!("v0.1.0-round-{round}"),
    })
}
