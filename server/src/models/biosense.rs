/// BioSenseExport — standardized JSON schema for BioSense device/pipeline output.
///
/// This is the canonical format for passing measured χ_Ze values from the
/// BioSense Python pipeline (or any compatible sensor) into LongevityCommon.
///
/// Schema version: 1.0
/// Reference: Tkemaladze J., Ze Vectors Theory, DOI 10.65649/nhjtra67
///
/// Usage:
///   POST /api/biosense/compute  →  BioSenseComputeResponse
///   POST /api/data/import       →  stores as ze_samples record
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ── Sub-structs ────────────────────────────────────────────────────────────────

/// EEG-derived Ze metrics from BioSense Python pipeline.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EegMetrics {
    /// χ_Ze computed from EEG signal  ∈ [0, 1]
    pub chi_ze_eeg: f64,
    /// Ze velocity v = N_S / (N−1)  ∈ [0, 1]
    pub v_eeg: f64,
    /// Number of EEG samples used
    pub n_samples: u32,
    /// Computation method: "narrowband" | "proxy_alpha"
    pub method: String,
    /// EEG band used, e.g. "alpha_8_12hz" or "broadband"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band: Option<String>,
    /// Sampling rate (Hz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate_hz: Option<u32>,
    /// Number of EEG channels averaged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_channels: Option<u8>,
}

/// HRV-derived metrics from BioSense Python pipeline.
///
/// NOTE (CONCEPT v3.2, 2026-04-12): chi_ze_hrv (LF/HF switching rate) failed empirical
/// validation on PhysioNet Fantasia N=40 (d=−0.112, p=0.725). It is retained as a
/// research field. The validated organism score is now sdnn_ms (d=0.724, p=0.028).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HrvMetrics {
    /// SDNN (ms) — validated interim organism biomarker (Fantasia N=40, d=0.724, p=0.028, BCa).
    /// Primary organism score per CONCEPT v3.2. Use for organism component of health score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdnn_ms: Option<f64>,
    /// RMSSD (ms) — secondary parasympathetic HRV metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmssd_ms: Option<f64>,
    /// χ_Ze computed from HRV LF/HF switching rate  ∈ [0, 1].
    /// RESEARCH ONLY — failed validation (d=−0.112, p=0.725). Do NOT use as organism score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chi_ze_hrv: Option<f64>,
    /// LF/HF ratio from Welch PSD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lf_hf_ratio: Option<f64>,
    /// Number of NN intervals used
    pub n_beats: u32,
    /// Ze velocity from RR intervals  ∈ [0, 1]. Research field.
    pub v_hrv: f64,
}

/// Optional device/sensor metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    /// Device model, e.g. "BioSense-1" or "Polar H10"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Firmware version string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    /// Software/pipeline version that generated these metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<String>,
}

// ── Main export struct ─────────────────────────────────────────────────────────

/// BioSenseExport: full standardized export from one measurement session.
///
/// At minimum one of `eeg` or `hrv` must be present.
/// `chrono_age` enables biological age estimation (optional).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BioSenseExport {
    /// Schema version — always "1.0"
    pub schema_version: String,
    /// ISO 8601 timestamp of measurement
    pub recorded_at: DateTime<Utc>,
    /// Chronological age of subject in years (enables bio_age estimation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrono_age: Option<f64>,
    /// Anonymous subject ID (not stored, only for client-side tracking)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    /// EEG metrics (optional if HRV is present)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eeg: Option<EegMetrics>,
    /// HRV metrics (optional if EEG is present)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrv: Option<HrvMetrics>,
    /// Device/pipeline metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceInfo>,
    /// Free-text notes (e.g. "eyes closed", "post-exercise")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

// ── Request / Response ─────────────────────────────────────────────────────────

/// Request body for POST /api/biosense/compute
pub type ComputeChiZeRequest = BioSenseExport;

/// Response from POST /api/biosense/compute
///
/// Organism score hierarchy (CONCEPT v3.2):
///   1. organism_sdnn  — from sdnn_ms (validated, PRIMARY)
///   2. chi_ze_combined — from χ_Ze (research, SECONDARY; null if no χ_Ze input)
#[derive(Debug, Serialize)]
pub struct ComputeChiZeResponse {
    // ── Validated organism biomarkers (SDNN/RMSSD) ──────────────────────────
    /// SDNN (ms) echoed from input; primary validated organism signal.
    pub sdnn_ms: Option<f64>,
    /// RMSSD (ms) echoed from input.
    pub rmssd_ms: Option<f64>,
    /// Normalised organism score from SDNN: clamp((sdnn_ms − 10) / 170, 0, 1).
    /// Null if sdnn_ms not provided. Use as W_ORGANISM component in health score.
    pub organism_sdnn: Option<f64>,

    // ── Research χ_Ze fields ─────────────────────────────────────────────────
    /// χ_Ze from EEG  ∈ [0, 1] (null if no EEG input). RESEARCH ONLY.
    pub chi_ze_eeg: Option<f64>,
    /// χ_Ze from HRV  ∈ [0, 1] (null if no HRV input). RESEARCH ONLY.
    pub chi_ze_hrv: Option<f64>,
    /// Combined χ_Ze  ∈ [0, 1]. Null if no χ_Ze input. RESEARCH ONLY.
    pub chi_ze_combined: Option<f64>,
    /// Normalised damage index: D_norm = 1.2 * (1 − χ_Ze_combined)  ∈ [0, 1].
    /// Null if chi_ze_combined is null.
    pub d_norm: Option<f64>,

    // ── Biological age ───────────────────────────────────────────────────────
    /// Biological age estimate (years); null if chrono_age not provided
    pub bio_age: Option<f64>,
    /// 95% CI lower bound (years); null if chrono_age not provided
    pub bio_age_ci_low: Option<f64>,
    /// 95% CI upper bound (years); null if chrono_age not provided
    pub bio_age_ci_high: Option<f64>,
    /// CI stability: "high" (<2y half-width) | "medium" (<5y) | "low" (≥5y)
    pub ci_stability: Option<String>,
    /// Calibration constant used: "dual" | "eeg_only" | "hrv_only" | "sdnn"
    pub calibration: String,
    /// Schema version echo
    pub schema_version: String,
}
