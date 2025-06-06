//! Dataset-related types.

use serde::{Deserialize, Serialize};

/// Represents a Kaggle dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    /// Unique dataset identifier
    pub id: String,
    /// Dataset title
    pub title: String,
    /// Dataset subtitle/description
    pub subtitle: Option<String>,
    /// Name of the dataset creator
    pub creator_name: String,
    /// Total size of the dataset in bytes
    pub total_bytes: i64,
    /// Full URL to the dataset page
    pub url: String,
}