use anyhow::Result;
use kaggle_mcp_rs::server::KaggleMcpServer;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Initialize tracing to stderr to avoid interfering with stdio transport
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Kaggle MCP server");

    // Create and run the server
    let service = KaggleMcpServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("Server error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
