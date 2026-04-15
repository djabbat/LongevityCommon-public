use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{db, models::{AuditEntry, ErrorResponse}, state::AppState};

/// GET /api/audit
///
/// Return the full hash-chain audit log ordered by round_number ascending.
/// Each entry's `prev_hash` must equal the previous entry's `entry_hash`
/// (genesis: prev_hash = '0' × 64).  Consumers can verify chain integrity
/// by recomputing entry_hash = SHA-256(round_id ‖ round_number ‖ gradient_hash ‖ prev_hash).
pub async fn audit_chain(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AuditEntry>>, (StatusCode, Json<ErrorResponse>)> {
    match db::get_audit_chain(&state.pool).await {
        Ok(chain) => Ok(Json(chain)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("DB error: {e}"))),
        )),
    }
}
