use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,      // user id
    pub exp: usize,     // expiry timestamp
    pub iat: usize,     // issued at
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
}

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, &'static str)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header"))?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?
    .claims;

    req.extensions_mut().insert(AuthUser { id: claims.sub });
    Ok(next.run(req).await)
}

pub fn create_token(user_id: Uuid, secret: &str, expiry_hours: u64) -> anyhow::Result<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let claims = Claims {
        sub: user_id,
        iat: now,
        exp: now + (expiry_hours as usize * 3600),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}
