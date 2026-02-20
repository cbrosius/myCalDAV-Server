FROM rust:1.85

WORKDIR /workspace

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libsqlite3-dev \
    pkg-config \
    inotify-tools \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli for database migrations and compile-time checks
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Install cargo-watch for development hot-reload
RUN cargo install cargo-watch --locked

EXPOSE 8080 3000
