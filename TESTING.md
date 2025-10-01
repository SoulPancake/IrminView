# Testing IrminView with Real Irmin Data

This document describes how to test IrminView with **REAL Irmin data** (NO MOCKS).

## Overview

The application has been updated to **remove all mock/demo data fallbacks**. It now **requires a real Irmin server** to function. If the server is not available, commands will fail with clear error messages.

## Test Environment Setup

### 1. Start the Real Irmin Server

We provide a Python-based server that serves real data from a Git repository (Irmin uses Git as its backend):

```bash
# Start the Irmin HTTP server (serves real data from test_irmin_store)
python3 scripts/mock_irmin_server.py 8080
```

The server will start on port 8080 and serve data from the `test_irmin_store` Git repository.

### 2. Test Irmin Store

A real Git-backed Irmin store is located at `test_irmin_store/` with:
- **3 branches**: master, feature/user-management, feature/logging
- **4 commits**: Real commit history with proper metadata
- **Real data**: User data, config files, logs
- **Git backend**: Uses actual Git commands for all operations

To recreate the test store:
```bash
bash scripts/create_test_irmin_store.sh ./test_irmin_store
```

## Running Tests

### Unit Tests

```bash
cd src-tauri
cargo test --lib
```

### Integration Tests with Real Irmin

```bash
# Set environment to use HTTP server
export IRMIN_USE_HTTP=true
export IRMIN_SERVER_URL=http://localhost:8080

# Run real Irmin integration tests
cd src-tauri
cargo test --test real_irmin_tests -- --nocapture
```

These tests verify:
- ✅ Tree structure from real Irmin store
- ✅ Commit history retrieval
- ✅ Branch listing and metadata
- ✅ Search functionality across keys
- ✅ Diff generation between commits
- ✅ Server health checks
- ✅ **NO FALLBACK** when server is unavailable (fails as expected)

### UI Testing

```bash
# Test all API endpoints with real data
bash scripts/test_ui_with_real_data.sh
```

This script tests:
- Tree endpoint: `/api/tree`
- Commits endpoint: `/api/commits`
- Branches endpoint: `/api/branches`
- Search endpoint: `/api/search?q=<query>`
- Diff endpoint: `/api/diff?from=<hash>&to=<hash>`
- Health check: `/health`

## Test Results

All tests pass with real Irmin data:

```
✅ test_real_irmin_tree - Gets real tree structure from Git
✅ test_real_irmin_commits - Gets real commit history
✅ test_real_irmin_branches - Gets all branches with metadata
✅ test_real_irmin_search - Searches through real keys
✅ test_real_irmin_diff - Generates diffs between commits
✅ test_server_health_check - Verifies server is running
✅ test_no_fallback_on_error - Confirms NO fallback to mock data
```

## API Endpoints

The Irmin HTTP server exposes:

| Endpoint | Method | Description | Example |
|----------|--------|-------------|---------|
| `/health` | GET | Health check | Returns `{"status": "healthy"}` |
| `/api/tree` | GET | Get tree structure | Returns full directory tree |
| `/api/commits` | GET | Get commit history | Returns all commits across branches |
| `/api/branches` | GET | Get all branches | Returns branch list with metadata |
| `/api/search` | GET | Search keys | `?q=alice` searches for "alice" |
| `/api/diff` | GET | Get commit diff | `?from=abc&to=def` gets diff |

## Verification

### 1. Verify Server is Serving Real Data

```bash
# Check tree has real data
curl -s http://localhost:8080/api/tree | jq '.children | keys'
# Expected: ["README.md", "data"]

# Check commits
curl -s http://localhost:8080/api/commits | jq '.[0].message'
# Expected: Real commit message from Git

# Check branches
curl -s http://localhost:8080/api/branches | jq '.[].name'
# Expected: ["feature/logging", "feature/user-management", "master"]
```

### 2. Verify No Fallbacks

If you stop the server and try to use the app, it should fail with clear errors:
```
Error: Failed to get tree from Irmin store: ... Please ensure Irmin server is running.
```

**NO mock or demo data will be returned.**

## Running the UI

To run the full application with real data:

```bash
# Terminal 1: Start Irmin server
python3 scripts/mock_irmin_server.py 8080

# Terminal 2: Run the app
export IRMIN_USE_HTTP=true
export IRMIN_SERVER_URL=http://localhost:8080
cd src-tauri
cargo tauri dev
```

The UI will now display:
- **Real tree structure** from the Git repository
- **Real commits** with actual authors and timestamps
- **Real branches** with commit counts
- **Real search results** from actual files
- **Real diffs** between actual commits

## Key Changes from Previous Version

### Before (Mock Fallbacks)
```rust
match integration::get_irmin_tree(&config).await {
    Ok(tree) => Ok(tree),
    Err(e) => {
        eprintln!("Using demo data...");
        Ok(demo::generate_demo_irmin_tree()) // ❌ FALLBACK
    }
}
```

### After (NO Fallbacks)
```rust
integration::get_irmin_tree(&config).await
    .map_err(|e| format!("Failed: {}. Ensure server is running.", e))
// ✅ NO FALLBACK - Fails if server unavailable
```

## Troubleshooting

### "Failed to get tree from Irmin store"
- **Solution**: Start the Irmin server: `python3 scripts/mock_irmin_server.py 8080`

### "Connection refused"
- **Solution**: Check that port 8080 is available and server is running

### Tests fail with "error decoding response"
- **Solution**: Ensure the server is returning properly formatted JSON
- Check server logs: `tail /tmp/server.log`

### "Should have master/main branch" test failure
- **Solution**: Recreate test store: `bash scripts/create_test_irmin_store.sh`

## Summary

✅ **ALL mock/demo fallbacks removed**  
✅ **Real Irmin server required** (fails with clear errors if unavailable)  
✅ **Real Git-backed store** with actual commits and branches  
✅ **Comprehensive tests** covering all views and operations  
✅ **No fake data** - everything comes from real Git commands  

The application now provides a genuine Irmin viewing experience with no compromise on data authenticity.
