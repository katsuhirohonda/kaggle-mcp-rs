//! Competition-related types.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a Kaggle competition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    /// Competition reference/slug (e.g., "titanic")
    pub ref_: String,
    /// Competition title
    pub title: String,
    /// Full URL to the competition page
    pub url: String,
    /// Competition category
    pub category: String,
    /// Competition deadline
    pub deadline: Option<DateTime<Utc>>,
    /// Prize/reward information
    pub reward: Option<String>,
    /// Number of participating teams
    pub team_count: i32,
    /// Whether the current user has entered the competition
    pub user_has_entered: bool,
}

/// Request parameters for listing competitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionListRequest {
    /// Search terms to filter competitions
    pub search: Option<String>,
    /// Filter by category (e.g., "featured", "research", "playground")
    pub category: Option<String>,
    /// Filter by group (e.g., "general", "entered", "inClass")
    pub group: Option<String>,
    /// Sort order (e.g., "prize", "deadline", "teamCount")
    pub sort_by: Option<String>,
    /// Page number for pagination
    pub page: Option<i32>,
}