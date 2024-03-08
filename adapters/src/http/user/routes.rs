// adapters/src/http/user/routes.rs

use crate::http::user::handlers::{get_users, get_user, create_user};
use axum::{
    routing::get,
    Router,
    Extension,
};
use std::sync::Arc;
use application::user_service::UserService;

pub fn user_routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user))
        .layer(Extension(user_service))
}