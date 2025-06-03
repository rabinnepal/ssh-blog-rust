#!/bin/bash

# Registration script for SSH Blog Platform
# This script handles user registration from outside the container

set -e

CONTAINER_NAME="ssh-blog"
DB_PATH="/var/lib/ssh-blog/blog.db"

echo "🔐 SSH Blog Platform Registration"
echo "=================================="

# Get username
while true; do
    read -p "Enter your desired username: " USERNAME
    
    if [[ -z "$USERNAME" ]]; then
        echo "Username cannot be empty. Please try again."
        continue
    fi
    
    if [[ ${#USERNAME} -lt 3 ]]; then
        echo "Username must be at least 3 characters long."
        continue
    fi
    
    if [[ ! "$USERNAME" =~ ^[a-zA-Z0-9_-]+$ ]]; then
        echo "Username can only contain letters, numbers, underscores, and hyphens."
        continue
    fi
    
    # Check if username already exists in database
    if docker exec "$CONTAINER_NAME" sqlite3 "$DB_PATH" "SELECT username FROM users WHERE username='$USERNAME';" 2>/dev/null | grep -q "$USERNAME"; then
        echo "Username '$USERNAME' is already taken. Please choose another."
        continue
    fi
    
    break
done

# Detect SSH public key
SSH_KEY=""
POSSIBLE_KEYS=(
    "$HOME/.ssh/id_rsa.pub"
    "$HOME/.ssh/id_ed25519.pub"
    "$HOME/.ssh/id_ecdsa.pub"
    "$HOME/.ssh/id_dsa.pub"
)

for key_path in "${POSSIBLE_KEYS[@]}"; do
    if [[ -f "$key_path" ]]; then
        SSH_KEY=$(cat "$key_path")
        if [[ "$SSH_KEY" =~ ^ssh- ]]; then
            echo "✅ Detected SSH key: ${key_path##*/}"
            break
        fi
    fi
done

if [[ -z "$SSH_KEY" ]]; then
    echo "❌ Could not automatically detect your SSH public key."
    echo "Please paste your SSH public key below:"
    echo "(You can find it in ~/.ssh/id_rsa.pub or ~/.ssh/id_ed25519.pub)"
    
    while true; do
        read -p "SSH Public Key: " SSH_KEY
        
        if [[ -z "$SSH_KEY" ]]; then
            echo "SSH key cannot be empty."
            continue
        fi
        
        if [[ ! "$SSH_KEY" =~ ^ssh- ]]; then
            echo "SSH key should start with 'ssh-rsa', 'ssh-ed25519', etc."
            continue
        fi
        
        break
    done
fi

# Get optional bio
read -p "Optional bio (press Enter to skip): " BIO

# Register user in the application database
echo "Registering user in database..."
if docker exec "$CONTAINER_NAME" /opt/ssh-blog/ssh-blog --register-user "$USERNAME" "$SSH_KEY" "$BIO"; then
    echo "✅ User registered in database"
else
    echo "❌ Failed to register user in database"
    exit 1
fi

# Create system user in container
echo "Creating system user..."
if docker exec "$CONTAINER_NAME" sudo /usr/local/bin/manage-user.sh create "$USERNAME" "$SSH_KEY"; then
    echo "✅ System user created"
else
    echo "❌ Failed to create system user"
    exit 1
fi

echo ""
echo "🎉 Registration completed successfully!"
echo "Username: $USERNAME"
if [[ -n "$BIO" ]]; then
    echo "Bio: $BIO"
fi
echo ""
echo "You can now connect using:"
echo "ssh $USERNAME@localhost -p 2222"
echo ""
echo "Make sure your SSH agent is running and your key is loaded:"
echo "ssh-add ~/.ssh/id_rsa  # or your key file"