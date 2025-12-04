# ===== Build stage =====
FROM rust:1.90-slim-bullseye AS builder

# Install MUSL target and build dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    build-essential \
 && rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy Cargo manifests for caching
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Create dummy main to cache dependencies
RUN mkdir -p fantasy_age_discord_bot/src && echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy full source
COPY fantasy_age_discord_bot/src ./src

# Build final binary
ENV OPENSSL_STATIC=1
RUN cargo build --release --target x86_64-unknown-linux-musl

# ===== Runtime stage =====
FROM debian:12-slim AS runtime
# Install CA certificates for Discord TLS
RUN apt-get update && apt-get install -y ca-certificates \
    && update-ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy Rust binary from builder
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .

# Cloud Run expects the container to listen on $PORT
ENV PORT 8080
EXPOSE 8080

# Start the bot
CMD ["./fantasy_age_discord_bot"]
