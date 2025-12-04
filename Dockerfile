# ===== Build stage =====
FROM rust:1.90-slim-bullseye AS builder
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev && \
    rustup target add x86_64-unknown-linux-musl
WORKDIR /app

# Copy manifests
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Create dummy main for caching
RUN mkdir -p fantasy_age_discord_bot/src && echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy full source
COPY fantasy_age_discord_bot/src ./src

# Build final binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# ===== Runtime =====
FROM gcr.io/distroless/cc-debian11
WORKDIR /app
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .

EXPOSE 8080
CMD ["./fantasy_age_discord_bot"]
