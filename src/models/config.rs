use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaggleConfig {
    pub competition: Option<String>,
    pub path: Option<PathBuf>,
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