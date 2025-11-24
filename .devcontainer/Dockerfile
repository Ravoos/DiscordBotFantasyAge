# ===== Build stage =====
FROM rust:1.90-slim-bullseye AS builder

# Install MUSL and build dependencies
RUN apt-get update && apt-get install -y musl-tools pkg-config && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy manifest files for dependency caching
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/
COPY fantasy_age_discord_bot/src ./fantasy_age_discord_bot/src

# Build dependencies only
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Rebuild with actual source
COPY fantasy_age_discord_bot/ ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# ===== Runtime stage (zero-dependency) =====
FROM scratch

# Bring in certificates (Discord requires HTTPS)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy statically linked Rust binary
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot /

# Set the startup command
ENTRYPOINT ["/fantasy_age_discord_bot"]
