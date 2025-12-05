# ===== Build stage =====
FROM rust:1.79-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev ca-certificates build-essential
WORKDIR /app
COPY fantasy_age_discord_bot/ .

# Build
RUN cargo build --release

# ===== Runtime stage =====
FROM debian:12-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/fantasy_age_discord_bot .

# Cloud Run requires $PORT
ENV PORT=8080
EXPOSE 8080

CMD ["./fantasy_age_discord_bot"]
