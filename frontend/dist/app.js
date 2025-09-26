// Tauri API wrapper
const invoke = window.__TAURI__ ? window.__TAURI__.core.invoke : async (cmd, args) => {
    console.log(`Mock invoke: ${cmd}`, args);
    // Mock responses for development without Tauri
    switch (cmd) {
        case 'get_mock_tree':
            return mockTreeData();
        case 'get_mock_commits':
            return mockCommitsData();
        case 'get_mock_branches':
            return mockBranchesData();
        case 'search_keys':
            return mockSearchResults(args.query);
        case 'get_commit_diff':
            return mockDiffData(args.from_commit, args.to_commit);
        default:
            return null;
    }
};

// Application state
let currentView = 'tree';
let currentTheme = 'light';
let treeData = null;
let commitsData = [];
let branchesData = [];

// Initialize application
document.addEventListener('DOMContentLoaded', async () => {
    initializeEventListeners();
    await loadInitialData();
    updateView('tree');
});

// Event listeners
function initializeEventListeners() {
    // Navigation
    document.querySelectorAll('.nav-item').forEach(item => {
        item.addEventListener('click', (e) => {
            const view = e.currentTarget.dataset.view;
            updateView(view);
        });
    });

    // Theme toggle
    document.getElementById('theme-toggle').addEventListener('click', toggleTheme);

    // Search
    document.getElementById('search-btn').addEventListener('click', performSearch);
    document.getElementById('search-input').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            performSearch();
        }
    });

    // Diff controls
    document.getElementById('show-diff').addEventListener('click', showDiff);

    // Refresh
    document.getElementById('refresh-btn').addEventListener('click', async () => {
        await loadInitialData();
        updateCurrentView();
    });
}

// Load initial data
async function loadInitialData() {
    try {
        treeData = await invoke('get_mock_tree');
        commitsData = await invoke('get_mock_commits');
        branchesData = await invoke('get_mock_branches');
        
        // Populate commit selects
        populateCommitSelects();
        
        console.log('Data loaded successfully');
    } catch (error) {
        console.error('Error loading data:', error);
    }
}

// Update view
function updateView(viewName) {
    // Update navigation
    document.querySelectorAll('.nav-item').forEach(item => {
        item.classList.toggle('active', item.dataset.view === viewName);
    });

    // Update content
    document.querySelectorAll('.view').forEach(view => {
        view.classList.toggle('active', view.id === `${viewName}-view`);
    });

    currentView = viewName;
    updateCurrentView();
}

// Update current view content
function updateCurrentView() {
    switch (currentView) {
        case 'tree':
            renderTree();
            break;
        case 'commits':
            renderCommits();
            break;
        case 'branches':
            renderBranches();
            break;
        case 'diff':
            // Diff is rendered on demand
            break;
    }
}

// Render tree view
function renderTree() {
    const container = document.getElementById('tree-container');
    if (!treeData) {
        container.innerHTML = '<div class="loading">Loading tree...</div>';
        return;
    }

    container.innerHTML = renderTreeNode(treeData, 0);
    updateItemCount(countTreeNodes(treeData));
}

// Render tree node recursively
function renderTreeNode(node, level) {
    const isDirectory = node.node_type === 'Directory';
    const icon = isDirectory ? '📁' : '📄';
    const sizeInfo = node.metadata.size ? `${formatBytes(node.metadata.size)}` : '';
    
    let html = `
        <div class="tree-node ${isDirectory ? 'expandable' : ''}" style="margin-left: ${level * 20}px">
            <div class="tree-node-content" onclick="toggleTreeNode(this)">
                <span class="tree-node-icon">${icon}</span>
                <span class="tree-node-name">${node.key}</span>
                <span class="tree-node-size">${sizeInfo}</span>
            </div>
    `;

    if (isDirectory && Object.keys(node.children).length > 0) {
        html += '<div class="tree-node-children">';
        for (const child of Object.values(node.children)) {
            html += renderTreeNode(child, level + 1);
        }
        html += '</div>';
    }

    html += '</div>';
    return html;
}

// Toggle tree node expansion
function toggleTreeNode(element) {
    const node = element.parentElement;
    node.classList.toggle('expanded');
}

