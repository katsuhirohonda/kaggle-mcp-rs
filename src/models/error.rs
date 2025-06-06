use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaggleError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for KaggleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for KaggleError {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("API error: {0}")]
    ApiError(#[from] KaggleError),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Not authenticated")]
    NotAuthenticated,
    
    #[error("{0}")]
    Other(String),
}

// Removed custom Result type - use std::result::Result directly