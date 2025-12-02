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

# Start dummy HTTP server to satisfy Cloud Run
PORT=${PORT:-8080}
echo "Starting dummy HTTP server on port $PORT"

# Use busybox nc with -k to handle multiple connections
# -l listen mode, -p port, -k keep listening
exec sh -c "while true; do { echo -e 'HTTP/1.1 200 OK\r\n'; } | nc -l -p $PORT -q 1; done"
