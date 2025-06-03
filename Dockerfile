FROM ubuntu:22.04

# Install required packages
RUN apt update && apt install -y \
    openssh-server \
    sqlite3 \
    sudo \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust (for building the app if needed)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create blog admin user (for system operations)
RUN useradd -m -s /bin/bash blogadmin && \
    echo 'blogadmin ALL=(ALL) NOPASSWD: /usr/local/bin/manage-user.sh' >> /etc/sudoers

# Create the blog app directory
RUN mkdir -p /opt/ssh-blog && \
    chown blogadmin:blogadmin /opt/ssh-blog

# Create shared database directory
RUN mkdir -p /var/lib/ssh-blog && \
    chmod 755 /var/lib/ssh-blog

# Configure SSH server
RUN mkdir /var/run/sshd && \
    echo 'PasswordAuthentication no' >> /etc/ssh/sshd_config && \
    echo 'PubkeyAuthentication yes' >> /etc/ssh/sshd_config && \
    echo 'PermitRootLogin no' >> /etc/ssh/sshd_config && \
    echo 'AuthorizedKeysFile .ssh/authorized_keys' >> /etc/ssh/sshd_config && \
    echo 'ForceCommand /opt/ssh-blog/ssh-blog' >> /etc/ssh/sshd_config && \
    echo 'PermitTunnel no' >> /etc/ssh/sshd_config && \
    echo 'AllowAgentForwarding no' >> /etc/ssh/sshd_config && \
    echo 'AllowTcpForwarding no' >> /etc/ssh/sshd_config && \
    echo 'X11Forwarding no' >> /etc/ssh/sshd_config

# Copy the compiled app
COPY --chown=blogadmin:blogadmin target/release/app /opt/ssh-blog/ssh-blog
RUN chmod +x /opt/ssh-blog/ssh-blog

# Copy user management script
COPY scripts/manage-user.sh /usr/local/bin/manage-user.sh
RUN chmod +x /usr/local/bin/manage-user.sh

# Copy registration script
COPY scripts/register.sh /usr/local/bin/register.sh
RUN chmod +x /usr/local/bin/register.sh

# Copy startup script
COPY scripts/startup.sh /startup.sh
RUN chmod +x /startup.sh

# Create registration directory
RUN mkdir -p /opt/ssh-blog/registration && \
    chown blogadmin:blogadmin /opt/ssh-blog/registration

# Expose SSH port
EXPOSE 22

# Start SSH daemon and keep container running
CMD ["/startup.sh"]