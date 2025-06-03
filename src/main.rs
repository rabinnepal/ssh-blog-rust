// src/main.rs

use std::env;
use crate::models::{User, Post};
use crate::database::Database;
use crate::user::UserManager;
use crate::post::PostManager;
use crate::cli::CLI;

mod models;
mod database;
mod user;
mod post;
mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Handle different command line flags
    if args.len() > 1 {
        match args[1].as_str() {
            "--register" => {
                handle_registration();
                return;
            }
            "--register-user" => {
                if args.len() >= 4 {
                    handle_user_registration(&args[2], &args[3], args.get(4));
                } else {
                    eprintln!("Usage: ssh-blog --register-user <username> <ssh_key> [bio]");
                    std::process::exit(1);
                }
                return;
            }
            "--init-db" => {
                handle_db_init();
                return;
            }
            _ => {}
        }
    }

    let db = Database::new("/var/lib/ssh-blog/blog.db").expect("Failed to initialize database");
    let user_manager = UserManager::new(db.clone());
    let post_manager = PostManager::new(db.clone());
    let mut cli = CLI::new(user_manager, post_manager);

    println!("🚀 Welcome to SSH Blog Platform!");
    println!("Your terminal-based blogging experience starts here.\n");

    // Authenticate user based on SSH connection
    let current_user = match cli.authenticate_user() {
        Ok(user) => user,
        Err(e) => {
            eprintln!("❌ Authentication failed: {}", e);
            eprintln!("💡 If this is your first time, contact admin to register your account");
            return;
        }
    };

    println!("Welcome back, {}!", current_user.username);
    if let Some(bio) = &current_user.bio {
        println!("Bio: {}", bio);
    }

    cli.run_main_loop(current_user);
}

fn handle_registration() {
    println!("🔐 SSH Blog Registration");
    println!("Setting up your account...\n");

    let db = Database::new("/var/lib/ssh-blog/blog.db").expect("Failed to initialize database");
    let user_manager = UserManager::new(db);

    match user_manager.register_user_with_ssh() {
        Ok(user) => {
            println!("\n✅ Registration successful!");
            println!("Username: {}", user.username);
            if let Some(bio) = &user.bio {
                println!("Bio: {}", bio);
            }
            println!("\n🔑 Your SSH public key has been registered.");
            println!("You can now connect using: ssh {}@yourserver -p 2222", user.username);
        }
        Err(e) => {
            eprintln!("❌ Registration failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_user_registration(username: &str, ssh_key: &str, bio: Option<&String>) {
    let db = Database::new("/var/lib/ssh-blog/blog.db").expect("Failed to initialize database");
    let user_manager = UserManager::new(db);
    
    let bio_str = bio.map(|s| s.clone());
    let mut user = User::new(username.to_string(), ssh_key.to_string(), bio_str);
    
    match user_manager.create_user_direct(&mut user) {
        Ok(_) => {
            println!("User {} registered successfully", username);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to register user: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_db_init() {
    match Database::new("/var/lib/ssh-blog/blog.db") {
        Ok(_) => {
            println!("Database initialized successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    }
}
