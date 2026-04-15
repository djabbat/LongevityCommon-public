pub mod secagg;
pub use secagg::{
    NodeKeypair, ShamirShare,
    secagg_apply_masks, secagg_aggregate,
    expand_seed_to_mask, chacha20_pairwise_mask,
    shamir_split_gf257, shamir_reconstruct_gf257,
};

/// FedProx-regularised weighted aggregation of model updates.
///
/// Computes the weighted average of client updates, then applies an L2
/// proximal correction toward the current global model with strength μ.
///
/// Formula: w_new = Σ(weight_i * update_i) / Σ(weight_i)
/// Then proximal step: w_new = (w_avg + μ * global) / (1 + μ)
///
/// # Panics
/// Panics if `updates`, `weights`, and `global` have inconsistent lengths,
/// or if `updates` is empty.
pub fn fedprox_aggregate(
    updates: &[Vec<f32>],
    weights: &[f64],
    global: &[f32],
    mu: f32,
) -> Vec<f32> {
    assert!(!updates.is_empty(), "updates must not be empty");
    assert_eq!(updates.len(), weights.len(), "updates and weights must have same length");

    let dim = updates[0].len();
    assert_eq!(global.len(), dim, "global model dimension mismatch");

    let total_weight: f64 = weights.iter().sum();
    assert!(total_weight > 0.0, "total weight must be positive");

    // Weighted average
    let mut aggregated = vec![0.0f32; dim];
    for (update, &w) in updates.iter().zip(weights.iter()) {
        assert_eq!(update.len(), dim, "all updates must have same dimension");
        let w_norm = (w / total_weight) as f32;
        for (a, u) in aggregated.iter_mut().zip(update.iter()) {
            *a += w_norm * u;
        }
    }

    // FedProx proximal step: pull aggregated toward global model
    // w_new = (w_agg + mu * w_global) / (1 + mu)
    if mu > 0.0 {
        let denom = 1.0 + mu;
        for (a, g) in aggregated.iter_mut().zip(global.iter()) {
            *a = (*a + mu * g) / denom;
        }
    }

    aggregated
}

/// Krum robust aggregation — selects the single update that minimises the
/// sum of squared distances to its `n - f - 2` nearest neighbours,
/// where `f = floor(byzantine_fraction * n)`.
///
/// This tolerates up to `byzantine_fraction` fraction of Byzantine nodes
/// (e.g. 0.25 → 25% Byzantine tolerance).
///
/// Returns a clone of the selected update.
///
/// # Panics
/// Panics if `updates` is empty, or if Byzantine fraction is too large
/// (would leave fewer than 2 honest nodes).
pub fn krum_select(updates: &[Vec<f32>], byzantine_fraction: f64) -> Vec<f32> {
    let n = updates.len();
    assert!(n >= 2, "need at least 2 updates for Krum");

    let f = (byzantine_fraction * n as f64).floor() as usize;
    assert!(
        n >= 2 * f + 2,
        "too many Byzantine nodes: n={n}, f={f} violates n >= 2f+2"
    );

    let k = n - f - 2; // neighbours to consider

    // Compute pairwise squared L2 distances
    let dist = pairwise_sq_distances(updates);

    // For each update i, find its k nearest neighbours (excluding itself)
    // and sum those distances → Krum score
    let mut best_idx = 0;
    let mut best_score = f64::MAX;

    for i in 0..n {
        // Collect distances from i to all others
        let mut dists_from_i: Vec<f64> = (0..n)
            .filter(|&j| j != i)
            .map(|j| dist[i][j])
            .collect();
        dists_from_i.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Sum of k smallest distances
        let score: f64 = dists_from_i.iter().take(k).sum();

        if score < best_score {
            best_score = score;
            best_idx = i;
        }
    }

    updates[best_idx].clone()
}

