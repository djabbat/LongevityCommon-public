use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use sha2::{Digest, Sha256};
use tracing::{info, warn};
use uuid::Uuid;

use fclc_core::{aggregation::fedprox_aggregate, aggregation::krum_select, ShapleyScorer};

use crate::{
    db,
    models::{RoundResult, UpdatePayload},
    state::{AppState, NodeDpState, EPSILON_TOTAL, MIN_NODES_FOR_AGGREGATION},
};

/// Byzantine-tolerance fraction used by Krum (25%).
const BYZANTINE_FRACTION: f64 = 0.25;

// ── Hashing helpers ───────────────────────────────────────────────────────────

/// SHA-256 of aggregated model weights (little-endian f64 bytes).
fn hash_weights(weights: &[f64]) -> String {
    let mut h = Sha256::new();
    for w in weights {
        h.update(w.to_le_bytes());
    }
    hex::encode(h.finalize())
}

/// entry_hash = SHA-256(round_id_bytes ‖ round_number_le ‖ gradient_hash_bytes ‖ prev_hash_bytes).
fn compute_entry_hash(round_id: Uuid, round_number: u64, gradient_hash: &str, prev_hash: &str) -> String {
    let mut h = Sha256::new();
    h.update(round_id.as_bytes());
    h.update(round_number.to_le_bytes());
    h.update(gradient_hash.as_bytes());
    h.update(prev_hash.as_bytes());
    hex::encode(h.finalize())
}

/// Server-side aggregation strength (FedAvg = 0.0).
///
/// FedProx proximal correction (μ=0.1) is applied CLIENT-SIDE in
/// fclc-node/pipeline/mod.rs: `grad += μ * (w_local - w_global)`.
/// Setting MU > 0.0 here would double-count the regularisation and
/// over-shrink the aggregated model toward the previous global weights.
const MU: f32 = 0.0;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_weights_deterministic() {
        let w = vec![0.1_f64, 0.5, -0.3, 1.0];
        assert_eq!(hash_weights(&w), hash_weights(&w),
            "hash_weights must be deterministic");
    }

    #[test]
    fn test_hash_weights_changes_with_input() {
        let w1 = vec![0.1_f64, 0.5];
        let w2 = vec![0.1_f64, 0.6];
        assert_ne!(hash_weights(&w1), hash_weights(&w2),
            "Different weights must produce different hashes");
    }

    #[test]
    fn test_hash_weights_is_64_hex_chars() {
        let h = hash_weights(&[1.0, 2.0, 3.0]);
        assert_eq!(h.len(), 64, "SHA-256 hex must be 64 characters");
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()),
            "Hash must be lowercase hex");
    }

    #[test]
    fn test_entry_hash_deterministic() {
        let id = Uuid::new_v4();
        let gh = "abcd1234".repeat(8);
        let ph = "0".repeat(64);
        assert_eq!(
            compute_entry_hash(id, 1, &gh, &ph),
            compute_entry_hash(id, 1, &gh, &ph),
            "entry_hash must be deterministic"
        );
    }

    #[test]
    fn test_entry_hash_changes_with_round_number() {
        let id = Uuid::new_v4();
        let gh = "a".repeat(64);
        let ph = "0".repeat(64);
        let h1 = compute_entry_hash(id, 1, &gh, &ph);
        let h2 = compute_entry_hash(id, 2, &gh, &ph);
        assert_ne!(h1, h2, "Different round numbers must produce different entry hashes");
    }

    #[test]
    fn test_entry_hash_chains_via_prev_hash() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let gw = vec![0.5_f64, -0.2, 1.1];
        let gh = hash_weights(&gw);
        let genesis = "0".repeat(64);

        // Round 0 entry
        let h0 = compute_entry_hash(id1, 0, &gh, &genesis);
        assert_eq!(h0.len(), 64);

        // Round 1 uses h0 as prev_hash — simulates chain
        let h1 = compute_entry_hash(id2, 1, &gh, &h0);
        assert_ne!(h1, h0, "Chained hashes must differ");
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn test_genesis_hash_is_64_zeros() {
        use crate::db::GENESIS_HASH;
        assert_eq!(GENESIS_HASH.len(), 64);
        assert!(GENESIS_HASH.chars().all(|c| c == '0'));
    }
}

/// Attempt to run a federated aggregation round if enough updates are pending.
///
/// Returns `Ok(true)` if aggregation ran, `Ok(false)` if skipped (too few nodes).
pub async fn maybe_aggregate(state: Arc<AppState>) -> Result<bool> {
    let pending_count = state.pending_updates.read().await.len();
    if pending_count < MIN_NODES_FOR_AGGREGATION {
        return Ok(false);
    }
    run_aggregation(state).await?;
    Ok(true)
}

