// src/cli.rs

use crate::models::{User, Post};
use crate::user::UserManager;
use crate::post::PostManager;
use anyhow::Result;
use std::io::{self, Write};

pub struct CLI {
    user_manager: UserManager,
    post_manager: PostManager,
}

impl CLI {
    pub fn new(user_manager: UserManager, post_manager: PostManager) -> Self {
        Self {
            user_manager,
            post_manager,
        }
    }

    pub fn authenticate_user(&self) -> Result<User> {
        // Try the improved authentication with fallback
        self.user_manager.authenticate_with_fallback()
    }

    pub fn run_main_loop(&mut self, current_user: User) {
        loop {
            println!("\n📝 SSH Blog Platform - Welcome {}!", current_user.username);
            println!("1. Create new post");
            println!("2. View my posts");
            println!("3. View all posts");
            println!("4. Profile info");
            println!("5. Exit");
            print!("Choose an option (1-5): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }

            match input.trim() {
                "1" => self.create_post(&current_user),
                "2" => self.view_user_posts(&current_user),
                "3" => self.view_all_posts(),
                "4" => self.show_profile(&current_user),
                "5" => {
                    println!("Thanks for using SSH Blog Platform! Goodbye!");
                    break;
                }
                _ => println!("❌ Invalid option. Please choose 1-5."),
            }
        }
    }

    fn create_post(&mut self, user: &User) {
        println!("\n✍️  Create New Post");
        println!("{}", "=".repeat(40));
        print!("Title: ");
        io::stdout().flush().unwrap();

        let mut title = String::new();
        if io::stdin().read_line(&mut title).is_err() {
            println!("❌ Error reading title");
            return;
        }
        let title = title.trim().to_string();

        if title.is_empty() {
            println!("❌ Title cannot be empty");
            return;
        }

        println!("\nContent (end with a line containing only '.'):");
        println!("{}", "-".repeat(40));
        let mut content = String::new();
        let mut line_count = 0;
        
        loop {
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                break;
            }
            if line.trim() == "." {
                break;
            }
            content.push_str(&line);
            line_count += 1;
            
            // Show progress for longer posts
            if line_count % 10 == 0 {
                println!("({} lines written...)", line_count);
            }
        }

        if content.trim().is_empty() {
            println!("❌ Content cannot be empty");
            return;
        }

        let mut post = Post::new(user.id.unwrap(), title.clone(), content.trim().to_string());
        
        match self.post_manager.create_post(&mut post) {
            Ok(_) => {
                println!("✅ Post '{}' created successfully!", title);
                println!("📊 Post ID: {}", post.id.unwrap_or(0));
            }
            Err(e) => println!("❌ Error creating post: {}", e),
        }
    }

    fn view_user_posts(&self, user: &User) {
        println!("\n📚 Your Posts");
        println!("{}", "=".repeat(50));
        
        match self.post_manager.get_user_posts(user.id.unwrap()) {
            Ok(posts) => {
                if posts.is_empty() {
                    println!("📝 No posts yet. Create your first post!");
                    println!("💡 Choose option 1 from the main menu to get started.");
                } else {
                    println!("📊 Found {} post(s)", posts.len());
                    for (index, post) in posts.iter().enumerate() {
                        println!("\n📄 Post #{}", index + 1);
                        self.display_post(post);
                    }
                }
            }
            Err(e) => println!("❌ Error fetching posts: {}", e),
        }
    }

    fn view_all_posts(&self) {
        println!("\n🌍 All Posts");
        println!("{}", "=".repeat(50));
        
        match self.post_manager.get_all_posts() {
            Ok(posts) => {
                if posts.is_empty() {
                    println!("📝 No posts available on the platform yet.");
                    println!("🚀 Be the first to create a post!");
                } else {
                    println!("📊 Found {} post(s) on the platform", posts.len());
                    for (index, post) in posts.iter().enumerate() {
                        println!("\n📄 Post #{}", index + 1);
                        self.display_post_with_author(post);
                    }
                }
            }
            Err(e) => println!("❌ Error fetching posts: {}", e),
        }
    }

    fn show_profile(&self, user: &User) {
        println!("\n👤 Profile Information");
        println!("{}", "=".repeat(40));
        println!("Username: {}", user.username);
        println!("User ID: {}", user.id.unwrap_or(0));
        println!("Joined: {}", user.created_at.format("%Y-%m-%d %H:%M UTC"));
        
        if let Some(bio) = &user.bio {
            println!("Bio: {}", bio);
        } else {
            println!("Bio: (not set)");
        }

        // Show post count
        match self.post_manager.get_user_posts(user.id.unwrap()) {
            Ok(posts) => {
                println!("Total posts: {}", posts.len());
            }
            Err(_) => {
                println!("Total posts: (error fetching)");
            }
        }
    }

    fn display_post(&self, post: &Post) {
        println!("{}", "─".repeat(50));
        println!("📝 {}", post.title);
        println!("📅 Created: {}", post.created_at.format("%Y-%m-%d %H:%M UTC"));
        if post.updated_at != post.created_at {
            println!("📝 Updated: {}", post.updated_at.format("%Y-%m-%d %H:%M UTC"));
        }
        println!("{}", "─".repeat(50));
        
        // Display content with line numbers for longer posts
        let lines: Vec<&str> = post.content.lines().collect();
        if lines.len() > 20 {
            for (i, line) in lines.iter().enumerate() {
                println!("{:3}: {}", i + 1, line);
            }
        } else {
            println!("{}", post.content);
        }
        
        println!("{}", "─".repeat(50));
    }

    fn display_post_with_author(&self, post: &Post) {
        println!("{}", "─".repeat(50));
        println!("📝 {}", post.title);
        if let Some(username) = &post.author_username {
            println!("👤 Author: {}", username);
        } else {
            println!("👤 Author ID: {}", post.user_id);
        }
        println!("📅 Created: {}", post.created_at.format("%Y-%m-%d %H:%M UTC"));
        if post.updated_at != post.created_at {
            println!("📝 Updated: {}", post.updated_at.format("%Y-%m-%d %H:%M UTC"));
        }
        println!("{}", "─".repeat(50));
        
        // Display content
        let lines: Vec<&str> = post.content.lines().collect();
        if lines.len() > 20 {
            for (i, line) in lines.iter().enumerate() {
                println!("{:3}: {}", i + 1, line);
            }
        } else {
            println!("{}", post.content);
        }
        
        println!("{}", "─".repeat(50));
    }
}
