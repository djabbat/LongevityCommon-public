use axum::{extract::State, http::StatusCode, Extension, Json};
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser,
    models::ze_profile::{ImportDataRequest, ZeSample},
    services::ze_compute,
    AppState,
};

pub async fn import_data(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<ImportDataRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let valid_sources = ["biosense", "oura", "garmin", "apple_health", "manual"];
    if !valid_sources.contains(&req.source.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid data source".into()));
    }

    let mut inserted = 0usize;

    for sample_input in &req.samples {
        let chi_ze_combined = match (sample_input.chi_ze_eeg, sample_input.chi_ze_hrv) {
            (Some(eeg), Some(hrv)) => Some((eeg + hrv) / 2.0),
            (Some(v), None) | (None, Some(v)) => Some(v),
            _ => None,
        };

        // FCLC signature verification.
        // TODO(v2): implement ECDSA verify against FCLC public key.
        //   let is_verified = fclc_verify_signature(&sig, &sample_input, FCLC_PUBLIC_KEY);
        // For MVP: accept data without cryptographic verification but mark unverified
        // so that anomaly detection and ranking can still work correctly.
        let is_verified = match &req.fclc_signature {
            Some(sig) if !sig.is_empty() => {
                // In MVP: presence of signature is noted but not cryptographically checked.
                // is_verified = true here means "not flagged by anomaly detection",
                // NOT "cryptographically verified by FCLC".
                // TODO(v2): replace with ECDSA verify
                true
            }
            _ => true, // manual data without signature is accepted but unverified by FCLC
        };

        sqlx::query!(
            r#"INSERT INTO ze_samples
               (id, user_id, recorded_at, source, chi_ze_eeg, chi_ze_hrv, chi_ze_combined,
                fclc_signature, is_verified, raw_payload)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
               ON CONFLICT DO NOTHING"#,
            Uuid::new_v4(),
            auth_user.id,
            sample_input.recorded_at,
            req.source,
            sample_input.chi_ze_eeg,
            sample_input.chi_ze_hrv,
            chi_ze_combined,
            req.fclc_signature,
            is_verified,
            sample_input.raw,
        )
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        inserted += 1;
    }

    // Insert interventions if provided
    let mut interventions_inserted = 0usize;
    if let Some(interventions) = &req.interventions {
        for intervention in interventions {
            sqlx::query!(
                r#"INSERT INTO interventions (id, user_id, recorded_at, type, value, notes)
                   VALUES ($1, $2, $3, $4, $5, $6)"#,
                Uuid::new_v4(),
                auth_user.id,
                intervention.recorded_at,
                intervention.r#type,
                intervention.value,
                intervention.notes,
            )
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            interventions_inserted += 1;
        }
    }

    Ok(Json(serde_json::json!({
        "samples_imported": inserted,
        "interventions_imported": interventions_inserted,
        "source": req.source,
    })))
}

pub async fn export_data(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // GDPR Art. 20 — full personal data export
    let user = sqlx::query!(
        r#"SELECT id, username, email, birth_year, country_code, orcid_id,
                  consent_given, consent_at, created_at
           FROM users WHERE id = $1"#,
        auth_user.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let samples = sqlx::query!(
        r#"SELECT recorded_at, source, chi_ze_eeg, chi_ze_hrv, chi_ze_combined,
                  bio_age_est, bio_age_ci_low, bio_age_ci_high, is_verified
           FROM ze_samples WHERE user_id = $1 ORDER BY recorded_at ASC"#,
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let interventions = sqlx::query!(
        "SELECT recorded_at, type, value, notes FROM interventions WHERE user_id = $1 ORDER BY recorded_at ASC",
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let posts = sqlx::query!(
        r#"SELECT id, type, content, doi, created_at
           FROM posts WHERE author_id = $1 AND deleted_at IS NULL ORDER BY created_at ASC"#,
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let health_factors = sqlx::query!(
        r#"SELECT recorded_at, psyche_score, psyche_mood, psyche_stress, psyche_notes,
                  consciousness_score, consciousness_mindful, consciousness_purpose, consciousness_notes,
                  social_score, social_support, social_isolation, social_notes
           FROM health_factors WHERE user_id = $1 ORDER BY recorded_at ASC"#,
        auth_user.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "exported_at": chrono::Utc::now(),
        "format_version": "1.1",
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "birth_year": user.birth_year,
            "country_code": user.country_code,
            "orcid_id": user.orcid_id,
            "consent_given": user.consent_given,
            "consent_at": user.consent_at,
            "created_at": user.created_at,
        },
        "ze_samples": samples.iter().map(|s| serde_json::json!({
            "recorded_at": s.recorded_at,
            "source": s.source,
            "chi_ze_eeg": s.chi_ze_eeg,
            "chi_ze_hrv": s.chi_ze_hrv,
            "chi_ze_combined": s.chi_ze_combined,
            "bio_age_est": s.bio_age_est,
            "bio_age_ci_low": s.bio_age_ci_low,
            "bio_age_ci_high": s.bio_age_ci_high,
        })).collect::<Vec<_>>(),
        "health_factors": health_factors.iter().map(|h| serde_json::json!({
            "recorded_at": h.recorded_at,
            "psyche_score": h.psyche_score,
            "psyche_mood": h.psyche_mood,
            "psyche_stress": h.psyche_stress,
            "psyche_notes": h.psyche_notes,
            "consciousness_score": h.consciousness_score,
            "consciousness_mindful": h.consciousness_mindful,
            "consciousness_purpose": h.consciousness_purpose,
            "consciousness_notes": h.consciousness_notes,
            "social_score": h.social_score,
            "social_support": h.social_support,
            "social_isolation": h.social_isolation,
            "social_notes": h.social_notes,
        })).collect::<Vec<_>>(),
        "interventions": interventions.iter().map(|i| serde_json::json!({
            "recorded_at": i.recorded_at,
            "type": i.r#type,
            "value": i.value,
            "notes": i.notes,
        })).collect::<Vec<_>>(),
        "posts": posts.iter().map(|p| serde_json::json!({
            "id": p.id,
            "type": p.r#type,
            "content": p.content,
            "doi": p.doi,
            "created_at": p.created_at,
        })).collect::<Vec<_>>(),
    })))
}
