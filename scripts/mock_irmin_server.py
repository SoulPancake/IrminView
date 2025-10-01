#!/usr/bin/env python3
"""
Mock Irmin HTTP Server for testing
This server provides real data from a Git repository, simulating an Irmin store
"""

import json
import subprocess
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
from datetime import datetime
import os
import sys

STORE_PATH = os.path.join(os.path.dirname(__file__), "../test_irmin_store")

class IrminHandler(BaseHTTPRequestHandler):
    
    def do_OPTIONS(self):
        self.send_cors_headers()
        self.end_headers()
    
    def send_cors_headers(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.send_header('Content-Type', 'application/json')
    
    def do_GET(self):
        parsed_path = urlparse(self.path)
        path = parsed_path.path
        
        try:
            if path == '/health':
                self.handle_health()
            elif path == '/api/tree':
                self.handle_tree()
            elif path == '/api/commits':
                self.handle_commits()
            elif path == '/api/branches':
                self.handle_branches()
            elif path == '/api/search':
                query_params = parse_qs(parsed_path.query)
                query = query_params.get('q', [''])[0]
                self.handle_search(query)
            elif path == '/api/diff':
                query_params = parse_qs(parsed_path.query)
                from_commit = query_params.get('from', [''])[0]
                to_commit = query_params.get('to', [''])[0]
                self.handle_diff(from_commit, to_commit)
            else:
                self.send_error(404, 'Not Found')
        except Exception as e:
            self.send_error(500, str(e))
    
    def handle_health(self):
        self.send_cors_headers()
        self.end_headers()
        response = {"status": "healthy", "service": "mock-irmin-server"}
        self.wfile.write(json.dumps(response).encode())
    
    def handle_tree(self):
        """Build tree structure from Git repository"""
        tree = self.build_tree_from_git()
        self.send_cors_headers()
        self.end_headers()
        self.wfile.write(json.dumps(tree).encode())
    
    def handle_commits(self):
        """Get commit history from Git"""
        os.chdir(STORE_PATH)
        result = subprocess.run(
            ['git', 'log', '--all', '--format=%H|%s|%an|%aI|%P'],
            capture_output=True, text=True
        )
        
        commits = []
        for line in result.stdout.strip().split('\n'):
            if not line:
                continue
            parts = line.split('|')
            commit_hash = parts[0]
            message = parts[1] if len(parts) > 1 else ''
            author = parts[2] if len(parts) > 2 else 'Unknown'
            timestamp = parts[3] if len(parts) > 3 else ''
            parents = parts[4].split() if len(parts) > 4 else []
            
            # Get branch for this commit
            branch_result = subprocess.run(
                ['git', 'branch', '--contains', commit_hash],
                capture_output=True, text=True
            )
            branches = [b.strip('* \n') for b in branch_result.stdout.split('\n') if b.strip()]
            branch = branches[0] if branches else 'master'
            
            commits.append({
                "hash": commit_hash,
                "message": message,
                "author": author,
                "timestamp": timestamp,
                "parents": parents,
                "branch": branch
            })
        
        self.send_cors_headers()
        self.end_headers()
        self.wfile.write(json.dumps(commits).encode())
    
    def handle_branches(self):
        """Get branches from Git"""
        os.chdir(STORE_PATH)
        result = subprocess.run(
            ['git', 'branch', '-v'],
            capture_output=True, text=True
        )
        
        branches = []
        for line in result.stdout.split('\n'):
            if not line.strip():
                continue
            parts = line.strip().split()
            if len(parts) < 2:
                continue
            
            name = parts[0].strip('*').strip()
            head_commit = parts[1]
            
            # Get commit count
            count_result = subprocess.run(
                ['git', 'rev-list', '--count', name],
                capture_output=True, text=True
            )
            try:
                commit_count = int(count_result.stdout.strip())
            except ValueError:
                commit_count = 0
            
            # Get last updated time
            log_result = subprocess.run(
                ['git', 'log', '-1', '--format=%aI', name],
                capture_output=True, text=True
            )
            last_updated = log_result.stdout.strip()
            
            branches.append({
                "name": name,
                "head_commit": head_commit,
                "last_updated": last_updated,
                "commit_count": commit_count
            })
        
        self.send_cors_headers()
        self.end_headers()
        self.wfile.write(json.dumps(branches).encode())
    
    def handle_search(self, query):
        """Search for files matching query"""
        os.chdir(STORE_PATH)
        result = subprocess.run(
            ['git', 'ls-files'],
            capture_output=True, text=True
        )
        
        files = result.stdout.strip().split('\n')
        results = []
        
        for file_path in files:
            if query.lower() in file_path.lower():
                relevance = 1.0 if query.lower() == os.path.basename(file_path).lower() else 0.5
                
                # Read file content
                try:
                    with open(file_path, 'r') as f:
                        content = f.read()
                except:
                    content = None
                
                # Get file stats
                stat = os.stat(file_path)
                
                node = {
                    "key": os.path.basename(file_path),
                    "value": content,
                    "node_type": "file",
                    "children": {},
                    "metadata": {
                        "last_modified": datetime.fromtimestamp(stat.st_mtime).isoformat() + 'Z',
                        "size": stat.st_size,
                        "permissions": oct(stat.st_mode)[-3:]
                    }
                }
                
                results.append({
                    "path": file_path,
                    "node": node,
                    "relevance_score": relevance
                })
        
        self.send_cors_headers()
        self.end_headers()
        self.wfile.write(json.dumps(results).encode())
    
    def handle_diff(self, from_commit, to_commit):
        """Get diff between commits"""
        os.chdir(STORE_PATH)
        result = subprocess.run(
            ['git', 'diff', '--name-status', from_commit, to_commit],
            capture_output=True, text=True
        )
        
        changes = []
        for line in result.stdout.strip().split('\n'):
            if not line:
                continue
            parts = line.split('\t')
            if len(parts) < 2:
                continue
            
            status = parts[0]
            file_path = parts[1]
            
            change_type = {
                'A': 'added',
                'M': 'modified',
                'D': 'deleted'
            }.get(status, 'modified')
            
            # Get old and new content
            old_value = None
            new_value = None
            
            if change_type != 'added':
                try:
                    old_result = subprocess.run(
                        ['git', 'show', f'{from_commit}:{file_path}'],
                        capture_output=True, text=True
                    )
                    old_value = old_result.stdout
                except:
                    pass
            
            if change_type != 'deleted':
                try:
                    new_result = subprocess.run(
                        ['git', 'show', f'{to_commit}:{file_path}'],
                        capture_output=True, text=True
                    )
                    new_value = new_result.stdout
                except:
                    pass
            
            changes.append({
                "path": file_path,
                "change_type": change_type,
                "old_value": old_value,
                "new_value": new_value
            })
        
        diff = {
            "from_commit": from_commit,
            "to_commit": to_commit,
            "changes": changes
        }
        
        self.send_cors_headers()
        self.end_headers()
        self.wfile.write(json.dumps(diff).encode())
    
    def build_tree_from_git(self):
        """Build tree structure from Git files"""
        os.chdir(STORE_PATH)
        result = subprocess.run(
            ['git', 'ls-files'],
            capture_output=True, text=True
        )
        
        files = result.stdout.strip().split('\n')
        
        root = {
            "key": "root",
            "value": None,
            "node_type": "directory",
            "children": {},
            "metadata": {
                "last_modified": datetime.now().isoformat() + 'Z',
                "size": None,
                "permissions": "755"
            }
        }
        
        for file_path in files:
            if not file_path:
                continue
            
            parts = file_path.split('/')
            current = root
            
            for i, part in enumerate(parts):
                is_last = i == len(parts) - 1
                
                if part not in current["children"]:
                    if is_last:
                        # It's a file
                        try:
                            with open(file_path, 'r') as f:
                                content = f.read()
                        except:
                            content = None
                        
                        stat = os.stat(file_path)
                        current["children"][part] = {
                            "key": part,
                            "value": content,
                            "node_type": "file",
                            "children": {},
                            "metadata": {
                                "last_modified": datetime.fromtimestamp(stat.st_mtime).isoformat() + 'Z',
                                "size": stat.st_size,
                                "permissions": oct(stat.st_mode)[-3:]
                            }
                        }
                    else:
                        # It's a directory
                        current["children"][part] = {
                            "key": part,
                            "value": None,
                            "node_type": "directory",
                            "children": {},
                            "metadata": {
                                "last_modified": datetime.now().isoformat() + 'Z',
                                "size": None,
                                "permissions": "755"
                            }
                        }
                
                current = current["children"][part]
        
        return root

def run_server(port=8080):
    server_address = ('', port)
    httpd = HTTPServer(server_address, IrminHandler)
    print(f'Mock Irmin Server running on http://localhost:{port}')
    print(f'Using store: {STORE_PATH}')
    print('Available endpoints:')
    print('  GET /health - Health check')
    print('  GET /api/tree - Get tree structure')
    print('  GET /api/commits - Get commit history')
    print('  GET /api/branches - Get branches')
    print('  GET /api/search?q=<query> - Search keys')
    print('  GET /api/diff?from=<hash>&to=<hash> - Get diff')
    print('\nPress Ctrl+C to stop the server')
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print('\nShutting down server...')
        httpd.shutdown()

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
    run_server(port)
