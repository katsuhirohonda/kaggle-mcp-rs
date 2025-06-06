//! Kernel (notebook) related types.

use serde::{Deserialize, Serialize};

/// Represents a Kaggle kernel (notebook).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel {
    /// Kernel reference/slug
    pub ref_: String,
    /// Kernel title
    pub title: String,
    /// Author username
    pub author: String,
    /// Programming language (e.g., "python", "r")
    pub language: String,
    /// Type of kernel (e.g., "script", "notebook")
    pub kernel_type: String,
}