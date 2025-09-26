use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a commit in the Irmin store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrminCommit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub parents: Vec<String>,
    pub branch: String,
}

/// Represents a node in the Irmin tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrminNode {
    pub key: String,
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: HashMap<String, IrminNode>,
    pub metadata: NodeMetadata,
}

/// Types of nodes in the Irmin tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Directory,
    File,
    Link,
}

/// Metadata associated with a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub last_modified: DateTime<Utc>,
    pub size: Option<u64>,
    pub permissions: Option<String>,
}

/// Represents a branch in the Irmin store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrminBranch {
    pub name: String,
    pub head_commit: String,
    pub last_updated: DateTime<Utc>,
    pub commit_count: usize,
}

/// Represents a diff between two commits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrminDiff {
    pub from_commit: String,
    pub to_commit: String,
    pub changes: Vec<DiffChange>,
}

/// Individual change in a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffChange {
    pub path: String,
    pub change_type: ChangeType,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

/// Types of changes in a diff
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}

/// Search result for keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub node: IrminNode,
    pub relevance_score: f32,
}

/// Connection information for an Irmin store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrminConnection {
    pub id: Uuid,
    pub name: String,
    pub connection_type: ConnectionType,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub status: ConnectionStatus,
}

/// Types of connections to Irmin stores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Local,
    Remote,
    InMemory,
}

/// Status of a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Error(String),
}

/// Configuration for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: Theme,
    pub recent_connections: Vec<IrminConnection>,
    pub window_state: WindowState,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// Window state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}