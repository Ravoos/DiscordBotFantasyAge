#!/bin/sh
set -e

# Check Discord token
if [ -z "$DISCORD_TOKEN" ]; then
  echo "ERROR: DISCORD_TOKEN not set!"
  exit 1
fi

echo "Starting Discord bot..."
echo "DISCORD_TOKEN length: ${#DISCORD_TOKEN}"

# Port for Cloud Run health checks
PORT=${PORT:-8080}

# Start a minimal HTTP server in the background using nohup
# Logs go to /dev/null to avoid blocking
nohup sh -c "while true; do { echo -e 'HTTP/1.1 200 OK\r\n'; } | nc -l -p $PORT -q 1; done" >/dev/null 2>&1 &

# Start the Discord bot as the **foreground process**
exec ./fantasy_age_discord_bot
