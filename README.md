# 🚀 SSH Blog Platform

A terminal-based blogging platform accessible exclusively via SSH with public key authentication.

## 🏗️ Architecture

- **Docker Container**: Ubuntu 22.04 with SSH server
- **Database**: SQLite for user profiles and posts
- **Authentication**: SSH public key based
- **Language**: Rust application with CLI interface

## 📦 Setup

### 1. Build and Run

```bash
# Build and start the container
docker-compose up -d

# Check if it's running
docker-compose ps
```

### 2. Register Your First User

**Option A: Using the registration script (Recommended)**

```bash
# Run the interactive registration script
docker exec -it ssh-blog /usr/local/bin/register.sh
```

**Option B: Manual registration**

```bash
# Register a user manually
docker exec -it ssh-blog /opt/ssh-blog/ssh-blog --register-user "yourusername" "$(cat ~/.ssh/id_rsa.pub)" "Your bio here"

# Create the system user
docker exec ssh-blog sudo /usr/local/bin/manage-user.sh create "yourusername" "$(cat ~/.ssh/id_rsa.pub)"
```

### 3. Connect to Your Blog

```bash
# Connect using SSH
ssh yourusername@localhost -p 2222
```

## 🔐 Authentication Flow

### Registration (One-time)

1. Run registration script or command
2. Provide username (3+ chars, alphanumeric + \_ -)
3. System auto-detects your SSH public key
4. User created in database + system user created
5. SSH key linked to your account

### Login (Every time)

1. `ssh yourusername@localhost -p 2222`
2. SSH daemon validates your key
3. ForceCommand launches the blog app
4. App authenticates you against database
5. Direct access to your blog interface

## 📝 Features

### Current Features

- ✅ SSH key-based authentication
- ✅ User registration with auto SSH key detection
- ✅ Write and publish posts
- ✅ View your posts
- ✅ View all posts from all users
- ✅ User profiles with optional bio
- ✅ SQLite database persistence
- ✅ Multi-user support
- ✅ Secure containerized environment

### Blog Interface

```
🏠 Main Menu
1. ✍️  Write a new post
2. 📚 View my posts
3. 🌍 View all posts
4. 🚪 Exit
```

## 🛠️ Management Commands

### User Management

```bash
# Create a new user
docker exec ssh-blog sudo /usr/local/bin/manage-user.sh create username "ssh-rsa AAAA..."

# Update user's SSH key
docker exec ssh-blog sudo /usr/local/bin/manage-user.sh update username "ssh-rsa AAAA..."

# Delete a user
docker exec ssh-blog sudo /usr/local/bin/manage-user.sh delete username
```

### Database Operations

```bash
# View users
docker exec ssh-blog sqlite3 /var/lib/ssh-blog/blog.db "SELECT username, created_at FROM users;"

# View posts
docker exec ssh-blog sqlite3 /var/lib/ssh-blog/blog.db "SELECT title, created_at FROM posts ORDER BY created_at DESC;"

# Backup database
docker cp ssh-blog:/var/lib/ssh-blog/blog.db ./backup-$(date +%Y%m%d).db
```

## 🔧 Development

### Project Structure

```
├── src/
│   ├── main.rs          # Entry point and CLI argument handling
│   ├── models.rs        # Data structures (User, Post)
│   ├── database.rs      # SQLite operations
│   ├── user.rs          # User management and authentication
│   ├── post.rs          # Post creation and retrieval
│   └── cli.rs           # Command-line interface logic
├── scripts/
│   ├── manage-user.sh   # System user management
│   ├── register.sh      # User registration script
│   └── startup.sh       # Container startup script
├── Dockerfile           # Container configuration
├── docker-compose.yml   # Docker Compose setup
└── Cargo.toml          # Rust dependencies
```

### Building from Source

```bash
# Build the Rust application
cargo build --release

# Build Docker image
docker build -t ssh-blog .
```

## 🚨 Security Features

- **No Password Authentication**: Only SSH keys accepted
- **ForceCommand**: Users can only run the blog app
- **No Shell Access**: Direct shell access prevented
- **User Isolation**: Each user has their own system account
- **Key Validation**: SSH keys validated against database
- **Restricted SSH**: No tunneling, forwarding, or X11

## 🔍 Troubleshooting

### Connection Issues

```bash
# Check if SSH daemon is running
docker exec ssh-blog pgrep sshd

# Check SSH configuration
docker exec ssh-blog cat /etc/ssh/sshd_config

# View SSH logs
docker exec ssh-blog tail -f /var/log/auth.log
```

### Database Issues

```bash
# Check database file
docker exec ssh-blog ls -la /var/lib/ssh-blog/

# Initialize database manually
docker exec ssh-blog /opt/ssh-blog/ssh-blog --init-db
```

### User Issues

```bash
# List system users
docker exec ssh-blog cut -d: -f1 /etc/passwd

# Check user's SSH setup
docker exec ssh-blog ls -la /home/username/.ssh/
```

## 📋 Todo / Roadmap

- [ ] Post editing and deletion
- [ ] Post search and filtering
- [ ] User profile editing
- [ ] Post categories/tags
- [ ] Comments system
- [ ] Post scheduling
- [ ] Export functionality
- [ ] Admin interface
- [ ] Multi-server federation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with Docker
5. Submit a pull request

## 📄 License

MIT License - feel free to use and modify!
