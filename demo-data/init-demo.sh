#!/bin/bash

# Initialize a demo Irmin store with sample data
set -e

STORE_PATH=${1:-./demo-irmin-store}

echo "🧬 Initializing demo Irmin store at: $STORE_PATH"

# Create store directory
mkdir -p "$STORE_PATH"
cd "$STORE_PATH"

# Initialize git repository (Irmin uses Git as backend)
git init .
git config user.name "Irmin Demo"
git config user.email "demo@irmin.org"

# Create some demo files to simulate an Irmin store structure
echo "Creating demo data structure..."

# Create users directory
mkdir -p users
echo '{"name": "Alice Smith", "email": "alice@example.com", "role": "admin", "created": "2024-01-15"}' > users/alice.json
echo '{"name": "Bob Johnson", "email": "bob@example.com", "role": "user", "created": "2024-01-16"}' > users/bob.json
echo '{"name": "Carol Davis", "email": "carol@example.com", "role": "moderator", "created": "2024-01-17"}' > users/carol.json

# Create config directory
mkdir -p config
echo '{"database": {"host": "localhost", "port": 5432, "name": "irmin_db"}, "version": "1.0.0"}' > config/settings.json
echo '{"cache": {"ttl": 3600, "size": 1000}, "logging": {"level": "info"}}' > config/system.json

# Create data directory
mkdir -p data/metrics
echo '{"active_users": 150, "total_requests": 45623, "uptime": 864000}' > data/metrics/stats.json
echo '{"timestamp": "2024-01-20T10:30:00Z", "memory_usage": "256MB", "cpu_usage": "12%"}' > data/metrics/system.json

# Create documentation
mkdir -p docs
echo "# Irmin Demo Store

This is a demonstration Irmin store containing sample user data, configurations, and metrics.

## Structure

- \`users/\` - User account information
- \`config/\` - System configuration files  
- \`data/\` - Application data and metrics
- \`docs/\` - Documentation files

## Usage

This store can be explored using IrminView or accessed via the Irmin bridge server.
" > docs/README.md

# Add and commit initial data
git add .
git commit -m "Initial demo data setup

- Add user accounts (alice, bob, carol)
- Add system configuration files
- Add metrics and monitoring data
- Add documentation"

# Create additional commits to show history
echo '{"name": "Alice Smith", "email": "alice@example.com", "role": "admin", "created": "2024-01-15", "last_login": "2024-01-20T09:15:00Z"}' > users/alice.json
git add users/alice.json
git commit -m "Update Alice's last login timestamp"

echo '{"name": "David Wilson", "email": "david@example.com", "role": "user", "created": "2024-01-21"}' > users/david.json
git add users/david.json
git commit -m "Add new user: David Wilson"

# Update metrics
echo '{"active_users": 175, "total_requests": 48932, "uptime": 891600}' > data/metrics/stats.json
git add data/metrics/stats.json
git commit -m "Update system metrics

- Active users increased to 175
- Total requests: 48,932
- System uptime: 10+ days"

# Create a feature branch to demonstrate branching
git checkout -b feature/user-preferences
mkdir -p preferences
echo '{"alice": {"theme": "dark", "language": "en", "notifications": true}}' > preferences/alice.json
echo '{"bob": {"theme": "light", "language": "en", "notifications": false}}' > preferences/bob.json
git add preferences/
git commit -m "Add user preferences system

- Dark/light theme selection
- Language preferences
- Notification settings"

# Switch back to main and merge
git checkout main
git merge feature/user-preferences -m "Merge user preferences feature

Adds customizable user preferences including themes,
language settings, and notification controls."

# Create some more realistic commits
echo '{"cache": {"ttl": 7200, "size": 2000}, "logging": {"level": "debug"}}' > config/system.json
git add config/system.json
git commit -m "Increase cache settings for better performance"

echo '{"database": {"host": "db.example.com", "port": 5432, "name": "irmin_prod", "ssl": true}, "version": "1.1.0"}' > config/settings.json  
git add config/settings.json
git commit -m "Update database configuration for production

- Switch to production database host
- Enable SSL connections
- Bump version to 1.1.0"

echo "✅ Demo Irmin store initialized successfully!"
echo "📁 Store location: $STORE_PATH"
echo "🌿 Branches: $(git branch --list)"
echo "📝 Commits: $(git log --oneline | wc -l) total"
echo ""
echo "You can now:"
echo "1. Start the Irmin server: docker-compose up irmin-server"
echo "2. Point IrminView to this store: IRMIN_STORE_PATH=$STORE_PATH"
echo "3. Explore the data through the UI"