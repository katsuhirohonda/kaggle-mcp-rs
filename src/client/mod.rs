use crate::models::{Error, KaggleCredentials, KaggleConfig};
use reqwest::{Client, RequestBuilder};
use std::sync::Arc;
use tokio::sync::RwLock;

const KAGGLE_API_BASE: &str = "https://www.kaggle.com/api/v1";

pub struct KaggleClient {
    http_client: Client,
    credentials: Arc<RwLock<Option<KaggleCredentials>>>,
    config: Arc<RwLock<KaggleConfig>>,
}

impl KaggleClient {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .user_agent("kaggle-mcp-rs/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            credentials: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(KaggleConfig::default())),
        }
    }

    pub async fn authenticate(&self, username: String, key: String) -> Result<(), Error> {
        // Test authentication by making a simple API call
        let test_url = format!("{}/competitions/list", KAGGLE_API_BASE);
        let response = self.http_client
            .get(&test_url)
            .basic_auth(&username, Some(&key))
            .send()
            .await?;

        if response.status().is_success() {
            let mut creds = self.credentials.write().await;
            *creds = Some(KaggleCredentials { username: username.clone(), key: key.clone() });
            
            // Save credentials to file
            self.save_credentials(&username, &key).await?;
            Ok(())
        } else {
            Err(Error::AuthenticationError(format!(
                "Invalid credentials: {}",
                response.status()
            )))
        }
    }

    pub async fn is_authenticated(&self) -> bool {
        self.credentials.read().await.is_some()
    }

    async fn save_credentials(&self, username: &str, key: &str) -> Result<(), Error> {
        let kaggle_dir = directories::UserDirs::new()
            .ok_or_else(|| Error::Other("Could not determine home directory".to_string()))?
            .home_dir()
            .join(".kaggle");

        tokio::fs::create_dir_all(&kaggle_dir).await?;

        let kaggle_json = serde_json::json!({
            "username": username,
            "key": key
        });

        let kaggle_json_path = kaggle_dir.join("kaggle.json");
        tokio::fs::write(&kaggle_json_path, serde_json::to_string_pretty(&kaggle_json)?).await?;

        // Set file permissions to 0o600 on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = tokio::fs::metadata(&kaggle_json_path).await?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            tokio::fs::set_permissions(&kaggle_json_path, permissions).await?;
        }

        Ok(())
    }

    pub async fn load_credentials(&self) -> Result<(), Error> {
        let kaggle_json_path = directories::UserDirs::new()
            .ok_or_else(|| Error::Other("Could not determine home directory".to_string()))?
            .home_dir()
            .join(".kaggle")
            .join("kaggle.json");

        if kaggle_json_path.exists() {
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
                Err(Error::Other("Invalid kaggle.json format".to_string()))
            }
        } else {
            // Check environment variables
            if let (Ok(username), Ok(key)) = (
                std::env::var("KAGGLE_USERNAME"),
                std::env::var("KAGGLE_KEY"),
            ) {
                let mut credentials = self.credentials.write().await;
                *credentials = Some(KaggleCredentials { username, key });
                Ok(())
            } else {
                Err(Error::NotAuthenticated)
            }
        }
    }

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

    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    pub fn api_base() -> &'static str {
        KAGGLE_API_BASE
    }
}