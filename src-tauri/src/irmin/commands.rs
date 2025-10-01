use super::{integration, types::*};
use tauri::command;

/// Get the tree structure for display (from real Irmin store ONLY - NO FALLBACKS)
#[command]
pub async fn get_tree() -> Result<IrminNode, String> {
    let config = integration::IrminConfig::new();
    integration::get_irmin_tree(&config).await
        .map_err(|e| format!("Failed to get tree from Irmin store: {}. Please ensure Irmin server is running.", e))
}

/// Get commits for the commit history view (from real Irmin store ONLY - NO FALLBACKS)
#[command]
pub async fn get_commits() -> Result<Vec<IrminCommit>, String> {
    let config = integration::IrminConfig::new();
    integration::get_irmin_commits(&config).await
        .map_err(|e| format!("Failed to get commits from Irmin store: {}. Please ensure Irmin server is running.", e))
}

/// Get branches (from real Irmin store ONLY - NO FALLBACKS)
#[command]
pub async fn get_branches() -> Result<Vec<IrminBranch>, String> {
    let config = integration::IrminConfig::new();
    integration::get_irmin_branches(&config).await
        .map_err(|e| format!("Failed to get branches from Irmin store: {}. Please ensure Irmin server is running.", e))
}

/// Get a diff between two commits (from real Irmin store ONLY - NO FALLBACKS)
#[command]
pub async fn get_commit_diff(from_commit: String, to_commit: String) -> Result<IrminDiff, String> {
    let config = integration::IrminConfig::new();
    integration::get_irmin_diff(&config, &from_commit, &to_commit).await
        .map_err(|e| format!("Failed to get diff from Irmin store: {}. Please ensure Irmin server is running.", e))
}

/// Search for keys in the tree (from real Irmin store ONLY - NO FALLBACKS)
#[command]
pub async fn search_keys(query: String) -> Result<Vec<SearchResult>, String> {
    let config = integration::IrminConfig::new();
    integration::search_irmin_keys(&config, &query).await
        .map_err(|e| format!("Failed to search in Irmin store: {}. Please ensure Irmin server is running.", e))
}

/// Initialize or connect to an Irmin store
#[command]
pub async fn connect_to_irmin_store(store_path: Option<String>) -> Result<String, String> {
    let config = match store_path {
        Some(path) => integration::IrminConfig::new().with_path(path),
        None => integration::IrminConfig::new(),
    };
    
    match integration::initialize_irmin_store(&config).await {
        Ok(_) => Ok(format!("Successfully connected to Irmin store at: {}", config.store_path)),
        Err(e) => Err(format!("Failed to connect to Irmin store: {}", e)),
    }
}

/// Check if Irmin integration is available
#[command]
pub async fn check_irmin_availability() -> Result<bool, String> {
    let config = integration::IrminConfig::new();
    match integration::initialize_irmin_store(&config).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}