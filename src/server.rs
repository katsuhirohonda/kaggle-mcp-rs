use crate::client::KaggleClient;
use crate::models::AuthenticationResponse;
use rmcp::{
    model::*, schemars, service::RequestContext, tool, Error as McpError,
    RoleServer, ServerHandler,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct AuthenticateParams {
    #[schemars(description = "Your Kaggle username")]
    pub kaggle_username: String,
    #[schemars(description = "Your Kaggle API key")]
    pub kaggle_key: String,
}

#[derive(Clone)]
pub struct KaggleMcpServer {
    client: Arc<RwLock<KaggleClient>>,
}

#[tool(tool_box)]
impl KaggleMcpServer {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(KaggleClient::new())),
        }
    }

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