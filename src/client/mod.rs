//! Kaggle API client implementation.
//! 
//! This module provides a client for interacting with the Kaggle API, including
//! authentication, credential management, and HTTP request handling.

use crate::models::{Error, KaggleCredentials, KaggleConfig};
use reqwest::{Client, RequestBuilder};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};

#[cfg(test)]
mod tests;

/// Base URL for the Kaggle API
const KAGGLE_API_BASE: &str = "https://www.kaggle.com/api/v1";

/// Kaggle API client that handles authentication and HTTP requests.
/// 
/// The client manages credentials, configuration, and provides methods for
/// authenticating with the Kaggle API. It supports loading credentials from
/// environment variables or from the `~/.kaggle/kaggle.json` file.
/// 
/// # Example
/// 
/// ```no_run
/// use kaggle_mcp_rs::client::KaggleClient;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = KaggleClient::new();
///     
///     // Load credentials from environment or file
///     client.load_credentials().await?;
///     
///     // Or authenticate directly
///     client.authenticate("username".to_string(), "api_key".to_string()).await?;
///     
///     Ok(())
/// }
/// ```
pub struct KaggleClient {
    http_client: Client,
    credentials: Arc<RwLock<Option<KaggleCredentials>>>,
    #[allow(dead_code)]
    config: Arc<RwLock<KaggleConfig>>,
    #[cfg(test)]
    api_base_override: Option<String>,
    #[cfg(test)]
    skip_save_credentials: bool,
}

impl KaggleClient {
    /// Creates a new Kaggle API client instance.
    /// 
    /// Initializes the HTTP client with appropriate user agent and default settings.
    pub fn new() -> Self {
        let http_client = Client::builder()
            .user_agent("kaggle-mcp-rs/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            credentials: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(KaggleConfig::default())),
            #[cfg(test)]
            api_base_override: None,
            #[cfg(test)]
            skip_save_credentials: false,
        }
    }

    /// Authenticates with the Kaggle API using the provided credentials.
    /// 
    /// This method tests the credentials by making a simple API call to the competitions
    /// endpoint. If successful, the credentials are stored and optionally saved to disk.
    /// 
    /// # Arguments
    /// 
    /// * `username` - The Kaggle username
    /// * `key` - The Kaggle API key
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if authentication is successful, or an error if the credentials
    /// are invalid or if there's a network issue.
    pub async fn authenticate(&self, username: String, key: String) -> Result<(), Error> {
        info!("Authenticating with Kaggle API");
        debug!("Username: {}", username);
        
        // Test authentication by making a simple API call
        #[cfg(test)]
        let test_url = {
            if let Some(ref base) = self.api_base_override {
                // For tests, use the override URL directly
                format!("{}/api/v1/competitions/list", base)
            } else {
                format!("{}/competitions/list", KAGGLE_API_BASE)
            }
        };
        #[cfg(not(test))]
        let test_url = format!("{}/competitions/list", KAGGLE_API_BASE);
        
        debug!("Testing authentication with URL: {}", test_url);
        
        let response = self.http_client
            .get(&test_url)
            .basic_auth(&username, Some(&key))
            .send()
            .await?;

        if response.status().is_success() {
            info!("Authentication successful");
            let mut creds = self.credentials.write().await;
            *creds = Some(KaggleCredentials { username: username.clone(), key: key.clone() });
            
            // Save credentials to file
            #[cfg(test)]
            if !self.skip_save_credentials {
                self.save_credentials(&username, &key).await?;
            }
            #[cfg(not(test))]
            self.save_credentials(&username, &key).await?;
            
            Ok(())
        } else {
            let status = response.status();
            error!("Authentication failed with status: {}", status);
            Err(Error::AuthenticationError(format!(
                "Invalid credentials: {}",
                status
            )))
        }
    }

    /// Checks if the client has stored credentials.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if credentials are present, `false` otherwise.
    /// Note that this doesn't validate the credentials with the API.
    pub async fn is_authenticated(&self) -> bool {
        self.credentials.read().await.is_some()
    }

