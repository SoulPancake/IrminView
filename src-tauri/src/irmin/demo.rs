// Demo module to show how the Irmin integration would work
// This creates a mock OCaml bridge process for testing purposes

use super::types::*;
use std::collections::HashMap;
use chrono::Utc;

/// Generate demo data that would come from a real Irmin store
pub fn generate_demo_irmin_tree() -> IrminNode {
    let now = Utc::now();
    
    let mut root_children = HashMap::new();
    
    // Create files that look like they came from a real Irmin store
    root_children.insert("users.json".to_string(), IrminNode {
        key: "users.json".to_string(),
        value: Some(r#"{"admin": {"name": "Administrator", "role": "admin"}, "user1": {"name": "John Doe", "role": "user"}}"#.to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::days(2),
            size: Some(98),
            permissions: Some("644".to_string()),
        },
    });
    
    root_children.insert("config.toml".to_string(), IrminNode {
        key: "config.toml".to_string(),
        value: Some("[database]\nhost = \"localhost\"\nport = 5432\n[cache]\nttl = 3600".to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::hours(12),
            size: Some(56),
            permissions: Some("644".to_string()),
        },
    });
    
    // Create a directory structure that shows real Irmin usage
    let mut data_children = HashMap::new();
    data_children.insert("store_info.json".to_string(), IrminNode {
        key: "store_info.json".to_string(),
        value: Some(r#"{"format": "irmin-git", "version": "3.9.0", "created": "2024-01-15T10:30:00Z"}"#.to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::days(30),
            size: Some(78),
            permissions: Some("644".to_string()),
        },
    });
    
    data_children.insert("keys.txt".to_string(), IrminNode {
        key: "keys.txt".to_string(),
        value: Some("users/alice\nusers/bob\nconfig/settings\ndata/metrics".to_string()),
        node_type: NodeType::File,
        children: HashMap::new(),
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::hours(6),
            size: Some(45),
            permissions: Some("644".to_string()),
        },
    });
    
    root_children.insert("data".to_string(), IrminNode {
        key: "data".to_string(),
        value: None,
        node_type: NodeType::Directory,
        children: data_children,
        metadata: NodeMetadata {
            last_modified: now - chrono::Duration::days(1),
            size: None,
            permissions: Some("755".to_string()),
        },
    });
    
    IrminNode {
        key: "root".to_string(),
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

pub fn generate_demo_irmin_commits() -> Vec<IrminCommit> {
    let now = Utc::now();
    
    vec![
        IrminCommit {
            hash: "f47ac10b58cc4372a5670e02b2c3d479".to_string(),
            message: "Initial Irmin store setup with user data".to_string(),
            author: "System <system@irminview.com>".to_string(),
            timestamp: now - chrono::Duration::days(30),
            parents: vec![],
            branch: "main".to_string(),
        },
        IrminCommit {
            hash: "6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_string(),
            message: "Add configuration management".to_string(),
            author: "Admin <admin@irminview.com>".to_string(),
            timestamp: now - chrono::Duration::days(15),
            parents: vec!["f47ac10b58cc4372a5670e02b2c3d479".to_string()],
            branch: "main".to_string(),
        },
        IrminCommit {
            hash: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            message: "Update user permissions and add new store keys".to_string(),
            author: "Developer <dev@irminview.com>".to_string(),
            timestamp: now - chrono::Duration::hours(6),
            parents: vec!["6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_string()],
            branch: "main".to_string(),
        },
    ]
}

pub fn generate_demo_irmin_branches() -> Vec<IrminBranch> {
    let now = Utc::now();
    
    vec![
        IrminBranch {
            name: "main".to_string(),
            head_commit: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            last_updated: now - chrono::Duration::hours(6),
            commit_count: 3,
        },
        IrminBranch {
            name: "development".to_string(),
            head_commit: "6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_string(),
            last_updated: now - chrono::Duration::days(15),
            commit_count: 2,
        },
    ]
}