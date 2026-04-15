use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::Response,
    Json,
};

use crate::{
    db,
    models::{ErrorResponse, MetricsResponse},
    state::AppState,
};

/// GET /api/metrics
///
/// Return dashboard metrics: current round, node count, AUC history, and
/// average Shapley contribution score across all nodes and rounds.
pub async fn metrics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<MetricsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let current_round = *state.current_round.read().await;

    // Node count from DB
    let nodes = match db::list_nodes(&state.pool).await {
        Ok(n) => n,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(format!("DB error: {e}"))),
            ))
        }
    };
    let node_count = nodes.len();

    // AUC history from in-memory round history (fast read, no DB query needed)
    let auc_history: Vec<f64> = {
        let history = state.round_history.read().await;
        history.iter().map(|r| r.auc).collect()
    };

    // Average Shapley score from DB
    let avg_shapley = match db::avg_shapley(&state.pool).await {
        Ok(v) => v,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(format!("DB error: {e}"))),
            ))
        }
    };

    // Compute Rényi DP savings vs. linear accounting across all active nodes.
    let (rdp_epsilon_savings, nodes_near_budget) = {
        let budgets = state.node_budgets.read().await;
        let savings: f64 = budgets.values()
            .map(|s| (s.epsilon_linear - s.effective_epsilon()).max(0.0))
            .sum();
        let near_budget = budgets.values()
            .filter(|s| s.effective_epsilon() > crate::state::EPSILON_TOTAL * 0.8)
            .count();
        (savings, near_budget)
    };

    Ok(Json(MetricsResponse {
        current_round,
        node_count,
        auc_history,
        avg_shapley,
        rdp_epsilon_savings,
        nodes_near_budget,
    }))
}

/// GET /metrics
///
/// Prometheus-compatible plain-text metrics endpoint.
/// Content-Type: text/plain; version=0.0.4; charset=utf-8
///
/// Exposed metrics:
///   fclc_rounds_total   — counter: total completed FL rounds
///   fclc_active_nodes   — gauge:   number of registered nodes
///   fclc_auc_latest     — gauge:   AUC of the most recent round (0 if no rounds yet)
///   fclc_avg_shapley    — gauge:   average Shapley contribution score
pub async fn prometheus_metrics(
    State(state): State<Arc<AppState>>,
) -> Result<Response<String>, (StatusCode, Json<ErrorResponse>)> {
    let current_round = *state.current_round.read().await;

    let node_count = match db::list_nodes(&state.pool).await {
        Ok(n) => n.len(),
        Err(e) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    };

    let auc_latest: f64 = {
        let history = state.round_history.read().await;
        history.last().map(|r| r.auc).unwrap_or(0.0)
    };

    let avg_shapley = match db::avg_shapley(&state.pool).await {
        Ok(v) => v,
        Err(e) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    };

    let (rdp_savings, nodes_near_budget) = {
        let budgets = state.node_budgets.read().await;
        let savings: f64 = budgets.values()
            .map(|s| (s.epsilon_linear - s.effective_epsilon()).max(0.0))
            .sum();
        let near = budgets.values()
            .filter(|s| s.effective_epsilon() > crate::state::EPSILON_TOTAL * 0.8)
            .count();
        (savings, near)
    };

    let body = format!(
        "# HELP fclc_rounds_total Total number of completed federated learning rounds\n\
         # TYPE fclc_rounds_total counter\n\
         fclc_rounds_total {current_round}\n\
         # HELP fclc_active_nodes Number of registered clinic nodes\n\
         # TYPE fclc_active_nodes gauge\n\
         fclc_active_nodes {node_count}\n\
         # HELP fclc_auc_latest AUC of the most recently completed round\n\
         # TYPE fclc_auc_latest gauge\n\
         fclc_auc_latest {auc_latest:.6}\n\
         # HELP fclc_avg_shapley Average Shapley contribution score across all nodes\n\
         # TYPE fclc_avg_shapley gauge\n\
         fclc_avg_shapley {avg_shapley:.6}\n\
         # HELP fclc_rdp_epsilon_savings Total epsilon saved by Renyi DP vs linear accounting\n\
         # TYPE fclc_rdp_epsilon_savings gauge\n\
         fclc_rdp_epsilon_savings {rdp_savings:.6}\n\
         # HELP fclc_nodes_near_budget Nodes with >80% of DP budget consumed\n\
         # TYPE fclc_nodes_near_budget gauge\n\
         fclc_nodes_near_budget {nodes_near_budget}\n"
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )
        .body(body)
        .unwrap())
}