    /// Saves credentials to the kaggle.json file in the user's home directory.
    /// 
    /// The credentials are saved to `~/.kaggle/kaggle.json` with restricted
    /// permissions (0o600 on Unix systems).
    /// 
    /// # Arguments
    /// 
    /// * `username` - The Kaggle username
    /// * `key` - The Kaggle API key
    async fn save_credentials(&self, username: &str, key: &str) -> Result<(), Error> {
        info!("Saving credentials to kaggle.json");
        
        let kaggle_dir = directories::UserDirs::new()
            .ok_or_else(|| Error::Other("Could not determine home directory".to_string()))?
            .home_dir()
            .join(".kaggle");

        debug!("Creating directory: {:?}", kaggle_dir);
        tokio::fs::create_dir_all(&kaggle_dir).await?;

        let kaggle_json = serde_json::json!({
            "username": username,
            "key": key
        });

        let kaggle_json_path = kaggle_dir.join("kaggle.json");
        debug!("Writing credentials to: {:?}", kaggle_json_path);
        tokio::fs::write(&kaggle_json_path, serde_json::to_string_pretty(&kaggle_json)?).await?;

        // Set file permissions to 0o600 on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = tokio::fs::metadata(&kaggle_json_path).await?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            tokio::fs::set_permissions(&kaggle_json_path, permissions).await?;
            debug!("Set file permissions to 0o600");
        }

        info!("Credentials saved successfully");
        Ok(())
    }

    /// Loads Kaggle credentials from environment variables or file.
    /// 
    /// This method first checks for `KAGGLE_USERNAME` and `KAGGLE_KEY` environment
    /// variables. If not found, it looks for credentials in `~/.kaggle/kaggle.json`.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if credentials are found and loaded successfully,
    /// or `Error::NotAuthenticated` if no credentials are found.
    pub async fn load_credentials(&self) -> Result<(), Error> {
        info!("Loading Kaggle credentials");
        
        // First, check environment variables
        if let (Ok(username), Ok(key)) = (
            std::env::var("KAGGLE_USERNAME"),
            std::env::var("KAGGLE_KEY"),
        ) {
            info!("Found credentials in environment variables");
            let mut credentials = self.credentials.write().await;
            *credentials = Some(KaggleCredentials { username, key });
            return Ok(());
        }
        
        // Then, check kaggle.json file
        let kaggle_json_path = match directories::UserDirs::new() {
            Some(dirs) => dirs.home_dir().join(".kaggle").join("kaggle.json"),
            None => {
                warn!("Could not determine home directory");
                return Err(Error::NotAuthenticated);
            }
        };

        debug!("Checking for kaggle.json at: {:?}", kaggle_json_path);

        if kaggle_json_path.exists() {
            info!("Found kaggle.json file");
            let content = tokio::fs::read_to_string(&kaggle_json_path).await?;
            let creds: serde_json::Value = serde_json::from_str(&content)?;
            
            if let (Some(username), Some(key)) = (
                creds.get("username").and_then(|v| v.as_str()),
                creds.get("key").and_then(|v| v.as_str()),
            ) {
                let mut credentials = self.credentials.write().await;
                *credentials = Some(KaggleCredentials {
                    username: username.to_string(),
                    key: key.to_string(),
                });
                Ok(())
            } else {
                error!("Invalid kaggle.json format");
                Err(Error::Other("Invalid kaggle.json format".to_string()))
            }
        } else {
            warn!("No credentials found in environment variables or kaggle.json");
            Err(Error::NotAuthenticated)
        }
    }

    /// Makes an authenticated HTTP request to the Kaggle API.
    /// 
    /// This method adds authentication headers to the request and handles
    /// common error cases.
    /// 
    /// # Arguments
    /// 
    /// * `builder` - The request builder to execute
    /// 
    /// # Returns
    /// 
    /// Returns the HTTP response if successful, or an error if authentication
    /// fails or there's a network issue.
    pub(crate) async fn request(&self, builder: RequestBuilder) -> Result<reqwest::Response, Error> {
        let creds = self.credentials.read().await;
        let creds = creds.as_ref().ok_or(Error::NotAuthenticated)?;
        
        let response = builder
            .basic_auth(&creds.username, Some(&creds.key))
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(Error::ApiError(crate::models::KaggleError {
                code: status.to_string(),
                message: text,
            }))
        }
    }

    /// Returns a reference to the underlying HTTP client.
    /// 
    /// This can be used to make custom requests while reusing the client's
    /// connection pool.
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Returns the base URL for the Kaggle API.
    pub fn api_base() -> &'static str {
        KAGGLE_API_BASE
    }

    /// Test-only method to override the API base URL.
    #[cfg(test)]
    pub fn with_api_base(mut self, base: String) -> Self {
        self.api_base_override = Some(base);
        self
    }

    /// Test-only method to skip saving credentials to disk.
    #[cfg(test)]
    pub fn skip_save_credentials(mut self) -> Self {
        self.skip_save_credentials = true;
        self
    }
}