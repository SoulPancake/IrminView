use super::types::*;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// Configuration for Irmin HTTP server connection
#[derive(Clone)]
pub struct IrminHttpConfig {
    pub server_url: String,
    pub timeout: Duration,
}

impl IrminHttpConfig {
    pub fn new() -> Self {
        Self {
            server_url: "http://localhost:8080".to_string(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.server_url = url;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// HTTP client for Irmin server
pub struct IrminHttpClient {
    client: Client,
    config: IrminHttpConfig,
}

impl IrminHttpClient {
    pub fn new(config: IrminHttpConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client, config }
    }

    /// Check if the Irmin server is healthy
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.config.server_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    /// Get tree structure from Irmin server
    pub async fn get_tree(&self) -> Result<IrminNode> {
        let url = format!("{}/api/tree", self.config.server_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let node: IrminNode = response.json().await?;
            Ok(node)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Server error: {}", error_text))
        }
    }

    /// Get commits from Irmin server
    pub async fn get_commits(&self) -> Result<Vec<IrminCommit>> {
        let url = format!("{}/api/commits", self.config.server_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let commits: Vec<IrminCommit> = response.json().await?;
            Ok(commits)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Server error: {}", error_text))
        }
    }

    /// Get branches from Irmin server
    pub async fn get_branches(&self) -> Result<Vec<IrminBranch>> {
        let url = format!("{}/api/branches", self.config.server_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let branches: Vec<IrminBranch> = response.json().await?;
            Ok(branches)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Server error: {}", error_text))
        }
    }

    /// Search keys in Irmin server
    pub async fn search_keys(&self, query: &str) -> Result<Vec<SearchResult>> {
        let url = format!("{}/api/search?q={}", self.config.server_url, urlencoding::encode(query));
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let results: Vec<SearchResult> = response.json().await?;
            Ok(results)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Server error: {}", error_text))
        }
    }

    /// Get diff between commits from Irmin server
    pub async fn get_diff(&self, from_commit: &str, to_commit: &str) -> Result<IrminDiff> {
        let url = format!(
            "{}/api/diff?from={}&to={}", 
            self.config.server_url, 
            urlencoding::encode(from_commit),
            urlencoding::encode(to_commit)
        );
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let diff: IrminDiff = response.json().await?;
            Ok(diff)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Server error: {}", error_text))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_config_creation() {
        let config = IrminHttpConfig::new();
        assert_eq!(config.server_url, "http://localhost:8080");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_http_config_with_url() {
        let config = IrminHttpConfig::new().with_url("http://example.com:9000".to_string());
        assert_eq!(config.server_url, "http://example.com:9000");
    }

    #[tokio::test]
    async fn test_client_creation() {
        let config = IrminHttpConfig::new();
        let _client = IrminHttpClient::new(config);
        // If this doesn't panic, the client was created successfully
    }

    // Integration tests would require a running Irmin server
    // These would be run separately in CI/CD pipeline
}