// application/src/factories.rs

use std::sync::Arc;
use domain::user::repository::UserRepository;
use crate::user_service::UserService;
use infrastructure::repositories::user_repository_impl::UserRepositoryImpl;
use sqlx::PgPool;

pub fn create_user_service(pool: Arc<PgPool>) -> Arc<UserService> {

    let user_repository: Arc<dyn UserRepository + Send + Sync> = Arc::new(UserRepositoryImpl::new(pool));
    Arc::new(UserService::new(user_repository))
}