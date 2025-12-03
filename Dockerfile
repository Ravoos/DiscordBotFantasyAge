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

# Copy Cargo manifests first for dependency caching
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Dummy source to cache dependencies
RUN mkdir -p fantasy_age_discord_bot/src && echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy actual source
COPY fantasy_age_discord_bot/src ./src

# Use Rustls (serenity default) – avoids OpenSSL issues
ENV OPENSSL_STATIC=1

# Build final binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# ===== Runtime stage =====
FROM gcr.io/distroless/cc
WORKDIR /app

# Copy the Rust binary
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .

# Expose port for Cloud Run health checks
EXPOSE 8080

# Start the bot (DISCORD_TOKEN injected via Cloud Run)
CMD ["./fantasy_age_discord_bot"]
