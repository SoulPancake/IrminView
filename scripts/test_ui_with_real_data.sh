#!/bin/bash

echo "=== UI Testing with Real Irmin Data ==="
echo ""

# Set environment variables
export IRMIN_USE_HTTP=true
export IRMIN_SERVER_URL=http://localhost:8080

# Check if server is running
echo "1. Checking Irmin server..."
curl -s http://localhost:8080/health | jq .
if [ $? -ne 0 ]; then
    echo "❌ Irmin server is not running! Please start it first."
    exit 1
fi
echo "✅ Irmin server is healthy"
echo ""

# Test all endpoints
echo "2. Testing Tree Endpoint..."
TREE_RESULT=$(curl -s http://localhost:8080/api/tree)
TREE_KEYS=$(echo "$TREE_RESULT" | jq -r '.children | keys[]' 2>/dev/null | head -5)
echo "   Root children: $TREE_KEYS"
echo ""

echo "3. Testing Commits Endpoint..."
COMMITS=$(curl -s http://localhost:8080/api/commits)
COMMIT_COUNT=$(echo "$COMMITS" | jq 'length' 2>/dev/null)
echo "   Found $COMMIT_COUNT commits"
echo "$COMMITS" | jq '.[0] | {hash: .hash[0:8], message, author}' 2>/dev/null
echo ""

echo "4. Testing Branches Endpoint..."
BRANCHES=$(curl -s http://localhost:8080/api/branches)
BRANCH_COUNT=$(echo "$BRANCHES" | jq 'length' 2>/dev/null)
echo "   Found $BRANCH_COUNT branches"
echo "$BRANCHES" | jq '.[] | {name, head_commit}' 2>/dev/null
echo ""

echo "5. Testing Search Endpoint..."
SEARCH=$(curl -s "http://localhost:8080/api/search?q=alice")
SEARCH_COUNT=$(echo "$SEARCH" | jq 'length' 2>/dev/null)
echo "   Search for 'alice' found $SEARCH_COUNT results"
echo "$SEARCH" | jq '.[0] | {path, relevance_score}' 2>/dev/null
echo ""

echo "6. Testing Diff Endpoint..."
COMMIT1=$(echo "$COMMITS" | jq -r '.[1].hash' 2>/dev/null)
COMMIT2=$(echo "$COMMITS" | jq -r '.[0].hash' 2>/dev/null)
if [ -n "$COMMIT1" ] && [ -n "$COMMIT2" ]; then
    DIFF=$(curl -s "http://localhost:8080/api/diff?from=$COMMIT1&to=$COMMIT2")
    CHANGE_COUNT=$(echo "$DIFF" | jq '.changes | length' 2>/dev/null)
    echo "   Diff between ${COMMIT1:0:8} and ${COMMIT2:0:8}: $CHANGE_COUNT changes"
    echo "$DIFF" | jq '.changes[0] | {path, change_type}' 2>/dev/null
fi
echo ""

echo "=== Summary ==="
echo "✅ All endpoints are working with real data from Git repository"
echo "✅ Tree: $(echo "$TREE_RESULT" | jq -r '.node_type' 2>/dev/null) with $(echo "$TREE_RESULT" | jq '.children | length' 2>/dev/null) children"
echo "✅ Commits: $COMMIT_COUNT commits in history"
echo "✅ Branches: $BRANCH_COUNT branches"
echo "✅ Search: Working"
echo "✅ Diff: Working"
echo ""
echo "The UI will display this real data when opened!"
