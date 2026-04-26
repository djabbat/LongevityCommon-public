use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── 4-Factor Health Model ──────────────────────────────────────────────────────
// DEPRECATED weights (LongevityCommon/CONCEPT.md §A.2, 2026-04-22):
// The formula "0.40·organism + 0.25·psyche + 0.20·consciousness + 0.15·social"
// was REMOVED from CONCEPT because the weights had no mathematical derivation
// from MCOA L_tissue. Retained here only as transitional research composite
// (always accompanied by HEALTH_SCORE_DISCLAIMER). Planned replacement: L_tissue
// per tissue type from MCOA calibration. See LongevityCommon/TODO.md architectural item.
// Sensor source for organism component: organism_sdnn from biosense handler
// (χ_Ze_eeg + χ_Ze_hrv failed 4 pre-reg tests — EVIDENCE.md 2026-04-22).
pub const W_ORGANISM:      f64 = 0.40;
pub const W_PSYCHE:        f64 = 0.25;
pub const W_CONSCIOUSNESS: f64 = 0.20;
pub const W_SOCIAL:        f64 = 0.15;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct HealthFactor {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recorded_at: DateTime<Utc>,
    pub psyche_score: Option<f64>,
    pub psyche_mood: Option<String>,
    pub psyche_stress: Option<f64>,
    pub psyche_notes: Option<String>,
    pub consciousness_score: Option<f64>,
    pub consciousness_mindful: Option<f64>,
    pub consciousness_purpose: Option<f64>,
    pub consciousness_notes: Option<String>,
    pub social_score: Option<f64>,
    pub social_support: Option<f64>,
    pub social_isolation: Option<f64>,
    pub social_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// R7 fix: health score is an unvalidated research composite. Must never be presented as a
/// clinical indicator, used for insurance/employment screening, or displayed without this note.
pub const HEALTH_SCORE_DISCLAIMER: &str =
    "The integrated health score (0–1) is an UNVALIDATED experimental composite index \
     based on self-reported and sensor data. It has not been clinically validated, \
     does not constitute a medical assessment, and must NOT be used for \
     clinical, insurance, or employment decisions. \
     Scores may fluctuate significantly and do not reflect actual health status.";

#[derive(Debug, Serialize, Default)]
pub struct HealthFactorSummary {
    /// Average psyche score over the window (0–1)
    pub psyche: Option<f64>,
    /// Average consciousness score over the window (0–1)
    pub consciousness: Option<f64>,
    /// Average social score over the window (0–1)
    pub social: Option<f64>,
    /// DEPRECATED composite (weights removed from CONCEPT.md §A.2 on 2026-04-22):
    /// prior formula `0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social`
    /// is retained here only as a transitional research placeholder and MUST be
    /// accompanied by `health_score_disclaimer`. Slated for replacement by
    /// tissue-specific L_tissue from MCOA. None if fewer than 2 factors have data.
    pub health_score: Option<f64>,
    /// Mandatory disclaimer to accompany any health_score display. Never None when health_score is Some.
    pub health_score_disclaimer: Option<&'static str>,
    /// Number of factors with data (1–4; organism counts if chi_ze_combined is Some)
    pub factors_filled: u8,
}

#[derive(Debug, Deserialize)]
pub struct CreateHealthFactorRequest {
    pub recorded_at: DateTime<Utc>,
    pub psyche_score: Option<f64>,
    pub psyche_mood: Option<String>,
    pub psyche_stress: Option<f64>,
    pub psyche_notes: Option<String>,
    pub consciousness_score: Option<f64>,
    pub consciousness_mindful: Option<f64>,
    pub consciousness_purpose: Option<f64>,
    pub consciousness_notes: Option<String>,
    pub social_score: Option<f64>,
    pub social_support: Option<f64>,
    pub social_isolation: Option<f64>,
    pub social_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ZeSample {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recorded_at: DateTime<Utc>,
    pub source: String,
    pub chi_ze_eeg: Option<f64>,
    pub chi_ze_hrv: Option<f64>,
    pub chi_ze_combined: Option<f64>,
    pub d_norm: Option<f64>,
    pub bio_age_est: Option<f64>,
    pub bio_age_ci_low: Option<f64>,
    pub bio_age_ci_high: Option<f64>,
    pub ci_stability: Option<String>,
    pub fclc_signature: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ZeProfile {
    pub user_id: Uuid,
    pub username: String,
    pub chrono_age: Option<f64>,
    // ── Organism (χ_Ze) ──────────────────────────────────────────────
    pub bio_age_est: Option<f64>,
    pub bio_age_ci_low: Option<f64>,
    pub bio_age_ci_high: Option<f64>,
    pub bio_age_delta: Option<f64>,
    pub ci_stability: Option<String>,
    pub chi_ze_eeg: Option<f64>,
    pub chi_ze_hrv: Option<f64>,
    pub chi_ze_combined: Option<f64>,
    pub trend_7d: Option<f64>,
    pub trend_30d: Option<f64>,
    pub fclc_node_active: bool,
    pub cohort_percentile: Option<f64>,
    pub last_sample_at: Option<DateTime<Utc>>,
    pub sample_count: i64,
    // ── 4-Factor Health Summary ───────────────────────────────────────
    pub health_factors: HealthFactorSummary,
}

#[derive(Debug, Serialize)]
pub struct ZeTrend {
    pub period_days: i32,
    pub points: Vec<ZeTrendPoint>,
}

#[derive(Debug, Serialize)]
pub struct ZeTrendPoint {
    pub date: DateTime<Utc>,
    pub chi_ze_combined: Option<f64>,
    pub bio_age_est: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ImportDataRequest {
    pub source: String,
    pub device_id: Option<String>,
    pub fclc_signature: Option<String>,
    pub samples: Vec<SampleInput>,
    pub interventions: Option<Vec<InterventionInput>>,
}

#[derive(Debug, Deserialize)]
pub struct SampleInput {
    pub recorded_at: DateTime<Utc>,
    pub chi_ze_eeg: Option<f64>,
    pub chi_ze_hrv: Option<f64>,
    pub raw: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct InterventionInput {
    pub recorded_at: DateTime<Utc>,
    pub r#type: String,
    pub value: serde_json::Value,
    pub notes: Option<String>,
}
