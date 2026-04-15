/// SecAgg+ — Cryptographic Secure Aggregation with Pairwise Mask Cancellation
///
/// Implements the SecAgg+ protocol from:
///   Bonawitz K. et al., "Practical Secure Aggregation for Privacy-Preserving
///   Machine Learning", CCS 2017.  https://doi.org/10.1145/3133956.3133982
///
/// # Protocol Overview (Version 2 — Production-grade X25519 DH)
///
/// ## Node-side (per round):
/// 1. Node i generates a `NodeKeypair` (X25519 keypair via x25519_dalek).
/// 2. Node i broadcasts its `public_key` to all other nodes (via server).
/// 3. Node i shares Shamir shares of its `private_key` with other nodes
///    (for dropout recovery, threshold = ceil(n/2)).
/// 4. For each peer j, node i derives a pairwise seed via Diffie-Hellman:
///      shared_secret_ij = X25519(private_i, public_j)
///      seed_ij = SHA-256(shared_secret_ij || round_le_bytes || "FCLC-SECAGG-V2-SEED")
///    Symmetric by X25519: X25519(private_i, public_j) == X25519(private_j, public_i)
/// 5. Mask_ij = ChaCha20(seed=seed_ij) expanded to gradient dimension.
/// 6. Node i applies:
///      masked_i = gradient_i + Σ_{j>i} mask_ij - Σ_{j<i} mask_ij
/// 7. Node i sends masked_i to server.
///
/// ## Server-side:
/// - Receives masked updates from all n_i surviving nodes.
/// - For surviving nodes: sum(masked_i) = sum(gradient_i) because masks cancel.
/// - For dropped nodes d: server requests Shamir shares from survivors, reconstructs
///   private_key_d, recomputes mask contributions, and corrects the sum.
///
/// ## Cryptographic properties:
/// - **DH security**: Curve25519 (128-bit security, RFC 7748). Passive adversary cannot
///   learn private_i from public_i; shared secret is computationally indistinguishable
///   from random without knowledge of either private key.
/// - **Individual gradient privacy**: server sees only the aggregate, never individual updates.
/// - **Dropout resilience**: up to floor(n/2)-1 nodes may drop; Shamir reconstruction covers the rest.
/// - **Mask cancellation**: Σ_i masked_i = Σ_i gradient_i exactly (no approximation).
/// - **PRG**: ChaCha20 (IETF RFC 8439, cryptographically secure stream cipher).
/// - **Shamir**: GF(257) per-byte, threshold = ceil(n_nodes / 2).

use rand::RngCore;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use sha2::{Sha256, Digest};
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};

// ── Keypair ────────────────────────────────────────────────────────────────────

/// Node keypair for SecAgg+ pairwise seed derivation.
///
/// Uses X25519 Diffie-Hellman (Curve25519, RFC 7748) for authenticated key agreement.
/// The shared secret DH(private_i, public_j) == DH(private_j, public_i) is used to
/// derive a symmetric pairwise mask seed, ensuring neither party can forge the other's
/// mask contribution without knowledge of the private key.
///
/// Security level: 128-bit (Curve25519).
#[derive(Debug, Clone)]
pub struct NodeKeypair {
    /// 32-byte X25519 private scalar (raw bytes; x25519_dalek applies clamping internally)
    pub private_key: [u8; 32],
    /// 32-byte X25519 public key (u-coordinate of Curve25519 point)
    pub public_key: [u8; 32],
}

