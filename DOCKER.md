# Caddy Manager - Docker Deployment

This document describes how to build and deploy the Caddy Manager application using Docker.

## Architecture

The application consists of:

- **Frontend**: Vue.js 3 application with PrimeVue UI components
- **Backend**: Rust application using Axum web framework
- **Caddy**: Reverse proxy server

All components are packaged into a single Docker container using a multi-stage build.

## Quick Start

### Option 1: Using the build script

```bash
./build.sh
docker run -p 8080:8080 -p 80:80 haproxymanager:latest
```

### Option 2: Using Docker Compose (Recommended)

```bash
docker-compose up -d
```

### Option 3: Manual Docker commands

```bash
# Build the image
docker build -t haproxymanager:latest .

# Run the container
docker run -d \
  --name haproxymanager \
  -p 8080:8080 \
  -p 80:80 \
  -v haproxy_logs:/var/log/haproxy \
  haproxymanager:latest
```

## Access

- **Web Interface**: http://localhost:8080
- **API**: http://localhost:8080/api
- **HAProxy Stats** (if configured): http://localhost:80/stats

## Ports

- `8080`: Backend API and frontend web interface
- `80`: HAProxy (for load balancing your configured backends)

## Environment Variables

- `RUST_LOG`: Set logging level (default: `info`)
- `CARGO_MANIFEST_DIR`: Application directory (set automatically in container)

## Volumes

- `/var/log/haproxy`: HAProxy log files (mounted as volume for persistence)

## Health Check

The container includes a health check that verifies the backend API is responding:

- **Endpoint**: `http://localhost:8080/api/health`
- **Interval**: 30 seconds
- **Timeout**: 10 seconds
- **Retries**: 3

## Development

For development with live reloading, run the frontend and backend separately:

```bash
# Terminal 1 - Backend
cd backend
cargo run

# Terminal 2 - Frontend
cd frontend/haproxymanager-ui
npm run dev
```

## Troubleshooting

### Check container logs

```bash
docker logs haproxymanager
```

### Access container shell

```bash
docker exec -it haproxymanager sh
```

### Check HAProxy status

```bash
docker exec haproxymanager ps aux | grep haproxy
```

### View HAProxy logs

```bash
docker exec haproxymanager cat /var/log/haproxy/haproxy.log
```

## Production Deployment

For production deployment, consider:

1. **Using a reverse proxy** (nginx) in front of the container
2. **Setting up SSL/TLS termination**
3. **Using a proper orchestration platform** (Kubernetes, Docker Swarm)
4. **Configuring log rotation** for HAProxy logs
5. **Setting resource limits** for the container

Example production Docker Compose with nginx:

```yaml
version: "3.8"

services:
  nginx:
    image: nginx:alpine
    ports:
      - "443:443"
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/ssl
    depends_on:
      - haproxymanager

  haproxymanager:
    build: .
    expose:
      - "8080"
    volumes:
      - haproxy_logs:/var/log/haproxy
    restart: unless-stopped
    deploy:
      resources:
        limits:
          cpus: "1.0"
          memory: 512M

volumes:
  haproxy_logs:
```
