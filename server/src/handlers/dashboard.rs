use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser,
    models::{
        intervention::CreateInterventionRequest,
        ze_profile::{ZeSample, HealthFactor, CreateHealthFactorRequest},
    },
    services::ze_compute,
    AppState,
};

#[derive(Deserialize)]
pub struct TrendQuery {
    pub period: Option<i32>,
}

pub async fn get_dashboard(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let user = sqlx::query!(
        "SELECT id, username, birth_year, country_code, fclc_node_active FROM users WHERE id = $1",
        auth_user.id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "User not found".into()))?;

    let samples = sqlx::query_as!(
        ZeSample,
        r#"SELECT id, user_id, recorded_at, source, chi_ze_eeg, chi_ze_hrv, chi_ze_combined,
                  d_norm, bio_age_est, bio_age_ci_low, bio_age_ci_high, ci_stability,
                  fclc_signature, is_verified, created_at
           FROM ze_samples
           WHERE user_id = $1
           ORDER BY recorded_at DESC
           LIMIT 500"#,
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Cohort samples for percentile (same birth_year ±2)
    let cohort_samples = if let Some(by) = user.birth_year {
        sqlx::query_as!(
            ZeSample,
            r#"SELECT zs.id, zs.user_id, zs.recorded_at, zs.source,
                      zs.chi_ze_eeg, zs.chi_ze_hrv, zs.chi_ze_combined,
                      zs.d_norm, zs.bio_age_est, zs.bio_age_ci_low, zs.bio_age_ci_high,
                      zs.ci_stability, zs.fclc_signature, zs.is_verified, zs.created_at
               FROM ze_samples zs
               JOIN users u ON u.id = zs.user_id
               WHERE u.birth_year BETWEEN $1 AND $2
                 AND zs.user_id != $3
                 AND zs.is_verified = true
               ORDER BY zs.recorded_at DESC
               LIMIT 1000"#,
            by - 2,
            by + 2,
            auth_user.id
        )
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
    } else {
        vec![]
    };

    let health_factors = sqlx::query_as!(
        HealthFactor,
        r#"SELECT id, user_id, recorded_at,
                  psyche_score, psyche_mood, psyche_stress, psyche_notes,
                  consciousness_score, consciousness_mindful, consciousness_purpose, consciousness_notes,
                  social_score, social_support, social_isolation, social_notes,
                  created_at
           FROM health_factors
           WHERE user_id = $1
             AND recorded_at > NOW() - INTERVAL '30 days'
           ORDER BY recorded_at DESC"#,
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let profile = ze_compute::compute_profile(
        user.id,
        user.username,
        user.birth_year,
        user.country_code,
        user.fclc_node_active,
        &samples,
        &cohort_samples,
        &health_factors,
    );

    Ok(Json(serde_json::to_value(profile).unwrap()))
}

pub async fn get_trend(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(query): Query<TrendQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let period = query.period.unwrap_or(30).clamp(7, 365);

    let samples = sqlx::query_as!(
        ZeSample,
        r#"SELECT id, user_id, recorded_at, source, chi_ze_eeg, chi_ze_hrv, chi_ze_combined,
                  d_norm, bio_age_est, bio_age_ci_low, bio_age_ci_high, ci_stability,
                  fclc_signature, is_verified, created_at
           FROM ze_samples
           WHERE user_id = $1
             AND recorded_at > NOW() - ($2 || ' days')::interval
           ORDER BY recorded_at ASC"#,
        auth_user.id,
        period.to_string(),
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let trend = ze_compute::compute_trend_series(&samples, period);
    Ok(Json(serde_json::to_value(trend).unwrap()))
}

pub async fn create_health_factor(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateHealthFactorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO health_factors (
            id, user_id, recorded_at,
            psyche_score, psyche_mood, psyche_stress, psyche_notes,
            consciousness_score, consciousness_mindful, consciousness_purpose, consciousness_notes,
            social_score, social_support, social_isolation, social_notes
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)"#,
        id,
        auth_user.id,
        req.recorded_at,
        req.psyche_score,
        req.psyche_mood,
        req.psyche_stress,
        req.psyche_notes,
        req.consciousness_score,
        req.consciousness_mindful,
        req.consciousness_purpose,
        req.consciousness_notes,
        req.social_score,
        req.social_support,
        req.social_isolation,
        req.social_notes,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id })))
}

pub async fn create_intervention(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateInterventionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let valid_types = ["sleep", "exercise", "fasting", "supplement", "other"];
    if !valid_types.contains(&req.r#type.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid intervention type".into()));
    }

    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO interventions (id, user_id, recorded_at, type, value, notes)
           VALUES ($1, $2, $3, $4, $5, $6)"#,
        id,
        auth_user.id,
        req.recorded_at,
        req.r#type,
        req.value,
        req.notes,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id })))
}
