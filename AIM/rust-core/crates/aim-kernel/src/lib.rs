//! aim-kernel — decision laws + Ze-formula scoring.
//!
//! Port of `agents/kernel.py`. Covers:
//!   • Three Laws + Zeroth Law (L0–L3)
//!   • Extended laws (L_PRIVACY, L_CONSENT, L_VERIFIABILITY)
//!   • Impedance / 𝒞 / Φ_Ze / ethics scoring (deterministic checklist core)
//!
//! Skipped (deferred to higher-level crates with their own dependencies):
//!   • LLM-delta impedance (needs `aim-llm`)
//!   • DB audit logging (needs `aim-cost-ledger` style sink)
//!   • decide() / format_compact / format_verbose (CLI / locale concerns)

use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("kernel violation: {0}")]
    Violation(String),
}

pub type Result<T> = std::result::Result<T, KernelError>;

const EPS: f64 = 1e-6;

// ── data ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub description: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub meta: serde_json::Value,
}

impl Decision {
    pub fn new(id: impl Into<String>, action_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: String::new(),
            action_type: action_type.into(),
            payload: serde_json::json!({}),
            meta: serde_json::json!({}),
        }
    }

    pub fn payload_str(&self, key: &str) -> &str {
        self.payload.get(key).and_then(|v| v.as_str()).unwrap_or("")
    }

    pub fn payload_bool(&self, key: &str) -> bool {
        self.payload.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
    }

    pub fn payload_array<'a>(&'a self, key: &str) -> &'a [serde_json::Value] {
        self.payload
            .get(key)
            .and_then(|v| v.as_array())
            .map(|a| a.as_slice())
            .unwrap_or(&[])
    }

    fn payload_blob_lower(&self) -> String {
        serde_json::to_string(&self.payload)
            .unwrap_or_default()
            .to_lowercase()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Patient {
    pub allergies: Vec<String>,
    pub medications: Vec<Medication>,
    pub red_flags: Vec<String>,
    pub missing_labs_count: i32,
    pub history_contradictions: i32,
    pub unexplained_symptoms_count: i32,
    pub last_visit_years_ago: i32,
    pub dx_without_evidence: bool,
    /// `primary_complaint_undiagnosed` defaults to `true` per Python; use
    /// `Patient::with_diagnosed_complaint()` to override.
    pub primary_complaint_undiagnosed: bool,
    pub has_confirmed_dx: bool,
    pub refusal_noted: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Medication {
    pub name: String,
}

impl Patient {
    pub fn new() -> Self {
        Self {
            primary_complaint_undiagnosed: true,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Context {
    pub commanded_action_type: Option<String>,
    pub privacy_consent: bool,
    pub user_confirmed: Option<bool>,
    pub impedance_before: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LawsResult {
    pub l0: bool,
    pub l1: bool,
    pub l2: bool,
    pub l3: bool,
    pub reasons: Vec<String>,
}

impl LawsResult {
    pub fn passed(&self) -> bool {
        self.l0 && self.l1 && self.l2 && self.l3
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExtendedLawsResult {
    pub privacy: bool,
    pub consent: bool,
    pub verifiability: bool,
    pub reasons: Vec<String>,
}

impl ExtendedLawsResult {
    pub fn passed(&self) -> bool {
        self.privacy && self.consent && self.verifiability
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoringResult {
    pub impedance_before: f64,
    pub impedance_after: f64,
    pub instant_c: f64,
    pub phi_ze: f64,
    pub ethics_ze_learn_cheat: f64,
    pub ethics_autonomy: f64,
    pub ethics_beneficence: f64,
    pub ethics_nonmaleficence: f64,
    pub ethics_justice: f64,
    pub ethics_composite: f64,
    pub utility: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KernelWeights {
    pub alpha: f64,        // weight on 𝒞
    pub beta: f64,         // weight on Φ_Ze
    pub gamma: f64,        // weight on Ethics composite
    pub ethics_ze: f64,
    pub ethics_auto: f64,
    pub ethics_benef: f64,
    pub ethics_nonmal: f64,
    pub ethics_justice: f64,
}

impl Default for KernelWeights {
    fn default() -> Self {
        Self {
            alpha: 0.40,
            beta: 0.30,
            gamma: 0.30,
            ethics_ze: 0.40,
            ethics_auto: 0.15,
            ethics_benef: 0.15,
            ethics_nonmal: 0.15,
            ethics_justice: 0.15,
        }
    }
}

// ── L0 ──────────────────────────────────────────────────────────────────────

const DANGER_SIGNALS: &[&str] = &[
    "synthesize_biohazard",
    "make_explosive",
    "weapon_design",
    "forge_documents",
    "bypass_safety_system",
];

const BROAD_ABX: &[&str] = &["vancomycin", "meropenem", "piperacillin", "linezolid"];
const VIRAL_INDICATIONS: &[&str] = &["viral", "orvi", "uri", "common cold", "орви", "простуда"];

pub fn evaluate_l0(decision: &Decision) -> (bool, String) {
    let payload_lower = decision.payload_blob_lower();
    let desc_lower = decision.description.to_lowercase();
    for sig in DANGER_SIGNALS {
        let sig_space = sig.replace('_', " ");
        if payload_lower.contains(sig)
            || desc_lower.contains(sig)
            || desc_lower.contains(&sig_space)
            || payload_lower.contains(&sig_space)
        {
            return (false, format!("L0 block: danger signal '{}'", sig));
        }
    }
    if decision.action_type == "treatment" {
        let drug = decision.payload_str("drug").to_lowercase();
        let indication = decision.payload_str("indication").to_lowercase();
        let is_broad = BROAD_ABX.iter().any(|b| drug.contains(b));
        let is_viral = VIRAL_INDICATIONS.iter().any(|v| indication.contains(v));
        if is_broad && is_viral {
            return (
                false,
                "L0 risk: broad-spectrum ABx for likely viral — resistance pressure".into(),
            );
        }
    }
    (true, "L0 ok".into())
}

// ── L1 ──────────────────────────────────────────────────────────────────────

pub fn evaluate_l1(decision: &Decision, patient: &Patient, context: &Context) -> (bool, String) {
    let allergies: Vec<String> = patient.allergies.iter().map(|a| a.to_lowercase()).collect();
    if decision.action_type == "treatment" {
        let drug = decision.payload_str("drug").to_lowercase();
        for allergy in &allergies {
            if (allergy.contains("penicillin") || allergy.contains("пеницил"))
                && ["amoxi", "ampi", "penici", "пеницил"]
                    .iter()
                    .any(|k| drug.contains(k))
            {
                return (
                    false,
                    format!("L1 block: {} в семействе аллергии '{}'", drug, allergy),
                );
            }
            let first_word = allergy.split_whitespace().next().unwrap_or("");
            if !first_word.is_empty() && drug.contains(first_word) {
                return (
                    false,
                    format!("L1 block: {} совпадает с allergy '{}'", drug, allergy),
                );
            }
        }
        for intx in decision.payload_array("interactions") {
            let sev = intx.get("severity").and_then(|v| v.as_str()).unwrap_or("");
            if sev == "major" || sev == "contraindicated" {
                let summary = intx.get("summary").and_then(|v| v.as_str()).unwrap_or("");
                return (false, format!("L1 block: interaction '{}'", summary));
            }
        }
    }
    if decision.action_type == "wait" {
        let impedance = context.impedance_before.unwrap_or(0.5);
        if impedance > 0.85 || !patient.red_flags.is_empty() {
            return (
                false,
                format!(
                    "L1 inaction harm: impedance={:.2}, red_flags={:?}",
                    impedance, patient.red_flags
                ),
            );
        }
    }
    (true, "L1 ok".into())
}

// ── L2 ──────────────────────────────────────────────────────────────────────

pub fn evaluate_l2(decision: &Decision, context: &Context) -> (bool, String) {
    if let Some(commanded) = &context.commanded_action_type {
        if decision.action_type != *commanded {
            return (
                false,
                format!(
                    "L2: врач указал action_type='{}', decision='{}'",
                    commanded, decision.action_type
                ),
            );
        }
    }
    (true, "L2 ok".into())
}

// ── L3 ──────────────────────────────────────────────────────────────────────

pub fn evaluate_l3(decision: &Decision) -> (bool, String) {
    if decision.action_type == "system_modification" && decision.payload_bool("destructive") {
        return (false, "L3: destructive system-modification without explicit override".into());
    }
    (true, "L3 ok".into())
}

pub fn evaluate_laws(decision: &Decision, patient: &Patient, context: &Context) -> LawsResult {
    let (ok0, r0) = evaluate_l0(decision);
    let (ok1, r1) = evaluate_l1(decision, patient, context);
    let (ok2, r2) = evaluate_l2(decision, context);
    let (ok3, r3) = evaluate_l3(decision);
    LawsResult {
        l0: ok0,
        l1: ok1,
        l2: ok2,
        l3: ok3,
        reasons: vec![r0, r1, r2, r3],
    }
}

// ── extended laws ───────────────────────────────────────────────────────────

const PRIVACY_ACTIONS: &[&str] = &[
    "email_send",
    "web_post",
    "git_push_public",
    "upload_external",
    "telegram_broadcast",
    "external_api_call_with_data",
];

const CONSENT_ACTIONS: &[&str] = &[
    "email_send",
    "git_push_public",
    "telegram_broadcast",
    "slack_post",
    "web_publish",
    "submit_form",
    "delete_persistent",
    "irreversible_external",
];

const VERIFIABILITY_ACTIONS: &[&str] = &[
    "emit_text",
    "write_manuscript",
    "send_letter",
    "generate_citations",
    "peer_review_emit",
    "grant_letter",
];

static PHONE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\+?\d[\d\s().\-]{8,}\d").expect("phone regex"));
static BIRTHDATE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(19|20)\d{2}[-_/](0?[1-9]|1[0-2])[-_/](0?[1-9]|[12]\d|3[01])\b").expect("dob regex"));
static MRN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\b(?:passport|mrn|медкарт)[:#\s]+\S+").expect("mrn regex"));

pub fn evaluate_l_privacy(decision: &Decision, context: &Context) -> (bool, String) {
    if !PRIVACY_ACTIONS.contains(&decision.action_type.as_str()) {
        return (true, "L_PRIVACY n/a".into());
    }
    let blob = decision.payload_blob_lower();
    let mut flags: Vec<&str> = Vec::new();
    if blob.contains("patients/") || blob.contains("/patients/") {
        flags.push("Patients/ path in payload");
    }
    if PHONE_RE.is_match(&blob) {
        flags.push("phone-like number in payload");
    }
    if BIRTHDATE_RE.is_match(&blob) {
        flags.push("birthdate-like pattern in payload");
    }
    if MRN_RE.is_match(&blob) {
        flags.push("medical record / passport identifier");
    }
    if !flags.is_empty() && !context.privacy_consent {
        return (
            false,
            format!(
                "L_PRIVACY: {} (require privacy_consent=True)",
                flags.join("; ")
            ),
        );
    }
    (true, "L_PRIVACY ok".into())
}

pub fn evaluate_l_consent(decision: &Decision, context: &Context) -> (bool, String) {
    if !CONSENT_ACTIONS.contains(&decision.action_type.as_str()) {
        return (true, "L_CONSENT n/a".into());
    }
    if context.user_confirmed == Some(true) {
        return (true, "L_CONSENT confirmed by user".into());
    }
    (
        false,
        format!(
            "L_CONSENT: action='{}' has external blast radius and requires explicit user confirmation",
            decision.action_type
        ),
    )
}

/// L_VERIFIABILITY assumes the actual citation check is delegated to the
/// caller's `CitationChecker` (so this crate stays free of a literature
/// dependency). The function returns (false, ...) if `checker` reports any
/// rejected citation; (true, "n/a") if the action isn't a citation action.
pub trait CitationChecker: Send + Sync {
    fn check(&self, text: &str) -> CitationCheckResult;
}

#[derive(Clone, Debug, Default)]
pub struct CitationCheckResult {
    pub rejected: Vec<String>,
}

pub fn evaluate_l_verifiability(
    decision: &Decision,
    checker: Option<&dyn CitationChecker>,
) -> (bool, String) {
    if !VERIFIABILITY_ACTIONS.contains(&decision.action_type.as_str()) {
        return (true, "L_VERIFIABILITY n/a".into());
    }
    let text = match decision
        .payload
        .get("text")
        .or_else(|| decision.payload.get("body"))
        .and_then(|v| v.as_str())
    {
        Some(s) if !s.is_empty() => s,
        _ => return (true, "L_VERIFIABILITY: no text to verify".into()),
    };
    let Some(c) = checker else {
        // No checker wired up — fail closed (matches Python "failing closed").
        return (
            false,
            "L_VERIFIABILITY: citation checker unavailable; failing closed".into(),
        );
    };
    let res = c.check(text);
    if !res.rejected.is_empty() {
        return (
            false,
            format!(
                "L_VERIFIABILITY: {} unverified citation(s) — {}",
                res.rejected.len(),
                res.rejected.join(", ")
            ),
        );
    }
    (true, "L_VERIFIABILITY ok".into())
}

pub fn evaluate_extended(
    decision: &Decision,
    context: &Context,
    checker: Option<&dyn CitationChecker>,
) -> ExtendedLawsResult {
    let (p_ok, p_r) = evaluate_l_privacy(decision, context);
    let (c_ok, c_r) = evaluate_l_consent(decision, context);
    let (v_ok, v_r) = evaluate_l_verifiability(decision, checker);
    ExtendedLawsResult {
        privacy: p_ok,
        consent: c_ok,
        verifiability: v_ok,
        reasons: vec![p_r, c_r, v_r],
    }
}

// ── impedance ───────────────────────────────────────────────────────────────

pub fn impedance_checklist(patient: &Patient) -> f64 {
    let mut i = 0.0_f64;
    i += (patient.missing_labs_count as f64 * 0.04).min(0.20);
    i += (patient.history_contradictions as f64 * 0.10).min(0.30);
    i += (patient.unexplained_symptoms_count as f64 * 0.04).min(0.20);
    if patient.last_visit_years_ago > 2 {
        i += 0.05;
    }
    if patient.dx_without_evidence {
        i += 0.10;
    }
    if patient.primary_complaint_undiagnosed {
        i += 0.10;
    }
    i.min(0.8)
}

/// Without an LLM-delta, total impedance == checklist (mirrors the Python
/// path where `AIM_KERNEL_LLM_DELTA=0`).
pub fn impedance(patient: &Patient) -> f64 {
    impedance_checklist(patient)
}

// ── scoring ─────────────────────────────────────────────────────────────────

fn duration_days(action_type: &str) -> f64 {
    match action_type {
        "wait" => 7.0,
        "test" => 1.0,
        "imaging" => 3.0,
        "treatment" => 7.0,
        "dx" => 0.5,
        "referral" => 14.0,
        "clarify" => 0.1,
        _ => 1.0,
    }
}

pub fn expected_impedance_after(decision: &Decision, patient: &Patient) -> f64 {
    let i_before = impedance(patient);
    let reduction = match decision.action_type.as_str() {
        "test" => 0.4,
        "imaging" => 0.5,
        "referral" => 0.3,
        "dx" => 0.2,
        "treatment" => 0.15,
        "clarify" => 0.25,
        "wait" => 0.05,
        _ => 0.1,
    };
    let mut i_after = (i_before * (1.0 - reduction)).max(0.0);
    if decision.action_type == "treatment" && !patient.has_confirmed_dx {
        // cheating penalty: treatment without dx → barely reduces real I
        i_after = i_before * 0.9;
    }
    i_after
}

pub fn instant_c(decision: &Decision, patient: &Patient) -> f64 {
    let i_before = impedance(patient);
    let i_after = expected_impedance_after(decision, patient);
    let dur = duration_days(&decision.action_type).max(0.1);
    ((i_before - i_after) / dur).clamp(0.0, 1.0)
}

pub fn phi_ze_path_integral(decision: &Decision, patient: &Patient) -> f64 {
    let i_before = impedance(patient);
    let i_after = expected_impedance_after(decision, patient);
    let dur = duration_days(&decision.action_type);
    let avg = (i_before + i_after) / 2.0;
    let phi_raw = avg * dur;
    let phi_norm = (phi_raw / 30.0).min(1.0);
    1.0 - phi_norm
}

// ── ethics ──────────────────────────────────────────────────────────────────

pub fn ethics_ze_score(decision: &Decision, patient: &Patient) -> f64 {
    let x = match decision.action_type.as_str() {
        "test" | "imaging" => 0.9,
        "referral" => 0.8,
        "clarify" => 0.85,
        "dx" => 0.5,
        "treatment" => 0.3,
        "wait" => 0.2,
        _ => 0.5,
    };
    let y = match decision.action_type.as_str() {
        "test" | "imaging" | "referral" | "clarify" => 0.0,
        "dx" => 0.1,
        "treatment" => {
            if patient.has_confirmed_dx {
                0.1
            } else {
                0.5
            }
        }
        "wait" => {
            if patient.primary_complaint_undiagnosed {
                0.3
            } else {
                0.1
            }
        }
        _ => 0.1,
    };
    let raw = (x - y) / (x + y + EPS);
    (raw + 1.0) / 2.0
}

pub fn ethics_autonomy(decision: &Decision, patient: &Patient) -> f64 {
    let mut score = 0.7_f64;
    if decision.payload_bool("informed_consent_noted") {
        score += 0.15;
    }
    let pref_respected = decision
        .payload
        .get("patient_preference_respected")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    if pref_respected {
        score += 0.10;
    }
    if patient.refusal_noted && decision.action_type != "clarify" {
        score -= 0.30;
    }
    score.clamp(0.0, 1.0)
}

pub fn ethics_beneficence(decision: &Decision, patient: &Patient) -> f64 {
    let mut base: f64 = match decision.action_type.as_str() {
        "test" => 0.75,
        "imaging" => 0.80,
        "referral" => 0.75,
        "clarify" => 0.60,
        "dx" => 0.70,
        "treatment" => {
            if patient.has_confirmed_dx {
                0.85
            } else {
                0.40
            }
        }
        "wait" => 0.40,
        _ => 0.5,
    };
    if !patient.red_flags.is_empty()
        && matches!(decision.action_type.as_str(), "test" | "imaging" | "referral")
    {
        base = (base + 0.15).min(1.0);
    }
    base
}

pub fn ethics_nonmaleficence(decision: &Decision) -> f64 {
    let mut base = 0.9_f64;
    if decision.action_type == "treatment" {
        let drug = decision.payload_str("drug").to_lowercase();
        let risky = ["warfarin", "digoxin", "lithium", "amiodarone", "methotrexate"];
        if risky.iter().any(|r| drug.contains(r)) {
            base = 0.6;
        }
        let controlled = ["opioid", "morphine", "fentanyl", "oxycodone"];
        if controlled.iter().any(|c| drug.contains(c)) {
            base = 0.5;
        }
    }
    if decision.action_type == "imaging" {
        let modality = decision.payload_str("modality").to_lowercase();
        if modality.contains("ct") || modality.contains("x-ray") {
            base -= 0.05;
        }
    }
    base.clamp(0.0, 1.0)
}

pub fn ethics_justice(decision: &Decision) -> f64 {
    let mut score = 0.85_f64;
    if decision.payload_bool("demographic_gated") {
        score -= 0.40;
    }
    if decision.payload_bool("guideline_based") {
        score += 0.10;
    }
    score.clamp(0.0, 1.0)
}

#[derive(Clone, Debug)]
pub struct EthicsParts {
    pub ze_learn_cheat: f64,
    pub autonomy: f64,
    pub beneficence: f64,
    pub nonmaleficence: f64,
    pub justice: f64,
}

pub fn ethics_composite(
    decision: &Decision,
    patient: &Patient,
    weights: &KernelWeights,
) -> (f64, EthicsParts) {
    let ze = ethics_ze_score(decision, patient);
    let auto = ethics_autonomy(decision, patient);
    let benef = ethics_beneficence(decision, patient);
    let nonmal = ethics_nonmaleficence(decision);
    let justice = ethics_justice(decision);
    let composite = weights.ethics_ze * ze
        + weights.ethics_auto * auto
        + weights.ethics_benef * benef
        + weights.ethics_nonmal * nonmal
        + weights.ethics_justice * justice;
    (
        composite,
        EthicsParts {
            ze_learn_cheat: ze,
            autonomy: auto,
            beneficence: benef,
            nonmaleficence: nonmal,
            justice,
        },
    )
}

pub fn score_decision(
    decision: &Decision,
    patient: &Patient,
    weights: &KernelWeights,
) -> ScoringResult {
    let i_before = impedance(patient);
    let i_after = expected_impedance_after(decision, patient);
    let c = instant_c(decision, patient);
    let phi = phi_ze_path_integral(decision, patient);
    let (ethics, parts) = ethics_composite(decision, patient, weights);
    let utility = weights.alpha * c + weights.beta * phi + weights.gamma * ethics;
    ScoringResult {
        impedance_before: i_before,
        impedance_after: i_after,
        instant_c: c,
        phi_ze: phi,
        ethics_ze_learn_cheat: parts.ze_learn_cheat,
        ethics_autonomy: parts.autonomy,
        ethics_beneficence: parts.beneficence,
        ethics_nonmaleficence: parts.nonmaleficence,
        ethics_justice: parts.justice,
        ethics_composite: ethics,
        utility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(action: &str, payload: serde_json::Value) -> Decision {
        Decision {
            id: "d1".into(),
            description: String::new(),
            action_type: action.into(),
            payload,
            meta: serde_json::json!({}),
        }
    }

    // ── L0 ──────────────────────────────────────────────────────────────────

    #[test]
    fn l0_rejects_danger_signal_in_description() {
        let mut d = dec("dx", serde_json::json!({}));
        d.description = "weapon design plan".into();
        let (ok, _) = evaluate_l0(&d);
        assert!(!ok);
    }

    #[test]
    fn l0_rejects_broad_abx_for_viral() {
        let d = dec(
            "treatment",
            serde_json::json!({"drug": "vancomycin", "indication": "viral URI"}),
        );
        let (ok, r) = evaluate_l0(&d);
        assert!(!ok);
        assert!(r.contains("resistance"));
    }

    #[test]
    fn l0_accepts_clean_treatment() {
        let d = dec(
            "treatment",
            serde_json::json!({"drug": "amoxicillin", "indication": "strep throat"}),
        );
        let (ok, _) = evaluate_l0(&d);
        assert!(ok);
    }

    // ── L1 ──────────────────────────────────────────────────────────────────

    #[test]
    fn l1_blocks_drug_in_allergy_family() {
        let d = dec("treatment", serde_json::json!({"drug": "amoxicillin"}));
        let mut p = Patient::new();
        p.allergies.push("penicillin rash".into());
        let (ok, r) = evaluate_l1(&d, &p, &Context::default());
        assert!(!ok);
        assert!(r.contains("семействе"));
    }

    #[test]
    fn l1_blocks_major_interaction() {
        let d = dec(
            "treatment",
            serde_json::json!({
                "drug": "x",
                "interactions": [{"severity": "major", "summary": "x + y"}]
            }),
        );
        let p = Patient::new();
        let (ok, _) = evaluate_l1(&d, &p, &Context::default());
        assert!(!ok);
    }

    #[test]
    fn l1_blocks_wait_with_red_flags() {
        let d = dec("wait", serde_json::json!({}));
        let mut p = Patient::new();
        p.red_flags.push("chest pain".into());
        let (ok, r) = evaluate_l1(&d, &p, &Context::default());
        assert!(!ok);
        assert!(r.contains("inaction"));
    }

    #[test]
    fn l1_blocks_wait_with_high_impedance() {
        let d = dec("wait", serde_json::json!({}));
        let p = Patient::new();
        let mut ctx = Context::default();
        ctx.impedance_before = Some(0.95);
        let (ok, _) = evaluate_l1(&d, &p, &ctx);
        assert!(!ok);
    }

    #[test]
    fn l1_passes_clean_treatment() {
        let d = dec("treatment", serde_json::json!({"drug": "ibuprofen"}));
        let p = Patient::new();
        let (ok, _) = evaluate_l1(&d, &p, &Context::default());
        assert!(ok);
    }

    // ── L2 / L3 ─────────────────────────────────────────────────────────────

    #[test]
    fn l2_blocks_action_mismatch() {
        let d = dec("dx", serde_json::json!({}));
        let mut ctx = Context::default();
        ctx.commanded_action_type = Some("test".into());
        let (ok, _) = evaluate_l2(&d, &ctx);
        assert!(!ok);
    }

    #[test]
    fn l2_passes_when_no_command_or_match() {
        let d = dec("dx", serde_json::json!({}));
        let (ok, _) = evaluate_l2(&d, &Context::default());
        assert!(ok);
        let mut ctx = Context::default();
        ctx.commanded_action_type = Some("dx".into());
        let (ok2, _) = evaluate_l2(&d, &ctx);
        assert!(ok2);
    }

    #[test]
    fn l3_blocks_destructive_system_modification() {
        let d = dec(
            "system_modification",
            serde_json::json!({"destructive": true}),
        );
        let (ok, _) = evaluate_l3(&d);
        assert!(!ok);
    }

    #[test]
    fn evaluate_laws_combines_branches() {
        let d = dec("dx", serde_json::json!({}));
        let p = Patient::new();
        let r = evaluate_laws(&d, &p, &Context::default());
        assert!(r.passed());
        assert_eq!(r.reasons.len(), 4);
    }

    // ── L_PRIVACY ───────────────────────────────────────────────────────────

    #[test]
    fn privacy_na_for_non_egress_actions() {
        let d = dec("dx", serde_json::json!({"text": "Patients/X"}));
        let (ok, r) = evaluate_l_privacy(&d, &Context::default());
        assert!(ok);
        assert!(r.contains("n/a"));
    }

    #[test]
    fn privacy_blocks_patient_path_in_egress() {
        let d = dec(
            "email_send",
            serde_json::json!({"body": "see Patients/Smith_John"}),
        );
        let (ok, _) = evaluate_l_privacy(&d, &Context::default());
        assert!(!ok);
    }

    #[test]
    fn privacy_blocks_phone_pattern() {
        let d = dec(
            "email_send",
            serde_json::json!({"body": "call +995 555 1234567"}),
        );
        let (ok, _) = evaluate_l_privacy(&d, &Context::default());
        assert!(!ok);
    }

    #[test]
    fn privacy_passes_with_consent() {
        let d = dec(
            "email_send",
            serde_json::json!({"body": "see Patients/Smith_John"}),
        );
        let mut ctx = Context::default();
        ctx.privacy_consent = true;
        let (ok, _) = evaluate_l_privacy(&d, &ctx);
        assert!(ok);
    }

    // ── L_CONSENT ───────────────────────────────────────────────────────────

    #[test]
    fn consent_blocks_unconfirmed_email() {
        let d = dec("email_send", serde_json::json!({}));
        let (ok, _) = evaluate_l_consent(&d, &Context::default());
        assert!(!ok);
    }

    #[test]
    fn consent_passes_when_confirmed() {
        let d = dec("email_send", serde_json::json!({}));
        let mut ctx = Context::default();
        ctx.user_confirmed = Some(true);
        let (ok, _) = evaluate_l_consent(&d, &ctx);
        assert!(ok);
    }

    // ── L_VERIFIABILITY ─────────────────────────────────────────────────────

    struct AlwaysClean;
    impl CitationChecker for AlwaysClean {
        fn check(&self, _: &str) -> CitationCheckResult {
            CitationCheckResult::default()
        }
    }
    struct AlwaysReject;
    impl CitationChecker for AlwaysReject {
        fn check(&self, _: &str) -> CitationCheckResult {
            CitationCheckResult {
                rejected: vec!["PMID:99999".into()],
            }
        }
    }

    #[test]
    fn verif_na_for_non_emit_actions() {
        let d = dec("dx", serde_json::json!({"text": "x"}));
        let (ok, _) = evaluate_l_verifiability(&d, Some(&AlwaysReject));
        assert!(ok);
    }

    #[test]
    fn verif_passes_clean_text() {
        let d = dec("emit_text", serde_json::json!({"text": "claim"}));
        let (ok, _) = evaluate_l_verifiability(&d, Some(&AlwaysClean));
        assert!(ok);
    }

    #[test]
    fn verif_blocks_rejected_citation() {
        let d = dec("emit_text", serde_json::json!({"text": "see PMID 99999"}));
        let (ok, _) = evaluate_l_verifiability(&d, Some(&AlwaysReject));
        assert!(!ok);
    }

    #[test]
    fn verif_fails_closed_when_no_checker() {
        let d = dec("emit_text", serde_json::json!({"text": "x"}));
        let (ok, _) = evaluate_l_verifiability(&d, None);
        assert!(!ok);
    }

    #[test]
    fn verif_passes_when_no_text() {
        let d = dec("emit_text", serde_json::json!({}));
        let (ok, _) = evaluate_l_verifiability(&d, None);
        assert!(ok);
    }

    // ── impedance ───────────────────────────────────────────────────────────

    #[test]
    fn impedance_baseline_undiagnosed_complaint() {
        let p = Patient::new(); // primary_complaint_undiagnosed = true
        // = 0 + 0 + 0 + 0 + 0 + 0.10 = 0.10
        assert!((impedance(&p) - 0.10).abs() < 1e-9);
    }

    #[test]
    fn impedance_caps_at_eight_tenths() {
        let p = Patient {
            missing_labs_count: 100,
            history_contradictions: 100,
            unexplained_symptoms_count: 100,
            last_visit_years_ago: 5,
            dx_without_evidence: true,
            primary_complaint_undiagnosed: true,
            ..Default::default()
        };
        assert!((impedance(&p) - 0.8).abs() < 1e-9);
    }

    // ── scoring ─────────────────────────────────────────────────────────────

    #[test]
    fn score_test_action_has_higher_utility_than_wait_for_red_flag() {
        let mut p = Patient::new();
        p.red_flags.push("chest pain".into());
        let test = dec("test", serde_json::json!({}));
        let wait = dec("wait", serde_json::json!({}));
        let w = KernelWeights::default();
        let s_test = score_decision(&test, &p, &w);
        let s_wait = score_decision(&wait, &p, &w);
        assert!(s_test.utility > s_wait.utility);
    }

    #[test]
    fn score_treatment_without_dx_has_low_ze() {
        let p = Patient::new();
        let d = dec("treatment", serde_json::json!({"drug": "ibuprofen"}));
        let s = score_decision(&d, &p, &KernelWeights::default());
        // x=0.3, y=0.5 → raw=-0.25 → 0.375
        assert!(s.ethics_ze_learn_cheat < 0.5);
    }

    #[test]
    fn score_treatment_with_dx_has_higher_beneficence() {
        let mut p = Patient::new();
        p.has_confirmed_dx = true;
        let d = dec("treatment", serde_json::json!({}));
        let s = score_decision(&d, &p, &KernelWeights::default());
        assert!(s.ethics_beneficence > 0.7);
    }

    #[test]
    fn ethics_nonmal_drops_for_risky_drug() {
        let d = dec("treatment", serde_json::json!({"drug": "warfarin"}));
        assert_eq!(ethics_nonmaleficence(&d), 0.6);
    }

    #[test]
    fn ethics_nonmal_drops_for_controlled_substance() {
        let d = dec("treatment", serde_json::json!({"drug": "morphine sulfate"}));
        assert_eq!(ethics_nonmaleficence(&d), 0.5);
    }

    #[test]
    fn ethics_justice_penalises_demographic_gating() {
        let d = dec("test", serde_json::json!({"demographic_gated": true}));
        let j = ethics_justice(&d);
        assert!(j < 0.5);
    }

    #[test]
    fn instant_c_positive_for_information_gathering() {
        let p = Patient::new();
        let d = dec("test", serde_json::json!({}));
        assert!(instant_c(&d, &p) > 0.0);
    }

    #[test]
    fn phi_ze_in_unit_interval() {
        let p = Patient::new();
        let d = dec("imaging", serde_json::json!({}));
        let phi = phi_ze_path_integral(&d, &p);
        assert!((0.0..=1.0).contains(&phi));
    }

    #[test]
    fn extended_evaluates_three_laws() {
        let d = dec("emit_text", serde_json::json!({"text": "x"}));
        let r = evaluate_extended(&d, &Context::default(), Some(&AlwaysClean));
        assert!(r.privacy);
        assert!(r.consent);
        assert!(r.verifiability);
        assert_eq!(r.reasons.len(), 3);
    }
}

// keep unused alias quiet
#[allow(dead_code)]
type _BTreeMapAlias = BTreeMap<String, serde_json::Value>;
