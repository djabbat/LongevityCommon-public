use std::collections::HashMap;

/// Five-year age groups following OMOP CDM convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AgeGroup {
    Age0to4,
    Age5to9,
    Age10to14,
    Age15to19,
    Age20to24,
    Age25to29,
    Age30to34,
    Age35to39,
    Age40to44,
    Age45to49,
    Age50to54,
    Age55to59,
    Age60to64,
    Age65to69,
    Age70to74,
    Age75to79,
    Age80to84,
    Age85plus,
}

impl AgeGroup {
    /// Convert raw age (years) to the 5-year band.
    pub fn from_age(age: u8) -> Self {
        match age {
            0..=4 => Self::Age0to4,
            5..=9 => Self::Age5to9,
            10..=14 => Self::Age10to14,
            15..=19 => Self::Age15to19,
            20..=24 => Self::Age20to24,
            25..=29 => Self::Age25to29,
            30..=34 => Self::Age30to34,
            35..=39 => Self::Age35to39,
            40..=44 => Self::Age40to44,
            45..=49 => Self::Age45to49,
            50..=54 => Self::Age50to54,
            55..=59 => Self::Age55to59,
            60..=64 => Self::Age60to64,
            65..=69 => Self::Age65to69,
            70..=74 => Self::Age70to74,
            75..=79 => Self::Age75to79,
            80..=84 => Self::Age80to84,
            _ => Self::Age85plus,
        }
    }

