# scripts/manage-user.sh (Updated)
#!/bin/bash

set -e

ACTION=$1
USERNAME=$2
SSH_KEY=$3

case $ACTION in
    "create")
        echo "Creating system user: $USERNAME"
        
        # Create system user if it doesn't exist
        if ! id "$USERNAME" &>/dev/null; then
            useradd -m -s /bin/bash "$USERNAME"
            echo "✅ System user $USERNAME created"
        else
            echo "ℹ️  System user $USERNAME already exists"
        fi
        
        # Create SSH directory and set up key
        SSH_DIR="/home/$USERNAME/.ssh"
        mkdir -p "$SSH_DIR"
        
        # Write SSH key to authorized_keys
        echo "$SSH_KEY" > "$SSH_DIR/authorized_keys"
        
        # Create environment file for SSH
        echo "USER=$USERNAME" > "$SSH_DIR/environment"
        echo "LOGNAME=$USERNAME" >> "$SSH_DIR/environment"
        echo "HOME=/home/$USERNAME" >> "$SSH_DIR/environment"
        
        # Set proper permissions
        chmod 700 "$SSH_DIR"
        chmod 600 "$SSH_DIR/authorized_keys"
        chmod 600 "$SSH_DIR/environment"
        chown -R "$USERNAME:$USERNAME" "$SSH_DIR"
        
        echo "✅ SSH key and environment configured for $USERNAME"
        ;;
    
    "delete")
        echo "Deleting user: $USERNAME"
        if id "$USERNAME" &>/dev/null; then
            userdel -r "$USERNAME" 2>/dev/null || true
            echo "✅ User $USERNAME deleted"
        else
            echo "ℹ️  User $USERNAME does not exist"
        fi
        ;;
    
    *)
        echo "Usage: $0 {create|delete} <username> [ssh_key]"
        exit 1
        ;;
esac

---