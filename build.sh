#!/bin/bash

# Caddy Manager Build Script

set -e

echo "🔨 Building Caddy Manager Docker Image..."

# Build the Docker image
docker build -t caddymanager:latest .

echo "✅ Build completed successfully!"
echo ""
echo "To run the container:"
echo "  docker run -p 8080:8080 -p 80:80 -p 443:443 caddymanager:latest"
echo ""
echo "Or use docker-compose:"
echo "  docker-compose up -d"
echo ""
echo "Access the application at: http://localhost:8080"
