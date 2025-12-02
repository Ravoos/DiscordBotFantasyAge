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
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./fantasy_age_discord_bot/

# Dummy src to cache dependencies
RUN mkdir -p fantasy_age_discord_bot/src && echo "fn main() {}" > fantasy_age_discord_bot/src/main.rs
WORKDIR /app/fantasy_age_discord_bot
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy real source
COPY fantasy_age_discord_bot/src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/fantasy_age_discord_bot/target/x86_64-unknown-linux-musl/release/fantasy_age_discord_bot .
CMD ["./fantasy_age_discord_bot"]
