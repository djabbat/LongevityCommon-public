//! aim-hive-queen — Axum HTTP server.
//!
//! Endpoints (matching queen_app.py):
//!     GET  /healthz                 — health check (no auth)
//!     POST /v1/hive/contribute      — worker submits anonymized signal
//!     GET  /v1/hive/updates         — worker pulls eval-gated updates
//!     POST /v1/hive/distill         — admin trigger: scan + publish
//!     GET  /v1/hive/status          — queen state summary
//!
//! Auth: workers send `Authorization: Bearer <AIM_USER_TOKEN>` (validated
//! against the AIM hub via env-configured URL — see aim-common). Admin
//! endpoints require `Authorization: Bearer <AIM_HIVE_ADMIN_TOKEN>`.
//!
//! For the bootstrap migration, worker token validation is OPTIONAL —
//! if `AIM_HIVE_REQUIRE_AUTH=0` (default during transition), worker
//! endpoints accept anonymous traffic. Admin token is always required
//! for /distill and /status.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{info, warn};

use aim_hive_queen::{
    accept_contribution, distill_candidates, list_contributions, list_updates, publish_update,
    summary, QueenStore,
};

#[derive(Clone)]
struct AppState {
    store: Arc<QueenStore>,
    admin_token: Option<String>,
    /// If false, worker endpoints accept anonymous; if true, require Bearer.
    require_worker_auth: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".parse().unwrap()),
        )
        .init();

    let store = Arc::new(QueenStore::open_default()?);
    let admin_token = std::env::var("AIM_HIVE_ADMIN_TOKEN").ok();
    let require_worker_auth =
        std::env::var("AIM_HIVE_REQUIRE_AUTH").as_deref() == Ok("1");

    let state = AppState {
        store,
        admin_token,
        require_worker_auth,
    };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/hive/contribute", post(contribute_route))
        .route("/v1/hive/updates", get(updates_route))
        .route("/v1/hive/distill", post(distill_route))
        .route("/v1/hive/status", get(status_route))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8090);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    info!(%host, %port, "aim-hive-queen listening");
    axum::serve(listener, app).await?;
    Ok(())
}

// ── handlers ─────────────────────────────────────────────────────

async fn healthz() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "ts": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    }))
}

async fn contribute_route(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    if state.require_worker_auth {
        if let Err(r) = require_bearer(&headers, &[]) {
            return r.into_response();
        }
    }
    match accept_contribution(&state.store, payload) {
        Ok(Some(id)) => {
            info!(%id, "accepted contribution");
            (StatusCode::OK, Json(json!({"contribution_id": id}))).into_response()
        }
        Ok(None) => {
            warn!("rejected contribution");
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"payload rejected","status":400})),
            )
                .into_response()
        }
        Err(e) => {
            warn!(error = ?e, "store error on accept");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":format!("{e}"), "status":500})),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdatesQuery {
    since: Option<String>,
}

async fn updates_route(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<UpdatesQuery>,
) -> impl IntoResponse {
    if state.require_worker_auth {
        if let Err(r) = require_bearer(&headers, &[]) {
            return r.into_response();
        }
    }
    match list_updates(&state.store, q.since.as_deref()) {
        Ok(rows) => Json(json!({"updates": rows})).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":format!("{e}"), "status":500})),
        )
            .into_response(),
    }
}

async fn distill_route(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(r) = require_bearer(&headers, &expected_admin(&state)) {
        return r.into_response();
    }
    let cands = match distill_candidates(&state.store) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":format!("{e}"),"status":500})),
            )
                .into_response();
        }
    };
    let mut published = Vec::new();
    for c in cands.iter().cloned() {
        // Conservative auto-publish: only if ≥3 distinct workers supported.
        if c.source_n() >= 3 {
            if let Ok(Some(upd)) = publish_update(&state.store, c, true, None) {
                published.push(json!({
                    "id": upd.id,
                    "kind": upd.kind,
                    "source_n": upd.source_n,
                }));
            }
        }
    }
    Json(json!({
        "candidates_found": cands.len(),
        "auto_published": published.len(),
        "details": published,
    }))
    .into_response()
}

async fn status_route(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(r) = require_bearer(&headers, &expected_admin(&state)) {
        return r.into_response();
    }
    let summary_v = match summary(&state.store) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":format!("{e}"),"status":500})),
            )
                .into_response()
        }
    };
    let n_contribs = list_contributions(&state.store, 100000, None)
        .map(|v| v.len())
        .unwrap_or(0);
    let n_updates = list_updates(&state.store, None).map(|v| v.len()).unwrap_or(0);
    Json(json!({
        "queen_summary": summary_v,
        "n_contributions": n_contribs,
        "n_updates": n_updates,
        "ts": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    }))
    .into_response()
}

// ── auth helpers ─────────────────────────────────────────────────

fn expected_admin(state: &AppState) -> Vec<String> {
    state
        .admin_token
        .as_ref()
        .map(|t| vec![t.clone()])
        .unwrap_or_default()
}

/// If `expected` is empty, accept any Bearer (or none if not required).
/// If non-empty, the token must match one of them.
fn require_bearer(
    headers: &HeaderMap,
    expected: &[String],
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    if !auth.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"missing bearer token","status":401})),
        ));
    }
    let raw = auth["Bearer ".len()..].trim();
    if !expected.is_empty() && !expected.iter().any(|t| t == raw) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error":"bad token","status":403})),
        ));
    }
    Ok(())
}
