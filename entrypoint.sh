#!/bin/sh
set -e

# Check Discord token
if [ -z "$DISCORD_TOKEN" ]; then
  echo "ERROR: DISCORD_TOKEN not set!"
  exit 1
fi

echo "Starting Discord bot..."
echo "DISCORD_TOKEN length: ${#DISCORD_TOKEN}"

# Start dummy HTTP server in background for Cloud Run health checks
PORT=${PORT:-8080}
python3 -m http.server $PORT &

# Start bot as the main foreground process
exec ./fantasy_age_discord_bot
