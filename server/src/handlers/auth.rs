use axum::{extract::State, http::StatusCode, Json};
use rand::Rng;
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::create_token,
    models::user::{AuthResponse, PublicUser, RegisterRequest, User, VerifyOtpRequest},
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    req.validate()
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

    if !req.consent {
        return Err((StatusCode::BAD_REQUEST, "Consent is required".into()));
    }

    // Check uniqueness (email OR username)
    let exists = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE email = $1 OR username = $2",
        req.email, req.username
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .unwrap_or(0);

    if exists > 0 {
        return Err((StatusCode::CONFLICT, "Username or email already taken".into()));
    }

    // Generate 6-digit OTP
    let otp: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Uniform::new(0u32, 10))
        .take(6)
        .map(|d| d.to_string())
        .collect();
    let otp_expires = chrono::Utc::now() + chrono::Duration::minutes(15);

    // Upsert user (pre-verified=false, otp set)
    sqlx::query!(
        r#"INSERT INTO users (id, username, email, birth_year, country_code, consent_given, consent_at, otp_code, otp_expires_at, otp_attempts)
           VALUES ($1, $2, $3, $4, $5, true, NOW(), $6, $7, 0)
           ON CONFLICT (email) DO UPDATE SET
               otp_code = $6,
               otp_expires_at = $7,
               otp_attempts = 0"#,
        Uuid::new_v4(),
        req.username,
        req.email,
        req.birth_year,
        req.country_code,
        otp,
        otp_expires,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Send OTP via SendGrid
    send_otp_email(&state, &req.email, &otp).await;

    Ok(Json(serde_json::json!({
        "message": "Verification code sent to your email. Valid for 15 minutes.",
        // Only expose OTP in debug builds (dev convenience)
        "dev_otp": if cfg!(debug_assertions) { Some(&otp) } else { None }
    })))
}

pub async fn verify_otp(
    State(state): State<AppState>,
    Json(req): Json<VerifyOtpRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // Fetch user and check lockout (max 5 attempts)
    let row = sqlx::query!(
        r#"SELECT id, otp_code, otp_expires_at, otp_attempts
           FROM users
           WHERE email = $1 AND deleted_at IS NULL"#,
        req.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Email not found".into()))?;

    if row.otp_attempts.unwrap_or(0) >= 5 {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Too many failed attempts. Request a new code.".into()));
    }

    let otp_valid = row.otp_code.as_deref() == Some(req.otp.as_str())
        && row.otp_expires_at.map(|exp| exp > chrono::Utc::now()).unwrap_or(false);

    if !otp_valid {
        // Increment attempt counter
        sqlx::query!(
            "UPDATE users SET otp_attempts = otp_attempts + 1 WHERE id = $1",
            row.id
        )
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        return Err((StatusCode::UNAUTHORIZED, "Invalid or expired code".into()));
    }

    // OTP valid — clear it, mark email verified
    sqlx::query!(
        "UPDATE users SET email_verified = true, otp_code = NULL, otp_expires_at = NULL, otp_attempts = 0 WHERE id = $1",
        row.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, email_verified, birth_year, country_code,
                  orcid_id, degree_verified, is_pro, fclc_node_id, fclc_node_active,
                  consent_given, consent_at, created_at
           FROM users WHERE id = $1"#,
        row.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = create_token(user.id, &state.config.jwt_secret, state.config.jwt_expiry_hours)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse { token, user: user.into() }))
}

/// Send OTP via SendGrid transactional email API.
/// Falls back to logging if API key is not configured (dev mode).
async fn send_otp_email(state: &AppState, to_email: &str, otp: &str) {
    if state.config.sendgrid_api_key.is_empty() {
        tracing::warn!(
            email = %to_email,
            "SENDGRID_API_KEY not set — OTP not sent by email (dev mode: {})",
            otp
        );
        return;
    }

    let body = serde_json::json!({
        "personalizations": [{"to": [{"email": to_email}]}],
        "from": {"email": state.config.from_email},
        "subject": "LongevityCommon — Your verification code",
        "content": [{
            "type": "text/plain",
            "value": format!(
                "Your LongevityCommon verification code: {}\n\nValid for 15 minutes.\n\n\
                 If you did not request this code, ignore this email.\n\n\
                 LongevityCommon — longevity social network",
                otp
            )
        }]
    });

    let client = reqwest::Client::new();
    match client
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(&state.config.sendgrid_api_key)
        .json(&body)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            tracing::info!(email = %to_email, "OTP email sent via SendGrid");
        }
        Ok(resp) => {
            tracing::error!(
                email = %to_email,
                status = %resp.status(),
                "SendGrid returned error"
            );
        }
        Err(e) => {
            tracing::error!(email = %to_email, error = %e, "Failed to send OTP email");
        }
    }
}