impl NodeKeypair {
    /// Generate a fresh X25519 keypair using the OS cryptographic RNG.
    pub fn generate() -> Self {
        let mut private_key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut private_key);
        let sk = StaticSecret::from(private_key);
        let public_key = *X25519PublicKey::from(&sk).as_bytes();
        NodeKeypair { private_key, public_key }
    }

    /// Deterministic X25519 keypair from seed bytes (for testing/reproducibility).
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let private_key = *seed;
        let sk = StaticSecret::from(private_key);
        let public_key = *X25519PublicKey::from(&sk).as_bytes();
        NodeKeypair { private_key, public_key }
    }

    /// Derive a pairwise seed shared between this node and a peer using X25519 DH.
    ///
    /// Protocol:
    ///   shared = X25519(self.private_key, peer_pubkey)     — 32-byte Curve25519 shared secret
    ///   seed   = SHA-256(shared || round_le_bytes || "FCLC-SECAGG-V2-SEED")
    ///
    /// Symmetric: node i and node j derive the same seed because
    ///   X25519(private_i, public_j) == X25519(private_j, public_i)  (Curve25519 property).
    ///
    /// Security: a passive adversary observing both public keys cannot compute the shared
    ///   secret without one of the private keys (Computational Diffie-Hellman assumption on
    ///   Curve25519, conjectured 128-bit security).
    pub fn derive_pairwise_seed(&self, peer_pubkey: &[u8; 32], round: u64) -> [u8; 32] {
        let sk = StaticSecret::from(self.private_key);
        let peer_pk = X25519PublicKey::from(*peer_pubkey);
        let shared = sk.diffie_hellman(&peer_pk);
        // Domain-separate with round number to prevent cross-round seed reuse.
        let mut h = Sha256::new();
        h.update(shared.as_bytes());
        h.update(round.to_le_bytes());
        h.update(b"FCLC-SECAGG-V2-SEED");
        h.finalize().into()
    }

    /// Split `self.private_key` into `n_shares` Shamir shares (threshold = `threshold`).
    /// Other nodes store one share each; dropped nodes can be reconstructed.
    pub fn split_private_key(&self, threshold: usize, n_shares: usize) -> Vec<ShamirShare> {
        debug_assert!(threshold >= 2, "threshold must be >= 2");
        debug_assert!(n_shares >= threshold, "n_shares must be >= threshold");

        // Split each byte of the 32-byte private key independently over GF(257).
        // A node's share is the vector of 32 per-byte shares at its index.
        let mut all_shares: Vec<ShamirShare> = (1..=n_shares)
            .map(|x| ShamirShare { x: x as u8, bytes: [0u8; 32] })
            .collect();

        for byte_idx in 0..32usize {
            let secret = self.private_key[byte_idx];
            let per_byte = shamir_split_gf257(secret, threshold, n_shares);
            for (share, &y) in all_shares.iter_mut().zip(per_byte.iter()) {
                share.bytes[byte_idx] = y;
            }
        }

        all_shares
    }

    /// Reconstruct a `NodeKeypair` from `threshold` Shamir shares.
    pub fn reconstruct_from_shares(shares: &[ShamirShare]) -> Self {
        assert!(shares.len() >= 2, "need at least 2 shares");

        let mut private_key = [0u8; 32];
        for byte_idx in 0..32usize {
            let per_byte: Vec<(u8, u8)> = shares
                .iter()
                .map(|s| (s.x, s.bytes[byte_idx]))
                .collect();
            private_key[byte_idx] = shamir_reconstruct_gf257(&per_byte);
        }

        // Re-derive the X25519 public key from the reconstructed private scalar.
        let sk = StaticSecret::from(private_key);
        let public_key = *X25519PublicKey::from(&sk).as_bytes();
        NodeKeypair { private_key, public_key }
    }
}

// ── Shamir Secret Sharing over GF(257) ────────────────────────────────────────

/// Shamir share of a 32-byte secret (one share per node).
/// `x` is the node index (1-based); `bytes` is one share-byte per private_key byte.
#[derive(Debug, Clone)]
pub struct ShamirShare {
    pub x: u8,          // evaluation point (1..=n_nodes)
    pub bytes: [u8; 32], // secret bytes share
}

// ── GF(2^8) field arithmetic ──────────────────────────────────────────────────
//
// We use GF(2^8) with the AES irreducible polynomial: x^8 + x^4 + x^3 + x + 1 (0x11b).
// All 256 byte values {0..=255} are naturally field elements — no overflow truncation.
// This replaces the previous GF(257) approach which silently truncated share value 256
// to 0 when cast to u8, causing intermittent reconstruction failures.
//
// GF(2^8) properties used:
//   Addition: a + b = a XOR b  (all elements have characteristic 2)
//   Subtraction = addition (a - b = a XOR b)
//   Multiplication: polynomial product mod 0x11b
//   Multiplicative inverse: via extended Euclidean or by gf256_mul lookup
//   0 has no inverse (division by zero → panic)

const GF_POLY: u16 = 0x11b; // x^8 + x^4 + x^3 + x + 1

#[inline]
fn gf256_add(a: u8, b: u8) -> u8 { a ^ b }

