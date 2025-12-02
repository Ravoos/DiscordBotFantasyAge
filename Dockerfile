# Stage 1: Build dependencies
FROM node:20-slim AS build

# Install build tools
RUN apt-get update && apt-get install -y \
    python3 \
    make \
    g++ \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only package files first (cache npm install)
COPY package*.json ./

# Install dependencies
RUN npm install --only=production

# Copy source code
COPY . .

# Stage 2: Final image
FROM node:20-slim

WORKDIR /app

# Copy node_modules and source code from build stage
COPY --from=build /app /app

# Expose port (Cloud Run default)
ENV PORT=8080

# Run bot
CMD ["node", "index.js"]
