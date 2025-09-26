#[cfg(test)]
mod tests {
    use irmin_view::irmin::{mock_data, types::*};

    #[test]
    fn test_mock_tree_generation() {
        let tree = mock_data::generate_mock_tree();
        
        assert_eq!(tree.key, "/");
        assert_eq!(tree.node_type, NodeType::Directory);
        assert!(!tree.children.is_empty());
        
        // Check that we have some expected files
        assert!(tree.children.contains_key("config.json"));
        assert!(tree.children.contains_key("users"));
        assert!(tree.children.contains_key("README.md"));
        
        // Verify the users directory structure
        let users_dir = &tree.children["users"];
        assert_eq!(users_dir.node_type, NodeType::Directory);
        assert!(users_dir.children.contains_key("alice.json"));
        assert!(users_dir.children.contains_key("bob.json"));
    }

    #[test]
    fn test_mock_commits_generation() {
        let commits = mock_data::generate_mock_commits();
        
        assert!(!commits.is_empty());
        
        // Check first commit structure
        let first_commit = &commits[0];
        assert!(!first_commit.hash.is_empty());
        assert!(!first_commit.message.is_empty());
        assert!(!first_commit.author.is_empty());
        assert_eq!(first_commit.branch, "main");
    }

    #[test]
    fn test_mock_branches_generation() {
        let branches = mock_data::generate_mock_branches();
        
        assert!(!branches.is_empty());
        
        // Should have at least main branch
        let main_branch = branches.iter().find(|b| b.name == "main");
        assert!(main_branch.is_some());
        
        let main = main_branch.unwrap();
        assert!(!main.head_commit.is_empty());
        assert!(main.commit_count > 0);
    }

    #[test]
    fn test_mock_diff_generation() {
        let diff = mock_data::generate_mock_diff("commit1", "commit2");
        
        assert_eq!(diff.from_commit, "commit1");
        assert_eq!(diff.to_commit, "commit2");
        assert!(!diff.changes.is_empty());
        
        // Check that we have different types of changes
        let has_added = diff.changes.iter().any(|c| matches!(c.change_type, ChangeType::Added));
        let has_modified = diff.changes.iter().any(|c| matches!(c.change_type, ChangeType::Modified));
        let has_deleted = diff.changes.iter().any(|c| matches!(c.change_type, ChangeType::Deleted));
        
        assert!(has_added || has_modified || has_deleted);
    }

    #[test]  
    fn test_search_functionality() {
        let tree = mock_data::generate_mock_tree();
        
        // Test that we can find nodes by name
        fn search_tree_recursive(node: &IrminNode, query: &str, current_path: &str) -> Vec<String> {
            let mut results = Vec::new();
            let path = if current_path.is_empty() {
                node.key.clone()
            } else {
                format!("{}/{}", current_path, node.key)
            };
            
            if node.key.to_lowercase().contains(&query.to_lowercase()) {
                results.push(path.clone());
            }
            
            for child in node.children.values() {
                results.extend(search_tree_recursive(child, query, &path));
            }
            
            results
        }
        
        let results = search_tree_recursive(&tree, "json", "");
        assert!(!results.is_empty());
        
        // Should find config.json and user JSON files
        let has_config = results.iter().any(|r| r.contains("config.json"));
        assert!(has_config);
    }

    #[test]
    fn test_node_metadata() {
        let tree = mock_data::generate_mock_tree();
        
        // Check that files have size metadata
        let config_file = &tree.children["config.json"];
        assert!(config_file.metadata.size.is_some());
        assert!(config_file.metadata.size.unwrap() > 0);
        
        // Check that directories don't have size
        let users_dir = &tree.children["users"];
        assert!(users_dir.metadata.size.is_none());
        
        // Check permissions are set
        assert!(config_file.metadata.permissions.is_some());
        assert!(users_dir.metadata.permissions.is_some());
    }

    #[test]
    fn test_branch_functionality() {
        let branches = mock_data::generate_mock_branches();
        
        // Test that we have expected branch structure
        assert!(branches.len() >= 2);
        
        let main_branch = branches.iter().find(|b| b.name == "main").unwrap();
        let feature_branch = branches.iter().find(|b| b.name.contains("feature")).unwrap();
        
        // Main branch should have more commits
        assert!(main_branch.commit_count >= feature_branch.commit_count);
        
        // Both should have valid commit hashes
        assert!(!main_branch.head_commit.is_empty());
        assert!(!feature_branch.head_commit.is_empty());
    }

    #[test]  
    fn test_diff_change_types() {
        let diff = mock_data::generate_mock_diff("old", "new");
        
        // Should have at least one of each change type
        let added_count = diff.changes.iter().filter(|c| c.change_type == ChangeType::Added).count();
        let modified_count = diff.changes.iter().filter(|c| c.change_type == ChangeType::Modified).count();
        let deleted_count = diff.changes.iter().filter(|c| c.change_type == ChangeType::Deleted).count();
        
        assert!(added_count > 0 || modified_count > 0 || deleted_count > 0);
        
        // Check that modified changes have both old and new values
        for change in &diff.changes {
            if change.change_type == ChangeType::Modified {
                assert!(change.old_value.is_some());
                assert!(change.new_value.is_some());
            }
        }
    }
}