// Render commits view
function renderCommits() {
    const container = document.getElementById('commits-container');
    if (!commitsData.length) {
        container.innerHTML = '<div class="loading">Loading commits...</div>';
        return;
    }

    const html = commitsData.map(commit => `
        <div class="commit-item">
            <div class="commit-hash">${commit.hash}</div>
            <div class="commit-message">${commit.message}</div>
            <div class="commit-meta">
                ${commit.author} • ${formatDate(commit.timestamp)} • ${commit.branch}
            </div>
        </div>
    `).join('');

    container.innerHTML = html;
}

// Render branches view
function renderBranches() {
    const container = document.getElementById('branches-container');
    if (!branchesData.length) {
        container.innerHTML = '<div class="loading">Loading branches...</div>';
        return;
    }

    const html = branchesData.map(branch => `
        <div class="branch-card">
            <div class="branch-name">${branch.name}</div>
            <div class="branch-info">
                <div>Head: ${branch.head_commit}</div>
                <div>Commits: ${branch.commit_count}</div>
                <div>Updated: ${formatDate(branch.last_updated)}</div>
            </div>
        </div>
    `).join('');

    container.innerHTML = html;
}

// Populate commit selects for diff viewer
function populateCommitSelects() {
    const fromSelect = document.getElementById('from-commit');
    const toSelect = document.getElementById('to-commit');
    
    const options = commitsData.map(commit => 
        `<option value="${commit.hash}">${commit.hash} - ${commit.message}</option>`
    ).join('');
    
    fromSelect.innerHTML = '<option value="">Select from commit...</option>' + options;
    toSelect.innerHTML = '<option value="">Select to commit...</option>' + options;
}

// Show diff
async function showDiff() {
    const fromCommit = document.getElementById('from-commit').value;
    const toCommit = document.getElementById('to-commit').value;
    
    if (!fromCommit || !toCommit) {
        alert('Please select both commits');
        return;
    }

    try {
        const diff = await invoke('get_commit_diff', { from_commit: fromCommit, to_commit: toCommit });
        renderDiff(diff);
    } catch (error) {
        console.error('Error getting diff:', error);
    }
}

// Render diff
function renderDiff(diff) {
    const container = document.getElementById('diff-container');
    
    if (!diff.changes.length) {
        container.innerHTML = '<div class="empty-state">No changes found</div>';
        return;
    }

    const html = diff.changes.map(change => {
        let changeTypeIcon;
        let changeTypeClass;
        
        switch (change.change_type) {
            case 'Added':
                changeTypeIcon = '✅';
                changeTypeClass = 'added';
                break;
            case 'Modified':
                changeTypeIcon = '✏️';
                changeTypeClass = 'modified';
                break;
            case 'Deleted':
                changeTypeIcon = '🗑️';
                changeTypeClass = 'deleted';
                break;
        }

        let content = '';
        if (change.old_value && change.new_value) {
            content = `
                <div class="diff-line-old">- ${change.old_value}</div>
                <div class="diff-line-new">+ ${change.new_value}</div>
            `;
        } else if (change.new_value) {
            content = `<div class="diff-line-new">+ ${change.new_value}</div>`;
        } else if (change.old_value) {
            content = `<div class="diff-line-old">- ${change.old_value}</div>`;
        }

        return `
            <div class="diff-change ${changeTypeClass}">
                <div class="diff-change-header">
                    <span>${changeTypeIcon}</span>
                    <span>${change.path}</span>
                    <span>(${change.change_type})</span>
                </div>
                <div class="diff-change-content">${content}</div>
            </div>
        `;
    }).join('');

    container.innerHTML = html;
}

// Perform search
async function performSearch() {
    const query = document.getElementById('search-input').value.trim();
    if (!query) return;

    try {
        const results = await invoke('search_keys', { query });
        renderSearchResults(results);
    } catch (error) {
        console.error('Search error:', error);
    }
}

// Render search results
function renderSearchResults(results) {
    const container = document.getElementById('search-results');
    
    if (!results.length) {
        container.innerHTML = '<div class="empty-state">No results found</div>';
        return;
    }

    const html = results.map(result => {
        const typeIcon = result.node.node_type === 'Directory' ? '📁' : '📄';
        return `
            <div class="search-result" onclick="highlightTreeNode('${result.path}')">
                <div class="search-result-path">${typeIcon} ${result.path}</div>
                <div class="search-result-type">${result.node.node_type}</div>
            </div>
        `;
    }).join('');

    container.innerHTML = html;
}

