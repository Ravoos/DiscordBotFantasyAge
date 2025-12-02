# ===== Build stage =====
FROM rust:1.90-slim-bullseye AS builder

# Install MUSL target and build dependencies
RUN apt-get update && apt-get install -y musl-tools pkg-config && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy manifest first for caching
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Create dummy src to build dependencies
RUN mkdir -p fantasy_age_discord_bot/src && \
    echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs

# Build dependencies
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy real source
COPY fantasy_age_discord_bot/src ./src

# Build final binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM debian:bullseye-slim

# Install certificates and Python3
RUN apt-get update && \
    apt-get install -y ca-certificates python3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app


# Copy binary from builder
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .

# Copy entrypoint script
COPY entrypoint.sh .

# Make executable
RUN chmod +x fantasy_age_discord_bot entrypoint.sh

# Start bot
CMD ["./entrypoint.sh"]