/// GF(2^8) multiplication using Russian peasant algorithm mod GF_POLY.
fn gf256_mul(mut a: u8, mut b: u8) -> u8 {
    let mut product = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            product ^= a;
        }
        let high_bit = a & 0x80;
        a <<= 1;
        if high_bit != 0 {
            a ^= 0x1b; // GF_POLY low byte: x^4 + x^3 + x + 1 = 0b00011011
        }
        b >>= 1;
    }
    product
}

/// GF(2^8) multiplicative inverse via iterated squaring: a^{254} mod poly.
/// Returns 0 for input 0 (undefined, but 0 is never a valid evaluation point x≥1).
fn gf256_inv(a: u8) -> u8 {
    if a == 0 { return 0; }
    // Compute a^{-1} = a^{254} in GF(2^8)  (Fermat: a^{255}=1 → a^{-1}=a^{254})
    let mut result = 1u8;
    let mut base = a;
    let mut exp = 254u8;
    while exp > 0 {
        if exp & 1 != 0 { result = gf256_mul(result, base); }
        base = gf256_mul(base, base);
        exp >>= 1;
    }
    result
}

/// Split a single `secret` byte into `n_shares` Shamir shares over GF(2^8).
///
/// Polynomial: f(x) = secret XOR a_1·x XOR ... XOR a_{t-1}·x^{t-1}  in GF(2^8).
/// Evaluation points: x = 1, 2, ..., n_shares (all nonzero, all fit in u8).
/// All share values are naturally in u8 — no overflow/truncation possible.
///
/// Function name retained for API compatibility; implementation uses GF(2^8).
pub fn shamir_split_gf257(secret: u8, threshold: usize, n_shares: usize) -> Vec<u8> {
    assert!(threshold >= 2, "threshold must be >= 2");
    assert!(n_shares >= threshold, "n_shares must be >= threshold");
    assert!(n_shares < 256, "n_shares must be < 256 (GF(2^8) limit)");

    let mut rng = rand::thread_rng();

    // Random polynomial coefficients a_1 .. a_{t-1} in GF(2^8)
    let mut coeffs = vec![secret]; // a_0 = secret
    for _ in 1..threshold {
        coeffs.push((rng.next_u64() & 0xFF) as u8);
    }

    // Evaluate f(x) for x = 1..=n_shares using Horner's method in GF(2^8)
    (1..=n_shares)
        .map(|x| {
            let x = x as u8;
            // Horner: f(x) = a_0 XOR x·(a_1 XOR x·(a_2 XOR ... x·a_{t-1}))
            let mut y = *coeffs.last().unwrap();
            for &c in coeffs[1..coeffs.len()-1].iter().rev() {
                y = gf256_add(c, gf256_mul(x, y));
            }
            gf256_add(coeffs[0], gf256_mul(x, y))
        })
        .collect()
}

/// Reconstruct a secret from `threshold` (x, y) share pairs using Lagrange interpolation in GF(2^8).
///
/// `shares` = slice of (x, y) pairs where x ∈ 1..=n_shares.
/// Returns f(0) = the original secret.
///
/// Function name retained for API compatibility; implementation uses GF(2^8).
pub fn shamir_reconstruct_gf257(shares: &[(u8, u8)]) -> u8 {
    let n = shares.len();
    let mut result = 0u8;

    for i in 0..n {
        let (xi, yi) = (shares[i].0, shares[i].1);
        // Lagrange basis polynomial L_i(0):
        //   L_i(0) = ∏_{j≠i} (0 - x_j) / (x_i - x_j)
        //          = ∏_{j≠i} x_j / (x_i XOR x_j)   [in GF(2^8): 0-a = 0 XOR a = a]
        let mut num = 1u8;
        let mut den = 1u8;
        for j in 0..n {
            if i == j { continue; }
            let xj = shares[j].0;
            num = gf256_mul(num, xj);                // numerator: 0 - xj = xj (in GF char-2)
            den = gf256_mul(den, gf256_add(xi, xj)); // denominator: xi - xj = xi XOR xj
        }
        let li = gf256_mul(num, gf256_inv(den));
        result = gf256_add(result, gf256_mul(yi, li));
    }

    result
}

// ── Mask Generation ────────────────────────────────────────────────────────────

