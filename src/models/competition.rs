use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub ref_: String,
    pub title: String,
    pub url: String,
    pub category: String,
    pub deadline: Option<DateTime<Utc>>,
    pub reward: Option<String>,
    pub team_count: i32,
    pub user_has_entered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionListRequest {
    pub search: Option<String>,
    pub category: Option<String>,
    pub group: Option<String>,
    pub sort_by: Option<String>,
    pub page: Option<i32>,
}