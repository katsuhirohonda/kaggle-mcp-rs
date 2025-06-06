//! # kaggle-mcp-rs
//! 
//! A Rust implementation of the Kaggle MCP (Model Context Protocol) server.
//! 
//! This crate provides a complete MCP server implementation that enables Claude AI
//! and other MCP-compatible clients to interact with the Kaggle API. It supports
//! authentication, competitions, datasets, kernels, models, and configuration management.
//! 
//! ## Features
//! 
//! - **Authentication**: Secure authentication with Kaggle API credentials
//! - **Competitions**: Browse, search, and download competition data
//! - **Datasets**: Discover, explore, and download datasets
//! - **Kernels**: Search and analyze Kaggle notebooks
//! - **Models**: Access pre-trained models
//! - **Configuration**: Manage API settings
//! 
//! ## Example
//! 
//! ```no_run
//! use kaggle_mcp_rs::server::KaggleMcpServer;
//! use rmcp::{transport::stdio, ServiceExt};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let service = KaggleMcpServer::new()
//!         .serve(stdio())
//!         .await?;
//!     
//!     service.waiting().await?;
//!     Ok(())
//! }
//! ```

/// Client module for interacting with the Kaggle API
pub mod client;

/// Data models and types used throughout the crate
pub mod models;

/// MCP server implementation
pub mod server;

#[cfg(test)]
mod test_utils;