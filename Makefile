# Makefile for IrminView Docker Setup

.PHONY: help build up down logs test demo-data clean dev

# Default target
help: ## Show this help message
	@echo "🧬 IrminView Docker Commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

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