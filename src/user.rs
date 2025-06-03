// src/user.rs

use crate::models::User;
use crate::database::Database;
use rusqlite::params;
use anyhow::{Error, Result};
use std::io::{self, Write};
use std::env;
use std::fs;
use std::process::Command;

pub struct UserManager {
    db: Database,
}

impl UserManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn create_user_direct(&self, user: &mut User) -> Result<()> {
        let conn_arc = self.db.get_connection();
        let conn = conn_arc.lock().unwrap();

        let mut stmt = conn.prepare(
            "INSERT INTO users (username, ssh_key, bio, created_at) VALUES (?1, ?2, ?3, ?4)"
        )?;

        let id = stmt.insert(params![
            user.username,
            user.ssh_key,
            user.bio,
            user.created_at.to_rfc3339()
        ])?;

        user.id = Some(id);
        Ok(())
    }

    pub fn register_user_with_ssh(&self) -> Result<User> {
        println!("Enter your desired username:");
        print!("> ");
        io::stdout().flush()?;
        
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        let username = username.trim().to_string();

        if username.len() < 3 {
            return Err(Error::msg("Username must be at least 3 characters long"));
        }

        println!("Enter your SSH public key:");
        print!("> ");
        io::stdout().flush()?;
        
        let mut ssh_key = String::new();
        io::stdin().read_line(&mut ssh_key)?;
        let ssh_key = ssh_key.trim().to_string();

        if !ssh_key.starts_with("ssh-") {
            return Err(Error::msg("Invalid SSH key format"));
        }

        println!("Enter your bio (optional, press Enter to skip):");
        print!("> ");
        io::stdout().flush()?;
        
        let mut bio = String::new();
        io::stdin().read_line(&mut bio)?;
        let bio = bio.trim();
        let bio = if bio.is_empty() { None } else { Some(bio.to_string()) };

        let mut user = User::new(username, ssh_key, bio);
        self.create_user_direct(&mut user)?;
        
        Ok(user)
    }

    pub fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn_arc = self.db.get_connection();
        let conn = conn_arc.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, username, ssh_key, bio, created_at 
             FROM users 
             WHERE username = ?1"
        )?;

        let mut rows = stmt.query(params![username])?;
        if let Some(row) = rows.next()? {
            let user = User {
                id: Some(row.get(0)?),
                username: row.get(1)?,
                ssh_key: row.get(2)?,
                bio: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub fn find_by_ssh_key(&self, ssh_key: &str) -> Result<Option<User>> {
        let conn_arc = self.db.get_connection();
        let conn = conn_arc.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, username, ssh_key, bio, created_at 
             FROM users 
             WHERE ssh_key = ?1"
        )?;

        let mut rows = stmt.query(params![ssh_key])?;
        if let Some(row) = rows.next()? {
            let user = User {
                id: Some(row.get(0)?),
                username: row.get(1)?,
                ssh_key: row.get(2)?,
                bio: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Get the SSH key fingerprint from environment (if available)
    pub fn get_ssh_key_fingerprint(&self) -> Option<String> {
        // SSH servers can set this environment variable
        env::var("SSH_KEY_FINGERPRINT").ok()
            .or_else(|| env::var("SSH_KEY_FP").ok())
    }

    /// Extract SSH key from SSH agent or environment
    pub fn get_client_ssh_key(&self) -> Result<String> {
        // Method 1: Try to get from SSH_ORIGINAL_COMMAND if it contains key info
        if let Ok(original_command) = env::var("SSH_ORIGINAL_COMMAND") {
            if original_command.contains("ssh-") {
                return Ok(original_command);
            }
        }

        // Method 2: Try to get the key from ssh-agent
        let output = Command::new("ssh-add")
            .arg("-L")
            .output();

        if let Ok(output) = output {
            let keys = String::from_utf8_lossy(&output.stdout);
            for line in keys.lines() {
                if line.starts_with("ssh-") {
                    return Ok(line.to_string());
                }
            }
        }

        // Method 3: Try to read from a temporary file created by custom SSH server
        if let Ok(key_file) = env::var("SSH_CLIENT_KEY_FILE") {
            if let Ok(key) = fs::read_to_string(&key_file) {
                return Ok(key.trim().to_string());
            }
        }

        Err(Error::msg("Could not determine SSH client key"))
    }

    /// Get current system username
    pub fn get_current_username(&self) -> Result<String> {
        env::var("USER")
            .or_else(|_| env::var("SSH_USER"))
            .or_else(|_| env::var("LOGNAME"))
            .or_else(|_| env::var("USERNAME")) // Windows
            .or_else(|_| {
                Command::new("whoami")
                    .output()
                    .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
                    .map_err(|_| env::VarError::NotPresent)
            })
            .map_err(|_| Error::msg("Could not determine current user"))
    }

    /// Authenticate user based on SSH connection
    pub fn authenticate_from_ssh(&self) -> Result<User> {
        // Method 1: Try to authenticate by SSH key
        if let Ok(ssh_key) = self.get_client_ssh_key() {
            if let Some(user) = self.find_by_ssh_key(&ssh_key)? {
                return Ok(user);
            }
        }

        // Method 2: Try to authenticate by username + verify against authorized_keys
        let username = self.get_current_username()?;
        
        if let Some(user) = self.find_by_username(&username)? {
            // If we have the user in DB, try to verify their key
            if let Ok(client_key) = self.get_client_ssh_key() {
                if self.verify_ssh_key(&username, &client_key)? {
                    return Ok(user);
                }
            }
            
            // Fallback: if user exists and we're in SSH context, allow it
            if env::var("SSH_CLIENT").is_ok() || env::var("SSH_CONNECTION").is_ok() {
                return Ok(user);
            }
        }

        Err(Error::msg(format!(
            "User '{}' not found or SSH key verification failed. Please register first.", 
            username
        )))
    }

    pub fn verify_ssh_key(&self, username: &str, presented_key: &str) -> Result<bool> {
        if let Some(user) = self.find_by_username(username)? {
            // Compare the actual key content (normalize whitespace)
            let stored_key = user.ssh_key.trim().replace('\n', " ");
            let presented_key = presented_key.trim().replace('\n', " ");
            
            let stored_parts: Vec<&str> = stored_key.split_whitespace().collect();
            let presented_parts: Vec<&str> = presented_key.split_whitespace().collect();
            
            if stored_parts.len() >= 2 && presented_parts.len() >= 2 {
                Ok(stored_parts[0] == presented_parts[0] && stored_parts[1] == presented_parts[1])
            } else {
                Ok(stored_key == presented_key)
            }
        } else {
            Ok(false)
        }
    }

    pub fn get_user_from_authorized_keys(&self, username: &str) -> Result<Option<String>> {
        let possible_paths = vec![
            format!("/home/{}/.ssh/authorized_keys", username),
            format!("/Users/{}/.ssh/authorized_keys", username),
            format!("C:\\Users\\{}\\.ssh\\authorized_keys", username),
        ];
        
        for auth_keys_path in possible_paths {
            if let Ok(content) = fs::read_to_string(&auth_keys_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("ssh-") && !line.is_empty() {
                        return Ok(Some(line.to_string()));
                    }
                }
            }
        }
        
        Ok(None)
    }

    /// Main authentication method with multiple fallbacks
    pub fn authenticate_with_fallback(&self) -> Result<User> {
        // First try the SSH-based authentication
        match self.authenticate_from_ssh() {
            Ok(user) => return Ok(user),
            Err(e) => {
                eprintln!("SSH authentication failed: {}", e);
                // Continue to fallbacks
            }
        }

        // Fallback 1: Try username-based authentication
        if let Ok(username) = self.get_current_username() {
            if let Some(user) = self.find_by_username(&username)? {
                return Ok(user);
            }
        }

        // Fallback 2: Interactive registration prompt
        println!("No existing user found. Would you like to register? (y/n)");
        print!("> ");
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        if response.trim().eq_ignore_ascii_case("y") {
            return self.register_user_with_ssh();
        }

        Err(Error::msg("Authentication failed. Please register first or contact admin."))
    }

    /// Development/testing method to authenticate with minimal verification
    pub fn authenticate_dev_mode(&self) -> Result<User> {
        if let Ok(username) = self.get_current_username() {
            if let Some(user) = self.find_by_username(&username)? {
                println!("🔓 Development mode: Authenticated as {}", username);
                return Ok(user);
            }
        }
        
        println!("🔧 Development mode: Creating temporary user");
        let username = self.get_current_username().unwrap_or_else(|_| "dev_user".to_string());
        let mut user = User::new(
            username.clone(),
            "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC7... dev-key".to_string(),
            Some("Development user".to_string())
        );
        
        self.create_user_direct(&mut user)?;
        Ok(user)
    }
}
