// Library entry point — exposes modules for integration tests.
// Binary entry point is src/main.rs.

pub mod config;
pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

pub use config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: AppConfig,
}
