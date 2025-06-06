//! Configuration types for the Kaggle API client.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration settings for the Kaggle API client.
/// 
/// This struct holds various configuration options that affect how
/// the client interacts with the Kaggle API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaggleConfig {
    /// Default competition to use for operations
    pub competition: Option<String>,
    /// Default download path for files
    pub path: Option<PathBuf>,
    /// HTTP proxy URL to use for API requests
    pub proxy: Option<String>,
}

impl Default for KaggleConfig {
    fn default() -> Self {
        Self {
            competition: None,
            path: None,
            proxy: None,
        }
    }
}