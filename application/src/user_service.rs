// application/src/user_service.rs

use domain::user::user::User;
use domain::user::repository::UserRepository;
use std::sync::Arc;

pub struct UserService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        UserService { user_repository }
    }

    pub async fn get_users(&self) -> Vec<User> {
        self.user_repository.get_users().await
    }

    pub async fn get_user(&self, id: u64) -> Option<User> {
        self.user_repository.get_user(id).await
    }

    pub async fn create_user(&self, username: &str, email: &str) -> User {
        let user = User {
            id: 0, // Assume ID is auto-generated by the database
            username: username.to_string(),
            email: email.to_string(),
            password_hash: "".to_string(),
            bio: None,
        };
        self.user_repository.create_user(user).await
    }
}