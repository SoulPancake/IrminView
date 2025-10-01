#!/bin/bash
set -e
STORE_PATH="${1:-./test_irmin_store}"
echo "Creating test Irmin store at: $STORE_PATH"
mkdir -p "$STORE_PATH"
cd "$STORE_PATH"
git init
mkdir -p data/users data/config data/logs
cat > data/config/app.json << 'EOF'
{"database": {"host": "localhost", "port": 5432}, "cache": {"enabled": true, "ttl": 3600}}
EOF
cat > data/users/alice.json << 'EOF'
{"id": 1, "name": "Alice Smith", "email": "alice@example.com", "role": "admin"}
EOF
cat > data/users/bob.json << 'EOF'
{"id": 2, "name": "Bob Johnson", "email": "bob@example.com", "role": "user"}
EOF
echo "Test logs" > data/logs/access.log
cat > README.md << 'EOF'
# Test Irmin Store
This is a test Irmin store for IrminView.
EOF
git config user.email "test@irminview.local"
git config user.name "IrminView Test"
git add .
git commit -m "Initial commit: Setup test Irmin store structure"
git checkout -b feature/user-management
echo '{"id": 3, "name": "Charlie Brown", "email": "charlie@example.com", "role": "user"}' > data/users/charlie.json
git add data/users/charlie.json
git commit -m "Add user Charlie"
git checkout main
echo "" >> data/config/app.json
git add data/config/app.json
git commit -m "Update app config"
git checkout -b feature/logging
mkdir -p data/logs/archive
echo "Archived logs" > data/logs/archive/2024-09.log
git add data/logs/archive/
git commit -m "Add archived logs directory"
git checkout main
echo "✅ Test Irmin store created at: $STORE_PATH"
git branch
git log --oneline --all --graph
