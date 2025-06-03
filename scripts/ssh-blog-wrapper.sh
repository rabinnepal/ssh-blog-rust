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
