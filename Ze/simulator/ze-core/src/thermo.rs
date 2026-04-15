/// Ze Thermodynamic Simulator — Level 2 Digital Twin
///
/// Scientific claim (corrected from NOTE-Z4):
///   S_Ze is a Ze information-theoretic entropy (Axiom Z2); it is non-decreasing by
///   construction (Theorem 3.1). S_Boltzmann measures kinetic energy variance.
///   They are NOT identical. The simulation demonstrates that during non-equilibrium
///   thermalization from a cold initial state (v_i = 0), BOTH entropies increase
///   monotonically — supporting the Ze claim that Axiom Z2 captures Second-Law behaviour.
///
/// T-event rule: |v_new − v_pred| / σ > θ_Z  →  τ_Z decrements.
///
/// Reference: Tkemaladze (2026) Ze Vector Theory §3, §4

use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use crate::types::{K_B, TAU_INIT, THETA_Z, ThermoResult};

struct Molecule {
    velocity: f64,
    tau_z: i64,
    v_prediction: f64,
    t_events: u64,
    s_events: u64,
}

impl Molecule {
    fn new(velocity: f64, tau_z: i64) -> Self {
        Self { velocity, tau_z, v_prediction: velocity, t_events: 0, s_events: 0 }
    }

    /// Update velocity; return true if T-event (Axiom Z3: τ_Z decrements on T-event).
    fn update(&mut self, new_velocity: f64, sigma: f64) -> bool {
        let surprise = (new_velocity - self.v_prediction).abs() / sigma.max(1e-9);
        let is_t = surprise > THETA_Z;
        if is_t {
            self.tau_z = (self.tau_z - 1).max(0);
            self.t_events += 1;
        } else {
            self.s_events += 1;
        }
        // Exponential moving average prediction: α = 0.8
        self.v_prediction = 0.8 * self.v_prediction + 0.2 * new_velocity;
        self.velocity = new_velocity;
        is_t
    }
}

/// Pearson correlation between two equal-length vectors.
fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let mean_a = a.iter().sum::<f64>() / n;
    let mean_b = b.iter().sum::<f64>() / n;
    let num: f64 = a.iter().zip(b.iter()).map(|(x, y)| (x - mean_a) * (y - mean_b)).sum();
    let da: f64 = a.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>().sqrt();
    let db: f64 = b.iter().map(|y| (y - mean_b).powi(2)).sum::<f64>().sqrt();
    if da * db < 1e-12 { 0.0 } else { num / (da * db) }
}

/// Spearman rank correlation: more appropriate than Pearson for comparing
/// two monotone series on different scales (S_Ze is information-theoretic,
/// S_Boltzmann is kinetic energy variance — their units are incomparable).
fn spearman(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len();
    let rank = |v: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| v[i].partial_cmp(&v[j]).unwrap());
        let mut r = vec![0.0f64; n];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && (v[idx[j]] - v[idx[i]]).abs() < 1e-15 { j += 1; }
            let avg = (i + j - 1) as f64 / 2.0 + 1.0;
            for k in i..j { r[idx[k]] = avg; }
            i = j;
        }
        r
    };
    let ra = rank(a);
    let rb = rank(b);
    pearson(&ra, &rb)
}

