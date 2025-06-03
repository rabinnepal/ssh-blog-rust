#!/bin/bash

# quick-fix.sh - Fix authentication issues in SSH Blog Platform

set -e

echo "🔧 Fixing SSH Blog Platform Authentication"
echo "=========================================="

# Update the Rust source files
echo "📝 Updating source files..."

# Backup existing files
if [ -f "src/user.rs" ]; then
    cp src/user.rs src/user.rs.backup
    echo "✅ Backed up src/user.rs"
fi

if [ -f "src/cli.rs" ]; then
    cp src/cli.rs src/cli.rs.backup
    echo "✅ Backed up src/cli.rs"
fi

# Create the wrapper script
cat > ssh-blog-wrapper.sh << 'EOF'
#!/bin/bash

# SSH Blog Platform Wrapper
# Ensures proper environment setup

# Set the username from various sources
if [ -z "$USER" ]; then
    export USER="$(whoami)"
fi

if [ -z "$LOGNAME" ]; then
    export LOGNAME="$USER"
fi

# Set HOME if not set
if [ -z "$HOME" ]; then
    export HOME="/home/$USER"
fi

# Run the blog platform
exec /opt/ssh-blog/ssh-blog
EOF

chmod +x ssh-blog-wrapper.sh

# Update sshd_config to use wrapper
cat > sshd_config << 'EOF'
Port 2222
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
AuthorizedKeysFile .ssh/authorized_keys
ForceCommand /opt/ssh-blog/ssh-blog-wrapper.sh
AllowUsers *
ClientAliveInterval 60
ClientAliveCountMax 3
PermitUserEnvironment yes
AcceptEnv USER LOGNAME HOME
X11Forwarding no
AllowTcpForwarding no
AllowAgentForwarding no
PermitTunnel no
SyslogFacility AUTH
LogLevel INFO
EOF

echo "✅ Created wrapper script and updated SSH config"

# Rebuild the container
echo "🏗️  Rebuilding container..."
docker-compose down
docker-compose build --no-cache
docker-compose up -d

echo "⏳ Waiting for container to start..."
sleep 5

# Test container status
if docker ps | grep -q ssh-blog; then
    echo "✅ Container is running"
    
    # Show container logs
    echo "📋 Recent container logs:"
    docker-compose logs --tail=10 ssh-blog
    
    echo ""
    echo "🎉 Fix applied successfully!"
    echo ""
    echo "Now try connecting again:"
    echo "ssh rabin@localhost -p 2222"
    
else
    echo "❌ Container failed to start"
    echo "📋 Container logs:"
    docker-compose logs ssh-blog
    exit 1
fi