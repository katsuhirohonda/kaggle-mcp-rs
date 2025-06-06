//! Example demonstrating basic authentication with the Kaggle MCP server.
//! 
//! This example shows how to create a Kaggle MCP server and authenticate
//! with Kaggle API credentials.

use kaggle_mcp_rs::server::KaggleMcpServer;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Kaggle MCP server example");

    // Create the server
    let server = KaggleMcpServer::new();
    
    // Serve on stdio transport (for MCP protocol)
    let service = server.serve(stdio()).await?;
    
    tracing::info!("Server is ready to accept connections");
    
    // Wait for the service to complete
    service.waiting().await?;
    
    Ok(())
}