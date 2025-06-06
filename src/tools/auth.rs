use rmcp::prelude::*;
use rmcp_macros::tool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::client::KaggleClient;
use crate::models::{AuthenticationRequest, AuthenticationResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateInput {
    /// Your Kaggle username
    pub kaggle_username: String,
    /// Your Kaggle API key
    pub kaggle_key: String,
}

#[tool(
    description = "Authenticate with the Kaggle API using your username and API key"
)]
async fn authenticate(
    client: Arc<KaggleClient>,
    input: AuthenticateInput,
) -> Result<AuthenticationResponse, String> {
    client
        .authenticate(input.kaggle_username.clone(), input.kaggle_key)
        .await
        .map(|_| AuthenticationResponse {
            success: true,
            message: "Successfully authenticated with Kaggle API".to_string(),
            username: Some(input.kaggle_username),
        })
        .map_err(|e| e.to_string())
}

pub fn register_auth_tools<T: ToolProvider>(server: &mut Server<T>, client: Arc<KaggleClient>) {
    // Try to load credentials on startup
    let client_clone = client.clone();
    tokio::spawn(async move {
        let _ = client_clone.load_credentials().await;
    });

    // Register the authenticate tool
    server.tool(move |input: AuthenticateInput| {
        let client = client.clone();
        async move { authenticate(client, input).await }
    });
}