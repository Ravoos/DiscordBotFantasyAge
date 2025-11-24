# ===== Build stage =====
FROM rust:1.90-slim-bullseye AS builder

# Install MUSL target
RUN apt-get update && apt-get install -y musl-tools pkg-config && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy manifest first (dependency caching)
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Create dummy src to force dependency build
RUN mkdir -p fantasy_age_discord_bot/src && \
    echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs

# Build dependencies
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Now copy REAL src
COPY fantasy_age_discord_bot/src ./src

# Build final binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# ===== Runtime stage =====
FROM scratch

# Certificates (Discord requires HTTPS)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy statically-linked binary
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot /

# Run it
ENTRYPOINT ["/fantasy_age_discord_bot"]
