use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// R7 fix: Ze·Guide meets the MDR 2017/745 / FDA definition of Software as a Medical Device (SaMD)
/// if used for health decisions. Until CE/FDA clearance is obtained, it MUST NOT be used for
/// clinical decisions. This disclaimer must appear in every API response (logged: disclaimer_sent=true).
pub const ZE_GUIDE_DISCLAIMER: &str =
    "Ze·Guide is a research assistant for scientific exploration only. \
     It is NOT a certified medical device (MDR 2017/745) and has NOT received CE or FDA clearance. \
     χ_Ze, D_norm, and biological age estimates are experimental research metrics — \
     NOT validated diagnostic tools. \
     Do NOT use these outputs to make clinical or personal health decisions. \
     Always consult a licensed healthcare professional. \
     This system may produce errors or misleading information.";

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ZeGuideLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub prompt: String,
    pub response: String,
    pub model_used: String,
    pub cited_dois: Vec<String>,
    pub cited_files: Vec<String>,
    pub disclaimer_sent: bool,
    pub latency_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ZeGuideAskRequest {
    pub session_id: Option<Uuid>,
    #[validate(length(min = 5, max = 2000))]
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ZeGuideResponse {
    pub session_id: Uuid,
    pub disclaimer: String,
    pub response: String,
    pub cited_dois: Vec<String>,
    pub cited_files: Vec<String>,
    pub model_used: String,
}
