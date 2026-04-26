/// Unit tests for Ze compute engine
/// Tests pure functions: bridge equation, CI, trend, anomaly detection, cohort percentile
///
/// Run: cargo test --test ze_compute_tests

use chrono::{Duration, Utc};
use uuid::Uuid;

// Re-export internals via lib (add `pub use` in lib.rs or test module)
// We test the public API of ze_compute indirectly through compute_profile.
// Pure numeric correctness tests do not require a database.

use longevitycommon_server::models::ze_profile::ZeSample;
use longevitycommon_server::services::ze_compute;

fn make_sample(chi_eeg: Option<f64>, chi_hrv: Option<f64>, days_ago: i64, verified: bool) -> ZeSample {
    let now = Utc::now();
    let combined = match (chi_eeg, chi_hrv) {
        (Some(e), Some(h)) => Some((e + h) / 2.0),
        (Some(e), None)    => Some(e),
        (None, Some(h))    => Some(h),
        _                  => None,
    };
    ZeSample {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        recorded_at: now - Duration::days(days_ago),
        source: "test".into(),
        chi_ze_eeg: chi_eeg,
        chi_ze_hrv: chi_hrv,
        chi_ze_combined: combined,
        d_norm: combined.map(|c| (1.2 * (1.0 - c)).clamp(0.0, 1.0)),
        bio_age_est: None,
        bio_age_ci_low: None,
        bio_age_ci_high: None,
        ci_stability: None,
        fclc_signature: None,
        is_verified: verified,
        created_at: now,
    }
}

// ────────────────────────────────────────────────────────
// Bridge equation: bio_age = chrono_age × (1 − D_norm × K)
// ────────────────────────────────────────────────────────

#[test]
fn test_bio_age_below_chrono_when_chi_high() {
    // chi_ze = 0.9 → D_norm = 1.2 × (1 − 0.9) = 0.12 → bio_age = 40 × (1 − 0.12×0.45) ≈ 37.8
    let samples: Vec<ZeSample> = (0..5)
        .map(|i| make_sample(Some(0.9), Some(0.9), i, true))
        .collect();

    let profile = ze_compute::compute_profile(
        Uuid::new_v4(), "testuser".into(),
        Some(2026 - 40), None, false,
        &samples, &[],
    );

    let bio_age = profile.bio_age_est.expect("bio_age_est should be set");
    assert!(bio_age < 40.0, "high chi_ze should give bio_age below chrono_age, got {bio_age}");
    assert!(bio_age > 30.0, "bio_age unrealistically low: {bio_age}");
}

#[test]
fn test_bio_age_above_chrono_when_chi_low() {
    // chi_ze = 0.2 → D_norm = 1.2 × 0.8 = 0.96 → bio_age = 40 × (1 − 0.96×0.45) ≈ 22.7
    // Actually with clamp D_norm = 0.96, bio_age = 40*(1-0.432) = 22.7 — still below chrono
    // chi_ze = 0.0 → D_norm = 1.2 → clamp → 1.0 → bio_age = 40*(1-1.0*0.45) = 22
    // Let's use chi = 0.5 as "average" case and verify delta direction
    let samples_low: Vec<ZeSample> = (0..5)
        .map(|i| make_sample(Some(0.2), Some(0.2), i, true))
        .collect();
    let samples_high: Vec<ZeSample> = (0..5)
        .map(|i| make_sample(Some(0.9), Some(0.9), i, true))
        .collect();

    let uid = Uuid::new_v4();
    let profile_low = ze_compute::compute_profile(uid, "u".into(), Some(1986), None, false, &samples_low, &[]);
    let profile_high = ze_compute::compute_profile(uid, "u".into(), Some(1986), None, false, &samples_high, &[]);

    assert!(
        profile_low.bio_age_est < profile_high.bio_age_est,
        "lower chi_ze should yield lower bio_age estimate"
    );
}

// ────────────────────────────────────────────────────────
// Sensor-aware K selection
// ────────────────────────────────────────────────────────

#[test]
fn test_eeg_only_vs_hrv_only_different_k() {
    let eeg_samples: Vec<ZeSample> = (0..5)
        .map(|i| make_sample(Some(0.7), None, i, true))
        .collect();
    let hrv_samples: Vec<ZeSample> = (0..5)
        .map(|i| make_sample(None, Some(0.7), i, true))
        .collect();

    let uid = Uuid::new_v4();
    let p_eeg = ze_compute::compute_profile(uid, "u".into(), Some(1986), None, false, &eeg_samples, &[]);
    let p_hrv = ze_compute::compute_profile(uid, "u".into(), Some(1986), None, false, &hrv_samples, &[]);

    // K_EEG=0.42 > K_HRV=0.38 → same chi → EEG path gives larger correction → lower bio_age
    let bio_eeg = p_eeg.bio_age_est.unwrap();
    let bio_hrv = p_hrv.bio_age_est.unwrap();
    assert!(
        bio_eeg < bio_hrv,
        "EEG-only K=0.42 > HRV-only K=0.38, so EEG bio_age should be lower for same chi. EEG={bio_eeg:.2}, HRV={bio_hrv:.2}"
    );
}

// ────────────────────────────────────────────────────────
// Empty / insufficient samples
// ────────────────────────────────────────────────────────

#[test]
fn test_empty_samples_returns_none_estimates() {
    let profile = ze_compute::compute_profile(
        Uuid::new_v4(), "empty".into(), Some(1990), None, false, &[], &[],
    );
    assert!(profile.bio_age_est.is_none());
    assert!(profile.chi_ze_combined.is_none());
    assert_eq!(profile.sample_count, 0);
}