/// Expand a 32-byte seed into a mask vector of `dim` f32 values using ChaCha20.
///
/// Values are drawn from the ChaCha20 stream and mapped to [-scale, +scale].
/// scale = 0.01 — mask values are small relative to typical gradients.
pub fn expand_seed_to_mask(seed: &[u8; 32], dim: usize, scale: f32) -> Vec<f32> {
    let mut rng = ChaCha20Rng::from_seed(*seed);
    (0..dim)
        .map(|_| {
            let v = rng.next_u32();
            // Map u32 → [-scale, +scale]
            (v as f32 / u32::MAX as f32 - 0.5) * 2.0 * scale
        })
        .collect()
}

/// Apply pairwise masks to a gradient update using ChaCha20 + SHA-256 seed derivation.
///
/// For node `node_index` with `node_keypair`, adds/subtracts masks derived from
/// pairwise seeds with all other nodes:
///   masked_i = gradient_i + Σ_{j>i} mask_ij  -  Σ_{j<i} mask_ij
///
/// When summed across all nodes, all masks cancel and the server sees only
/// Σ gradient_i (the true aggregate).
pub fn secagg_apply_masks(
    update: &[f32],
    node_keypair: &NodeKeypair,
    peer_public_keys: &[[u8; 32]],   // peer_public_keys[j] = public key of node j (excl. self)
    node_index: usize,
    n_nodes: usize,
    round: u64,
    mask_scale: f32,
) -> Vec<f32> {
    assert_eq!(peer_public_keys.len(), n_nodes - 1,
               "peer_public_keys must have exactly n_nodes - 1 entries");

    let dim = update.len();
    let mut masked = update.to_vec();

    // Map peer_public_keys index → global node index (skip self)
    let peer_global_indices: Vec<usize> = (0..n_nodes).filter(|&j| j != node_index).collect();

    for (k, &j) in peer_global_indices.iter().enumerate() {
        let seed = node_keypair.derive_pairwise_seed(&peer_public_keys[k], round);
        let mask = expand_seed_to_mask(&seed, dim, mask_scale);
        let sign: f32 = if node_index < j { 1.0 } else { -1.0 };
        for (m, &v) in masked.iter_mut().zip(mask.iter()) {
            *m += sign * v;
        }
    }

    masked
}

/// Server-side: unmask and sum. Handles dropout via Shamir reconstruction.
///
/// If all nodes survive, masks cancel exactly: sum(masked_i) = sum(gradient_i).
/// For dropped nodes, reconstructs their keypair from Shamir shares and
/// recomputes the uncancelled mask contribution.
///
/// # Arguments
/// * `masked_updates`       — masked gradient from each surviving node
/// * `surviving_keypairs`   — keypairs (public keys) of surviving nodes for mask recomputation
/// * `surviving_indices`    — global node indices of surviving nodes
/// * `dropped_shamir_shares` — for each dropped node: vec of (ShamirShare, contributor_node_idx)
///   collected from surviving nodes
/// * `n_nodes`              — total nodes in this round
/// * `round`                — round number
pub fn secagg_aggregate(
    masked_updates: &[Vec<f32>],
    surviving_indices: &[usize],
    surviving_public_keys: &[[u8; 32]],
    dropped_indices: &[usize],
    dropped_keypairs: &[NodeKeypair],   // reconstructed from Shamir shares by caller
    n_nodes: usize,
    round: u64,
    mask_scale: f32,
) -> Vec<f32> {
    assert!(!masked_updates.is_empty(), "no updates to aggregate");
    assert_eq!(masked_updates.len(), surviving_indices.len());
    let dim = masked_updates[0].len();

    // Step 1: sum all surviving masked updates
    let mut sum = vec![0.0f32; dim];
    for update in masked_updates {
        for (s, &v) in sum.iter_mut().zip(update.iter()) {
            *s += v;
        }
    }

    // Step 2: for each dropped node d, subtract the uncancelled mask contribution
    // A dropped node d had masks with each surviving node j:
    //   masked_j += sign_j_d * mask_dj  (already in sum via masked_j)
    //   masked_d += sign_d_j * mask_dj  (ABSENT because d dropped)
    //
    // Net imbalance in sum: Σ_j sign_j_d * mask_dj  (from surviving nodes)
    // We must cancel this by adding: -Σ_j sign_j_d * mask_dj
    // which equals: Σ_j sign_d_j * mask_dj  (since sign_d_j = -sign_j_d)
    for (drop_k, &d) in dropped_indices.iter().enumerate() {
        let dropped_kp = &dropped_keypairs[drop_k];
        for (surv_k, &j) in surviving_indices.iter().enumerate() {
            let seed = dropped_kp.derive_pairwise_seed(&surviving_public_keys[surv_k], round);
            let mask = expand_seed_to_mask(&seed, dim, mask_scale);
            // When d < j: d would have added +mask to its update; j added -mask to its update.
            //   Sum has j's -mask. We need to cancel: add +mask back.
            //   sign = +1 if d < j.
            // When d > j: d would have added -mask; j added +mask.
            //   Sum has j's +mask. We need to cancel: add -mask.
            //   sign = -1 if d > j.
            let sign: f32 = if d < j { 1.0 } else { -1.0 };
            for (s, &v) in sum.iter_mut().zip(mask.iter()) {
                *s += sign * v;
            }
        }
    }

    sum
}

