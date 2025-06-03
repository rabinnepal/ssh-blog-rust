#!/bin/bash
# setup_ssh_server.sh - Configure SSH server for the blog platform

echo "Setting up SSH server for blog platform..."

# Create SSH configuration directory if it doesn't exist
sudo mkdir -p /etc/ssh/sshd_config.d

# Create custom SSH configuration for the blog
cat > /tmp/blog_ssh.conf << 'EOF'
# SSH Blog Platform Configuration
Port 2222
Protocol 2

# Allow password authentication for initial setup
PasswordAuthentication yes
PubkeyAuthentication yes
AuthorizedKeysFile .ssh/authorized_keys

# Set environment variables for the blog application
AcceptEnv SSH_ORIGINAL_COMMAND
AcceptEnv SSH_CLIENT_KEY_FILE
PermitUserEnvironment yes

# Force command for blog users (optional)
# Match User blog_user
#     ForceCommand /path/to/your/blog_app

# Allow TCP forwarding
AllowTcpForwarding no
X11Forwarding no
AllowAgentForwarding yes

# Security settings
MaxAuthTries 3
MaxSessions 2
ClientAliveInterval 300
ClientAliveCountMax 2

# Logging
LogLevel INFO
SyslogFacility AUTH
EOF

# Copy the configuration
sudo cp /tmp/blog_ssh.conf /etc/ssh/sshd_config.d/blog.conf

# Create a wrapper script that sets environment variables
cat > /tmp/ssh_blog_wrapper.sh << 'EOF'
#!/bin/bash
# SSH Blog Wrapper Script

# Export useful environment variables
export SSH_CLIENT_IP=$(echo $SSH_CLIENT | cut -d' ' -f1)
export SSH_CLIENT_PORT=$(echo $SSH_CLIENT | cut -d' ' -f2)

# If we can detect the SSH key, export it
if [ -n "$SSH_ORIGINAL_COMMAND" ]; then
    export SSH_ORIGINAL_COMMAND="$SSH_ORIGINAL_COMMAND"
fi

# Try to get the SSH key fingerprint
if command -v ssh-keygen >/dev/null 2>&1; then
    if [ -n "$SSH_CLIENT" ]; then
        # This is a simplified approach - in production you'd want more sophisticated key detection
        export SSH_KEY_FINGERPRINT="$(ssh-keygen -lf ~/.ssh/authorized_keys 2>/dev/null | head -1 | cut -d' ' -f2)"
    fi
fi

# Run the blog application
exec /path/to/your/ssh-blog/target/release/ssh-blog "$@"
EOF

sudo cp /tmp/ssh_blog_wrapper.sh /usr/local/bin/ssh_blog_wrapper.sh
sudo chmod +x /usr/local/bin/ssh_blog_wrapper.sh

# Create a system user for the blog (optional)
sudo useradd -m -s /usr/local/bin/ssh_blog_wrapper.sh blog_user 2>/dev/null || true

# Restart SSH service
echo "Restarting SSH service..."
sudo systemctl restart sshd

# Create test keys for development
if [ "$1" = "--dev" ]; then
    echo "Creating development SSH keys..."
    mkdir -p ~/.ssh
    
    if [ ! -f ~/.ssh/id_rsa ]; then
        ssh-keygen -t rsa -b 2048 -f ~/.ssh/id_rsa -N "" -C "dev@ssh-blog"
    fi
    
    # Add the public key to authorized_keys
    cat ~/.ssh/id_rsa.pub >> ~/.ssh/authorized_keys
    chmod 600 ~/.ssh/authorized_keys
    
    echo "Development setup complete!"
    echo "Your public key:"
    cat ~/.ssh/id_rsa.pub
fi

echo "SSH server setup complete!"
echo "You can now connect with: ssh username@localhost -p 2222"