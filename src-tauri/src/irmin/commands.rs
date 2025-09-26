use super::{mock_data, integration, types::*};
use tauri::command;

/// Get the tree structure for display (from real Irmin store)
#[command]
pub async fn get_tree() -> Result<IrminNode, String> {
    let config = integration::IrminConfig::new();
    match integration::get_irmin_tree(&config).await {
        Ok(tree) => Ok(tree),
        Err(e) => {
            eprintln!("Failed to get tree from Irmin store: {}. Falling back to mock data.", e);
            Ok(mock_data::generate_mock_tree())
        }
    }
}

/// Get commits for the commit history view (from real Irmin store)
#[command]
pub async fn get_commits() -> Result<Vec<IrminCommit>, String> {
    let config = integration::IrminConfig::new();
    match integration::get_irmin_commits(&config).await {
        Ok(commits) => Ok(commits),
        Err(e) => {
            eprintln!("Failed to get commits from Irmin store: {}. Falling back to mock data.", e);
            Ok(mock_data::generate_mock_commits())
        }
    }
}

/// Get branches (from real Irmin store)
#[command]
pub async fn get_branches() -> Result<Vec<IrminBranch>, String> {
    let config = integration::IrminConfig::new();
    match integration::get_irmin_branches(&config).await {
        Ok(branches) => Ok(branches),
        Err(e) => {
            eprintln!("Failed to get branches from Irmin store: {}. Falling back to mock data.", e);
            Ok(mock_data::generate_mock_branches())
        }
    }
}

/// Get a diff between two commits (from real Irmin store)
#[command]
pub async fn get_commit_diff(from_commit: String, to_commit: String) -> Result<IrminDiff, String> {
    let config = integration::IrminConfig::new();
    match integration::get_irmin_diff(&config, &from_commit, &to_commit).await {
        Ok(diff) => Ok(diff),
        Err(e) => {
            eprintln!("Failed to get diff from Irmin store: {}. Falling back to mock data.", e);
            Ok(mock_data::generate_mock_diff(&from_commit, &to_commit))
        }
    }
}

/// Search for keys in the tree (from real Irmin store)
#[command]
pub async fn search_keys(query: String) -> Result<Vec<SearchResult>, String> {
    let config = integration::IrminConfig::new();
    match integration::search_irmin_keys(&config, &query).await {
        Ok(results) => Ok(results),
        Err(e) => {
            eprintln!("Failed to search in Irmin store: {}. Falling back to mock data.", e);
            let tree = mock_data::generate_mock_tree();
            let results = search_tree_recursive(&tree, &query, "");
            Ok(results)
        }
    }
}

/// Recursive function to search through the tree
fn search_tree_recursive(node: &IrminNode, query: &str, current_path: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let path = if current_path.is_empty() {
        node.key.clone()
    } else {
        format!("{}/{}", current_path, node.key)
    };
    
    // Check if current node matches
    if node.key.to_lowercase().contains(&query.to_lowercase()) {
        let relevance_score = if node.key.to_lowercase() == query.to_lowercase() {
            1.0
        } else if node.key.to_lowercase().starts_with(&query.to_lowercase()) {
            0.8
        } else {
            0.5
        };
        
        results.push(SearchResult {
            path: path.clone(),
            node: node.clone(),
            relevance_score,
        });
    }
    
    // Search in children
    for child in node.children.values() {
        results.extend(search_tree_recursive(child, query, &path));
    }
    
    // Sort by relevance score (highest first)
    results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
    
    results
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