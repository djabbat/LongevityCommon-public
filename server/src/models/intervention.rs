use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Intervention {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recorded_at: DateTime<Utc>,
    pub r#type: String,
    pub value: serde_json::Value,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInterventionRequest {
    pub recorded_at: DateTime<Utc>,
    pub r#type: String,
    pub value: serde_json::Value,
    pub notes: Option<String>,
}
