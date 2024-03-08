// api-server/src/router.rs

use std::sync::Arc;
use axum::Router;
use adapters::http::user::routes::user_routes;
use adapters::http::wellknown::routes::wellknown_routes;
use application::create_instance;
use sqlx;

pub fn app_router(pool: Arc<sqlx::PgPool>) -> Router {

    let user_service = create_instance!(create_user_service, pool);

    Router::new()
        .nest("/wellknown", wellknown_routes())
        .nest("/users", user_routes(user_service))
}

