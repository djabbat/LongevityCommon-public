//! Distillation: walk contributions, output candidate updates.
//!
//! Pure function. Same patterns as
//! `AI/ai/hive_queen.py::distill_candidates`:
//!
//! 1. Compliance drift — if ≥2 workers and average compliance < 0.5,
//!    emit a `prompt_patch` candidate.
//! 2. Theme convergence — if a reflexion theme appears across ≥2
//!    workers, emit a `skill` candidate keyed by theme hash.

use std::collections::{BTreeMap, BTreeSet};

use crate::{Candidate, Contribution};

/// Minimum distinct workers showing the same pattern before it becomes
/// a candidate.
pub const MIN_WORKERS_FOR_PATTERN: usize = 2;

pub fn distill(contribs: &[Contribution]) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = Vec::new();

    // 1. Compliance drift detection.
    let mut by_worker: BTreeMap<String, f64> = BTreeMap::new();
    for c in contribs {
        let led = c.payload.get("ledger");
        let n_runs = led
            .and_then(|l| l.get("n_runs"))
            .and_then(|n| n.as_u64())
            .unwrap_or(0);
        if n_runs > 0 {
            let avg = led
                .and_then(|l| l.get("avg_compliance"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            by_worker.insert(c.worker_id.clone(), avg);
        }
    }
    if by_worker.len() >= MIN_WORKERS_FOR_PATTERN {
        let avg: f64 = by_worker.values().sum::<f64>() / by_worker.len() as f64;
        if avg < 0.5 {
            let workers: BTreeSet<String> = by_worker.keys().cloned().collect();
            out.push(Candidate {
                kind: "prompt_patch".to_string(),
                body: serde_json::json!({
                    "patch_type": "tighten_compliance_rule",
                    "current_avg": (avg * 1000.0).round() / 1000.0,
                    "rationale": "Cross-worker compliance ≤50% — prompt rule may not be enforcing path:line",
                }),
                source_workers: workers.clone(),
                rationale: format!(
                    "avg compliance {:.0}% across {} workers — consider stronger rule wording",
                    avg * 100.0,
                    workers.len()
                ),
            });
        }
    }

    // 2. Reflexion theme convergence.
    let mut theme_workers: BTreeMap<Vec<String>, BTreeSet<String>> = BTreeMap::new();
    for c in contribs {
        let clusters = c
            .payload
            .get("reflexion")
            .and_then(|r| r.get("clusters"))
            .and_then(|c| c.as_array());
        let Some(clusters) = clusters else { continue };
        for cl in clusters {
            let theme = cl
                .get("theme")
                .and_then(|t| t.as_array())
                .map(|a| {
                    let mut v: Vec<String> = a
                        .iter()
                        .filter_map(|w| w.as_str())
                        .map(|s| s.to_string())
                        .collect();
                    v.sort();
                    v
                })
                .unwrap_or_default();
            if theme.is_empty() {
                continue;
            }
            theme_workers
                .entry(theme)
                .or_default()
                .insert(c.worker_id.clone());
        }
    }
    for (theme, ws) in theme_workers {
        if ws.len() >= MIN_WORKERS_FOR_PATTERN {
            use sha2::Digest;
            let key = theme.join(" ");
            let mut h = sha2::Sha256::new();
            h.update(key.as_bytes());
            let digest = h.finalize();
            let skill_id = format!("auto-{}", &hex::encode(&digest[..4]));
            out.push(Candidate {
                kind: "skill".to_string(),
                body: serde_json::json!({
                    "skill_id": skill_id,
                    "theme": theme.clone(),
                    "rationale": format!("theme {:?} appeared across {} workers", theme, ws.len()),
                }),
                source_workers: ws.clone(),
                rationale: format!("theme {:?} clustered across {} workers", theme, ws.len()),
            });
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contrib(worker: &str, payload: serde_json::Value) -> Contribution {
        Contribution {
            id: uuid::Uuid::new_v4().to_string(),
            ts: "2026-01-01T00:00:00Z".to_string(),
            worker_id: worker.to_string(),
            payload,
        }
    }

    #[test]
    fn empty_contribs_no_candidates() {
        assert!(distill(&[]).is_empty());
    }

    #[test]
    fn single_worker_no_candidate() {
        let cs = vec![contrib(
            "w1",
            serde_json::json!({"ledger":{"n_runs":1,"avg_compliance":0.2}}),
        )];
        assert!(distill(&cs).is_empty());
    }

    #[test]
    fn compliance_drift_detected_two_workers_low() {
        let cs = vec![
            contrib(
                "w1",
                serde_json::json!({"ledger":{"n_runs":5,"avg_compliance":0.3}}),
            ),
            contrib(
                "w2",
                serde_json::json!({"ledger":{"n_runs":5,"avg_compliance":0.4}}),
            ),
        ];
        let out = distill(&cs);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind, "prompt_patch");
        assert_eq!(out[0].source_n(), 2);
    }

    #[test]
    fn compliance_drift_not_detected_when_high() {
        let cs = vec![
            contrib(
                "w1",
                serde_json::json!({"ledger":{"n_runs":5,"avg_compliance":0.7}}),
            ),
            contrib(
                "w2",
                serde_json::json!({"ledger":{"n_runs":5,"avg_compliance":0.8}}),
            ),
        ];
        assert!(distill(&cs).is_empty());
    }

    #[test]
    fn theme_convergence() {
        let cs = vec![
            contrib(
                "w1",
                serde_json::json!({"reflexion":{"clusters":[{"theme":["bug","retry"],"n":3}]}}),
            ),
            contrib(
                "w2",
                serde_json::json!({"reflexion":{"clusters":[{"theme":["retry","bug"],"n":2}]}}),
            ),
        ];
        let out = distill(&cs);
        // theme normalised to sorted ["bug","retry"], shared by both → 1 candidate
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind, "skill");
        assert_eq!(out[0].source_n(), 2);
        assert!(out[0].body["skill_id"].as_str().unwrap().starts_with("auto-"));
    }

    #[test]
    fn theme_skipped_if_only_one_worker() {
        let cs = vec![contrib(
            "w1",
            serde_json::json!({"reflexion":{"clusters":[{"theme":["a","b"],"n":1}]}}),
        )];
        assert!(distill(&cs).is_empty());
    }
}
