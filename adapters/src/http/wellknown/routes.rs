// adapters/src/http/wellknown/routes.rs

use axum::Router;
use axum::routing::get;
use crate::http::wellknown::handlers::get_version;

pub fn wellknown_routes() -> Router {
    Router::new().route("/version", get(get_version))
}