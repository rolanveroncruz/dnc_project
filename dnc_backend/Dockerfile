# ----------------------------------------------------------------------
# STAGE 1: Builder Stage (Compile the Rust Application)
# ----------------------------------------------------------------------
FROM rust:1.74-slim-bullseye AS builder

# Set the working directory inside the container
WORKDIR /app

# Install necessary dependencies for Axum and PostgreSQL (SQLx/SeaORM)
# This includes openssl (for TLS/SSL connections) and musl-tools if using musl
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    # libpq-dev is required for PostgreSQL connections
    libpq-dev \
    musl-tools && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo files first to leverage Docker's caching.
# If these don't change, the dependencies aren't rebuilt.
COPY Cargo.toml Cargo.lock /app/

# Create a dummy src/main.rs to build dependencies
# This is a key trick to cache the potentially large dependency build
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf target/release/axum-server

# Copy the actual source code
COPY src /app/src

# Copy migrations folder (needed for potential schema initialization)
COPY migrations /app/migrations

# Re-build the application, linking statically for a minimal final image
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release

# ----------------------------------------------------------------------
# STAGE 2: Final Production Stage (Minimal Image)
# ----------------------------------------------------------------------
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Install runtime dependencies (libpq needed for runtime connection to Postgres)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Copy the final compiled binary from the builder stage
# The binary name is usually the project name defined in Cargo.toml (e.g., 'backend')
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Copy the migrations directory (needed if you run migrations on startup)
COPY migrations /app/migrations

# Expose the port Axum listens on (matches the port mapping in docker-compose.yml)
EXPOSE 3000

# Set the entry point to run the compiled Axum binary
# CMD is executed when the container starts
CMD ["/usr/local/bin/backend"]