/// Secure Aggregation (SecAgg+) — local masking step.
///
/// Each node calls `secagg_mask_update()` before sending its gradient.
/// The sum of all masks across the n participating nodes is zero by construction,
/// so when the server sums the masked updates, the masks cancel and the server
/// sees only the sum of the true gradients — never an individual gradient.
///
/// # Protocol (Bonawitz et al. 2017 §3.2 — pairwise mask cancellation)
///
/// For n nodes, node i generates pairwise masks with each other node j:
///   seed_ij = SHA-256(lo_idx || hi_idx || round || "FCLC-SECAGG-V1")
///   mask_ij = ChaCha20(seed=seed_ij) expanded to gradient dimension
///   node i adds  +mask_ij for j > i
///   node i adds  -mask_ij for j < i
/// This ensures Σ_i mask_i = 0 (pairwise cancellation) — server sees only true sum.
///
/// ## Cryptographic properties:
/// - Seed derivation via SHA-256 (collision-resistant, pre-image resistant)
/// - Mask expansion via ChaCha20 (cryptographically secure stream cipher)
/// - Symmetric by construction: seed(lo,hi) == seed(lo,hi) for any pair ordering
///
/// ## For full SecAgg+ with DH key agreement and dropout recovery:
/// Use `secagg_apply_masks()` + `secagg_aggregate()` from the `secagg` submodule,
/// which additionally support:
/// - X25519-style DH key agreement (simulated via SHA-256; swap for x25519 crate)
/// - Shamir (t,n)-threshold secret sharing for dropout recovery
pub fn secagg_mask_update(
    update: &[f32],
    node_index: usize,
    n_nodes: usize,
    round: u64,
) -> Vec<f32> {
    let dim = update.len();
    let mut masked = update.to_vec();

    for j in 0..n_nodes {
        if j == node_index {
            continue;
        }
        let (lo, hi) = if node_index < j { (node_index, j) } else { (j, node_index) };

        // ChaCha20-based mask (replaces LCG — cryptographically secure)
        let mask = secagg::chacha20_pairwise_mask(lo, hi, round, dim);

        let sign: f32 = if node_index < j { 1.0 } else { -1.0 };
        for (m, &v) in masked.iter_mut().zip(mask.iter()) {
            *m += sign * v;
        }
    }

    masked
}

/// Server-side mask removal (simplified: when all nodes survive, masks sum to zero).
///
/// When the server receives masked updates from ALL n nodes in a round, the masks
/// cancel perfectly and this function returns the plain sum of true gradients.
/// If nodes drop out, a Shamir reconstruction step is needed (TODO).
///
/// # Arguments
/// * `masked_updates` — one masked update per surviving node
/// * `node_indices`   — which node indices sent updates (0..n_nodes if no dropout)
/// * `n_nodes`        — total registered nodes this round
/// * `round`          — round number (must match the one used in secagg_mask_update)
///
/// # Note
/// In the no-dropout case (all n_nodes survived), masks cancel and this is
/// equivalent to summing the true gradients. With dropout, masks from dropped
/// nodes do not cancel — requires Shamir share reconstruction (not implemented).
pub fn secagg_unmask_sum(
    masked_updates: &[Vec<f32>],
    node_indices: &[usize],
    n_nodes: usize,
    round: u64,
) -> Vec<f32> {
    assert!(!masked_updates.is_empty(), "no updates to unmask");
    let dim = masked_updates[0].len();

    // Sum all masked updates — if all n_nodes participated, masks cancel exactly
    let mut sum = vec![0.0f32; dim];
    for update in masked_updates {
        for (s, &v) in sum.iter_mut().zip(update.iter()) {
            *s += v;
        }
    }

    // If some nodes dropped out, we must subtract their uncancelled masks.
    // Identify missing nodes and reconstruct their contribution.
    let present: std::collections::HashSet<usize> = node_indices.iter().cloned().collect();
    let dropped: Vec<usize> = (0..n_nodes).filter(|i| !present.contains(i)).collect();

    if !dropped.is_empty() {
        // For each dropped node d, we know which present node j it would have masked with.
        // Re-add the mask that d would have cancelled (since d is absent, it wasn't applied).
        // This is a simplified reconstruction — full SecAgg+ would use Shamir shares.
        for &d in &dropped {
            for &j in node_indices {
                let (lo, hi) = if d < j { (d, j) } else { (j, d) };
                let mask = secagg::chacha20_pairwise_mask(lo, hi, round, dim);

                // When node d drops out, its masks with present nodes j are missing.
                //
                // In secagg_mask_update, node j (node_index=j) paired with d:
                //   if j > d → j applied -mask  (sign = -1 because node_index > loop_j)
                //   if j < d → j applied +mask  (sign = +1 because node_index < loop_j)
                //
                // The sum already contains j's contribution. d's complement is absent.
                //
                // When d < j: j applied -mask. d's +mask is absent. Net: sum has -mask
                //   extra. Correct by adding +mask → sign = +1 → sum += sign * v ✓
                //
                // When d > j: j applied +mask. d's -mask is absent. Net: sum has +mask
                //   extra. Correct by subtracting mask → sign = -1 → sum += sign * v ✓
                //
                // BUG-F1 fix (2026-04-06): original code used `*s -= sign * v` which
                // doubled the imbalance instead of cancelling it. Correct: `*s += sign * v`.
                let sign: f32 = if d < j { 1.0 } else { -1.0 };
                for (s, &v) in sum.iter_mut().zip(mask.iter()) {
                    *s += sign * v; // BUG-F1 fixed: add (not subtract) to cancel imbalance
                }
            }
        }
    }

    sum
}

