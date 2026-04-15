/// POST /api/biosense/compute
///
/// Standalone, **public** (no auth required) endpoint.
/// Accepts a BioSenseExport JSON payload and returns:
///   1. Validated organism score from SDNN/RMSSD (CONCEPT v3.2, primary path)
///   2. Research χ_Ze metrics + D_norm + bio_age (secondary path, research only)
///
/// ORGANISM SCORE HIERARCHY (CONCEPT v3.2, 2026-04-12):
///   PRIMARY:   organism_sdnn = clamp((sdnn_ms − 10) / 170, 0, 1)
///              Validated: d=0.724, p=0.028, Fantasia N=40 (BCa CI [0.096, 1.653])
///   SECONDARY: chi_ze_combined (research; χ_Ze failed 4 pre-registered tests)
///
/// χ_Ze calibration constants (research path, R²=0.84 on Cuban+Dortmund data):
///   dual     K = 0.45
///   eeg_only K = 0.42
///   hrv_only K = 0.38
///
/// Bridge equation (research path):
///   D_norm  = 1.2 * (1 − χ_Ze_combined)        [clamped 0..1]
///   bio_age = chrono_age * (1 − D_norm * K)
///
/// CI via Jacobian error propagation (single-sample → default SE = 0.05):
///   CI_half = chrono_age * 1.2 * K * SE * 1.96
use axum::{http::StatusCode, Json};

use crate::models::biosense::{ComputeChiZeRequest, ComputeChiZeResponse};

// χ_Ze calibration constants — research path (kept in sync with services/ze_compute.rs)
const K_DUAL: f64 = 0.45;
const K_EEG_ONLY: f64 = 0.42;
const K_HRV_ONLY: f64 = 0.38;
const D_NORM_ALPHA: f64 = 1.2;
/// Default SE for single-sample CI (no history available)
const DEFAULT_SE_CHI: f64 = 0.05;

/// SDNN normalisation range (ms): clinical reference 10–180 ms.
/// organism_sdnn = clamp((sdnn_ms − SDNN_MIN) / SDNN_RANGE, 0, 1)
const SDNN_MIN: f64 = 10.0;
const SDNN_RANGE: f64 = 170.0; // 180 − 10

pub async fn compute_chi_ze(
    Json(payload): Json<ComputeChiZeRequest>,
) -> Result<Json<ComputeChiZeResponse>, (StatusCode, String)> {
    // ── SDNN/RMSSD (validated organism path) ────────────────────────────────
    let sdnn_ms = payload.hrv.as_ref().and_then(|h| h.sdnn_ms);
    let rmssd_ms = payload.hrv.as_ref().and_then(|h| h.rmssd_ms);

    if let Some(sdnn) = sdnn_ms {
        if sdnn < 0.0 || sdnn > 1000.0 {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("sdnn_ms must be in [0, 1000], got {sdnn}"),
            ));
        }
    }

    let organism_sdnn = sdnn_ms.map(|s| ((s - SDNN_MIN) / SDNN_RANGE).clamp(0.0, 1.0));

    // ── χ_Ze (research path) ─────────────────────────────────────────────────
    let chi_eeg = payload.eeg.as_ref().map(|e| e.chi_ze_eeg);
    let chi_hrv = payload.hrv.as_ref().and_then(|h| h.chi_ze_hrv);

    // Validate χ_Ze range [0, 1]
    for (label, val) in [("chi_ze_eeg", chi_eeg), ("chi_ze_hrv", chi_hrv)]
        .iter()
        .filter_map(|(l, v)| v.map(|x| (*l, x)))
    {
        if !(0.0..=1.0).contains(&val) {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("{label} must be in [0, 1], got {val}"),
            ));
        }
    }

    // Require at least one sensor input (SDNN or χ_Ze)
    if sdnn_ms.is_none() && chi_eeg.is_none() && chi_hrv.is_none() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "At least one of: hrv.sdnn_ms, eeg.chi_ze_eeg, or hrv.chi_ze_hrv must be provided".into(),
        ));
    }

    // χ_Ze combined (research only — null if no χ_Ze provided)
    let (chi_combined_opt, d_norm_opt, calibration_label) = match (chi_eeg, chi_hrv) {
        (Some(eeg), Some(hrv)) => {
            let combined = (eeg + hrv) / 2.0;
            let d = (D_NORM_ALPHA * (1.0 - combined)).clamp(0.0, 1.0);
            (Some(combined), Some(d), "dual")
        }
        (Some(eeg), None) => {
            let d = (D_NORM_ALPHA * (1.0 - eeg)).clamp(0.0, 1.0);
            (Some(eeg), Some(d), "eeg_only")
        }
        (None, Some(hrv)) => {
            let d = (D_NORM_ALPHA * (1.0 - hrv)).clamp(0.0, 1.0);
            (Some(hrv), Some(d), "hrv_only")
        }
        (None, None) => (None, None, if sdnn_ms.is_some() { "sdnn" } else { "none" }),
    };

    // ── Biological age (research path — only if χ_Ze available) ─────────────
    let (bio_age, ci_low, ci_high, ci_stability) = match (chi_combined_opt, payload.chrono_age) {
        (Some(chi_combined), Some(ca)) if ca > 0.0 && ca < 150.0 => {
            let d_norm = (D_NORM_ALPHA * (1.0 - chi_combined)).clamp(0.0, 1.0);
            let k = match (chi_eeg, chi_hrv) {
                (Some(_), Some(_)) => K_DUAL,
                (Some(_), None) => K_EEG_ONLY,
                (None, Some(_)) => K_HRV_ONLY,
                _ => K_DUAL,
            };
            let est = ca * (1.0 - d_norm * k);
            let jacobian = ca * D_NORM_ALPHA * k;
            let ci_half = (jacobian * DEFAULT_SE_CHI * 1.96).max(0.5);
            let stability = if ci_half < 2.0 {
                "high"
            } else if ci_half < 5.0 {
                "medium"
            } else {
                "low"
            };
            (
                Some(round2(est)),
                Some(round2(est - ci_half)),
                Some(round2(est + ci_half)),
                Some(stability.to_string()),
            )
        }
        (_, Some(ca)) if !(ca > 0.0 && ca < 150.0) => {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("chrono_age must be in (0, 150), got {ca}"),
            ));
        }
        _ => (None, None, None, None),
    };

    Ok(Json(ComputeChiZeResponse {
        // Validated organism fields
        sdnn_ms: sdnn_ms.map(round2),
        rmssd_ms: rmssd_ms.map(round2),
        organism_sdnn: organism_sdnn.map(round6),
        // Research χ_Ze fields
        chi_ze_eeg: chi_eeg.map(round6),
        chi_ze_hrv: chi_hrv.map(round6),
        chi_ze_combined: chi_combined_opt.map(round6),
        d_norm: d_norm_opt.map(round6),
        // Biological age
        bio_age,
        bio_age_ci_low: ci_low,
        bio_age_ci_high: ci_high,
        ci_stability,
        calibration: calibration_label.to_string(),
        schema_version: "1.1".to_string(),
    }))
}

// ── helpers ────────────────────────────────────────────────────────────────────

#[inline]
fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

#[inline]
fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}
