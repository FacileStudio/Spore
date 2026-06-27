# Spore Docker Image
# Using install script from GitHub

FROM debian:bookworm-slim

# Install runtime dependencies including Rust toolchain
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/bash spore

# Install Rust for the install script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install spore using the install script from GitHub
RUN curl -fsSL https://raw.githubusercontent.com/saravenpi/spore/main/install.sh | bash

# Create working directory
WORKDIR /workspace
RUN chown spore:spore /workspace

# Switch to non-root user
USER spore

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD spore --version || exit 1

# Default command
ENTRYPOINT ["spore"]
CMD ["--help"]
