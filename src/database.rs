// src/database.rs

use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use crate::models::{User, Post};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    /// Expose the Arc<Mutex<Connection>> so managers can lock() on it.
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                username        TEXT UNIQUE NOT NULL,
                ssh_key         TEXT NOT NULL,
                bio             TEXT,
                created_at      TEXT NOT NULL
            )",
            [],
        )?;

        // Posts table: 'content' instead of 'body', and add 'updated_at'
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id         INTEGER NOT NULL,
                title           TEXT NOT NULL,
                content         TEXT NOT NULL,
                created_at      TEXT NOT NULL,
                updated_at      TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_user(&self, user: &User) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO users (username, ssh_key, bio, created_at) VALUES (?1, ?2, ?3, ?4)",
            (
                &user.username,
                &user.ssh_key,
                &user.bio,
                user.created_at.to_rfc3339(),
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, ssh_key, bio, created_at 
             FROM users 
             WHERE username = ?1"
        )?;
        
        let mut rows = stmt.query([username])?;
        if let Some(row) = rows.next()? {
            let u = User {
                id: Some(row.get(0)?),
                username: row.get(1)?,
                ssh_key: row.get(2)?,
                bio: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
            };
            Ok(Some(u))
        } else {
            Ok(None)
        }
    }

    pub fn create_post(&self, post: &Post) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO posts (user_id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                post.user_id,
                &post.title,
                &post.content,
                post.created_at.to_rfc3339(),
                post.updated_at.to_rfc3339(),
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_posts_by_user(&self, user_id: i64) -> Result<Vec<Post>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 
                p.id, 
                p.user_id, 
                p.title, 
                p.content, 
                p.created_at, 
                p.updated_at, 
                u.username 
             FROM posts p 
             JOIN users u ON p.user_id = u.id 
             WHERE p.user_id = ?1 
             ORDER BY p.created_at DESC"
        )?;
        
        let mut rows = stmt.query([user_id])?;
        let mut posts = Vec::new();
        while let Some(row) = rows.next()? {
            let post = Post {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                author_username: Some(row.get(6)?),
            };
            posts.push(post);
        }
        Ok(posts)
    }

    pub fn get_all_posts(&self) -> Result<Vec<Post>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 
                p.id, 
                p.user_id, 
                p.title, 
                p.content, 
                p.created_at, 
                p.updated_at, 
                u.username 
             FROM posts p 
             JOIN users u ON p.user_id = u.id 
             ORDER BY p.created_at DESC"
        )?;
        
        let mut rows = stmt.query([])?;
        let mut posts = Vec::new();
        while let Some(row) = rows.next()? {
            let post = Post {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                author_username: Some(row.get(6)?),
            };
            posts.push(post);
        }
        Ok(posts)
    }
}
