#!/bin/sh
set -e

echo "Starting Discord bot..."

if [ -z "$DISCORD_TOKEN" ]; then
    echo "ERROR: DISCORD_TOKEN is not set!"
    exit 1
fi

echo "DISCORD_TOKEN length: ${#DISCORD_TOKEN}"

# Start Discord bot in background
./fantasy_age_discord_bot &

# Start minimal Python HTTP server to satisfy Cloud Run
PORT=${PORT:-8080}
echo "Starting dummy HTTP server on port $PORT"

exec python3 -m http.server "$PORT"
