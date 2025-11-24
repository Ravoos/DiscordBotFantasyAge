#!/bin/sh
set -e

# Log environment info
echo "Starting Discord bot..."
if [ -z "$DISCORD_TOKEN" ]; then
    echo "ERROR: DISCORD_TOKEN is not set!"
    exit 1
fi

echo "DISCORD_TOKEN length: ${#DISCORD_TOKEN}"

# Start the bot
./fantasy_age_discord_bot

# If the bot exits, log it
echo "Bot exited."
