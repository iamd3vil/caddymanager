# Multi-stage build for Caddy Manager
# Stage 1: Build Frontend (Vue.js)
FROM node:18-alpine AS frontend-builder

WORKDIR /app/frontend
COPY frontend/caddymanager-ui/package*.json ./
RUN npm ci

COPY frontend/caddymanager-ui/ ./
RUN npm run build

# Stage 2: Build Backend (Rust)
FROM rust:1.82-alpine AS backend-builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app/backend
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src/ ./src/

# Build the application
RUN cargo build --release

# Stage 3: Final runtime image
FROM alpine:3.18

# Install Caddy and other runtime dependencies
RUN apk add --no-cache caddy ca-certificates

# Create app directory
WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/backend/target/release/caddymanager ./

# Copy frontend build
COPY --from=frontend-builder /app/frontend/dist ./static

# Copy Caddy configuration
COPY backend/Caddyfile ./

# Create necessary directories and set permissions
RUN mkdir -p /data /config && \
    chmod +x /app/caddymanager

# Expose ports
# 8080 for the backend API
# 80 for Caddy HTTP
# 443 for Caddy HTTPS
EXPOSE 8080 80 443

# Set environment variables
ENV CARGO_MANIFEST_DIR=/app

# Start the application
CMD ["./caddymanager"] 