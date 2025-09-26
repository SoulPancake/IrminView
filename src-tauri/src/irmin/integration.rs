use std::process::Command;
use tokio::task;
use super::types::*;
use anyhow::Result;

/// Configuration for Irmin store connection
#[derive(Clone)]
pub struct IrminConfig {
    pub store_path: String,
    pub bridge_executable: String,
}

impl IrminConfig {
    pub fn new() -> Self {
        Self {
            store_path: "./irmin_store".to_string(),
            bridge_executable: "irmin-bridge-cli".to_string(),
        }
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.store_path = path;
        self
    }
}

/// Execute OCaml bridge command and parse JSON response
async fn execute_bridge_command(config: &IrminConfig, args: &[&str]) -> Result<String> {
    let config_clone = config.clone();
    let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    
    task::spawn_blocking(move || {
        let mut cmd = Command::new(&config_clone.bridge_executable);
        cmd.args(&args_vec);
        cmd.arg("--path").arg(&config_clone.store_path);
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Bridge command failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }).await?
}

/// Get tree structure from real Irmin store
pub async fn get_irmin_tree(config: &IrminConfig) -> Result<IrminNode> {
    let json_str = execute_bridge_command(config, &["tree"]).await?;
    let node: IrminNode = serde_json::from_str(&json_str)?;
    Ok(node)
}

/// Get commits from real Irmin store
pub async fn get_irmin_commits(config: &IrminConfig) -> Result<Vec<IrminCommit>> {
    let json_str = execute_bridge_command(config, &["commits"]).await?;
    let commits: Vec<IrminCommit> = serde_json::from_str(&json_str)?;
    Ok(commits)
}

/// Get branches from real Irmin store
pub async fn get_irmin_branches(config: &IrminConfig) -> Result<Vec<IrminBranch>> {
    let json_str = execute_bridge_command(config, &["branches"]).await?;
    let branches: Vec<IrminBranch> = serde_json::from_str(&json_str)?;
    Ok(branches)
}

/// Search keys in real Irmin store
pub async fn search_irmin_keys(config: &IrminConfig, query: &str) -> Result<Vec<SearchResult>> {
    let json_str = execute_bridge_command(config, &["search", query]).await?;
    let results: Vec<SearchResult> = serde_json::from_str(&json_str)?;
    Ok(results)
}

/// Get diff between commits in real Irmin store
pub async fn get_irmin_diff(config: &IrminConfig, from_commit: &str, to_commit: &str) -> Result<IrminDiff> {
    let json_str = execute_bridge_command(config, &["diff", from_commit, to_commit]).await?;
    let diff: IrminDiff = serde_json::from_str(&json_str)?;
    Ok(diff)
}

/// Initialize or check Irmin store
pub async fn initialize_irmin_store(config: &IrminConfig) -> Result<()> {
    // Try to get tree to verify store is accessible
    match get_irmin_tree(config).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to access Irmin store: {}", e);
            eprintln!("Make sure the OCaml bridge is installed and the store path is correct.");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_irmin_config_creation() {
        let config = IrminConfig::new();
        assert_eq!(config.store_path, "./irmin_store");
        assert_eq!(config.bridge_executable, "irmin-bridge-cli");
    }

    #[tokio::test]
    async fn test_irmin_config_with_path() {
        let config = IrminConfig::new().with_path("/custom/path".to_string());
        assert_eq!(config.store_path, "/custom/path");
    }

    // Integration tests would require the OCaml bridge to be built and installed
    // These would be run separately in CI/CD pipeline after building the OCaml components
}