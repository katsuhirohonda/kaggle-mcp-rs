use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel {
    pub ref_: String,
    pub title: String,
    pub author: String,
    pub language: String,
    pub kernel_type: String,
}