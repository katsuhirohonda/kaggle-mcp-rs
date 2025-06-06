use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
}