    /// Midpoint age for feature encoding.
    pub fn midpoint(&self) -> f32 {
        match self {
            Self::Age0to4 => 2.0,
            Self::Age5to9 => 7.0,
            Self::Age10to14 => 12.0,
            Self::Age15to19 => 17.0,
            Self::Age20to24 => 22.0,
            Self::Age25to29 => 27.0,
            Self::Age30to34 => 32.0,
            Self::Age35to39 => 37.0,
            Self::Age40to44 => 42.0,
            Self::Age45to49 => 47.0,
            Self::Age50to54 => 52.0,
            Self::Age55to59 => 57.0,
            Self::Age60to64 => 62.0,
            Self::Age65to69 => 67.0,
            Self::Age70to74 => 72.0,
            Self::Age75to79 => 77.0,
            Self::Age80to84 => 82.0,
            Self::Age85plus => 90.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

/// A single patient record normalised to OMOP CDM conventions.
/// All direct identifiers have been removed; only derived/grouped values remain.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OmopRecord {
    /// 5-year age band (no exact DOB stored)
    pub age_group: AgeGroup,
    pub sex: Sex,
    /// Year of first diabetes diagnosis (not exact date)
    pub diabetes_diagnosis_year: Option<u16>,
    /// Most recent HbA1c (%)
    pub hba1c_last: Option<f32>,
    /// Body Mass Index
    pub bmi: Option<f32>,
    pub has_nephropathy: bool,
    pub has_retinopathy: bool,
    pub hospitalized_last_12m: bool,
    /// Prediction target: was patient hospitalised in the next 12 months?
    pub hospitalized_next_12m: bool,
}

impl OmopRecord {
    /// Encode record as a fixed-length feature vector (for ML).
    /// Returns a Vec<f32> ready for gradient computation.
    pub fn to_features(&self) -> Vec<f32> {
        let age = self.age_group.midpoint() / 90.0; // normalise to [0,1]
        let sex = match self.sex {
            Sex::Male => 1.0,
            Sex::Female => 0.0,
            Sex::Unknown => 0.5,
        };
        let diag_year = self
            .diabetes_diagnosis_year
            .map(|y| ((y as f32 - 1960.0) / 65.0).clamp(0.0, 1.0))
            .unwrap_or(0.0);
        let hba1c = self.hba1c_last.map(|h| ((h - 4.0) / 10.0).clamp(0.0, 1.0)).unwrap_or(0.0);
        let bmi = self.bmi.map(|b| ((b - 15.0) / 45.0).clamp(0.0, 1.0)).unwrap_or(0.0);

        vec![
            age,
            sex,
            diag_year,
            hba1c,
            bmi,
            if self.has_nephropathy { 1.0 } else { 0.0 },
            if self.has_retinopathy { 1.0 } else { 0.0 },
            if self.hospitalized_last_12m { 1.0 } else { 0.0 },
        ]
    }

    /// Number of features produced by `to_features`.
    pub const FEATURE_DIM: usize = 8;
}

/// Apply k-anonymity: suppress age_group/sex combos with fewer than
/// `suppress_rare_threshold` records. Rare records are replaced with
/// generalised values (Unknown sex, midpoint age group).
pub fn anonymize_record(r: &mut OmopRecord) {
    // Generalise exact diagnosis year to decade
    if let Some(year) = r.diabetes_diagnosis_year {
        r.diabetes_diagnosis_year = Some((year / 10) * 10);
    }
    // Round HbA1c to 1 decimal
    if let Some(h) = r.hba1c_last {
        r.hba1c_last = Some((h * 10.0).round() / 10.0);
    }
    // Round BMI to integer
    if let Some(b) = r.bmi {
        r.bmi = Some(b.round());
    }
}

/// Count records per (AgeGroup, Sex) quasi-identifier pair.
fn count_qi(records: &[OmopRecord]) -> HashMap<(AgeGroup, Sex), usize> {
    let mut counts: HashMap<(AgeGroup, Sex), usize> = HashMap::new();
    for r in records {
        *counts.entry((r.age_group, r.sex)).or_insert(0) += 1;
    }
    counts
}

/// Suppress rare quasi-identifier combinations: for records whose
/// (age_group, sex) pair appears fewer than `threshold` times,
/// generalise sex to Unknown and shift age_group to Age85plus.
pub fn suppress_rare_records(records: &mut Vec<OmopRecord>, threshold: usize) {
    let counts = count_qi(records);
    for r in records.iter_mut() {
        let count = counts.get(&(r.age_group, r.sex)).copied().unwrap_or(0);
        if count < threshold {
            r.sex = Sex::Unknown;
            // Generalise: round age group to nearest decade midpoint
            r.age_group = match r.age_group {
                AgeGroup::Age0to4 | AgeGroup::Age5to9 => AgeGroup::Age0to4,
                AgeGroup::Age10to14 | AgeGroup::Age15to19 => AgeGroup::Age10to14,
                AgeGroup::Age20to24 | AgeGroup::Age25to29 => AgeGroup::Age20to24,
                AgeGroup::Age30to34 | AgeGroup::Age35to39 => AgeGroup::Age30to34,
                AgeGroup::Age40to44 | AgeGroup::Age45to49 => AgeGroup::Age40to44,
                AgeGroup::Age50to54 | AgeGroup::Age55to59 => AgeGroup::Age50to54,
                AgeGroup::Age60to64 | AgeGroup::Age65to69 => AgeGroup::Age60to64,
                AgeGroup::Age70to74 | AgeGroup::Age75to79 => AgeGroup::Age70to74,
                _ => AgeGroup::Age80to84,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record() -> OmopRecord {
        OmopRecord {
            age_group: AgeGroup::from_age(52),
            sex: Sex::Male,
            diabetes_diagnosis_year: Some(2015),
            hba1c_last: Some(7.234),
            bmi: Some(28.6),
            has_nephropathy: false,
            has_retinopathy: true,
            hospitalized_last_12m: false,
            hospitalized_next_12m: false,
        }
    }

    #[test]
    fn test_age_group() {
        assert_eq!(AgeGroup::from_age(0), AgeGroup::Age0to4);
        assert_eq!(AgeGroup::from_age(52), AgeGroup::Age50to54);
        assert_eq!(AgeGroup::from_age(90), AgeGroup::Age85plus);
    }

    #[test]
    fn test_to_features_length() {
        let r = make_record();
        assert_eq!(r.to_features().len(), OmopRecord::FEATURE_DIM);
    }

    #[test]
    fn test_anonymize() {
        let mut r = make_record();
        anonymize_record(&mut r);
        // Year should be rounded to decade
        assert_eq!(r.diabetes_diagnosis_year, Some(2010));
        // HbA1c rounded to 1 decimal
        assert!((r.hba1c_last.unwrap() - 7.2).abs() < 0.01);
        // BMI rounded to integer
        assert!((r.bmi.unwrap() - 29.0).abs() < 0.01);
    }
}