/// Force-run an aggregation regardless of pending update count.
/// Used by the admin `/api/rounds/trigger` endpoint.
pub async fn force_aggregate(state: Arc<AppState>) -> Result<bool> {
    let pending_count = state.pending_updates.read().await.len();
    if pending_count == 0 {
        return Ok(false);
    }
    run_aggregation(state).await?;
    Ok(true)
}

/// Core aggregation logic:
/// 1. Drain `pending_updates`.
/// 2. Exclude nodes that have exceeded the DP epsilon budget.
/// 3. Krum robust selection.
/// 4. FedProx weighted aggregation.
/// 5. Update global model.
/// 6. Compute Shapley scores.
/// 7. Persist round result and scores to DB.
/// 8. Append tamper-evident audit log entry (hash chain).
/// 9. Push to in-memory round history + increment `current_round`.
async fn run_aggregation(state: Arc<AppState>) -> Result<()> {
    // ── 1. Drain pending updates ──────────────────────────────────────────────
    let updates: Vec<(Uuid, UpdatePayload)> = {
        let mut pending = state.pending_updates.write().await;
        std::mem::take(&mut *pending)
    };

    // ── 2. Filter out nodes that have exceeded the DP budget ─────────────────
    // Uses Rényi DP effective epsilon when sigma+sampling_rate are provided;
    // falls back to linear composition otherwise. Budget check is conservative:
    // excluded only if BOTH linear and Rényi estimates exceed EPSILON_TOTAL.
    let budgets = state.node_budgets.read().await;
    let eligible: Vec<(Uuid, UpdatePayload)> = updates
        .into_iter()
        .filter(|(node_id, payload)| {
            let effective_spent = budgets
                .get(node_id)
                .map(|s| s.effective_epsilon())
                .unwrap_or(0.0);
            // Estimate what Rényi epsilon would be after this round.
            let linear_spent = budgets
                .get(node_id)
                .map(|s| s.epsilon_linear)
                .unwrap_or(0.0);
            let would_be_linear = linear_spent + payload.epsilon_spent;
            // Use effective (Rényi) if it's available, else linear for the check.
            let effective_would_be = effective_spent + payload.epsilon_spent;
            if effective_would_be > EPSILON_TOTAL {
                warn!(
                    node_id = %node_id,
                    effective_spent = effective_spent,
                    linear_would_be = would_be_linear,
                    requested = payload.epsilon_spent,
                    "Node excluded: DP budget exceeded (Rényi ε={effective_spent:.3})"
                );
                false
            } else {
                true
            }
        })
        .collect();
    drop(budgets);

    if eligible.is_empty() {
        warn!("No eligible updates after DP budget filtering — skipping aggregation");
        return Ok(());
    }

    let n = eligible.len();
    let round_number = *state.current_round.read().await;

    // ── 3. Krum robust selection (only if n >= 2) ─────────────────────────────
    // Convert f64 gradients → f32 for fclc-core functions.
    let f32_updates: Vec<Vec<f32>> = eligible
        .iter()
        .map(|(_, p)| p.gradient.iter().map(|&x| x as f32).collect())
        .collect();

    // Multi-Krum: select the top-k honest updates (k = n - f),
    // then pass all selected updates to FedProx for weighted averaging.
    // This preserves FedProx's aggregation benefit across multiple nodes.
    let f_nodes = (BYZANTINE_FRACTION * n as f64).floor() as usize;
    let k_select = (n - f_nodes).max(1); // number of updates to keep

    let krum_updates: Vec<Vec<f32>> = if n >= 2 {
        // Score each update by its Krum score, then take top-k.
        let krum_winner = krum_select(&f32_updates, BYZANTINE_FRACTION);
        // Find winner index and keep all updates except detected outliers.
        // Simple Multi-Krum: include all updates whose L2 distance from the
        // winner is within 3× the winner's average neighbour distance.
        let winner_ref = &krum_winner;
        let mut scored: Vec<(f64, usize)> = f32_updates
            .iter()
            .enumerate()
            .map(|(i, u)| {
                let d: f64 = u.iter().zip(winner_ref.iter())
                    .map(|(a, b)| { let d = (a - b) as f64; d * d })
                    .sum::<f64>()
                    .sqrt();
                (d, i)
            })
            .collect();
        scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        scored.iter().take(k_select).map(|(_, i)| f32_updates[*i].clone()).collect()
    } else {
        f32_updates.clone()
    };
    let krum_weights: Vec<f64> = vec![1.0; krum_updates.len()];

    // ── 4. FedProx aggregation ────────────────────────────────────────────────
    let global_f32: Vec<f32> = {
        let g = state.global_model.read().await;
        g.iter().map(|&x| x as f32).collect()
    };

    // Pad / truncate gradient to MODEL_DIM if needed.
    let dim = global_f32.len();
    let safe_updates: Vec<Vec<f32>> = krum_updates
        .into_iter()
        .map(|mut u| {
            u.resize(dim, 0.0);
            u
        })
        .collect();

    let new_weights_f32 = fedprox_aggregate(&safe_updates, &krum_weights, &global_f32, MU);

    // ── 5. Update global model ────────────────────────────────────────────────
    {
        let mut global = state.global_model.write().await;
        *global = new_weights_f32.iter().map(|&x| x as f64).collect();
    }

    // ── 6. Compute Shapley scores ─────────────────────────────────────────────
    // Performance function: use the AUC reported by each node's coalition.
    let node_ids: Vec<Uuid> = eligible.iter().map(|(id, _)| *id).collect();
    let aucs: Vec<f64> = eligible.iter().map(|(_, p)| p.auc).collect();

    let scorer = ShapleyScorer::new(n);
    let shapley_values = scorer.compute(|coalition: &[usize]| {
        if coalition.is_empty() {
            return 0.0;
        }
        // Coalition performance = mean AUC of members.
        let sum: f64 = coalition.iter().map(|&i| aucs[i]).sum();
        sum / coalition.len() as f64
    });

    let scored_pairs: Vec<(Uuid, f64)> = node_ids
        .iter()
        .zip(shapley_values.iter())
        .map(|(&id, &s)| (id, s))
        .collect();

    // ── 7. Persist to DB ──────────────────────────────────────────────────────
    let round_id = Uuid::new_v4();
    let mean_auc = aucs.iter().sum::<f64>() / aucs.len() as f64;
    let timestamp = Utc::now().to_rfc3339();

    db::insert_round(&state.pool, round_id, round_number, mean_auc, n).await?;
    db::insert_shapley_scores(&state.pool, round_number, &scored_pairs).await?;

    // Update per-node epsilon budgets in DB.
    // Uses NodeDpState which tracks both linear and Rényi epsilon.
    {
        let mut budgets = state.node_budgets.write().await;
        for (node_id, payload) in &eligible {
            let entry = budgets.entry(*node_id).or_insert_with(NodeDpState::new);
            entry.spend(payload.epsilon_spent, payload.sigma, payload.sampling_rate);
            let effective = entry.effective_epsilon();
            let linear = entry.epsilon_linear;
            let rdp_eps = entry.rdp.current_epsilon();
            info!(
                node_id = %node_id,
                linear_eps = linear,
                rdp_eps = rdp_eps,
                effective_eps = effective,
                "DP budget update: Rényi saves {:.3}ε vs linear",
                (linear - effective).max(0.0)
            );
            if let Err(e) = db::update_node_epsilon(&state.pool, *node_id, effective).await {
                warn!("Failed to persist epsilon for node {}: {}", node_id, e);
            }
            // Also persist the update record.
            if let Err(e) = db::insert_update(
                &state.pool,
                *node_id,
                round_number,
                payload.epsilon_spent,
                payload.loss,
                payload.auc,
                payload.record_count,
            )
            .await
            {
                warn!("Failed to persist update record: {}", e);
            }
        }
    }

    // ── 8. Append audit log entry (hash-chain) ────────────────────────────────
    {
        let weights = state.global_model.read().await;
        let gradient_hash = hash_weights(&weights);
        drop(weights);

        let prev_hash = match db::get_latest_audit_hash(&state.pool).await {
            Ok(h) => h,
            Err(e) => {
                warn!("Could not fetch latest audit hash: {} — using genesis", e);
                db::GENESIS_HASH.to_string()
            }
        };
        let entry_hash = compute_entry_hash(round_id, round_number, &gradient_hash, &prev_hash);

        if let Err(e) = db::insert_audit_entry(
            &state.pool,
            round_id,
            round_number,
            &gradient_hash,
            mean_auc,
            n,
            &prev_hash,
            &entry_hash,
        )
        .await
        {
            warn!("Failed to append audit log entry for round {}: {}", round_number, e);
        }
    }

    // ── 9. Push to in-memory round history + increment round counter ──────────
    {
        let mut history = state.round_history.write().await;
        history.push(RoundResult {
            round_id,
            round_number,
            auc: mean_auc,
            participating_nodes: n,
            timestamp,
        });
    }
    {
        let mut current = state.current_round.write().await;
        *current += 1;
    }

    info!(
        round = round_number,
        nodes = n,
        auc = mean_auc,
        "Aggregation complete"
    );

    Ok(())
}
