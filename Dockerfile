# ===== Build stage =====
FROM rust:1.90 AS builder

# Install MUSL toolchain
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    build-essential \
    ca-certificates \
 && rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY fantasy_age_discord_bot ./fantasy_age_discord_bot
WORKDIR /app/fantasy_age_discord_bot

# Build release binary with MUSL (no OpenSSL)
RUN cargo build --release --target x86_64-unknown-linux-musl

# ===== Runtime stage =====
FROM debian:12-slim
RUN apt-get update && apt-get install -y ca-certificates \
 && update-ca-certificates \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .

ENV PORT=8080
EXPOSE 8080
CMD ["./fantasy_age_discord_bot"]
