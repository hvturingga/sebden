// domain/src/user/user.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
}

impl User {
    pub fn new(id: u64, username: &str, email: &str, password_hash: &str) -> Self {
        User {
            id,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            bio: None,
        }
    }
}