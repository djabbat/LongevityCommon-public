use anyhow::{Context, Result};
use fclc_core::schema::{AgeGroup, OmopRecord, Sex};
use serde::Deserialize;
use std::path::Path;

/// Raw CSV row format expected from the HIS export.
#[derive(Debug, Deserialize)]
struct CsvPatientRow {
    age: u8,
    sex: String,
    diabetes_diagnosis_year: Option<u16>,
    hba1c_last: Option<f32>,
    bmi: Option<f32>,
    has_nephropathy: Option<bool>,
    has_retinopathy: Option<bool>,
    hospitalized_last_12m: Option<bool>,
    hospitalized_next_12m: Option<bool>,
}

fn parse_sex(s: &str) -> Sex {
    match s.trim().to_lowercase().as_str() {
        "m" | "male" | "1" => Sex::Male,
        "f" | "female" | "2" => Sex::Female,
        _ => Sex::Unknown,
    }
}

/// Load patient records from a CSV file exported by the HIS.
pub fn load_csv(path: &Path) -> Result<Vec<OmopRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(path)
        .with_context(|| format!("Cannot open CSV: {}", path.display()))?;

    let mut records = Vec::new();

    for result in rdr.deserialize::<CsvPatientRow>() {
        let row = result.with_context(|| "CSV parse error")?;

        // Range validation — skip physiologically impossible values
        if row.age > 120 {
            continue;
        }
        if let Some(hba1c) = row.hba1c_last {
            if !(3.0..=20.0).contains(&hba1c) { continue; }
        }
        if let Some(bmi) = row.bmi {
            if !(10.0..=70.0).contains(&bmi) { continue; }
        }

        let record = OmopRecord {
            age_group: AgeGroup::from_age(row.age),
            sex: parse_sex(&row.sex),
            diabetes_diagnosis_year: row.diabetes_diagnosis_year,
            hba1c_last: row.hba1c_last,
            bmi: row.bmi,
            has_nephropathy: row.has_nephropathy.unwrap_or(false),
            has_retinopathy: row.has_retinopathy.unwrap_or(false),
            hospitalized_last_12m: row.hospitalized_last_12m.unwrap_or(false),
            hospitalized_next_12m: row.hospitalized_next_12m.unwrap_or(false),
        };
        records.push(record);
    }

    Ok(records)
}

/// Minimal FHIR Patient resource fields we care about.
#[derive(Debug, Deserialize)]
struct FhirPatient {
    #[serde(rename = "birthDate")]
    birth_date: Option<String>,
    gender: Option<String>,
}

/// Parse a FHIR R4 Bundle JSON containing Patient and Observation resources.
/// Returns a vector of OmopRecord (best-effort, missing fields → None/false).
///
/// Supports LOINC codes:
///   4548-4  — HbA1c (%)
///   39156-5 — BMI (kg/m²)
///   85354-9 — Blood pressure panel (ignored, reserved for future use)
pub fn load_fhir_json(json_str: &str) -> Result<Vec<OmopRecord>> {
    let v: serde_json::Value =
        serde_json::from_str(json_str).context("Invalid FHIR JSON")?;

    let entries = v["entry"].as_array().cloned().unwrap_or_default();

    // First pass: index Patient resources by id
    use std::collections::HashMap;
    let mut patients: HashMap<String, (AgeGroup, Sex)> = HashMap::new();
    let mut patient_hba1c: HashMap<String, f32> = HashMap::new();
    let mut patient_bmi: HashMap<String, f32> = HashMap::new();

    let current_year = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() / 31_557_600 + 1970) as u16;

    for entry in &entries {
        let resource = &entry["resource"];
        match resource["resourceType"].as_str().unwrap_or("") {
            "Patient" => {
                let id = resource["id"].as_str().unwrap_or("").to_string();
                let sex = resource["gender"].as_str().map(parse_sex).unwrap_or(Sex::Unknown);
                let age_group = resource["birthDate"]
                    .as_str()
                    .and_then(|d| d.split('-').next())
                    .and_then(|y| y.parse::<u16>().ok())
                    .map(|birth_year| {
                        let age = (current_year.saturating_sub(birth_year)) as u8;
                        AgeGroup::from_age(age)
                    })
                    .unwrap_or(AgeGroup::Age50to54);
                patients.insert(id, (age_group, sex));
            }
            "Observation" => {
                // Extract LOINC code from coding array
                let loinc = resource["code"]["coding"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|c| {
                        c["system"].as_str().unwrap_or("").contains("loinc")
                    }))
                    .and_then(|c| c["code"].as_str())
                    .unwrap_or("");

                // Patient reference: "Patient/abc-123" → "abc-123"
                let patient_id = resource["subject"]["reference"]
                    .as_str()
                    .and_then(|r| r.strip_prefix("Patient/"))
                    .unwrap_or("")
                    .to_string();

                // Value as quantity
                let value = resource["valueQuantity"]["value"].as_f64().map(|v| v as f32);

                match (loinc, value) {
                    ("4548-4", Some(v)) if (3.0..=20.0).contains(&v) => {
                        patient_hba1c.insert(patient_id, v);
                    }
                    ("39156-5", Some(v)) if (10.0..=70.0).contains(&v) => {
                        patient_bmi.insert(patient_id, v);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Second pass: build OmopRecord for each patient
    let records = patients
        .into_iter()
        .map(|(id, (age_group, sex))| OmopRecord {
            age_group,
            sex,
            diabetes_diagnosis_year: None,
            hba1c_last: patient_hba1c.get(&id).copied(),
            bmi: patient_bmi.get(&id).copied(),
            has_nephropathy: false,
            has_retinopathy: false,
            hospitalized_last_12m: false,
            hospitalized_next_12m: false,
        })
        .collect();

    Ok(records)
}

/// Generate sample records for demo/testing purposes.
pub fn generate_demo_records(count: usize) -> Vec<OmopRecord> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| {
            let age: u8 = rng.gen_range(30..80);
            OmopRecord {
                age_group: AgeGroup::from_age(age),
                sex: if rng.gen_bool(0.5) { Sex::Male } else { Sex::Female },
                diabetes_diagnosis_year: Some(rng.gen_range(2000u16..2023)),
                hba1c_last: Some(rng.gen_range(5.0f32..12.0)),
                bmi: Some(rng.gen_range(18.0f32..40.0)),
                has_nephropathy: rng.gen_bool(0.1),
                has_retinopathy: rng.gen_bool(0.1),
                hospitalized_last_12m: rng.gen_bool(0.15),
                hospitalized_next_12m: rng.gen_bool(0.12),
            }
        })
        .collect()
}
