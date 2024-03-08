// domain/src/user/repository.rs

use crate::user::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self) -> Vec<User>;
    async fn get_user(&self, id: u64) -> Option<User>;
    async fn create_user(&self, user: User) -> User;
}