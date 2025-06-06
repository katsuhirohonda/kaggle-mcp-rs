//! Model-related types.

use serde::{Deserialize, Serialize};

/// Represents a Kaggle model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Unique model identifier
    pub id: String,
    /// Model title
    pub title: String,
    /// Model subtitle/description
    pub subtitle: Option<String>,
    /// Author username
    pub author: String,
}