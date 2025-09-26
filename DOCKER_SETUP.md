# 🐳 Docker Setup for IrminView

This document explains how to run IrminView with Docker Compose, using a containerized Irmin server.

## 🚀 Quick Start

### 1. Start the Irmin Server

```bash
# Start the Irmin bridge server
docker-compose up irmin-server

# Or run in background
docker-compose up -d irmin-server
```

The server will be available at http://localhost:8080

### 2. Test the Server

```bash
# Check server health
curl http://localhost:8080/health

# Get tree structure (empty initially)
curl http://localhost:8080/api/tree

# Get commits
curl http://localhost:8080/api/commits
```

### 3. Run IrminView Desktop App

```bash
# Set environment to use HTTP server
export IRMIN_USE_HTTP=true
export IRMIN_SERVER_URL=http://localhost:8080

# Run the desktop application
cd src-tauri
cargo run
```

## 📋 Available Services

### Production Setup
```bash
# Start just the Irmin server
docker-compose up irmin-server

# Start all services including the desktop app (headless)
docker-compose --profile testing up
```

### Development Setup
```bash
# Development mode with live reload and nginx proxy
docker-compose -f docker-compose.dev.yml up

# Access via nginx proxy at http://localhost:3001
```

### Initialize Demo Data
```bash
# Initialize the Irmin store with demo data
docker-compose --profile init up irmin-init
```

## 🔧 Configuration Options

### Environment Variables

#### Irmin Server
- `IRMIN_PORT` - Server port (default: 8080)
- `IRMIN_STORE_PATH` - Path to Irmin store (default: /data/irmin_store)

#### IrminView Application
- `IRMIN_USE_HTTP` - Use HTTP server instead of CLI (set to "true")
- `IRMIN_SERVER_URL` - URL of Irmin server (default: http://localhost:8080)

### Volume Mounts

#### Persistent Data
```yaml
volumes:
  - irmin-data:/home/opam/data/irmin_store
```

#### Custom Irmin Store
```yaml
volumes:
  - ./my-irmin-store:/home/opam/data/irmin_store
```

## 🛠️ Development Workflow

### 1. Development with Live Reload

```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up

# The server automatically rebuilds when OCaml code changes
# Frontend is served via nginx at http://localhost:3001
```

### 2. Testing Changes

```bash
# Run tests against the containerized server
cd src-tauri
IRMIN_USE_HTTP=true IRMIN_SERVER_URL=http://localhost:8080 cargo test

# Run desktop app in HTTP mode
IRMIN_USE_HTTP=true cargo run
```

### 3. Building Custom Images

```bash
# Build Irmin server image
docker build -f docker/irmin-server/Dockerfile -t my-irmin-server .

# Build IrminView app image
docker build -f docker/irminview/Dockerfile -t my-irminview .
```

## 📊 API Endpoints

The Irmin bridge server exposes the following REST API:

| Endpoint | Method | Description |
|----------|---------|-------------|
| `/health` | GET | Server health check |
| `/api/tree` | GET | Get complete tree structure |
| `/api/commits` | GET | Get commit history |
| `/api/branches` | GET | Get all branches |
| `/api/search?q=<query>` | GET | Search for keys |
| `/api/diff?from=<hash>&to=<hash>` | GET | Get diff between commits |

### Example Requests

```bash
# Health check
curl http://localhost:8080/health

# Get tree structure
curl http://localhost:8080/api/tree | jq

# Search for keys containing "user"
curl "http://localhost:8080/api/search?q=user" | jq

# Get diff between commits
curl "http://localhost:8080/api/diff?from=abc123&to=def456" | jq
```

## 📁 Directory Structure

```
docker/
├── irmin-server/
│   └── Dockerfile              # Irmin bridge server container
├── irminview/
│   └── Dockerfile              # IrminView desktop app container
└── nginx/
    └── nginx.conf              # Development proxy configuration

docker-compose.yml              # Production setup
docker-compose.dev.yml          # Development setup
```

## 🐛 Troubleshooting

### Server Won't Start
```bash
# Check logs
docker-compose logs irmin-server

# Rebuild the image
docker-compose build --no-cache irmin-server
```

### Connection Issues
```bash
# Test server connectivity
docker-compose exec irmin-server curl http://localhost:8080/health

# Check network connectivity
docker network ls
docker network inspect irminview_irmin-network
```

### Data Persistence Issues
```bash
# Check volume
docker volume ls
docker volume inspect irminview_irmin-data

# Reset data
docker-compose down -v
docker-compose up
```

### OCaml Build Issues
```bash
# Rebuild with verbose output
docker-compose build --no-cache --progress=plain irmin-server

# Check OCaml environment
docker-compose exec irmin-server opam list
```

## 🚢 Production Deployment

### Docker Swarm
```bash
# Deploy to swarm
docker stack deploy -c docker-compose.yml irminview

# Scale services
docker service scale irminview_irmin-server=3
```

### Kubernetes
```bash
# Convert to Kubernetes manifests
kompose convert -f docker-compose.yml

# Apply to cluster
kubectl apply -f .
```

### Custom Configuration

Create a `.env` file:
```env
IRMIN_PORT=8080
IRMIN_STORE_PATH=/data/irmin_store
IRMIN_USE_HTTP=true
IRMIN_SERVER_URL=http://localhost:8080
```

Then run:
```bash
docker-compose --env-file .env up
```

## 📈 Monitoring

### Health Checks
The containers include built-in health checks:
```bash
# Check container health
docker-compose ps

# View health check logs
docker inspect --format='{{.State.Health}}' irmin-bridge-server
```

### Logs
```bash
# Follow all logs
docker-compose logs -f

# Follow specific service
docker-compose logs -f irmin-server

# Export logs
docker-compose logs > irminview.log
```

This Docker setup provides a complete, production-ready environment for running IrminView with real Irmin integration!