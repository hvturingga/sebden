use application::wellknown_service::VersionService;
use axum::Json;
use domain::wellknown::version::Version;

pub async fn get_version() -> Json<Version> {
    let version = VersionService::get_version();
    Json(version)
}
