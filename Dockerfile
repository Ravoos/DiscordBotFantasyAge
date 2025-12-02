# ===== Build stage =====
FROM rust:1.90-slim-bullseye AS builder

WORKDIR /app
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Dummy src to build dependencies
RUN mkdir -p fantasy_age_discord_bot/src && echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs

WORKDIR /app/fantasy_age_discord_bot
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools pkg-config
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy real source code
COPY fantasy_age_discord_bot/src ./src

# Build final binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM gcr.io/distroless/cc
WORKDIR /app

# Copy binary
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .

# Set environment variable placeholder
ENV DISCORD_TOKEN=""

# Run bot
CMD ["./fantasy_age_discord_bot"]
