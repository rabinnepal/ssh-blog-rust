// src/models.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub ssh_key: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, ssh_key: String, bio: Option<String>) -> Self {
        Self {
            id: None,
            username,
            ssh_key,
            bio,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i64>,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author_username: Option<String>, // Newly added field
}

impl Post {
    pub fn new(user_id: i64, title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            user_id,
            title,
            content,
            created_at: now,
            updated_at: now,
            author_username: None, // Default to None
        }
    }
}