// Highlight tree node (placeholder for future implementation)
function highlightTreeNode(path) {
    // Switch to tree view and highlight the node
    updateView('tree');
    console.log('Highlighting node:', path);
}

// Toggle theme
async function toggleTheme() {
    try {
        const newTheme = await invoke('toggle_theme');
        currentTheme = newTheme.toLowerCase();
        document.documentElement.setAttribute('data-theme', currentTheme === 'dark' ? 'dark' : 'light');
        
        // Update theme toggle icon
        const themeIcon = document.querySelector('.theme-icon');
        themeIcon.textContent = currentTheme === 'dark' ? '☀️' : '🌙';
    } catch (error) {
        // Fallback for mock mode
        currentTheme = currentTheme === 'light' ? 'dark' : 'light';
        document.documentElement.setAttribute('data-theme', currentTheme);
        const themeIcon = document.querySelector('.theme-icon');
        themeIcon.textContent = currentTheme === 'dark' ? '☀️' : '🌙';
    }
}

// Utility functions
function formatBytes(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function countTreeNodes(node) {
    let count = 1;
    for (const child of Object.values(node.children || {})) {
        count += countTreeNodes(child);
    }
    return count;
}

function updateItemCount(count) {
    document.getElementById('item-count').textContent = `${count} items`;
}

// Mock data for development without Tauri
function mockTreeData() {
    return {
        key: "/",
        value: null,
        node_type: "Directory",
        children: {
            "config.json": {
                key: "config.json",
                value: '{"database": {"host": "localhost", "port": 5432}}',
                node_type: "File",
                children: {},
                metadata: {
                    last_modified: new Date().toISOString(),
                    size: 45,
                    permissions: "644"
                }
            },
            "users": {
                key: "users",
                value: null,
                node_type: "Directory",
                children: {
                    "alice.json": {
                        key: "alice.json",
                        value: '{"name": "Alice", "email": "alice@example.com"}',
                        node_type: "File",
                        children: {},
                        metadata: {
                            last_modified: new Date().toISOString(),
                            size: 65,
                            permissions: "644"
                        }
                    }
                },
                metadata: {
                    last_modified: new Date().toISOString(),
                    size: null,
                    permissions: "755"
                }
            }
        },
        metadata: {
            last_modified: new Date().toISOString(),
            size: null,
            permissions: "755"
        }
    };
}

function mockCommitsData() {
    return [
        {
            hash: "a1b2c3d4e5f6",
            message: "Add user management system",
            author: "Alice <alice@example.com>",
            timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
            parents: ["f6e5d4c3b2a1"],
            branch: "main"
        },
        {
            hash: "f6e5d4c3b2a1",
            message: "Update configuration schema",
            author: "Bob <bob@example.com>",
            timestamp: new Date(Date.now() - 6 * 60 * 60 * 1000).toISOString(),
            parents: ["b2a1f6e5d4c3"],
            branch: "main"
        }
    ];
}

function mockBranchesData() {
    return [
        {
            name: "main",
            head_commit: "a1b2c3d4e5f6",
            last_updated: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
            commit_count: 3
        },
        {
            name: "feature/logging",
            head_commit: "d4c3b2a1f6e5",
            last_updated: new Date(Date.now() - 1 * 60 * 60 * 1000).toISOString(),
            commit_count: 1
        }
    ];
}

function mockSearchResults(query) {
    return [
        {
            path: "/config.json",
            node: {
                key: "config.json",
                node_type: "File"
            },
            relevance_score: 0.8
        }
    ];
}

function mockDiffData(fromCommit, toCommit) {
    return {
        from_commit: fromCommit,
        to_commit: toCommit,
        changes: [
            {
                path: "/users/alice.json",
                change_type: "Added",
                old_value: null,
                new_value: '{"name": "Alice", "email": "alice@example.com", "role": "admin"}'
            },
            {
                path: "/config.json",
                change_type: "Modified",
                old_value: '{"database": {"host": "localhost", "port": 5432}}',
                new_value: '{"database": {"host": "localhost", "port": 5432}, "version": "1.0"}'
            }
        ]
    };
}