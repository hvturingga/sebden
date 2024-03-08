// adapters/src/http/handler.rs

use axum::{extract::Extension, response::IntoResponse};
use async_trait::async_trait;

#[async_trait]
pub trait Handler<S, A> {
    type Response: IntoResponse;
    async fn handle(service: Extension<S>, args: A) -> Self::Response;
}