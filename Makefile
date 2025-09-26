# Makefile for IrminView Docker Setup and Desktop App

.PHONY: help build up down logs test demo-data clean dev run-app build-app install-deps

# Default target
help: ## Show this help message
	@echo "🧬 IrminView Commands"
	@echo ""
	@echo "Desktop App Commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(run-app|build-app|install-deps)" | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[32m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Docker Commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -v -E "(run-app|build-app|install-deps)" | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Desktop App Commands
install-deps: ## Install system dependencies for desktop app
	@echo "📦 Installing system dependencies..."
	@if command -v apt-get >/dev/null 2>&1; then \
		echo "Installing dependencies on Ubuntu/Debian..."; \
		sudo apt-get update && sudo apt-get install -y \
			pkg-config libssl-dev libgtk-3-dev libglib2.0-dev \
			libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev \
			build-essential curl; \
	elif command -v brew >/dev/null 2>&1; then \
		echo "Installing dependencies on macOS..."; \
		brew install pkg-config openssl gtk+3; \
	else \
		echo "❌ Unsupported system. Please install dependencies manually."; \
		exit 1; \
	fi
	@echo "✅ System dependencies installed!"

build-app: ## Build the desktop application
	@echo "🔨 Building IrminView desktop application..."
	@cd src-tauri && \
		export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig && \
		cargo build --release
	@echo "✅ Desktop application built successfully!"
	@echo "📍 Binary location: src-tauri/target/release/irmin-view"

run-app: ## Run the desktop application (with fallback to demo data)
	@echo "🚀 Starting IrminView desktop application..."
	@echo "ℹ️  Using demo data fallback (no Irmin server required)"
	@cd src-tauri && \
		export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig && \
		export IRMIN_USE_HTTP=false && \
		cargo run
	@echo "✅ Desktop application started!"

run-app-http: ## Run desktop app connected to HTTP server
	@echo "🚀 Starting IrminView with HTTP server connection..."
	@echo "ℹ️  Make sure Irmin server is running: make up"
	@cd src-tauri && \
		export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig && \
		export IRMIN_USE_HTTP=true && \
		export IRMIN_SERVER_URL=http://localhost:8080 && \
		cargo run
	@echo "✅ Desktop application started with HTTP backend!"

dev-app: ## Run desktop app in development mode with hot reload
	@echo "🛠️ Starting IrminView in development mode..."
	@cd src-tauri && \
		export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig && \
		export RUST_LOG=debug && \
		cargo run

# Combined commands
full-stack: up run-app-http ## Start Irmin server and desktop app together
	@echo "🎉 Full stack started!"

complete-setup: install-deps demo-data build up build-app ## Complete setup: install deps, create demo data, start server, build app
	@echo ""
	@echo "🎉 Complete setup finished!"
	@echo ""
	@echo "🖥️  To run desktop app (standalone): make run-app"  
	@echo "🌐 To run with server backend: make run-app-http"
	@echo "🐳 To manage Docker: make help"

# Docker Commands
build: ## Build all Docker images
	@echo "🔨 Building Docker images..."
	docker-compose build

up: ## Start the Irmin server
	@echo "🚀 Starting Irmin server..."
	docker-compose up -d irmin-server

down: ## Stop all services
	@echo "🛑 Stopping all services..."
	docker-compose down

logs: ## Show logs from all services
	@echo "📋 Showing logs..."
	docker-compose logs -f

test: ## Run tests against the Docker setup
	@echo "🧪 Testing Docker setup..."
	@echo "Starting server..."
	@docker-compose up -d irmin-server
	@echo "Waiting for server to be ready..."
	@timeout 60 sh -c 'until curl -s http://localhost:8080/health >/dev/null; do sleep 2; done'
	@echo "Testing endpoints..."
	@curl -s http://localhost:8080/health | grep -q "healthy" && echo "✅ Health check passed"
	@curl -s http://localhost:8080/api/tree >/dev/null && echo "✅ Tree endpoint accessible"
	@curl -s http://localhost:8080/api/commits >/dev/null && echo "✅ Commits endpoint accessible"
	@curl -s http://localhost:8080/api/branches >/dev/null && echo "✅ Branches endpoint accessible"
	@echo "🎉 All tests passed!"

demo-data: ## Initialize demo Irmin store data
	@echo "🧬 Creating demo Irmin store..."
	@./demo-data/init-demo.sh ./demo-irmin-store
	@echo "✅ Demo data created in ./demo-irmin-store"

dev: ## Start development environment with live reload
	@echo "🛠️ Starting development environment..."
	docker-compose -f docker-compose.dev.yml up

clean: ## Clean up Docker resources
	@echo "🧹 Cleaning up Docker resources..."
	docker-compose down -v --remove-orphans
	docker system prune -f

status: ## Show status of all services
	@echo "📊 Service Status:"
	@docker-compose ps

shell-server: ## Open shell in Irmin server container
	@echo "🐚 Opening shell in Irmin server..."
	docker-compose exec irmin-server bash

health: ## Check health of all services
	@echo "🏥 Health Check:"
	@curl -s http://localhost:8080/health 2>/dev/null | jq '.' || echo "❌ Server not responding"

# Quick start commands
quick-start: build demo-data up ## Quick start: build, create demo data, and start server
	@echo ""
	@echo "🎉 Quick start complete!"
	@echo ""
	@echo "📊 Services running:"
	@docker-compose ps
	@echo ""
	@echo "🌐 Available endpoints:"
	@echo "  Health: http://localhost:8080/health"
	@echo "  Tree:   http://localhost:8080/api/tree"
	@echo "  Commits: http://localhost:8080/api/commits"
	@echo ""
	@echo "🖥️ To run IrminView desktop app:"
	@echo "  export IRMIN_USE_HTTP=true"
	@echo "  cd src-tauri && cargo run"

# Development commands
dev-build: ## Build development images
	docker-compose -f docker-compose.dev.yml build

dev-logs: ## Show development logs
	docker-compose -f docker-compose.dev.yml logs -f

dev-down: ## Stop development environment
	docker-compose -f docker-compose.dev.yml down

# Production commands
prod-up: ## Start production environment
	docker-compose --profile testing up -d

prod-logs: ## Show production logs
	docker-compose --profile testing logs -f

prod-down: ## Stop production environment
	docker-compose --profile testing down