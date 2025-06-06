//! MCP server implementation for Kaggle API integration.
//! 
//! This module provides the Model Context Protocol (MCP) server that exposes
//! Kaggle API functionality as tools that can be used by Claude AI and other
//! MCP-compatible clients.

use crate::client::KaggleClient;
use crate::models::AuthenticationResponse;
use rmcp::{
    model::*, schemars, service::RequestContext, tool, Error as McpError,
    RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(test)]
mod tests;

/// Parameters for the authenticate tool.
#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AuthenticateParams {
    #[schemars(description = "Your Kaggle username")]
    pub kaggle_username: String,
    #[schemars(description = "Your Kaggle API key")]
    pub kaggle_key: String,
}

/// The main MCP server implementation for Kaggle API integration.
/// 
/// This server provides tools for interacting with the Kaggle API through
/// the Model Context Protocol. It manages a Kaggle API client and exposes
/// various tools for authentication, competitions, datasets, and more.
/// 
/// # Example
/// 
/// ```no_run
/// use kaggle_mcp_rs::server::KaggleMcpServer;
/// use rmcp::{transport::stdio, ServiceExt};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let server = KaggleMcpServer::new();
///     let service = server.serve(stdio()).await?;
///     service.waiting().await?;
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct KaggleMcpServer {
    client: Arc<RwLock<KaggleClient>>,
}

#[tool(tool_box)]
impl KaggleMcpServer {
    /// Creates a new instance of the Kaggle MCP server.
    /// 
    /// Initializes the server with a new Kaggle API client.
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(KaggleClient::new())),
        }
    }

    /// Authenticates with the Kaggle API using the provided credentials.
    /// 
    /// This tool allows users to authenticate with their Kaggle username and API key.
    /// The credentials can be obtained from the Kaggle account settings page.
    /// 
    /// # Arguments
    /// 
    /// * `params` - Authentication parameters containing username and API key
    /// 
    /// # Returns
    /// 
    /// Returns a success message if authentication is successful, or an error
    /// if the credentials are invalid.
    #[tool(description = "Authenticate with the Kaggle API using your username and API key")]
    async fn authenticate(
        &self,
        #[tool(aggr)] params: AuthenticateParams,
    ) -> std::result::Result<CallToolResult, McpError> {
        let client = self.client.read().await;
        
        match client
            .authenticate(params.kaggle_username.clone(), params.kaggle_key)
            .await
        {
            Ok(_) => {
                let response = AuthenticationResponse {
                    success: true,
                    message: "Successfully authenticated with Kaggle API".to_string(),
                    username: Some(params.kaggle_username),
                };
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for KaggleMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "kaggle-mcp-rs".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "This server provides access to the Kaggle API through MCP. \
                 First authenticate using the 'authenticate' tool with your Kaggle credentials."
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<InitializeResult, McpError> {
        // Try to load credentials on startup
        let client = self.client.read().await;
        let _ = client.load_credentials().await;
        
        Ok(self.get_info())
    }
}