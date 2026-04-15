use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub r#type: String,
    pub content: String,
    pub doi: Option<String>,
    pub doi_verified: bool,
    pub code_url: Option<String>,
    pub data_url: Option<String>,
    pub score: f64,
    pub rank_penalty: f64,
    pub parent_id: Option<Uuid>,
    pub study_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostWithAuthor {
    #[serde(flatten)]
    pub post: Post,
    pub author_username: String,
    pub author_degree_verified: bool,
    pub reactions: ReactionCounts,
}

#[derive(Debug, Serialize, Default)]
pub struct ReactionCounts {
    pub support: i64,
    pub replicate: i64,
    pub challenge: i64,
    pub cite: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[serde(rename = "type")]
    pub post_type: String,
    #[validate(length(min = 10, max = 10000))]
    pub content: String,
    pub doi: Option<String>,
    pub code_url: Option<String>,
    pub data_url: Option<String>,
    pub parent_id: Option<Uuid>,
    pub study_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct FeedQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    #[serde(rename = "type")]
    pub post_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReactRequest {
    #[serde(rename = "type")]
    pub reaction_type: String,
}
