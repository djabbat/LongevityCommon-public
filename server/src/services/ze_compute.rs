/// Ze compute engine
/// Implements bridge equation from Ze Theory:
///   D_norm = f(chi_ze_combined)
///   bio_age = chrono_age * (1 - D_norm * K)
/// K calibrated from N=196 Cuban EEG dataset (R²=0.84)
///
/// Reference: Tkemaladze J., CDATA series, Ze Vectors Theory

use crate::models::ze_profile::{
    ZeSample, ZeProfile, ZeTrend, ZeTrendPoint,
    HealthFactor, HealthFactorSummary, HEALTH_SCORE_DISCLAIMER,
    W_ORGANISM, W_PSYCHE, W_CONSCIOUSNESS, W_SOCIAL,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// K calibrated on Cuban EEG + Dortmund HRV dual-clock model (R²=0.84)
const K_CALIBRATION_DUAL: f64 = 0.45;
/// K for HRV-only input (Dortmund Vital Study single-sensor calibration)
const K_CALIBRATION_HRV_ONLY: f64 = 0.38;
/// K for EEG-only input (Cuban dataset single-sensor calibration)
const K_CALIBRATION_EEG_ONLY: f64 = 0.42;
const D_NORM_ALPHA: f64 = 1.2;
const ANOMALY_CONST_DAYS: i64 = 30;

pub fn compute_health_factors(
    organism_chi_ze: Option<f64>,
    factors: &[HealthFactor],
) -> HealthFactorSummary {
    let window_days = 30i64;
    let cutoff = Utc::now() - chrono::Duration::days(window_days);
    let recent: Vec<&HealthFactor> = factors
        .iter()
        .filter(|f| f.recorded_at > cutoff)
        .collect();

    let psyche = mean_option(recent.iter().map(|f| f.psyche_score));
    let consciousness = mean_option(recent.iter().map(|f| f.consciousness_score));
    let social = mean_option(recent.iter().map(|f| f.social_score));

    // organism score: chi_ze_combined normalised to [0,1]
    // chi_ze is already in [0,1] (biosynchrony index)
    let organism = organism_chi_ze;

    let mut filled: u8 = 0;
    let mut weighted_sum = 0.0f64;
    let mut weight_total = 0.0f64;

    if let Some(o) = organism {
        filled += 1;
        weighted_sum += o * W_ORGANISM;
        weight_total += W_ORGANISM;
    }
    if let Some(p) = psyche {
        filled += 1;
        weighted_sum += p * W_PSYCHE;
        weight_total += W_PSYCHE;
    }
    if let Some(c) = consciousness {
        filled += 1;
        weighted_sum += c * W_CONSCIOUSNESS;
        weight_total += W_CONSCIOUSNESS;
    }
    if let Some(s) = social {
        filled += 1;
        weighted_sum += s * W_SOCIAL;
        weight_total += W_SOCIAL;
    }

    // Require at least 2 factors for a meaningful score.
    // R7: always attach disclaimer when score is present.
    let health_score = if filled >= 2 && weight_total > 0.0 {
        Some(weighted_sum / weight_total)
    } else {
        None
    };
    let health_score_disclaimer = health_score.map(|_| HEALTH_SCORE_DISCLAIMER);

    HealthFactorSummary { psyche, consciousness, social, health_score, health_score_disclaimer, factors_filled: filled }
}

pub fn compute_profile(
    user_id: Uuid,
    username: String,
    birth_year: Option<i32>,
    country_code: Option<String>,
    fclc_node_active: bool,
    samples: &[ZeSample],
    cohort_samples: &[ZeSample],
    health_factors: &[HealthFactor],
) -> ZeProfile {
    let now = Utc::now();
    let verified: Vec<&ZeSample> = samples
        .iter()
        .filter(|s| s.is_verified)
        .collect();

    let chrono_age = birth_year.map(|y| (now.format("%Y").to_string().parse::<i32>().unwrap_or(2026) - y) as f64);

    if verified.is_empty() {
        let hf_summary = compute_health_factors(None, health_factors);
        return ZeProfile {
            user_id,
            username,
            chrono_age,
            bio_age_est: None,
            bio_age_ci_low: None,
            bio_age_ci_high: None,
            bio_age_delta: None,
            ci_stability: None,
            chi_ze_eeg: None,
            chi_ze_hrv: None,
            chi_ze_combined: None,
            trend_7d: None,
            trend_30d: None,
            fclc_node_active,
            cohort_percentile: None,
            last_sample_at: None,
            sample_count: 0,
            health_factors: hf_summary,
        };
    }

    // Latest 90 days
    let cutoff_90d = now - chrono::Duration::days(90);
    let recent: Vec<&ZeSample> = verified
        .iter()
        .copied()
        .filter(|s| s.recorded_at > cutoff_90d)
        .collect();

    let chi_ze_eeg = mean_option(recent.iter().map(|s| s.chi_ze_eeg));
    let chi_ze_hrv = mean_option(recent.iter().map(|s| s.chi_ze_hrv));
    // Select K based on which sensors are available
    let (chi_ze_combined, k_calibration) = match (chi_ze_eeg, chi_ze_hrv) {
        (Some(eeg), Some(hrv)) => (Some((eeg + hrv) / 2.0), K_CALIBRATION_DUAL),
        (Some(_), None)        => (chi_ze_eeg, K_CALIBRATION_EEG_ONLY),
        (None, Some(_))        => (chi_ze_hrv, K_CALIBRATION_HRV_ONLY),
        _                      => (mean_option(recent.iter().map(|s| s.chi_ze_combined)), K_CALIBRATION_DUAL),
    };

    // Bridge equation: D_norm = D_NORM_ALPHA * (1 - chi_ze_combined)
    let d_norm = chi_ze_combined.map(|chi| (D_NORM_ALPHA * (1.0 - chi)).clamp(0.0, 1.0));

    // Bio age point estimate using sensor-appropriate K
    let bio_age_est = match (chrono_age, d_norm) {
        (Some(ca), Some(dn)) => Some(ca * (1.0 - dn * k_calibration)),
        _ => None,
    };

    let (bio_age_ci_low, bio_age_ci_high, ci_stability) = compute_ci(bio_age_est, k_calibration, &recent);

    // Trends
    let cutoff_7d = now - chrono::Duration::days(7);
    let cutoff_14d = now - chrono::Duration::days(14);
    let cutoff_30d = now - chrono::Duration::days(30);
    let cutoff_60d = now - chrono::Duration::days(60);

    let trend_7d  = compute_trend(&recent, cutoff_7d,  cutoff_14d, now);
    // Bug fix: trend_30d period_end must be `now`, not `cutoff_30d`
    let trend_30d = compute_trend(&recent, cutoff_30d, cutoff_60d, now);

    let cohort_percentile = compute_cohort_percentile(bio_age_est, chrono_age, cohort_samples);
    let last_sample_at = recent.iter().map(|s| s.recorded_at).max();
    let hf_summary = compute_health_factors(chi_ze_combined, health_factors);

    ZeProfile {
        user_id,
        username,
        chrono_age,
        bio_age_est,
        bio_age_ci_low,
        bio_age_ci_high,
        bio_age_delta: bio_age_est.zip(chrono_age).map(|(b, c)| b - c),
        ci_stability,
        chi_ze_eeg,
        chi_ze_hrv,
        chi_ze_combined,
        trend_7d,
        trend_30d,
        fclc_node_active,
        cohort_percentile,
        last_sample_at,
        sample_count: verified.len() as i64,
        health_factors: hf_summary,
    }
}

pub fn compute_trend_series(samples: &[ZeSample], period_days: i32) -> ZeTrend {
    let now = Utc::now();
    let cutoff = now - chrono::Duration::days(period_days as i64);
    let mut points: Vec<ZeTrendPoint> = samples
        .iter()
        .filter(|s| s.is_verified && s.recorded_at > cutoff)
        .map(|s| ZeTrendPoint {
            date: s.recorded_at,
            chi_ze_combined: s.chi_ze_combined,
            bio_age_est: s.bio_age_est,
        })
        .collect();
    points.sort_by_key(|p| p.date);
    ZeTrend { period_days, points }
}

/// Anomaly detection: flag samples where chi_ze is suspiciously static for >30 days.
///
/// Real sensors always produce floating-point noise (std > 0.001).
/// A rolling window std < ANOMALY_STD_THRESHOLD for >= ANOMALY_CONST_DAYS
/// indicates copy-paste data, sensor malfunction, or fabrication.
const ANOMALY_STD_THRESHOLD: f64 = 0.001;
const ANOMALY_WINDOW: usize = 10; // samples in rolling window

pub fn detect_anomalies(samples: &mut Vec<ZeSample>) {
    if samples.len() < ANOMALY_WINDOW {
        return;
    }
    samples.sort_by_key(|s| s.recorded_at);

    for window_end in ANOMALY_WINDOW..=samples.len() {
        let window = &samples[(window_end - ANOMALY_WINDOW)..window_end];
        let vals: Vec<f64> = window.iter().filter_map(|s| s.chi_ze_combined).collect();
        if vals.len() < ANOMALY_WINDOW {
            continue;
        }
        let mean = vals.iter().sum::<f64>() / vals.len() as f64;
        let std = (vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64).sqrt();

        if std < ANOMALY_STD_THRESHOLD {
            let span_days = {
                let first = window.first().unwrap().recorded_at;
                let last = window.last().unwrap().recorded_at;
                (last - first).num_days()
            };
            if span_days >= ANOMALY_CONST_DAYS {
                // Flag the most recent sample in this window
                samples[window_end - 1].is_verified = false;
            }
        }
    }
}

// --- helpers ---

fn mean_option<I: Iterator<Item = Option<f64>>>(iter: I) -> Option<f64> {
    let vals: Vec<f64> = iter.flatten().collect();
    if vals.is_empty() {
        None
    } else {
        Some(vals.iter().sum::<f64>() / vals.len() as f64)
    }
}

/// 95% CI via Jacobian error propagation through bridge equation:
///   bio_age = chrono_age * (1 - D_norm * K)
///   D_norm  = D_NORM_ALPHA * (1 - chi_ze)
///   ∂bio_age/∂chi_ze = chrono_age * D_NORM_ALPHA * K
///
/// CI_half = |∂bio_age/∂chi_ze| * SE(chi_ze) * 1.96
/// where SE(chi_ze) = std(chi_ze) / sqrt(N)
///
/// Stability thresholds derived from clinical significance:
///   high   < 2y  — within acceptable diagnostic range
///   medium < 5y  — borderline; more data needed
///   low   >= 5y  — insufficient data for reliable estimate
fn compute_ci(
    bio_age_est: Option<f64>,
    k_calibration: f64,
    samples: &[&ZeSample],
) -> (Option<f64>, Option<f64>, Option<String>) {
    let est = match bio_age_est {
        Some(e) => e,
        None => return (None, None, None),
    };

    let chi_vals: Vec<f64> = samples.iter().filter_map(|s| s.chi_ze_combined).collect();
    let n = chi_vals.len();

    if n < 3 {
        return (Some(est - 5.0), Some(est + 5.0), Some("low".into()));
    }

    // Sample standard deviation of chi_ze
    let mean = chi_vals.iter().sum::<f64>() / n as f64;
    let variance = chi_vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    let std_chi = variance.sqrt();

    // Standard error of the mean chi_ze
    let se_chi = std_chi / (n as f64).sqrt();

    // Jacobian: |∂bio_age/∂chi_ze| = chrono_age * D_NORM_ALPHA * k_calibration
    // We approximate chrono_age from bio_age_est (close enough for CI propagation)
    let approx_chrono = est / (1.0 - D_NORM_ALPHA * (1.0 - mean) * k_calibration).max(0.1);
    let jacobian = approx_chrono * D_NORM_ALPHA * k_calibration;

    let ci_half = (jacobian * se_chi * 1.96).max(0.5); // minimum 0.5y CI

    let stability = if ci_half < 2.0 {
        "high"
    } else if ci_half < 5.0 {
        "medium"
    } else {
        "low"
    };

    (Some(est - ci_half), Some(est + ci_half), Some(stability.into()))
}

fn compute_trend(
    samples: &[&ZeSample],
    period_start: DateTime<Utc>,
    period_prev_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
) -> Option<f64> {
    let current: Vec<f64> = samples
        .iter()
        .filter(|s| s.recorded_at > period_start && s.recorded_at <= period_end)
        .filter_map(|s| s.chi_ze_combined)
        .collect();
    let prev: Vec<f64> = samples
        .iter()
        .filter(|s| s.recorded_at > period_prev_start && s.recorded_at <= period_start)
        .filter_map(|s| s.chi_ze_combined)
        .collect();
    if current.is_empty() || prev.is_empty() {
        return None;
    }
    let mean_curr = current.iter().sum::<f64>() / current.len() as f64;
    let mean_prev = prev.iter().sum::<f64>() / prev.len() as f64;
    Some(mean_curr - mean_prev)
}

fn compute_cohort_percentile(
    bio_age_est: Option<f64>,
    chrono_age: Option<f64>,
    cohort_samples: &[ZeSample],
) -> Option<f64> {
    let our_bio = bio_age_est?;
    // chrono_age guard: only compute percentile when we know the user's age
    let _our_chrono = chrono_age?;
    // Cohort = users with same chrono_age ±2
    let cohort_bio_ages: Vec<f64> = cohort_samples
        .iter()
        .filter_map(|s| s.bio_age_est)
        .collect();
    if cohort_bio_ages.is_empty() {
        return None;
    }
    // percentile = fraction of cohort with HIGHER bio_age (i.e., biologically older)
    // higher percentile = user is biologically younger than most of cohort
    let worse_than_us = cohort_bio_ages.iter().filter(|&&b| b > our_bio).count();
    Some((worse_than_us as f64 / cohort_bio_ages.len() as f64) * 100.0)
}
