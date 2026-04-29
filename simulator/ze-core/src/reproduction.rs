/// Ze Reproduction — Axiom Z4
///
/// T-event between Ze-observer Z_A and system S instantiates daughter Z_A':
///   τ_Z^(A') = τ_Z^(A) − 1   (already decremented)
///   ρ_A' = Π_i ρ_S Π_i† / p_i  (post-measurement state = definite outcome)
///
/// S-events: no spawn — system stays in superposition.
///
/// Double-slit visibility comparison (Prediction P4, §7.3):
///   Ze prediction:    V_ze = 1 − 2·p_T      (linear in detector efficiency)
///   Standard QM:      V_qm = √(1 − D²)      (V² + D² = 1, where D = p_T)
///   Key distinction:  V_ze < V_qm for 0 < p_T < 1 (Ze predicts LESS visibility than QM).
///   Falsifiability:   precise single-photon experiments could distinguish these curves.
///
/// Reference: Tkemaladze (2026) Ze Vector Theory §7

use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, WeightedIndex};
use serde::{Deserialize, Serialize};
use crate::types::THETA_Q;

const DS_DIM: usize = 2; // double-slit: 2 outcomes (slit 1, slit 2)

/// Result for a single Ze-chain simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainResult {
    pub strategy: String,
    pub chain_depth: usize,
    pub t_events: usize,
    pub s_events: usize,
    pub t_rate: f64,
    pub history_tau: Vec<i64>,
}

/// Aggregate result over N chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReproductionResult {
    pub tau0: i64,
    pub n_chains: usize,
    pub dim: usize,
    pub born_depth_mean: f64,
    pub uniform_depth_mean: f64,
    pub born_t_rate_mean: f64,
    pub uniform_t_rate_mean: f64,
    pub born_first_chain: ChainResult,
    pub uniform_first_chain: ChainResult,
    /// Double-slit visibility data: list of (p_T, V_ze, V_qm).
    ///   p_T   = detector efficiency / measured T-event rate
    ///   V_ze  = Ze prediction:  1 − 2·p_T    (linear, clamped to [0,1])
    ///   V_qm  = QM prediction:  √(1 − p_T²)  (complementarity bound, V² + D² = 1)
    /// Ze predicts strictly lower visibility than QM for all 0 < p_T < 1.
    pub double_slit_visibility: Vec<(f64, f64, f64)>,
}

/// Generate Haar-random Born probabilities (BUG-Z2 fix 2026-04-06: Gaussian components).
fn haar_state(dim: usize, rng: &mut impl Rng) -> Vec<f64> {
    use rand_distr::{Normal, Distribution};
    let gauss = Normal::new(0.0_f64, 1.0).expect("valid normal");
    let re: Vec<f64> = (0..dim).map(|_| gauss.sample(rng)).collect();
    let im: Vec<f64> = (0..dim).map(|_| gauss.sample(rng)).collect();
    let norm_sq: f64 = re.iter().zip(&im).map(|(r, i)| r*r + i*i).sum::<f64>().max(1e-24);
    re.iter().zip(&im).map(|(r, i)| (r*r + i*i) / norm_sq).collect()
}

fn sample(probs: &[f64], rng: &mut impl Rng) -> usize {
    WeightedIndex::new(probs).unwrap().sample(rng)
}

fn run_chain(tau0: i64, dim: usize, uniform: bool, rng: &mut impl Rng) -> ChainResult {
    let strategy = if uniform { "uniform".into() } else { "born".into() };
    let mut tau = tau0;
    let mut t_events = 0usize;
    let mut s_events = 0usize;
    let mut depth = 0usize;
    let mut history = vec![tau];

    loop {
        if tau <= 0 { break; }
        let born = haar_state(dim, rng);
        let q: Vec<f64> = if uniform {
            vec![1.0 / dim as f64; dim]
        } else {
            born.clone()
        };
        let outcome = sample(&born, rng);
        let surprise = -q[outcome].max(1e-15_f64).log2();
        let is_t = surprise > THETA_Q;
        if is_t {
            tau -= 1;
            t_events += 1;
            depth += 1;
        } else {
            s_events += 1;
        }
        history.push(tau);
        if tau <= 0 { break; }
    }

    let total = (t_events + s_events).max(1);
    ChainResult {
        strategy,
        chain_depth: depth,
        t_events,
        s_events,
        t_rate: t_events as f64 / total as f64,
        history_tau: history,
    }
}

