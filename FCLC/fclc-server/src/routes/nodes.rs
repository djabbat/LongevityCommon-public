use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    db,
    models::{ErrorResponse, NodeInfo, NodeScore, RegisterRequest, RegisterResponse},
    state::AppState,
};

/// POST /api/nodes/register
///
/// Register a new node (or re-register an existing one).
/// Returns the assigned `node_id`.
pub async fn register_node(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(node_id = %req.node_id, name = %req.node_name, "Node registration request");

    if let Err(e) = db::insert_node(&state.pool, req.node_id, &req.node_name).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        ));
    }

    // Ensure budget tracker has an entry for this node (Rényi DP accountant).
    {
        use crate::state::NodeDpState;
        let mut budgets = state.node_budgets.write().await;
        budgets.entry(req.node_id).or_insert_with(NodeDpState::new);
    }

    Ok(Json(RegisterResponse {
        node_id: req.node_id,
        status: "registered".to_string(),
    }))
}

/// GET /api/nodes
///
/// List all registered nodes with their current epsilon expenditure.
pub async fn list_nodes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<NodeInfo>>, (StatusCode, Json<ErrorResponse>)> {
    match db::list_nodes(&state.pool).await {
        Ok(nodes) => Ok(Json(nodes)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    }
}

/// GET /api/nodes/:node_id/score
///
/// Return Shapley score history for the given node.
pub async fn node_score(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<Uuid>,
) -> Result<Json<Vec<NodeScore>>, (StatusCode, Json<ErrorResponse>)> {
    match db::get_shapley_history(&state.pool, node_id).await {
        Ok(scores) => Ok(Json(scores)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    }
}
