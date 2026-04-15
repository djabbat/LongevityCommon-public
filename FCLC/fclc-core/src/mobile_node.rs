//! FCLC Mobile / BioSense Node
//!
//! Privacy-preserving ingestion of Ze-index data from BioSense wearable devices.
//!
//! ## Data flow
//! ```text
//! BioSense device (EEG/HRV/VOC)
//!   → computes χ_Ze locally (Rust on nRF52840)
//!   → MobileNodeData { age_exact, chi_ze_eeg, chi_ze_hrv, chi_ze_voc }
//!   → deidentify_mobile()       [Layer 1+2: age → 5-yr bin, round χ_Ze to 2dp]
//!   → k-anonymity check         [Layer 3: suppress groups < k=5]
//!   → add_dp_noise_mobile()     [Layer 4: Laplace ε=2.0 on each χ_Ze]
//!   → MobileNodeRecord          [safe to upload]
//!   → FCLC orchestrator POST /api/biosense/upload
//! ```
//!
//! ## Privacy guarantees (same 5-layer stack as FCLC clinical nodes)
//! - **Layer 1**: Exact age → 5-year bin; no timestamps more precise than week-of-year
//! - **Layer 2**: χ_Ze rounded to 2 decimal places; no raw EEG/HRV signals
//! - **Layer 3**: Groups smaller than k=5 suppressed before upload
//! - **Layer 4**: Laplace noise on each χ_Ze value (ε = 2.0 per field)
//! - **Layer 5**: SecAgg masking at upload time (handled by BioSenseUpload layer)

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Constants ────────────────────────────────────────────────────────────────

/// Ze fixed point v* — matches BioSense constants exactly.
pub const V_STAR: f64 = 0.45631;

/// DP epsilon per field per upload (Laplace mechanism).
pub const DP_EPSILON_MOBILE: f64 = 2.0;

/// Sensitivity of χ_Ze ∈ [0,1]: maximum possible change per record = 1.0.
pub const CHI_ZE_SENSITIVITY: f64 = 1.0;

/// k-anonymity threshold: groups smaller than this are suppressed.
pub const K_ANONYMITY_MIN: usize = 5;

// ── Age binning ───────────────────────────────────────────────────────────────

/// 5-year age bins aligned with OMOP CDM convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgeBin5 {
    Under20,
    Age20_24,
    Age25_29,
    Age30_34,
    Age35_39,
    Age40_44,
    Age45_49,
    Age50_54,
    Age55_59,
    Age60_64,
    Age65_69,
    Age70_74,
    Age75_79,
    Age80Plus,
}

impl AgeBin5 {
    pub fn from_age(age: u8) -> Self {
        match age {
            0..=19   => Self::Under20,
            20..=24  => Self::Age20_24,
            25..=29  => Self::Age25_29,
            30..=34  => Self::Age30_34,
            35..=39  => Self::Age35_39,
            40..=44  => Self::Age40_44,
            45..=49  => Self::Age45_49,
            50..=54  => Self::Age50_54,
            55..=59  => Self::Age55_59,
            60..=64  => Self::Age60_64,
            65..=69  => Self::Age65_69,
            70..=74  => Self::Age70_74,
            75..=79  => Self::Age75_79,
            _        => Self::Age80Plus,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Under20  => "<20",
            Self::Age20_24 => "20-24",
            Self::Age25_29 => "25-29",
            Self::Age30_34 => "30-34",
            Self::Age35_39 => "35-39",
            Self::Age40_44 => "40-44",
            Self::Age45_49 => "45-49",
            Self::Age50_54 => "50-54",
            Self::Age55_59 => "55-59",
            Self::Age60_64 => "60-64",
            Self::Age65_69 => "65-69",
            Self::Age70_74 => "70-74",
            Self::Age75_79 => "75-79",
            Self::Age80Plus=> "80+",
        }
    }
}

// ── Raw input (before privacy) ────────────────────────────────────────────────

/// Raw data from BioSense device — NEVER leaves the local node without deidentification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileNodeData {
    /// Exact age in years — replaced by 5-yr bin during deidentification.
    pub age_exact: u8,
    /// χ_Ze from EEG channel (25–35 Hz band), None if not measured this session.
    pub chi_ze_eeg: Option<f64>,
    /// χ_Ze from HRV/RR channel, None if not measured.
    pub chi_ze_hrv: Option<f64>,
    /// χ_Ze from VOC/olfaction channel, None if not measured.
    pub chi_ze_voc: Option<f64>,
    /// Unix timestamp — replaced by week-of-year during deidentification.
    pub timestamp_unix: u64,
    /// Device UUID — removed during deidentification.
    pub device_id: String,
}

// ── Deidentified record (safe to aggregate) ───────────────────────────────────

