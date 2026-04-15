use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{
    models::{ErrorResponse, UpdatePayload},
    orchestrator,
    state::{AppState, EPSILON_TOTAL},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub status: String,
    pub aggregation_triggered: bool,
}

/// POST /api/nodes/:node_id/update
///
/// Receive a gradient update from a node for the current round.
/// Rejects updates from nodes that have exceeded their DP epsilon budget.
/// After storing, checks if aggregation should be triggered automatically.
pub async fn submit_update(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<Uuid>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<UpdateResponse>, (StatusCode, Json<ErrorResponse>)> {
    // ── Budget check (Rényi DP + linear fallback) ─────────────────────────────
    {
        let budgets = state.node_budgets.read().await;
        let effective_spent = budgets
            .get(&node_id)
            .map(|s| s.effective_epsilon())
            .unwrap_or(0.0);
        if effective_spent + payload.epsilon_spent > EPSILON_TOTAL {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::new(format!(
                    "DP budget exceeded: effective_spent={effective_spent:.4}, requested={:.4}, total_allowed={EPSILON_TOTAL}",
                    payload.epsilon_spent
                ))),
            ));
        }
    }

    // ── Validate batch size (must be ≥ 32 for meaningful DP guarantees) ───────
    if payload.record_count < 32 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(format!(
                "Batch size too small: got {}, minimum required is 32",
                payload.record_count
            ))),
        ));
    }

    // ── Validate gradient dimension (must match global model) ─────────────────
    {
        let global = state.global_model.read().await;
        if !payload.gradient.is_empty() && payload.gradient.len() != global.len() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(format!(
                    "Gradient dimension mismatch: got {}, expected {}",
                    payload.gradient.len(),
                    global.len()
                ))),
            ));
        }
    }

    let round = *state.current_round.read().await;
    info!(
        node_id = %node_id,
        round = round,
        loss = payload.loss,
        auc = payload.auc,
        epsilon = payload.epsilon_spent,
        records = payload.record_count,
        "Update received"
    );

    // ── Enqueue update ────────────────────────────────────────────────────────
    {
        let mut pending = state.pending_updates.write().await;
        pending.push((node_id, payload));
    }

    // ── Maybe trigger aggregation ─────────────────────────────────────────────
    let aggregation_triggered = match orchestrator::maybe_aggregate(Arc::clone(&state)).await {
        Ok(triggered) => triggered,
        Err(e) => {
            // Log but don't fail the request — update is already stored.
            tracing::error!("Aggregation error: {e}");
            false
        }
    };

    Ok(Json(UpdateResponse {
        status: "accepted".to_string(),
        aggregation_triggered,
    }))
}