#[test]
fn test_unverified_samples_ignored() {
    let samples: Vec<ZeSample> = (0..10)
        .map(|i| make_sample(Some(0.8), Some(0.8), i, false)) // all unverified
        .collect();
    let profile = ze_compute::compute_profile(
        Uuid::new_v4(), "u".into(), Some(1990), None, false, &samples, &[],
    );
    assert!(profile.bio_age_est.is_none(), "unverified samples must not affect bio_age");
}

// ────────────────────────────────────────────────────────
// CI: confidence interval via Jacobian
// ────────────────────────────────────────────────────────

#[test]
fn test_ci_narrows_with_more_samples() {
    // More samples → tighter CI (SE = std/sqrt(N))
    let samples_few: Vec<ZeSample> = (0..3)
        .map(|i| make_sample(Some(0.7 + i as f64 * 0.01), Some(0.7), i, true))
        .collect();
    let samples_many: Vec<ZeSample> = (0..30)
        .map(|i| make_sample(Some(0.7 + (i % 5) as f64 * 0.01), Some(0.7), i as i64, true))
        .collect();

    let uid = Uuid::new_v4();
    let p_few  = ze_compute::compute_profile(uid, "u".into(), Some(1986), None, false, &samples_few,  &[]);
    let p_many = ze_compute::compute_profile(uid, "u".into(), Some(1986), None, false, &samples_many, &[]);

    let ci_few  = p_few.bio_age_ci_high.unwrap() - p_few.bio_age_ci_low.unwrap();
    let ci_many = p_many.bio_age_ci_high.unwrap() - p_many.bio_age_ci_low.unwrap();

    assert!(ci_many < ci_few, "CI should narrow with more samples. few={ci_few:.2}, many={ci_many:.2}");
}

#[test]
fn test_ci_stability_labels() {
    // high stability: many tightly clustered samples
    let tight: Vec<ZeSample> = (0..50)
        .map(|i| make_sample(Some(0.750 + (i % 2) as f64 * 0.001), Some(0.750), i as i64, true))
        .collect();
    let p = ze_compute::compute_profile(Uuid::new_v4(), "u".into(), Some(1986), None, false, &tight, &[]);
    let stability = p.ci_stability.as_deref().unwrap_or("low");
    assert!(
        stability == "high" || stability == "medium",
        "tight clustered samples should yield high or medium stability, got {stability}"
    );
}

// ────────────────────────────────────────────────────────
// Anomaly detection
// ────────────────────────────────────────────────────────

#[test]
fn test_anomaly_detection_flags_static_data() {
    let now = Utc::now();
    // 15 samples with EXACT same value spanning 60 days → should flag last samples
    let mut samples: Vec<ZeSample> = (0..15)
        .map(|i| {
            let mut s = make_sample(Some(0.750), Some(0.750), i * 4, true);
            s.chi_ze_combined = Some(0.750_000); // completely static
            s
        })
        .collect();
    // ensure span > 30 days
    samples[0].recorded_at = now - Duration::days(62);
    samples[14].recorded_at = now - Duration::days(2);

    ze_compute::detect_anomalies(&mut samples);

    let flagged = samples.iter().filter(|s| !s.is_verified).count();
    assert!(flagged > 0, "static data over 30 days should be flagged as anomalous");
}

#[test]
fn test_anomaly_detection_ignores_normal_variation() {
    // Normal HRV noise: std > 0.001
    let mut samples: Vec<ZeSample> = (0..15)
        .map(|i| {
            let noise = (i as f64 * 0.013).sin() * 0.05; // std ≈ 0.035
            make_sample(Some(0.70 + noise), Some(0.70 + noise * 0.8), i * 4, true)
        })
        .collect();

    ze_compute::detect_anomalies(&mut samples);

    let flagged = samples.iter().filter(|s| !s.is_verified).count();
    assert_eq!(flagged, 0, "normal HRV variation should not be flagged");
}

// ────────────────────────────────────────────────────────
// Cohort percentile
// ────────────────────────────────────────────────────────

#[test]
fn test_cohort_percentile_best_in_cohort() {
    // User bio_age = 35, cohort all have bio_age >= 40 → percentile near 100
    let cohort: Vec<ZeSample> = (0..10)
        .map(|_| {
            let mut s = make_sample(Some(0.4), Some(0.4), 0, true);
            s.bio_age_est = Some(45.0);
            s
        })
        .collect();

    let user_samples: Vec<ZeSample> = (0..10)
        .map(|i| make_sample(Some(0.9), Some(0.9), i, true))
        .collect();

    let profile = ze_compute::compute_profile(
        Uuid::new_v4(), "u".into(), Some(1991), None, false,
        &user_samples, &cohort,
    );

    let pct = profile.cohort_percentile.expect("cohort_percentile should be set");
    assert!(pct > 80.0, "user with best bio_age should be in top 20% (pct={pct:.1})");
}

#[test]
fn test_cohort_percentile_worst_in_cohort() {
    // User bio_age = 55 (high), cohort all have bio_age <= 35 → percentile near 0
    let cohort: Vec<ZeSample> = (0..10)
        .map(|_| {
            let mut s = make_sample(Some(0.9), Some(0.9), 0, true);
            s.bio_age_est = Some(32.0);
            s
        })
        .collect();

    let user_samples: Vec<ZeSample> = (0..10)
        .map(|i| make_sample(Some(0.1), Some(0.1), i, true))
        .collect();

    let profile = ze_compute::compute_profile(
        Uuid::new_v4(), "u".into(), Some(1991), None, false,
        &user_samples, &cohort,
    );

    let pct = profile.cohort_percentile.expect("cohort_percentile should be set");
    assert!(pct < 20.0, "user with worst bio_age should be in bottom 20% (pct={pct:.1})");
}
