#!/bin/bash

# Startup script for SSH Blog Platform container

set -e

echo "🚀 Starting SSH Blog Platform..."

# Initialize the database if it doesn't exist
if [[ ! -f /var/lib/ssh-blog/blog.db ]]; then
    echo "📊 Initializing database..."
    cd /opt/ssh-blog && ./ssh-blog --init-db
    chown blogadmin:blogadmin /var/lib/ssh-blog/blog.db
fi

# Start SSH daemon
echo "🔐 Starting SSH daemon..."
/usr/sbin/sshd -D &

# Keep the container running and show logs
echo "✅ SSH Blog Platform is ready!"
echo "📡 SSH server listening on port 22"
echo "🔧 To register a new user, run: docker exec -it <container_name> /usr/local/bin/register.sh"
echo ""
echo "📝 Logs:"

# Follow SSH logs
tail -f /var/log/auth.log 2>/dev/null || tail -f /dev/null