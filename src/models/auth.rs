//! Authentication-related types.

use serde::{Deserialize, Serialize};

/// Represents Kaggle API credentials.
/// 
/// This struct holds the username and API key required for authenticating
/// with the Kaggle API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaggleCredentials {
    /// Kaggle username
    pub username: String,
    /// Kaggle API key
    pub key: String,
}

/// Request parameters for authentication.
/// 
/// Used when authenticating through the MCP tool interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    /// Kaggle username
    pub kaggle_username: String,
    /// Kaggle API key
    pub kaggle_key: String,
}

/// Response returned after authentication attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    /// Whether authentication was successful
    pub success: bool,
    /// Human-readable message about the authentication result
    pub message: String,
    /// The authenticated username (present on success)
    pub username: Option<String>,
}