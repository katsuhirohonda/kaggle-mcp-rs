//! Error types for the Kaggle MCP server.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an error response from the Kaggle API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaggleError {
    /// Error code returned by the API
    pub code: String,
    /// Human-readable error message
    pub message: String,
}

impl fmt::Display for KaggleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for KaggleError {}

/// The main error type for the Kaggle MCP server.
/// 
/// This enum represents all possible errors that can occur when
/// interacting with the Kaggle API or during server operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Authentication failed with the provided credentials
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    /// Error response from the Kaggle API
    #[error("API error: {0}")]
    ApiError(#[from] KaggleError),
    
    /// HTTP client error
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// I/O error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Invalid parameter provided to a method
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    /// No authentication credentials available
    #[error("Not authenticated")]
    NotAuthenticated,
    
    /// Other errors not covered by specific variants
    #[error("{0}")]
    Other(String),
}