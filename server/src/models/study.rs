use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Study {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub hypothesis: String,
    pub protocol: serde_json::Value,
    pub target_n: i32,
    pub enrolled_n: Option<i32>,
    pub duration_days: i32,
    pub status: Option<String>,
    pub dua_template_id: Option<String>,
    pub result_doi: Option<String>,
    pub arbiter_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StudyEnrollment {
    pub id: Uuid,
    pub study_id: Uuid,
    pub user_id: Uuid,
    pub consent_text: String,
    pub consented_at: DateTime<Utc>,
    pub status: String,
    pub shapley_weight: f64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStudyRequest {
    #[validate(length(min = 10, max = 200))]
    pub title: String,
    #[validate(length(min = 20))]
    pub hypothesis: String,
    pub protocol: serde_json::Value,
    pub target_n: i32,
    pub duration_days: i32,
    pub dua_template_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JoinStudyRequest {
    pub consent_text: String,
}

#[derive(Debug, Deserialize)]
pub struct StudiesQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
}
