use super::{mock_data, types::*};
use tauri::command;

/// Get the mock tree structure for display
#[command]
pub async fn get_mock_tree() -> Result<IrminNode, String> {
    Ok(mock_data::generate_mock_tree())
}

/// Get mock commits for the commit history view
#[command]
pub async fn get_mock_commits() -> Result<Vec<IrminCommit>, String> {
    Ok(mock_data::generate_mock_commits())
}

/// Get mock branches
#[command]
pub async fn get_mock_branches() -> Result<Vec<IrminBranch>, String> {
    Ok(mock_data::generate_mock_branches())
}

/// Get a diff between two commits
#[command]
pub async fn get_commit_diff(from_commit: String, to_commit: String) -> Result<IrminDiff, String> {
    Ok(mock_data::generate_mock_diff(&from_commit, &to_commit))
}

/// Search for keys in the tree
#[command]
pub async fn search_keys(query: String) -> Result<Vec<SearchResult>, String> {
    let tree = mock_data::generate_mock_tree();
    let results = search_tree_recursive(&tree, &query, "");
    Ok(results)
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