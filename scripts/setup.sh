 #setup.sh
#!/bin/bash

set -e

echo "🚀 Setting up SSH Blog Platform"
echo "==============================="

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Create project structure
echo "📁 Creating project structure..."
mkdir -p src scripts

# Build and start the container
echo "🏗️  Building Docker container..."
docker-compose build

echo "🚀 Starting SSH Blog Platform..."
docker-compose up -d

echo "⏳ Waiting for container to be ready..."
sleep 5

# Check if container is running
if docker ps | grep -q ssh-blog; then
    echo "✅ SSH Blog Platform is running!"
    echo ""
    echo "📋 Next steps:"
    echo "1. Make the registration script executable:"
    echo "   chmod +x register.sh"
    echo ""
    echo "2. Register a user:"
    echo "   ./register.sh"
    echo ""
    echo "3. Connect via SSH:"
    echo "   ssh username@localhost -p 2222"
    echo ""
    echo "📊 Container status:"
    docker ps --filter name=ssh-blog
else
    echo "❌ Failed to start SSH Blog Platform"
    echo "Check logs with: docker-compose logs"
    exit 1
fi