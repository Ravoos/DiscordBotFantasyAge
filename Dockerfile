# Stage 1: Build
FROM node:20-slim AS build

WORKDIR /app

# Copy only package files first for better caching
COPY package*.json ./
RUN npm ci --only=production
COPY . .

# Stage 2: Final image
FROM node:20-slim
WORKDIR /app

# Copy only necessary files from build stage
COPY --from=build /app .
ENV PORT=8080

# Run the bot
CMD ["node", "index.js"]
