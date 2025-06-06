//! Data models and types for the Kaggle MCP server.
//! 
//! This module contains all the data structures used throughout the crate,
//! including request/response types, error types, and domain models for
//! competitions, datasets, kernels, and models.

/// Authentication-related types
pub mod auth;

/// Competition-related types
pub mod competition;

/// Dataset-related types
pub mod dataset;

/// Kernel (notebook) related types
pub mod kernel;

/// Model-related types
pub mod model;

/// Configuration types
pub mod config;

/// Error types and result aliases
pub mod error;

#[cfg(test)]
mod tests;

// Re-export all types for convenience
pub use auth::*;
pub use competition::*;
pub use dataset::*;
pub use kernel::*;
pub use model::*;
pub use config::*;
pub use error::*;