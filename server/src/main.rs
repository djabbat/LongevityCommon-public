use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// All modules are defined in lib.rs and re-exported.
// main.rs only contains the binary entry point.
use longevitycommon_server::{AppState, AppConfig, db, routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "longevitycommon_server=debug,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let config = AppConfig::from_env()?;

    let db = db::connect(&config.database_url).await?;
    db::run_migrations(&db).await?;
    tracing::info!("Database connected and migrations applied");

    let cors = build_cors(&config.allowed_origins);

    let state = AppState { db, config: config.clone() };

    let app = routes::all_routes(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", config.app_host, config.app_port).parse()?;
    tracing::info!("LongevityCommon API listening on {}", addr);
    tracing::info!("Allowed origins: {:?}", config.allowed_origins);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_cors(origins: &[String]) -> CorsLayer {
    use axum::http::{HeaderValue, Method, header};

    let allowed: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| HeaderValue::from_str(o).ok())
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true)
}