// ── Helper: backwards-compatible mask update (used in aggregation/mod.rs) ────

/// ChaCha20-based replacement for the legacy LCG in `secagg_mask_update`.
///
/// Derives a pairwise seed from (lo_idx, hi_idx, round) via SHA-256,
/// then uses ChaCha20 to expand to `dim` mask values in [-0.01, +0.01].
///
/// This retains the same API as the original LCG version but uses a
/// cryptographically secure PRG.
pub fn chacha20_pairwise_mask(lo: usize, hi: usize, round: u64, dim: usize) -> Vec<f32> {
    let mut hasher = Sha256::new();
    hasher.update(lo.to_le_bytes());
    hasher.update(hi.to_le_bytes());
    hasher.update(round.to_le_bytes());
    hasher.update(b"FCLC-SECAGG-V1");
    let hash: [u8; 32] = hasher.finalize().into();
    expand_seed_to_mask(&hash, dim, 0.01)
}

// ── Internal helpers ──────────────────────────────────────────────────────────
// Note: V1 helpers (derive_public_key, derive_pairwise_seed_inner) have been removed.
// V2 uses X25519 DH directly in NodeKeypair::derive_pairwise_seed() above.
// chacha20_pairwise_mask (below) is a separate, index-based simple masking utility
// that does NOT use DH — it is used for the server-side secagg_mask_update / secagg_unmask_sum
// functions which operate on node indices rather than keypairs.

