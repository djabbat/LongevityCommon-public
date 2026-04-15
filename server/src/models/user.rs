use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub birth_year: Option<i32>,
    pub country_code: Option<String>,
    pub orcid_id: Option<String>,
    pub degree_verified: bool,
    pub is_pro: bool,
    pub fclc_node_id: Option<String>,
    pub fclc_node_active: bool,
    pub consent_given: bool,
    pub consent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub country_code: Option<String>,
    pub orcid_id: Option<String>,
    pub degree_verified: bool,
    pub fclc_node_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            country_code: u.country_code,
            orcid_id: u.orcid_id,
            degree_verified: u.degree_verified,
            fclc_node_active: u.fclc_node_active,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub birth_year: Option<i32>,
    pub country_code: Option<String>,
    pub consent: bool,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: PublicUser,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 3, max = 30))]
    pub username: Option<String>,
    pub birth_year: Option<i32>,
    pub country_code: Option<String>,
    pub orcid_id: Option<String>,
    pub fclc_node_id: Option<String>,
}
