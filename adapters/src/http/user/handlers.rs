// adapters/src/http/user/handlers.rs

use application::user_service::UserService;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use domain::user::user::User;
use serde_json::json;
use std::sync::Arc;
use crate::http::user::models::UserResponse;

pub async fn get_users(
    Extension(user_service): Extension<Arc<UserService>>,
) -> impl IntoResponse {
    let users = user_service.get_users().await;
    let users_response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    Json(users_response)
}

pub async fn get_user(
    Path(id): Path<u64>,
    Extension(user_service): Extension<Arc<UserService>>,
) -> impl IntoResponse {
    match user_service.get_user(id).await {
        Some(user) => {
            let user_response = UserResponse::from(user);
            (StatusCode::OK, Json(user_response)).into_response()
        },
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        )
            .into_response(),
    }
}

pub async fn create_user(
    Extension(user_service): Extension<Arc<UserService>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let user = user_service
        .create_user(&payload.username, &payload.email)
        .await;
    let user_response = UserResponse::from(user);
    (StatusCode::CREATED, Json(user_response))
}