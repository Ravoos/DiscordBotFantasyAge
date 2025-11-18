# ====== Build stage ======
FROM rust:1.90.0-slim-bullseye AS builder

# Install only essential build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Cache dependencies (Cargo.toml + Cargo.lock only)
COPY fantasy_age_discord_bot/Cargo.toml fantasy_age_discord_bot/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release; rm -rf src

# Copy actual source and build release binary
COPY fantasy_age_discord_bot/ .
RUN cargo build --release

# ====== Runtime stage ======
FROM debian:bullseye-slim

# Install runtime dependencies (not dev packages)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/fantasy_age_discord_bot .

# Default command
CMD ["./fantasy_age_discord_bot"]