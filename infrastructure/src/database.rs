// infrastructure/src/database

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use crate::config::AppConfig;

pub async fn create_database_pool(config: &AppConfig) -> Arc<sqlx::PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.url)
        .await
        .expect("Failed to create database pool.");
    Arc::new(pool)
}