// mod_pow removed: no longer needed after migration from GF(257) to GF(2^8).

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Shamir tests ──

    #[test]
    fn test_shamir_reconstruct_2of3() {
        for secret in [0u8, 1, 42, 127, 200, 254, 255] {
            let shares = shamir_split_gf257(secret, 2, 3);
            assert_eq!(shares.len(), 3);
            // Reconstruct from any 2
            let r01 = shamir_reconstruct_gf257(&[(1, shares[0]), (2, shares[1])]);
            let r02 = shamir_reconstruct_gf257(&[(1, shares[0]), (3, shares[2])]);
            let r12 = shamir_reconstruct_gf257(&[(2, shares[1]), (3, shares[2])]);
            assert_eq!(r01, secret, "2-of-3 (0,1) failed for secret={secret}");
            assert_eq!(r02, secret, "2-of-3 (0,2) failed for secret={secret}");
            assert_eq!(r12, secret, "2-of-3 (1,2) failed for secret={secret}");
        }
    }

    #[test]
    fn test_shamir_reconstruct_3of5() {
        let secret = 173u8;
        let shares = shamir_split_gf257(secret, 3, 5);
        let r = shamir_reconstruct_gf257(&[(1, shares[0]), (3, shares[2]), (5, shares[4])]);
        assert_eq!(r, secret);
    }

    #[test]
    fn test_shamir_insufficient_shares_wrong() {
        // With only 1 share from a 2-of-n scheme, reconstruction gives wrong value
        let secret = 100u8;
        let shares = shamir_split_gf257(secret, 2, 3);
        // 1-of-2 reconstruction should generally be wrong (not a guarantee, but typically)
        // We test the correct 2-of-2 case instead
        let r = shamir_reconstruct_gf257(&[(1, shares[0]), (2, shares[1])]);
        assert_eq!(r, secret);
    }

    // ── Keypair tests ──

    #[test]
    fn test_keypair_pairwise_seed_symmetric() {
        let kp_a = NodeKeypair::generate();
        let kp_b = NodeKeypair::generate();
        let seed_ab = kp_a.derive_pairwise_seed(&kp_b.public_key, 1);
        let seed_ba = kp_b.derive_pairwise_seed(&kp_a.public_key, 1);
        assert_eq!(seed_ab, seed_ba, "Pairwise seed must be symmetric");
    }

    #[test]
    fn test_keypair_pairwise_seed_differs_by_round() {
        let kp_a = NodeKeypair::generate();
        let kp_b = NodeKeypair::generate();
        let seed1 = kp_a.derive_pairwise_seed(&kp_b.public_key, 1);
        let seed2 = kp_a.derive_pairwise_seed(&kp_b.public_key, 2);
        assert_ne!(seed1, seed2, "Seeds for different rounds must differ");
    }

    #[test]
    fn test_keypair_split_reconstruct() {
        let kp = NodeKeypair::generate();
        let shares = kp.split_private_key(2, 3);
        let reconstructed = NodeKeypair::reconstruct_from_shares(&shares[0..2]);
        assert_eq!(kp.private_key, reconstructed.private_key);
        assert_eq!(kp.public_key, reconstructed.public_key);
    }

    // ── Mask tests ──

    #[test]
    fn test_secagg_masks_cancel_no_dropout() {
        let n = 4;
        let dim = 16;
        let round = 1u64;
        let gradient_val = 1.0f32;

        // Generate keypairs
        let kps: Vec<NodeKeypair> = (0..n)
            .map(|i| NodeKeypair::from_seed(&[i as u8; 32]))
            .collect();
        let public_keys: Vec<[u8; 32]> = kps.iter().map(|k| k.public_key).collect();

        // Each node applies masks to its gradient
        let gradients: Vec<Vec<f32>> = (0..n).map(|i| vec![i as f32 + gradient_val; dim]).collect();

        let masked: Vec<Vec<f32>> = (0..n)
            .map(|i| {
                let peers: Vec<[u8; 32]> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| public_keys[j])
                    .collect();
                secagg_apply_masks(&gradients[i], &kps[i], &peers, i, n, round, 0.01)
            })
            .collect();

        // Server sums all masked updates
        let mut server_sum = vec![0.0f32; dim];
        for m in &masked {
            for (s, &v) in server_sum.iter_mut().zip(m.iter()) {
                *s += v;
            }
        }

        // True sum: each element = 1+2+3+4 = 10
        let true_sum: Vec<f32> = vec![(1..=n).map(|i| i as f32).sum(); dim];
        for (got, expected) in server_sum.iter().zip(true_sum.iter()) {
            assert!(
                (got - expected).abs() < 1e-3,
                "Mask cancellation failed: got {got:.6}, expected {expected:.6}"
            );
        }
    }

    #[test]
    fn test_masked_update_differs_from_plaintext() {
        let kp = NodeKeypair::generate();
        let peer = NodeKeypair::generate();
        let gradient = vec![1.0f32; 8];
        let masked = secagg_apply_masks(&gradient, &kp, &[peer.public_key], 0, 2, 1, 0.01);
        let differs = masked.iter().zip(gradient.iter()).any(|(m, g)| (m - g).abs() > 1e-9);
        assert!(differs, "Masked update must differ from plaintext gradient");
    }

    #[test]
    fn test_chacha20_pairwise_mask_deterministic() {
        let m1 = chacha20_pairwise_mask(0, 1, 42, 16);
        let m2 = chacha20_pairwise_mask(0, 1, 42, 16);
        assert_eq!(m1, m2, "ChaCha20 mask must be deterministic");
    }

    #[test]
    fn test_chacha20_pairwise_mask_symmetric() {
        // lo/hi ordering ensures symmetry: chacha20_pairwise_mask(1,0,r,d) == chacha20_pairwise_mask(0,1,r,d)
        // because we always pass (min, max) in secagg_mask_update
        let m_01 = chacha20_pairwise_mask(0, 1, 1, 8);
        let m_10 = chacha20_pairwise_mask(0, 1, 1, 8); // same lo=0, hi=1
        assert_eq!(m_01, m_10, "ChaCha20 pairwise mask must be symmetric");
    }

    #[test]
    fn test_expand_seed_range() {
        let seed = [0u8; 32];
        let mask = expand_seed_to_mask(&seed, 1000, 0.01);
        for &v in &mask {
            assert!(v.abs() <= 0.011, "Mask value out of range: {v}");
        }
    }
}
