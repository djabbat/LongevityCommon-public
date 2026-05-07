use axum::{
    middleware,
    response::Json as JsonResp,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers::{admin, auth, biosense, dashboard, data, disclosures, posts, studies, users, ze_guide},
    middleware::{
        auth::require_auth,
        rate_limit::{self, rate_limit_fn},
    },
    AppState,
};

/// Liveness probe. Mounted at both `/health` (legacy) and `/api/health`
/// (so nginx can probe through the standard /api/ prefix).
async fn health() -> JsonResp<serde_json::Value> {
    JsonResp(serde_json::json!({
        "status": "ok",
        "service": "longevitycommon-server",
        "ts": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    }))
}

/// Build/version metadata for ops visibility. Returns CARGO_PKG_VERSION
/// and the git short-SHA injected at build time via env (or "dev" if
/// not provided).
async fn version() -> JsonResp<serde_json::Value> {
    JsonResp(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "git_sha": option_env!("GIT_SHA").unwrap_or("dev"),
        "build_ts": option_env!("BUILD_TS").unwrap_or("unknown"),
    }))
}

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .merge(public_routes())
        .merge(auth_routes())
        .merge(protected_routes(state.clone()))
        .with_state(state)
}

/// Public routes — read-only, general API rate limit
fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/api/users/:id", get(users::get_user_profile))
        .route("/api/users/by-username/:username", get(users::get_user_by_username))
        .route("/api/feed", get(posts::get_feed))
        .route("/api/studies", get(studies::list_studies))
        .route("/api/studies/:id", get(studies::get_study))
        .route("/api/biosense/compute", post(biosense::compute_chi_ze))
        .route("/api/disclosures/v5_changes", get(disclosures::get_v5_changes))
        .route("/health", get(health))
        .route("/api/health", get(health))
        .route("/api/version", get(version))
        .route_layer(middleware::from_fn_with_state(
            rate_limit::api_limiter(),
            rate_limit_fn,
        ))
}

/// Auth routes — strict rate limit (5 req/min)
fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/verify-otp", post(auth::verify_otp))
        .route_layer(middleware::from_fn_with_state(
            rate_limit::auth_limiter(),
            rate_limit_fn,
        ))
}

/// Protected routes — require JWT, per-route rate limits
fn protected_routes(state: AppState) -> Router<AppState> {
    let ze_guide_router = Router::new()
        .route("/api/ze-guide/ask", post(ze_guide::ask_ze_guide))
        .route("/api/ze-guide/history", get(ze_guide::get_ze_guide_history))
        .route_layer(middleware::from_fn_with_state(
            rate_limit::ze_guide_limiter(),
            rate_limit_fn,
        ));

    let general_router = Router::new()
        .route("/api/users/me", patch(users::update_profile))
        .route("/api/users/me", delete(users::delete_account))
        .route("/api/posts", post(posts::create_post))
        .route("/api/posts/:id", delete(posts::delete_post))
        .route("/api/posts/:id/react", post(posts::react_to_post))
        .route("/api/dashboard", get(dashboard::get_dashboard))
        .route("/api/dashboard/trend", get(dashboard::get_trend))
        .route("/api/interventions", post(dashboard::create_intervention))
        .route("/api/health-factors", post(dashboard::create_health_factor))
        .route("/api/data/import", post(data::import_data))
        .route("/api/data/export", get(data::export_data))
        .route("/api/studies", post(studies::create_study))
        .route("/api/studies/:id/join", post(studies::join_study))
        .route("/api/studies/:id/leave", delete(studies::leave_study))
        .route("/api/admin/stats", get(admin::get_stats))
        .route_layer(middleware::from_fn_with_state(
            rate_limit::api_limiter(),
            rate_limit_fn,
        ));

    Router::new()
        .merge(ze_guide_router)
        .merge(general_router)
        .route_layer(middleware::from_fn_with_state(state, require_auth))
}
