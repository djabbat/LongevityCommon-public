mod db;
mod models;
mod orchestrator;
mod routes;
mod state;

use std::sync::Arc;

use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use state::AppState;

/// Bearer token auth middleware.
/// Reads FCLC_API_TOKEN env var; if set, rejects requests without matching Authorization header.
async fn auth_middleware(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    if let Ok(expected) = std::env::var("FCLC_API_TOKEN") {
        if !expected.is_empty() {
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "));
            if token != Some(expected.as_str()) {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── Tracing ───────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("fclc_server=info,tower_http=debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── Configuration ─────────────────────────────────────────────────────────
    // Load .env file if present (silently ignore if missing).
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://fclc:fclc@localhost:5432/fclc".to_string()
    });
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // ── Database pool ─────────────────────────────────────────────────────────
    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    // ── Security check ────────────────────────────────────────────────────────
    match std::env::var("FCLC_API_TOKEN") {
        Ok(t) if !t.is_empty() => info!("API token authentication enabled"),
        _ => tracing::warn!(
            "FCLC_API_TOKEN not set — API is unauthenticated! \
             Set this variable before production deployment."
        ),
    }

    // ── Application state ─────────────────────────────────────────────────────
    let app_state = Arc::new(AppState::new(pool));

    // ── Router ────────────────────────────────────────────────────────────────
    let app = Router::new()
        // Node management
        .route("/api/nodes/register", post(routes::nodes::register_node))
        .route("/api/nodes", get(routes::nodes::list_nodes))
        .route("/api/nodes/:node_id/score", get(routes::nodes::node_score))
        // Update collection
        .route(
            "/api/nodes/:node_id/update",
            post(routes::updates::submit_update),
        )
        // Global model
        .route("/api/model/current", get(routes::model::current_model))
        // Round management
        .route("/api/rounds", get(routes::rounds::list_rounds))
        .route("/api/rounds/trigger", post(routes::rounds::trigger_round))
        .route("/api/rounds/:round_id", get(routes::rounds::get_round))
        // Dashboard metrics (JSON) + Prometheus scrape endpoint
        .route("/api/metrics", get(routes::metrics::metrics))
        .route("/metrics", get(routes::metrics::prometheus_metrics))
        // Audit chain
        .route("/api/audit", get(routes::audit::audit_chain))
        // Middleware
        .layer(middleware::from_fn(auth_middleware))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // ── Listen ────────────────────────────────────────────────────────────────
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("FCLC Server listening on {bind_addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