/// Ze visibility formula: V = 1 − 2·p_T  (clamped to [0, 1]).
///
/// Contrast with QM complementarity: V_qm = √(1 − D²) where D = p_T (distinguishability).
/// Both curves agree at p_T = 0 (V = 1) and p_T ≥ 0.5 (V_ze = 0; V_qm → 0 at p_T = 1).
/// For 0 < p_T < 0.5: V_ze = 1 − 2p_T < √(1 − p_T²) = V_qm.
/// Example at p_T = 0.3: V_ze = 0.40, V_qm = √(1−0.09) ≈ 0.954.
pub fn ze_visibility(p_t: f64) -> f64 {
    (1.0 - 2.0 * p_t).clamp(0.0, 1.0)
}

/// QM complementarity visibility: V_qm = √(1 − D²) where D = p_T.
/// Derived from V² + D² = 1 (Wootters-Zurek / Englert 1996).
fn qm_visibility(p_t: f64) -> f64 {
    (1.0 - p_t * p_t).max(0.0).sqrt()
}

/// Double-slit simulation comparing Ze and QM visibility predictions.
///
/// Model: detector efficiency `strength` ∈ [0, 1].
/// Each measurement trial is a Bernoulli(strength) event.
/// When detector fires → T-event for the Ze-observer (which-path info obtained).
/// p_T = empirical fraction of T-events ≈ strength.
///
/// BUG-Z3 fix (2026-04-06): previous model used fixed surprise = −log₂(0.5) = 1.0 < θ_Q = 1.5
/// → fires was always false → p_T = 0 → V = 1 always. Formula never tested.
/// Fix: model firing as Bernoulli(strength), giving p_T ≈ strength.
fn double_slit(n_trials: usize, rng: &mut impl Rng) -> Vec<(f64, f64, f64)> {
    let strengths: Vec<f64> = (0..=10).map(|i| i as f64 / 10.0).collect();
    strengths.iter().map(|&strength| {
        let mut t = 0usize;
        for _ in 0..n_trials {
            let _ = haar_state(DS_DIM, rng); // advance RNG by same amount for reproducibility
            if rng.gen::<f64>() < strength { t += 1; }
        }
        let p_t = t as f64 / n_trials as f64;
        (p_t, ze_visibility(p_t), qm_visibility(p_t))
    }).collect()
}

pub fn run_reproduction(tau0: i64, n_chains: usize, dim: usize, seed: u64) -> ReproductionResult {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut born_depths    = Vec::with_capacity(n_chains);
    let mut uniform_depths = Vec::with_capacity(n_chains);
    let mut born_rates     = Vec::with_capacity(n_chains);
    let mut uniform_rates  = Vec::with_capacity(n_chains);
    let mut first_born     = None;
    let mut first_uniform  = None;

    for i in 0..n_chains {
        let b = run_chain(tau0, dim, false, &mut rng);
        let u = run_chain(tau0, dim, true,  &mut rng);
        born_depths.push(b.chain_depth);
        uniform_depths.push(u.chain_depth);
        born_rates.push(b.t_rate);
        uniform_rates.push(u.t_rate);
        if i == 0 { first_born = Some(b); first_uniform = Some(u); }
    }

    let ds = double_slit(2000, &mut rng);

    let mean  = |v: &[usize]| v.iter().sum::<usize>() as f64 / v.len() as f64;
    let fmean = |v: &[f64]|   v.iter().sum::<f64>()   / v.len() as f64;

    ReproductionResult {
        tau0,
        n_chains,
        dim,
        born_depth_mean:     mean(&born_depths),
        uniform_depth_mean:  mean(&uniform_depths),
        born_t_rate_mean:    fmean(&born_rates),
        uniform_t_rate_mean: fmean(&uniform_rates),
        born_first_chain:    first_born.unwrap(),
        uniform_first_chain: first_uniform.unwrap(),
        double_slit_visibility: ds,
    }
}
