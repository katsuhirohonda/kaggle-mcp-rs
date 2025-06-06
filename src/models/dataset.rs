use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub creator_name: String,
    pub total_bytes: i64,
    pub url: String,
}