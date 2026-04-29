//! Ze Quantum Simulator — Level 3 Digital Twin
//!
//! Theorem 5.1 (Conditional Optimality of Born Strategy — corrected from NOTE-Z5):
//!
//! Given that nature samples outcomes from Born rule p_i = |<e_i|psi>|^2,
//! Born strategy q_i = p_i uniquely minimises the T-event rate
//! R(q) = E_p[1(-log2(q_i) > theta_Q)].
//!
//! Proof sketch: outcome i is a T-event iff q_i < 2^(-theta_Q).
//! Born sets q_i = p_i, so high-prob outcomes (p_i >= 2^(-theta_Q)) are never T-events.
//! Any other strategy must under-assign some high-prob outcome, increasing R. QED.
//!
//! Axiomatic note (NOTE-Z5): Born rule is assumed as Axiom QM (external to Ze theory).
//! Theorem 5.1 proves optimality *within* Ze given QM — NOT a derivation of Born rule.
//!
//! Three strategies compared via tau_Z depletion:
//! - Born:      q_i = p_i (Ze-optimal, minimum T-events)
//! - Uniform:   q_i = 1/d (maximum ignorance)
//! - Anti-Born: q_i proportional to (1-p_i)/(d-1) (worst case)
//!
//! Reference: Tkemaladze (2026) Ze Vector Theory §5

use rand::SeedableRng;
use rand_distr::{Distribution, WeightedIndex};
use crate::types::{TAU_INIT, THETA_Q, QuantumResult};

/// Generate a Haar-random pure state as Born probabilities p_i = |c_i|², Σp_i = 1.
///
/// Method (Muller 1959 / Mezzadri 2007):
///   Sample c_i ~ CN(0,1): Re(c_i), Im(c_i) ~ N(0,1) i.i.d.
///   Then p_i = |c_i|² / Σ_j |c_j|².
///   This gives the uniform (Haar) measure on CP^{d-1}.
///
/// BUG-Z2 fix (2026-04-06): previous Uniform([-1,1]) components are NOT Gaussian
/// → normalised vector NOT Haar-distributed.
fn haar_state(dim: usize, rng: &mut impl rand::Rng) -> Vec<f64> {
    use rand_distr::{Normal, Distribution};
    let gauss = Normal::new(0.0_f64, 1.0).expect("valid normal");
    let re: Vec<f64> = (0..dim).map(|_| gauss.sample(rng)).collect();
    let im: Vec<f64> = (0..dim).map(|_| gauss.sample(rng)).collect();
    let norm_sq: f64 = re.iter().zip(&im).map(|(r, i)| r*r + i*i).sum::<f64>().max(1e-24);
    re.iter().zip(&im).map(|(r, i)| (r*r + i*i) / norm_sq).collect()
}

/// Sample outcome index using Born probabilities as the physical distribution.
fn sample_outcome(born_probs: &[f64], rng: &mut impl rand::Rng) -> usize {
    WeightedIndex::new(born_probs).unwrap().sample(rng)
}

/// Born strategy: q_i = p_i. Assigns exactly the Born probability.
fn q_born(born_probs: &[f64]) -> Vec<f64> {
    born_probs.to_vec()
}

/// Uniform strategy: q_i = 1/d. Maximum entropy / maximum ignorance prior.
fn q_uniform(dim: usize) -> Vec<f64> {
    vec![1.0 / dim as f64; dim]
}

/// Anti-Born strategy: q_i ∝ (1 − p_i) / (d−1).
///
/// Note: Σ_i (1 − p_i)/(d−1) = (d − 1)/(d−1) = 1, so this distribution already sums to 1.
/// The renormalisation below is numerically defensive (handles edge cases near p_i ≈ 0 or 1).
fn q_anti_born(born_probs: &[f64]) -> Vec<f64> {
    let d = born_probs.len();
    if d == 1 { return vec![1.0]; }
    let anti: Vec<f64> = born_probs.iter().map(|p| (1.0 - p) / (d - 1) as f64).collect();
    let s: f64 = anti.iter().sum(); // should be ≈ 1.0 analytically
    anti.iter().map(|q| q / s.max(1e-12)).collect()
}

