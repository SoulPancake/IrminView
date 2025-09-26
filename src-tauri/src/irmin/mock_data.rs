use super::types::*;
use chrono::{Utc};
use std::collections::HashMap;

/// Generate mock tree data for testing and demo purposes
pub fn generate_mock_tree() -> IrminNode {
    let now = Utc::now();
    
    let mut root_children = HashMap::new();
    
    // Create some files
    root_children.insert("config.json".to_string(), IrminNode {
        key: "config.json".to_string(),
        value: Some(r#"{"database": {"host": "localhost", "port": 5432}}"#.to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::days(1),
            size: Some(45),
            permissions: Some("644".to_string()),
        },
    });
    
    root_children.insert("README.md".to_string(), IrminNode {
        key: "README.md".to_string(),
        value: Some("# Irmin Store\n\nThis is a sample Irmin store with demo data.".to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::hours(2),
            size: Some(58),
            permissions: Some("644".to_string()),
        },
    });
    
    // Create a directory with nested content
    let mut users_children = HashMap::new();
    users_children.insert("alice.json".to_string(), IrminNode {
        key: "alice.json".to_string(),
        value: Some(r#"{"name": "Alice", "email": "alice@example.com", "role": "admin"}"#.to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::hours(6),
            size: Some(65),
            permissions: Some("644".to_string()),
        },
    });
    
    users_children.insert("bob.json".to_string(), IrminNode {
        key: "bob.json".to_string(),
        value: Some(r#"{"name": "Bob", "email": "bob@example.com", "role": "user"}"#.to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::hours(4),
            size: Some(62),
            permissions: Some("644".to_string()),
        },
    });
    
    root_children.insert("users".to_string(), IrminNode {
        key: "users".to_string(),
        value: None,
        node_type: NodeType::Directory,
        children: users_children,
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::hours(4),
            size: None,
            permissions: Some("755".to_string()),
        },
    });
    
    // Create another directory
    let mut logs_children = HashMap::new();
    logs_children.insert("app.log".to_string(), IrminNode {
        key: "app.log".to_string(),
        value: Some("2023-01-01 10:00:00 INFO Application started\n2023-01-01 10:05:00 INFO User logged in".to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::minutes(30),
            size: Some(85),
            permissions: Some("644".to_string()),
        },
    });
    
    root_children.insert("logs".to_string(), IrminNode {
        key: "logs".to_string(),
        value: None,
        node_type: NodeType::Directory,
        children: logs_children,
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::minutes(30),
            size: None,
            permissions: Some("755".to_string()),
        },
    });
    
    IrminNode {
        key: "/".to_string(),
        value: None,
        node_type: NodeType::Directory,
        children: root_children,
        metadata: NodeMetadata {
            last_modified: now,
            size: None,
            permissions: Some("755".to_string()),
        },
    }
}

/// Generate mock commits for demo purposes
pub fn generate_mock_commits() -> Vec<IrminCommit> {
    let now = Utc::now();
    
    vec![
        IrminCommit {
            hash: "a1b2c3d4e5f6".to_string(),
            message: "Add user management system".to_string(),
            author: "Alice <alice@example.com>".to_string(),
            timestamp: now - chrono::Duration::hours(2),
            parents: vec!["f6e5d4c3b2a1".to_string()],
            branch: "main".to_string(),
        },
        IrminCommit {
            hash: "f6e5d4c3b2a1".to_string(),
            message: "Update configuration schema".to_string(),
            author: "Bob <bob@example.com>".to_string(),
            timestamp: now - chrono::Duration::hours(6),
            parents: vec!["b2a1f6e5d4c3".to_string()],
            branch: "main".to_string(),
        },
        IrminCommit {
            hash: "b2a1f6e5d4c3".to_string(),
            message: "Initial commit with basic structure".to_string(),
            author: "Admin <admin@example.com>".to_string(),
            timestamp: now - chrono::Duration::days(1),
            parents: vec![],
            branch: "main".to_string(),
        },
        IrminCommit {
            hash: "d4c3b2a1f6e5".to_string(),
            message: "Add logging functionality".to_string(),
            author: "Charlie <charlie@example.com>".to_string(),
            timestamp: now - chrono::Duration::hours(1),
            parents: vec!["a1b2c3d4e5f6".to_string()],
            branch: "feature/logging".to_string(),
        },
    ]
}

/// Generate mock branches
pub fn generate_mock_branches() -> Vec<IrminBranch> {
    let now = Utc::now();
    
    vec![
        IrminBranch {
            name: "main".to_string(),
            head_commit: "a1b2c3d4e5f6".to_string(),
            last_updated: now - chrono::Duration::hours(2),
            commit_count: 3,
        },
        IrminBranch {
            name: "feature/logging".to_string(),
            head_commit: "d4c3b2a1f6e5".to_string(),
            last_updated: now - chrono::Duration::hours(1),
            commit_count: 1,
        },
    ]
}

/// Generate a mock diff between two commits
pub fn generate_mock_diff(from: &str, to: &str) -> IrminDiff {
    IrminDiff {
        from_commit: from.to_string(),
        to_commit: to.to_string(),
        changes: vec![
            DiffChange {
                path: "/users/alice.json".to_string(),
                change_type: ChangeType::Added,
                old_value: None,
                new_value: Some(r#"{"name": "Alice", "email": "alice@example.com", "role": "admin"}"#.to_string()),
            },
            DiffChange {
                path: "/config.json".to_string(),
                change_type: ChangeType::Modified,
                old_value: Some(r#"{"database": {"host": "localhost", "port": 5432}}"#.to_string()),
                new_value: Some(r#"{"database": {"host": "localhost", "port": 5432}, "version": "1.0"}"#.to_string()),
            },
            DiffChange {
                path: "/old_file.txt".to_string(),
                change_type: ChangeType::Deleted,
                old_value: Some("This file was removed".to_string()),
                new_value: None,
            },
        ],
    }
}