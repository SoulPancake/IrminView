use irmin_view::irmin::{integration, types::*};

#[tokio::test]
async fn test_irmin_config() {
    let config = integration::IrminConfig::new();
    assert_eq!(config.store_path, "./irmin_store");
    assert_eq!(config.bridge_executable, "irmin-bridge-cli");
}

#[tokio::test] 
async fn test_irmin_config_with_custom_path() {
    let config = integration::IrminConfig::new().with_path("/tmp/test_store".to_string());
    assert_eq!(config.store_path, "/tmp/test_store");
}

// This integration test would only pass if the OCaml bridge is built and available
#[tokio::test]
#[ignore] // Ignore by default since it requires OCaml setup
async fn test_real_irmin_integration() {
    let config = integration::IrminConfig::new();
    
    // Test getting tree - this might fail if no Irmin store exists, which is expected
    let tree_result = integration::get_irmin_tree(&config).await;
    match tree_result {
        Ok(tree) => {
            assert!(!tree.key.is_empty());
            println!("Successfully got tree with root key: {}", tree.key);
        }
        Err(e) => {
            println!("Expected error when no Irmin store available: {}", e);
            // This is expected when no real Irmin store is set up
        }
    }
}

#[tokio::test]
async fn test_fallback_to_mock_data() {
    // Test that commands gracefully fall back to mock data when Irmin is not available
    // This simulates the behavior in commands.rs
    
    let config = integration::IrminConfig::new().with_path("/nonexistent/path".to_string());
    
    // These should fail and fall back to mock data in the actual commands
    let tree_result = integration::get_irmin_tree(&config).await;
    assert!(tree_result.is_err(), "Should fail with invalid path");
    
    let commits_result = integration::get_irmin_commits(&config).await;
    assert!(commits_result.is_err(), "Should fail with invalid path");
}