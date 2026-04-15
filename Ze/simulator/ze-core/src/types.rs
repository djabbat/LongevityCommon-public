use serde::{Deserialize, Serialize};

/// Boltzmann constant (dimensionless units: k_B = 1).
pub const K_B: f64 = 1.0;

/// Initial Ze-budget for every observer.
pub const TAU_INIT: i64 = 2000;

/// Quantum surprise threshold (bits).
///
/// T-event iff −log₂(q_i) > THETA_Q, i.e. the assigned probability q_i < 2^{−THETA_Q}.
/// THETA_Q = 1.5 → threshold probability 2^{−1.5} ≈ 0.354.
/// Outcomes assigned probability below ~35% are Ze-surprising.
pub const THETA_Q: f64 = 1.5;

/// Thermodynamic surprise threshold (σ-units).
///
/// T-event iff |v_new − v_pred| / σ > THETA_Z.
/// At OU stationarity, T-event probability ≈ P(|N(0,1)| > 0.3) ≈ 0.76 per molecule per step.
/// This high baseline rate is intentional: it ensures τ_Z depletes over the simulation window,
/// making the demon cost measurable.
pub const THETA_Z: f64 = 0.3;

/// Output from a thermodynamic simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermoResult {
    pub n_molecules: usize,
    pub n_steps: usize,
    pub with_demon: bool,
    /// Whether molecules were initialised at v = 0 (cold start) or at equilibrium.
    /// Cold start is required for the Second-Law demonstration (NOTE-Z4):
    /// both S_Ze and S_Boltzmann must start at zero and increase monotonically.
    pub cold_start: bool,
    pub final_tau_total: f64,
    pub tau_depletion_rate: f64,
    pub t_event_rate: f64,
    /// S_Ze = k_B · ln(1 + Ω) where Ω = cumulative T-events.
    /// Non-decreasing by construction (Axiom Z2 / Theorem 3.1).
    /// NOTE-Z4: S_Ze is a Ze-theoretic entropy functional. It is NOT identically equal
    /// to S_Boltzmann; it measures Ze-observer information surprise, not kinetic energy.
    /// The simulation tests monotone co-movement during non-equilibrium relaxation.
    pub s_ze_final: f64,
    /// S_Boltzmann = k_B · N · ½ · (1 + ln(2π ⟨v²⟩)), 1D ideal gas (Sackur–Tetrode analogue).
    pub s_boltz_final: f64,
    /// Pearson r between history_s_ze and history_s_boltz.
    /// Positive and large (> 0.8) during cold-start thermalization; near-zero at equilibrium.
    pub correlation: f64,
    /// Spearman ρ over the full time series (robust to scale differences).
    /// Low value is expected: S_Boltzmann equilibrates in ~20-50 steps and fluctuates,
    /// while S_Ze grows monotonically throughout. Full-series correlation is misleading.
    pub spearman_correlation: f64,
    /// Spearman ρ restricted to the first `thermalization_steps` steps.
    /// This captures the co-movement during the non-equilibrium relaxation phase,
    /// where both S_Ze and S_Boltzmann increase — the relevant window for Second Law.
    pub spearman_thermalization: f64,
    /// Number of steps used for spearman_thermalization (min(50, n_steps)).
    pub thermalization_steps: usize,
    pub history_s_ze: Vec<f64>,
    pub history_s_boltz: Vec<f64>,
    pub history_tau: Vec<f64>,
    pub history_t_rate: Vec<f64>,
    pub demon_cost: Option<f64>,
}

/// Output from Ze-Reproduction simulation (Axiom Z4).
/// Defined in reproduction.rs — re-exported here for ze-runner.
pub use crate::reproduction::ReproductionResult;

/// Output from a quantum simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResult {
    pub dim: usize,
    pub n_steps: usize,
    pub n_states: usize,
    pub theta_q: f64,
    pub born_tau_final: i64,
    pub uniform_tau_final: i64,
    pub anti_born_tau_final: i64,
    pub born_t_rate: f64,
    pub uniform_t_rate: f64,
    pub anti_born_t_rate: f64,
    /// Theoretical T-rate for Born strategy: Σ_i p_i · 1[p_i < 2^{−θ}].
    pub born_theory_rate: f64,
    /// Theoretical T-rate for Uniform strategy: 1 if d > 2^θ, else Σ_{p_i < 2^{−θ}} p_i.
    pub uniform_theory_rate: f64,
    /// Theoretical T-rate for Anti-Born strategy (worst case, upper bound).
    pub anti_born_theory_rate: f64,
    /// Theorem 5.1 verified: born_theory_rate ≤ uniform_theory_rate ≤ anti_born_theory_rate.
    pub theorem_5_1_holds: bool,
    pub history_born: Vec<i64>,
    pub history_uniform: Vec<i64>,
    pub history_anti_born: Vec<i64>,
}
