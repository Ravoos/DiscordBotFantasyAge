# Stage 1: Build
FROM node:20-bullseye AS build

# Install build tools (needed for native modules)
RUN apt-get update && apt-get install -y \
    python3 \
    make \
    g++ \
    git \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy package files first to leverage caching
COPY package*.json ./

# Install production dependencies
RUN npm install --production

# Copy the rest of the source code
COPY . .

# Stage 2: Final image
FROM node:20-bullseye-slim
WORKDIR /app

# Copy built node_modules and source code from build stage
COPY --from=build /app /app

# Set environment variable for Cloud Run
ENV PORT=8080

# Start bot
CMD ["node", "index.js"]
