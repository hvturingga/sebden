//api-server/src/main.rs
mod router;

use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use crate::router::app_router;
use infrastructure::config::AppConfig;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use infrastructure::database::create_database_pool;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Loading configuration...");
    let config = AppConfig::load_or_init().context("Failed to load or initialize configuration")?;

    let new_pool = create_database_pool(&config).await.expect("Failed to create database pool.");
    let pool = Arc::new(new_pool);

    let app = app_router(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    let listener = tokio::net::TcpListener::bind(&addr).await.context("Failed to bind to address")?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await.context("Server failed")?;

    Ok(())
}
