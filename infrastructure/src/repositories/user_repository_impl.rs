// infrastructure/src/repositories/user_repository_impl.rs

use async_trait::async_trait;
use domain::user::user::User;
use domain::user::repository::UserRepository;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UserRepositoryImpl {
    pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserRepositoryImpl { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_users(&self) -> Vec<User> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&*self.pool)
            .await
            .unwrap_or_else(|_| vec![]);
        users
    }

    async fn get_user(&self, id: u64) -> Option<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&*self.pool)
            .await
            .ok()
            .flatten();
        user
    }

    async fn create_user(&self, mut user: User) -> User {
        user.id = sqlx::query!(
            "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id",
            user.username,
            user.email
        )
            .fetch_one(&*self.pool)
            .await
            .map(|rec| rec.id)
            .unwrap_or_default();
        user
    }
}