/// Privacy-safe record after full 5-layer deidentification.
/// This is what the FCLC node aggregates and uploads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileNodeRecord {
    /// 5-year age bin (never exact age).
    pub age_bin: AgeBin5,
    /// χ_Ze(EEG) rounded to 2 d.p. ∈ [0.0, 1.0], None if not measured.
    pub chi_ze_eeg: Option<f64>,
    /// χ_Ze(HRV) rounded to 2 d.p. ∈ [0.0, 1.0], None if not measured.
    pub chi_ze_hrv: Option<f64>,
    /// χ_Ze(VOC) rounded to 2 d.p. ∈ [0.0, 1.0], None if not measured.
    pub chi_ze_voc: Option<f64>,
    /// ISO week number (1-53) — no day, no exact date.
    pub week_of_year: u8,
}

// ── Deidentification ──────────────────────────────────────────────────────────

/// Layer 1+2: Remove/generalise all direct and quasi-identifiers.
///
/// - device_id and timestamp_unix are dropped.
/// - age_exact → 5-year bin.
/// - χ_Ze values rounded to 2 decimal places.
/// - timestamp → ISO week number only.
pub fn deidentify_mobile(data: &MobileNodeData) -> MobileNodeRecord {
    let age_bin = AgeBin5::from_age(data.age_exact);

    // Compute ISO week number from Unix timestamp (days / 7, mod 53, range 1–53).
    let week_of_year = ((data.timestamp_unix / 86400 / 7) % 53 + 1) as u8;

    MobileNodeRecord {
        age_bin,
        chi_ze_eeg: data.chi_ze_eeg.map(|v| round2(v)),
        chi_ze_hrv: data.chi_ze_hrv.map(|v| round2(v)),
        chi_ze_voc: data.chi_ze_voc.map(|v| round2(v)),
        week_of_year,
    }
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

// ── DP noise ─────────────────────────────────────────────────────────────────

/// Sample from Laplace(0, scale) using the inverse CDF method.
/// Laplace noise is standard for the DP Laplace mechanism.
fn sample_laplace(scale: f64, rng: &mut impl Rng) -> f64 {
    // U ~ Uniform(-0.5, 0.5)
    let u: f64 = rng.gen::<f64>() - 0.5;
    -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln()
}

/// Layer 4: Add Laplace noise to each χ_Ze field.
///
/// Laplace mechanism: noise ~ Lap(0, Δf/ε) where Δf = CHI_ZE_SENSITIVITY = 1.0.
/// Scale parameter b = 1.0 / DP_EPSILON_MOBILE = 0.5.
/// Result clamped to [0.0, 1.0] after noise addition.
pub fn add_dp_noise_mobile(record: &mut MobileNodeRecord, rng: &mut impl Rng) {
    let scale = CHI_ZE_SENSITIVITY / DP_EPSILON_MOBILE; // b = 0.5

    if let Some(v) = record.chi_ze_eeg.as_mut() {
        *v = (*v + sample_laplace(scale, rng)).clamp(0.0, 1.0);
        *v = round2(*v);
    }
    if let Some(v) = record.chi_ze_hrv.as_mut() {
        *v = (*v + sample_laplace(scale, rng)).clamp(0.0, 1.0);
        *v = round2(*v);
    }
    if let Some(v) = record.chi_ze_voc.as_mut() {
        *v = (*v + sample_laplace(scale, rng)).clamp(0.0, 1.0);
        *v = round2(*v);
    }
}

// ── k-anonymity enforcement ───────────────────────────────────────────────────

/// Layer 3: Suppress records from groups with fewer than k records.
///
/// Groups are defined by (age_bin, week_of_year).
/// Returns the filtered records (small groups removed).
pub fn enforce_k_anonymity_mobile(
    records: Vec<MobileNodeRecord>,
    k: usize,
) -> Vec<MobileNodeRecord> {
    // Count per quasi-identifier combination
    let mut counts: HashMap<(u8, u8), usize> = HashMap::new();
    for r in &records {
        let key = (r.age_bin as u8, r.week_of_year);
        *counts.entry(key).or_insert(0) += 1;
    }
    records
        .into_iter()
        .filter(|r| {
            let key = (r.age_bin as u8, r.week_of_year);
            counts.get(&key).copied().unwrap_or(0) >= k
        })
        .collect()
}

// ── Feature vector ────────────────────────────────────────────────────────────

/// Compute feature vector from a deidentified record for federated model training.
///
/// Feature layout (length 5):
///   [0]: age_bin_idx / 13.0  (normalised 0–1)
///   [1]: chi_ze_eeg  (or 0.5 if missing — neutral imputation)
///   [2]: chi_ze_hrv  (or 0.5 if missing)
///   [3]: chi_ze_voc  (or 0.5 if missing)
///   [4]: n_channels  / 3.0   (fraction of channels measured)
pub fn compute_ze_feature_vector(record: &MobileNodeRecord) -> Vec<f64> {
    let age_idx = record.age_bin as u8 as f64 / 13.0;
    let eeg = record.chi_ze_eeg.unwrap_or(0.5);
    let hrv = record.chi_ze_hrv.unwrap_or(0.5);
    let voc = record.chi_ze_voc.unwrap_or(0.5);
    let n_ch = [record.chi_ze_eeg, record.chi_ze_hrv, record.chi_ze_voc]
        .iter().filter(|x| x.is_some()).count() as f64 / 3.0;
    vec![age_idx, eeg, hrv, voc, n_ch]
}

/// Validate χ_Ze value is in [0, 1] and finite.
pub fn is_valid_chi_ze(x: f64) -> bool {
    x.is_finite() && x >= 0.0 && x <= 1.0
}

// ── Full pipeline ─────────────────────────────────────────────────────────────

/// Run the complete 4-layer privacy pipeline on a batch of raw mobile records.
///
/// Returns (safe_records, n_suppressed): deidentified + DP-noised records,
/// and the count of records suppressed by k-anonymity.
pub fn process_mobile_batch(
    raw: Vec<MobileNodeData>,
    rng: &mut impl Rng,
    k: usize,
) -> (Vec<MobileNodeRecord>, usize) {
    // Layer 1+2: deidentify
    let mut records: Vec<MobileNodeRecord> = raw.iter().map(deidentify_mobile).collect();

    // Layer 3: k-anonymity
    let before = records.len();
    records = enforce_k_anonymity_mobile(records, k);
    let suppressed = before - records.len();

    // Layer 4: DP noise
    for r in &mut records {
        add_dp_noise_mobile(r, rng);
    }

    (records, suppressed)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn sample_raw(age: u8, chi: f64) -> MobileNodeData {
        MobileNodeData {
            age_exact:     age,
            chi_ze_eeg:    Some(chi),
            chi_ze_hrv:    Some(chi - 0.05),
            chi_ze_voc:    None,
            timestamp_unix: 1_744_000_000 + age as u64 * 100,
            device_id:     format!("dev_{age}"),
        }
    }

    #[test]
    fn deidentify_drops_exact_age_and_device() {
        let raw = sample_raw(27, 0.85);
        let rec = deidentify_mobile(&raw);
        assert_eq!(rec.age_bin, AgeBin5::Age25_29);
        // No device_id in output
        let json = serde_json::to_string(&rec).unwrap();
        assert!(!json.contains("dev_27"));
        assert!(!json.contains("27"));
    }

    #[test]
    fn chi_ze_rounded_to_2dp() {
        let raw = sample_raw(35, 0.87654);
        let rec = deidentify_mobile(&raw);
        let v = rec.chi_ze_eeg.unwrap();
        assert!((v - 0.88).abs() < 1e-9, "expected 0.88, got {v}");
    }

    #[test]
    fn dp_noise_stays_in_range() {
        let mut rng = StdRng::seed_from_u64(99);
        let raw = sample_raw(45, 0.75);
        let mut rec = deidentify_mobile(&raw);
        for _ in 0..200 {
            add_dp_noise_mobile(&mut rec, &mut rng);
            if let Some(v) = rec.chi_ze_eeg {
                assert!(v >= 0.0 && v <= 1.0, "chi_ze out of range: {v}");
            }
        }
    }

    #[test]
    fn k_anonymity_suppresses_small_groups() {
        // 10 records from age 25-29, 3 from age 70-74
        let mut batch: Vec<MobileNodeData> = (0..10).map(|_| sample_raw(27, 0.86)).collect();
        batch.extend((0..3).map(|_| sample_raw(72, 0.70)));
        let (safe, suppressed) = process_mobile_batch(batch, &mut rand::thread_rng(), 5);
        // age 70-74 group (3 records) should be suppressed
        assert_eq!(suppressed, 3, "expected 3 suppressed, got {suppressed}");
        assert!(safe.iter().all(|r| r.age_bin != AgeBin5::Age70_74));
    }

    #[test]
    fn feature_vector_length_and_bounds() {
        let raw = sample_raw(33, 0.82);
        let rec = deidentify_mobile(&raw);
        let fv = compute_ze_feature_vector(&rec);
        assert_eq!(fv.len(), 5);
        assert!(fv.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }

    #[test]
    fn is_valid_chi_ze_bounds() {
        assert!(is_valid_chi_ze(0.0));
        assert!(is_valid_chi_ze(1.0));
        assert!(is_valid_chi_ze(V_STAR));
        assert!(!is_valid_chi_ze(-0.1));
        assert!(!is_valid_chi_ze(1.01));
        assert!(!is_valid_chi_ze(f64::NAN));
        assert!(!is_valid_chi_ze(f64::INFINITY));
    }
}
