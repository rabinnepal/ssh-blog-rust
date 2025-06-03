// src/post.rs

use crate::models::Post;
use crate::database::Database;
use rusqlite::params;
use anyhow::{Error, Result};
use std::sync::Arc;
use std::sync::Mutex;

pub struct PostManager {
    db: Database,
}

impl PostManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn create_post(&self, post: &mut Post) -> Result<()> {
        let conn_arc: Arc<Mutex<rusqlite::Connection>> = self.db.get_connection();
        let conn = conn_arc.lock().unwrap();

        let mut stmt = conn.prepare(
            "INSERT INTO posts (user_id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)"
        )?;

        let id = stmt.insert(params![
            post.user_id,
            post.title,
            post.content,
            post.created_at.to_rfc3339(),
            post.updated_at.to_rfc3339()
        ])?;

        post.id = Some(id);
        Ok(())
    }

    pub fn get_user_posts(&self, user_id: i64) -> Result<Vec<Post>> {
        let conn_arc: Arc<Mutex<rusqlite::Connection>> = self.db.get_connection();
        let conn = conn_arc.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, user_id, title, content, created_at, updated_at 
             FROM posts 
             WHERE user_id = ?1 
             ORDER BY created_at DESC"
        )?;

        let rows = stmt.query_map(params![user_id], |row| {
            Ok(Post {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                author_username: None,
            })
        })?;

        let mut result = Vec::new();
        for post in rows {
            result.push(post?);
        }
        Ok(result)
    }

    pub fn get_all_posts(&self) -> Result<Vec<Post>> {
        let conn_arc: Arc<Mutex<rusqlite::Connection>> = self.db.get_connection();
        let conn = conn_arc.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, user_id, title, content, created_at, updated_at 
             FROM posts 
             ORDER BY created_at DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Post {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                author_username: None,
            })
        })?;

        let mut result = Vec::new();
        for post in rows {
            result.push(post?);
        }
        Ok(result)
    }
}