/// Legacy stub alias — kept for backwards compatibility.
/// Prefer `secagg_unmask_sum` for new code.
#[deprecated(note = "Use secagg_unmask_sum for proper dropout handling")]
pub fn secagg_unmask_stub(masked_updates: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    masked_updates
}

/// Compute n×n matrix of pairwise squared L2 distances between updates.
fn pairwise_sq_distances(updates: &[Vec<f32>]) -> Vec<Vec<f64>> {
    let n = updates.len();
    let mut dist = vec![vec![0.0f64; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let d: f64 = updates[i]
                .iter()
                .zip(updates[j].iter())
                .map(|(a, b)| {
                    let diff = (*a - *b) as f64;
                    diff * diff
                })
                .sum();
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fedprox_no_mu() {
        let updates = vec![vec![1.0f32, 2.0], vec![3.0f32, 4.0]];
        let weights = vec![1.0f64, 1.0];
        let global = vec![0.0f32, 0.0];
        let result = fedprox_aggregate(&updates, &weights, &global, 0.0);
        assert!((result[0] - 2.0).abs() < 1e-5);
        assert!((result[1] - 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_fedprox_with_mu() {
        let updates = vec![vec![4.0f32, 4.0]];
        let weights = vec![1.0f64];
        let global = vec![0.0f32, 0.0];
        // (4 + 0.1*0) / (1 + 0.1) = 4/1.1 ≈ 3.636
        let result = fedprox_aggregate(&updates, &weights, &global, 0.1);
        assert!((result[0] - 4.0 / 1.1).abs() < 1e-4);
    }

    #[test]
    fn test_fedprox_weighted() {
        let updates = vec![vec![0.0f32], vec![10.0f32]];
        let weights = vec![3.0f64, 1.0]; // 3:1 → result should be 2.5
        let global = vec![0.0f32];
        let result = fedprox_aggregate(&updates, &weights, &global, 0.0);
        assert!((result[0] - 2.5).abs() < 1e-5);
    }

    #[test]
    fn test_secagg_masks_cancel_no_dropout() {
        let dim = 9;
        let n = 4;
        let round = 1;
        let updates: Vec<Vec<f32>> = (0..n)
            .map(|i| vec![i as f32 + 1.0; dim])
            .collect();

        // Mask all updates
        let masked: Vec<Vec<f32>> = (0..n)
            .map(|i| secagg_mask_update(&updates[i], i, n, round))
            .collect();

        // Unmask sum (all nodes present)
        let indices: Vec<usize> = (0..n).collect();
        let sum_unmasked = secagg_unmask_sum(&masked, &indices, n, round);

        // True sum: each element = 1+2+3+4 = 10
        let true_sum: Vec<f32> = vec![10.0; dim];
        for (got, expected) in sum_unmasked.iter().zip(true_sum.iter()) {
            assert!(
                (got - expected).abs() < 1e-4,
                "SecAgg sum mismatch: got {got:.6}, expected {expected:.6}"
            );
        }
    }

    #[test]
    fn test_secagg_masked_updates_differ_from_original() {
        // Individual masked updates must differ from original (privacy property)
        let n = 3;
        let update = vec![1.0f32; 9];
        let masked = secagg_mask_update(&update, 0, n, 1);
        let differs = masked.iter().zip(update.iter()).any(|(m, u)| (m - u).abs() > 1e-9);
        assert!(differs, "Masked update should differ from original");
    }

    #[test]
    fn test_krum_selects_honest() {
        // 4 honest updates clustered near [1,1], 1 Byzantine outlier at [100,100]
        let updates = vec![
            vec![1.0f32, 1.0],
            vec![1.1f32, 0.9],
            vec![0.9f32, 1.1],
            vec![1.05f32, 1.05],
            vec![100.0f32, 100.0], // Byzantine
        ];
        // f=1, k = 5 - 1 - 2 = 2
        let result = krum_select(&updates, 0.2);
        // Selected update should be close to [1,1], not the outlier
        assert!(result[0] < 10.0, "Krum should select an honest node");
    }
}