/// Run the Ze thermodynamic simulation.
///
/// Parameters:
/// - `cold_start`: if true, initialise all velocities at v = 0 (required for Second-Law demo).
///   At equilibrium both S_Ze and S_Boltzmann are already maximal and show no monotone trend.
///   Cold start ensures both start at 0 and rise, yielding positive Spearman correlation.
pub fn run_thermo(
    n_molecules: usize,
    n_steps: usize,
    with_demon: bool,
    cold_start: bool,
    seed: u64,
) -> ThermoResult {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let temperature = 1.0_f64;
    let sigma = temperature.sqrt(); // σ = 1.0

    let normal = Normal::new(0.0, sigma).unwrap();

    // Initialise molecules.
    // cold_start = true (scientific default): v_i = 0 → both S_Ze and S_Boltz start at 0.
    // cold_start = false: sample from equilibrium Maxwell-Boltzmann distribution.
    let init_v = |rng: &mut rand::rngs::StdRng| -> f64 {
        if cold_start { 0.0 } else { normal.sample(rng) }
    };

    let mut molecules: Vec<Molecule> = (0..n_molecules)
        .map(|_| Molecule::new(init_v(&mut rng), TAU_INIT))
        .collect();

    let mut history_s_ze    = Vec::with_capacity(n_steps);
    let mut history_s_boltz = Vec::with_capacity(n_steps);
    let mut history_tau     = Vec::with_capacity(n_steps);
    let mut history_t_rate  = Vec::with_capacity(n_steps);
    let mut demon_cost = 0.0_f64;

    for step in 0..n_steps {
        let mut t_count = 0u64;

        // Maxwell's Demon: at step 50 sorts fast molecules, taxing each 1 τ_Z unit.
        // Cost = information gained by sorting = τ_Z units expended (Landauer principle analogue).
        if with_demon && step == 50 {
            let velocities: Vec<f64> = molecules.iter().map(|m| m.velocity).collect();
            let median_v = {
                let mut s = velocities.clone();
                s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                s[s.len() / 2]
            };
            demon_cost = molecules.iter().filter(|m| m.velocity > median_v).count() as f64;
            for mol in molecules.iter_mut().filter(|m| m.velocity > median_v) {
                mol.tau_z = (mol.tau_z - 1).max(0);
            }
        }

        // Ornstein-Uhlenbeck Langevin thermostat (BUG-Z1 fix 2026-04-06):
        //   v(t+1) = α · v(t) + σ · √(1 − α²) · ξ,  ξ ~ N(0,1)
        // α = 0.9, noise_scale = σ · √(1 − 0.81) ≈ 0.436.
        // Stationary variance: noise_scale² / (1 − α²) = σ² = T = 1.0 ✓
        // Fluctuation–dissipation theorem satisfied with purely Gaussian noise.
        const ALPHA: f64 = 0.9;
        let noise_scale = sigma * (1.0 - ALPHA * ALPHA).sqrt();
        for mol in molecules.iter_mut() {
            let new_v = ALPHA * mol.velocity + noise_scale * normal.sample(&mut rng);
            if mol.update(new_v, sigma) { t_count += 1; }
        }

        // S_Ze = k_B · ln(1 + Ω), Ω = cumulative T-events (non-decreasing, Axiom Z2).
        let omega: u64 = molecules.iter().map(|m| m.t_events).sum();
        let s_ze = K_B * (1.0 + omega as f64).ln();

        // S_Boltzmann: 1D ideal gas continuous entropy S = k_B · N · ½ · (1 + ln(2π ⟨v²⟩)).
        // ⟨v²⟩ = instantaneous kinetic temperature estimate.
        // Guard: replace with ε when all v = 0 (cold start t=0) to avoid ln(0).
        let vel_sq_mean = (molecules.iter().map(|m| m.velocity.powi(2)).sum::<f64>()
            / n_molecules as f64).max(1e-9);
        let s_boltz = K_B * n_molecules as f64 * 0.5 * (1.0 + (2.0 * std::f64::consts::PI * vel_sq_mean).ln());

        let tau_mean = molecules.iter().map(|m| m.tau_z as f64).sum::<f64>() / n_molecules as f64;
        let t_rate   = t_count as f64 / n_molecules as f64;

        history_s_ze.push(s_ze);
        history_s_boltz.push(s_boltz);
        history_tau.push(tau_mean);
        history_t_rate.push(t_rate);
    }

    let tau_depletion_rate = (TAU_INIT as f64 - history_tau.last().copied().unwrap_or(TAU_INIT as f64))
        / n_steps as f64;

    let total_t: u64  = molecules.iter().map(|m| m.t_events).sum();
    let total_all: u64 = molecules.iter().map(|m| m.t_events + m.s_events).sum();
    let t_event_rate = total_t as f64 / total_all.max(1) as f64;

    let corr_pearson  = pearson(&history_s_ze, &history_s_boltz);
    let corr_spearman = spearman(&history_s_ze, &history_s_boltz);

    // Thermalization Spearman: measured over the first ~50 steps where both entropies
    // are genuinely increasing during non-equilibrium relaxation.
    // Full-series correlation is misleading because S_Boltzmann plateaus at equilibrium
    // while S_Ze grows monotonically throughout (different physical meanings).
    let thermo_steps = n_steps.min(50);
    let corr_thermo = spearman(&history_s_ze[..thermo_steps], &history_s_boltz[..thermo_steps]);

    ThermoResult {
        n_molecules,
        n_steps,
        with_demon,
        cold_start,
        final_tau_total: history_tau.last().copied().unwrap_or(0.0),
        tau_depletion_rate,
        t_event_rate,
        s_ze_final:    history_s_ze.last().copied().unwrap_or(0.0),
        s_boltz_final: history_s_boltz.last().copied().unwrap_or(0.0),
        correlation:             corr_pearson,
        spearman_correlation:    corr_spearman,
        spearman_thermalization: corr_thermo,
        thermalization_steps:    thermo_steps,
        history_s_ze,
        history_s_boltz,
        history_tau,
        history_t_rate,
        demon_cost: if with_demon { Some(demon_cost) } else { None },
    }
}
