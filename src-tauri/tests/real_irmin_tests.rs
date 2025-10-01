// Real Irmin integration tests - NO MOCKS
// These tests require the Irmin HTTP server to be running

use irmin_view::irmin::{integration, types::*};
use std::env;

// Helper to create config with HTTP server
fn get_test_config() -> integration::IrminConfig {
    integration::IrminConfig::new()
        .with_http_server("http://localhost:8080".to_string())
}

#[tokio::test]
async fn test_real_irmin_tree() {
    let config = get_test_config();
    
    // This should succeed when server is running
    let tree_result = integration::get_irmin_tree(&config).await;
    
    match tree_result {
        Ok(tree) => {
            // Verify we got real data
            assert_eq!(tree.key, "root");
            assert_eq!(tree.node_type, NodeType::Directory);
            assert!(!tree.children.is_empty(), "Tree should have children");
            
            println!("✅ Successfully got tree from real Irmin server");
            println!("   Root has {} children", tree.children.len());
            
            // Check for expected data structure
            assert!(tree.children.contains_key("data") || tree.children.contains_key("README.md"), 
                "Tree should contain expected keys from test store");
        }
        Err(e) => {
            panic!("Failed to get tree from Irmin server. Is the server running? Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_irmin_commits() {
    let config = get_test_config();
    
    let commits_result = integration::get_irmin_commits(&config).await;
    
    match commits_result {
        Ok(commits) => {
            assert!(!commits.is_empty(), "Should have commits in test store");
            
            println!("✅ Successfully got {} commits from real Irmin server", commits.len());
            
            // Verify commit structure
            let first_commit = &commits[0];
            assert!(!first_commit.hash.is_empty(), "Commit should have hash");
            assert!(!first_commit.message.is_empty(), "Commit should have message");
            assert!(!first_commit.author.is_empty(), "Commit should have author");
            
            println!("   First commit: {} - {}", &first_commit.hash[..8], first_commit.message);
        }
        Err(e) => {
            panic!("Failed to get commits from Irmin server. Is the server running? Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_irmin_branches() {
    let config = get_test_config();
    
    let branches_result = integration::get_irmin_branches(&config).await;
    
    match branches_result {
        Ok(branches) => {
            assert!(!branches.is_empty(), "Should have branches in test store");
            
            println!("✅ Successfully got {} branches from real Irmin server", branches.len());
            
            // Verify branch structure
            for branch in &branches {
                if !branch.name.is_empty() {
                    assert!(!branch.head_commit.is_empty(), "Branch should have head commit");
                    println!("   Branch: {} ({})", branch.name, branch.head_commit);
                }
            }
            
            // Check for expected branches
            let branch_names: Vec<&str> = branches.iter()
                .filter(|b| !b.name.is_empty())
                .map(|b| b.name.as_str())
                .collect();
            assert!(
                branch_names.contains(&"master") || branch_names.contains(&"main"),
                "Should have master/main branch"
            );
        }
        Err(e) => {
            panic!("Failed to get branches from Irmin server. Is the server running? Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_irmin_search() {
    let config = get_test_config();
    
    let search_result = integration::search_irmin_keys(&config, "alice").await;
    
    match search_result {
        Ok(results) => {
            println!("✅ Successfully searched Irmin server, found {} results", results.len());
            
            if !results.is_empty() {
                // Verify search result structure
                let first_result = &results[0];
                assert!(!first_result.path.is_empty(), "Search result should have path");
                assert!(first_result.relevance_score >= 0.0 && first_result.relevance_score <= 1.0,
                    "Relevance score should be between 0 and 1");
                
                println!("   Found: {} (relevance: {:.2})", first_result.path, first_result.relevance_score);
            }
        }
        Err(e) => {
            panic!("Failed to search Irmin server. Is the server running? Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_irmin_diff() {
    let config = get_test_config();
    
    // First get commits to have valid hashes
    let commits_result = integration::get_irmin_commits(&config).await;
    
    match commits_result {
        Ok(commits) if commits.len() >= 2 => {
            let from_commit = &commits[1].hash;
            let to_commit = &commits[0].hash;
            
            let diff_result = integration::get_irmin_diff(&config, from_commit, to_commit).await;
            
            match diff_result {
                Ok(diff) => {
                    println!("✅ Successfully got diff from real Irmin server");
                    println!("   Diff from {} to {}", &from_commit[..8], &to_commit[..8]);
                    println!("   {} changes", diff.changes.len());
                    
                    assert_eq!(diff.from_commit, *from_commit);
                    assert_eq!(diff.to_commit, *to_commit);
                }
                Err(e) => {
                    panic!("Failed to get diff from Irmin server. Is the server running? Error: {}", e);
                }
            }
        }
        Ok(_) => {
            println!("⚠️  Not enough commits for diff test");
        }
        Err(e) => {
            panic!("Failed to get commits for diff test. Is the server running? Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_server_health_check() {
    let config = get_test_config();
    
    // Use a simple reqwest call to check health
    let client = reqwest::Client::new();
    let health_url = format!("{}/health", config.server_url.unwrap());
    
    let response = client.get(&health_url).send().await;
    
    match response {
        Ok(resp) => {
            assert!(resp.status().is_success(), "Health check should return 200");
            println!("✅ Irmin server is healthy");
        }
        Err(e) => {
            panic!("Server health check failed. Is the server running? Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_no_fallback_on_error() {
    // Test with invalid server URL to ensure we DON'T fallback to mock data
    let bad_config = integration::IrminConfig::new()
        .with_http_server("http://localhost:9999".to_string());
    
    let tree_result = integration::get_irmin_tree(&bad_config).await;
    
    // Should fail, NOT fallback
    assert!(tree_result.is_err(), "Should fail when server is unavailable, NO FALLBACKS");
    
    println!("✅ Correctly fails without fallback when server unavailable");
}