/// T-event: assigned surprise −log₂(q_i) exceeds threshold θ_Q.
fn is_t_event(q: &[f64], idx: usize) -> bool {
    -q[idx].max(1e-15).log2() > THETA_Q
}

/// Theoretical T-event rate under strategy q, given Born physical distribution p.
///
/// R(q) = Σ_i p_i · 1[−log₂(q_i) > θ_Q]
///       = Σ_i p_i · 1[q_i < 2^{−θ_Q}]
///
/// By Theorem 5.1: R(q_born) ≤ R(q_uniform) ≤ R(q_anti_born).
fn theoretical_t_rate(born_probs: &[f64], q: &[f64]) -> f64 {
    born_probs.iter().zip(q.iter()).map(|(p, qi)| {
        if -qi.max(1e-15).log2() > THETA_Q { *p } else { 0.0 }
    }).sum()
}

pub fn run_quantum(dim: usize, n_steps: usize, n_states: usize, seed: u64) -> QuantumResult {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut tau_born    = TAU_INIT;
    let mut tau_uniform = TAU_INIT;
    let mut tau_anti    = TAU_INIT;

    let mut t_born = 0u64;
    let mut t_uniform = 0u64;
    let mut t_anti = 0u64;
    let mut total = 0u64;

    let mut theory_born_acc    = 0.0_f64;
    let mut theory_uniform_acc = 0.0_f64;
    let mut theory_anti_acc    = 0.0_f64;
    let mut theory_count = 0u64;

    let mut history_born    = Vec::with_capacity(n_steps);
    let mut history_uniform = Vec::with_capacity(n_steps);
    let mut history_anti    = Vec::with_capacity(n_steps);

    for _ in 0..n_steps {
        for _ in 0..n_states {
            let born  = haar_state(dim, &mut rng);
            let q_b   = q_born(&born);
            let q_u   = q_uniform(dim);
            let q_a   = q_anti_born(&born);

            // Accumulate theoretical rates (Theorem 5.1 verification)
            theory_born_acc    += theoretical_t_rate(&born, &q_b);
            theory_uniform_acc += theoretical_t_rate(&born, &q_u);
            theory_anti_acc    += theoretical_t_rate(&born, &q_a);
            theory_count += 1;

            // Physical outcome sampled from Born distribution (Axiom QM)
            let outcome = sample_outcome(&born, &mut rng);
            total += 1;

            if is_t_event(&q_b, outcome) { tau_born    = (tau_born    - 1).max(0); t_born    += 1; }
            if is_t_event(&q_u, outcome) { tau_uniform = (tau_uniform - 1).max(0); t_uniform += 1; }
            if is_t_event(&q_a, outcome) { tau_anti    = (tau_anti    - 1).max(0); t_anti    += 1; }
        }

        history_born.push(tau_born);
        history_uniform.push(tau_uniform);
        history_anti.push(tau_anti);
    }

    let tc = theory_count.max(1) as f64;
    let born_theory    = theory_born_acc    / tc;
    let uniform_theory = theory_uniform_acc / tc;
    let anti_theory    = theory_anti_acc    / tc;

    // Theorem 5.1 verification: Born ≤ Uniform ≤ Anti-Born (allow 1e-9 floating-point slack)
    let theorem_holds = born_theory <= uniform_theory + 1e-9
                     && uniform_theory <= anti_theory + 1e-9;

    QuantumResult {
        dim,
        n_steps,
        n_states,
        theta_q: THETA_Q,
        born_tau_final:       tau_born,
        uniform_tau_final:    tau_uniform,
        anti_born_tau_final:  tau_anti,
        born_t_rate:          t_born    as f64 / total.max(1) as f64,
        uniform_t_rate:       t_uniform as f64 / total.max(1) as f64,
        anti_born_t_rate:     t_anti    as f64 / total.max(1) as f64,
        born_theory_rate:     born_theory,
        uniform_theory_rate:  uniform_theory,
        anti_born_theory_rate: anti_theory,
        theorem_5_1_holds:    theorem_holds,
        history_born,
        history_uniform,
        history_anti_born: history_anti,
    }
}
