//! MCOA core — multi-counter architecture of organismal aging.
//!
//! Implements Axioms M1–M4 from CONCEPT.md:
//!   M1 — parallel counters
//!   M2 — dimensional consistency (n → n/n*, t → t/τ)
//!   M3 — a-priori tissue weighting
//!   M4 — falsifiability first-class
//!
//! Reference: Tkemaladze (2026) "The Multi-Counter Architecture of Organismal Aging",
//! Nature Aging Perspective submission, 2026-04-25.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const N_COUNTERS: usize = 5;

/// MCOA counter numbering aligned with user decision 2026-05-07:
///   #1 = Centriolar (CDATA), #2 = Telomere, #3 = Mitochondrial,
///   #4 = Epigenetic, #5 = Proteostasis.
///
/// The discriminant is 0-indexed for zero-cost array indexing
/// (`c as usize`); the user-facing number is `as u8 + 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Counter {
    Centriolar = 0,    // MCOA #1 — CDATA
    Telomere = 1,      // MCOA #2
    Mitochondrial = 2, // MCOA #3 — MitoROS
    Epigenetic = 3,    // MCOA #4 — EpigeneticDrift
    Proteostasis = 4,  // MCOA #5
}

impl Counter {
    pub const ALL: [Counter; N_COUNTERS] = [
        Counter::Centriolar,
        Counter::Telomere,
        Counter::Mitochondrial,
        Counter::Epigenetic,
        Counter::Proteostasis,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Counter::Centriolar => "centriolar",
            Counter::Telomere => "telomere",
            Counter::Mitochondrial => "mito",
            Counter::Epigenetic => "epigenetic",
            Counter::Proteostasis => "proteostasis",
        }
    }

    /// User-facing 1-indexed number (Counter #1 … #5). Matches
    /// CONCEPT.md numbering and subproject CLAUDE.md / Cargo.toml
    /// descriptions. Decided 2026-05-07.
    pub fn mcoa_number(self) -> u8 {
        self as u8 + 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tissue {
    Fibroblast,
    Hsc,
    Neuron,
    Hepatocyte,
    BetaCell,
    CD8TMemory,
}

impl Tissue {
    pub fn as_str(self) -> &'static str {
        match self {
            Tissue::Fibroblast => "fibroblast",
            Tissue::Hsc => "hsc",
            Tissue::Neuron => "neuron",
            Tissue::Hepatocyte => "hepatocyte",
            Tissue::BetaCell => "beta_cell",
            Tissue::CD8TMemory => "cd8_t_memory",
        }
    }
}

/// Reference scales for counter `i` in a specific tissue. Both `n_star` and `tau` MUST be set
/// from independent cell-biology a priori (Axiom M3). Post-hoc refitting is forbidden.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReferenceScales {
    /// Reference division count. `Option::None` means counter is not division-linked (α → 0).
    pub n_star: Option<f64>,
    /// Reference time in seconds.
    pub tau_seconds: f64,
}

/// Per-counter drift rates. Dimensionless.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DriftRates {
    pub alpha: f64, // division-equivalent rate
    pub beta: f64,  // time-equivalent rate
}

/// State of a single counter at a single instant.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CounterState {
    pub value: f64,
}

/// Axiom M2 (dimensional consistency) is enforced here: division count `n` is normalised by
/// `n_star`, chronological time `t_seconds` is normalised by `tau_seconds`. Returns the per-step
/// *independent* drift contribution (before coupling).
///
/// The caller is responsible for adding the coupling term `gamma_i * influence(others)`.
pub fn independent_drift(
    d0: f64,
    n: f64,
    t_seconds: f64,
    rates: DriftRates,
    scales: ReferenceScales,
) -> f64 {
    let div_term = match scales.n_star {
        Some(n_star) if n_star > 0.0 => rates.alpha * (n / n_star),
        _ => 0.0, // post-mitotic: α → 0
    };
    let time_term = if scales.tau_seconds > 0.0 {
        rates.beta * (t_seconds / scales.tau_seconds)
    } else {
        0.0
    };
    d0 + div_term + time_term
}

/// Tissue-specific weight row. Enforces Σ w_i ≈ 1.0 (Axiom M3 completeness).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TissueWeights(pub [f64; N_COUNTERS]);

impl TissueWeights {
    pub fn sum(&self) -> f64 {
        self.0.iter().sum()
    }

    pub fn is_normalised(&self, tol: f64) -> bool {
        (self.sum() - 1.0).abs() < tol
    }

    pub fn get(&self, c: Counter) -> f64 {
        self.0[c as usize]
    }
}

/// A priori tissue-weight table from PARAMETERS.md §3. PROVISIONAL; to be calibrated by Test 1.
pub fn default_weights(t: Tissue) -> TissueWeights {
    match t {
        //                          tel   cent  mito  epi   proteo
        Tissue::Fibroblast  => TissueWeights([0.40, 0.20, 0.15, 0.15, 0.10]),
        Tissue::Hsc         => TissueWeights([0.10, 0.40, 0.25, 0.15, 0.10]),
        Tissue::Neuron      => TissueWeights([0.00, 0.15, 0.35, 0.25, 0.25]),
        Tissue::Hepatocyte  => TissueWeights([0.10, 0.05, 0.30, 0.25, 0.30]),
        Tissue::BetaCell    => TissueWeights([0.05, 0.05, 0.20, 0.40, 0.30]),
        Tissue::CD8TMemory  => TissueWeights([0.15, 0.30, 0.25, 0.15, 0.15]),
    }
}

/// A priori drift rates per (Counter, Tissue). PROVISIONAL.
pub fn default_drift_rates(c: Counter, t: Tissue) -> DriftRates {
    // Values from PARAMETERS.md §2; refined per tissue below.
    let (alpha, beta) = match (c, t) {
        // Telomere
        (Counter::Telomere,     Tissue::Fibroblast)  => (0.020, 0.002),
        (Counter::Telomere,     Tissue::Hsc)         => (0.005, 0.001), // hTERT rescue insufficient
        (Counter::Telomere,     Tissue::Neuron)      => (0.000, 0.001),
        (Counter::Telomere,     _)                   => (0.010, 0.002),
        // Centriolar polyglutamylation
        (Counter::Centriolar,   Tissue::Hsc)         => (0.015, 0.005),
        (Counter::Centriolar,   Tissue::Neuron)      => (0.000, 0.006), // post-mitotic
        (Counter::Centriolar,   _)                   => (0.012, 0.004),
        // Mitochondrial (mostly time-driven)
        (Counter::Mitochondrial, _)                  => (0.000, 0.010),
        // Epigenetic drift (time-driven)
        (Counter::Epigenetic,   _)                   => (0.000, 0.008),
        // Proteostasis
        (Counter::Proteostasis, _)                   => (0.005, 0.006),
    };
    DriftRates { alpha, beta }
}

/// A priori reference scales per (Counter, Tissue). PROVISIONAL.
pub fn default_reference_scales(c: Counter, t: Tissue) -> ReferenceScales {
    const YR: f64 = 365.25 * 24.0 * 3600.0;
    match (c, t) {
        (Counter::Telomere, Tissue::Fibroblast)  => ReferenceScales { n_star: Some(50.0),  tau_seconds: YR },
        (Counter::Telomere, Tissue::Hsc)         => ReferenceScales { n_star: Some(200.0), tau_seconds: YR },
        (Counter::Telomere, Tissue::Neuron)      => ReferenceScales { n_star: None,        tau_seconds: YR },
        (Counter::Telomere, _)                   => ReferenceScales { n_star: Some(50.0),  tau_seconds: YR },

        (Counter::Centriolar, Tissue::Hsc)       => ReferenceScales { n_star: Some(65.0),  tau_seconds: 0.5 * YR },
        (Counter::Centriolar, Tissue::Neuron)    => ReferenceScales { n_star: None,        tau_seconds: YR },
        (Counter::Centriolar, _)                 => ReferenceScales { n_star: Some(40.0),  tau_seconds: 0.5 * YR },

        (Counter::Mitochondrial, Tissue::Neuron) => ReferenceScales { n_star: None,        tau_seconds: 30.0 * 86400.0 },
        (Counter::Mitochondrial, _)              => ReferenceScales { n_star: None,        tau_seconds: 14.0 * 86400.0 },

        (Counter::Epigenetic, _)                 => ReferenceScales { n_star: None,        tau_seconds: YR },

        (Counter::Proteostasis, _)               => ReferenceScales { n_star: Some(80.0),  tau_seconds: YR },
    }
}

/// 5×5 coupling matrix Γ. Γ[i][j] = rate at which counter j accelerates counter i.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gamma(pub [[f64; N_COUNTERS]; N_COUNTERS]);

impl Default for Gamma {
    /// A priori coupling matrix from PARAMETERS.md §4. PROVISIONAL.
    fn default() -> Self {
        //                   ← from j: tel   cent  mito  epi   proteo
        Gamma([
            /* i=tel    */ [ 0.00, 0.00, 0.30, 0.05, 0.00 ],
            /* i=cent   */ [ 0.00, 0.00, 0.10, 0.20, 0.05 ],
            /* i=mito   */ [ 0.00, 0.00, 0.00, 0.10, 0.10 ],
            /* i=epi    */ [ 0.00, 0.00, 0.30, 0.00, 0.00 ],
            /* i=proteo */ [ 0.00, 0.05, 0.20, 0.10, 0.00 ],
        ])
    }
}

impl Gamma {
    pub fn influence(&self, i: Counter, states: &[CounterState; N_COUNTERS]) -> f64 {
        let i = i as usize;
        (0..N_COUNTERS).map(|j| self.0[i][j] * states[j].value).sum()
    }
}

#[derive(Debug, Error)]
pub enum McoaError {
    #[error("tissue weights for {tissue} do not sum to 1.0 within tol: got {sum}")]
    WeightsNotNormalised { tissue: &'static str, sum: f64 },
    #[error("dimensional consistency violated for counter {0}: n_star or tau_seconds invalid")]
    DimensionalInconsistency(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tissues_have_normalised_weights() {
        for &tissue in &[
            Tissue::Fibroblast,
            Tissue::Hsc,
            Tissue::Neuron,
            Tissue::Hepatocyte,
            Tissue::BetaCell,
            Tissue::CD8TMemory,
        ] {
            let w = default_weights(tissue);
            assert!(
                w.is_normalised(1e-9),
                "tissue {} weights sum = {}",
                tissue.as_str(),
                w.sum()
            );
        }
    }

    #[test]
    fn drift_is_dimensionless() {
        // Axiom M2 — passing raw n and t in different units must still produce a dimensionless
        // result because scales.n_star and scales.tau_seconds normalise them.
        let rates = DriftRates { alpha: 0.02, beta: 0.002 };
        let scales = ReferenceScales { n_star: Some(50.0), tau_seconds: 365.25 * 24.0 * 3600.0 };
        let d = independent_drift(0.0, 50.0, 365.25 * 24.0 * 3600.0, rates, scales);
        // n/n* = 1, t/tau = 1, so d = 0 + 0.02 * 1 + 0.002 * 1 = 0.022
        assert!((d - 0.022).abs() < 1e-12);
    }

    #[test]
    fn post_mitotic_has_zero_division_contribution() {
        let rates = DriftRates { alpha: 0.015, beta: 0.01 };
        let scales = ReferenceScales { n_star: None, tau_seconds: 86400.0 };
        let d = independent_drift(0.0, 9999.0, 86400.0, rates, scales);
        // α is ignored because n_star = None
        assert!((d - 0.01).abs() < 1e-12);
    }

    /// Aging_rate per CONCEPT.md formula: weighted sum of normalised counter values.
    /// Aging_rate = Σ w_i · Counter_i / threshold_i. With unit thresholds, reduces to dot product.
    #[test]
    fn aging_rate_is_weighted_sum() {
        let weights = TissueWeights([0.30, 0.20, 0.20, 0.15, 0.15]);
        assert!(weights.is_normalised(1e-9));
        let states = [
            CounterState { value: 0.5 },  // Telomere
            CounterState { value: 0.3 },  // Centriolar
            CounterState { value: 0.6 },  // Mitochondrial
            CounterState { value: 0.4 },  // Epigenetic
            CounterState { value: 0.2 },  // Proteostasis
        ];
        // Aging_rate = 0.30·0.5 + 0.20·0.3 + 0.20·0.6 + 0.15·0.4 + 0.15·0.2
        //            = 0.15 + 0.06 + 0.12 + 0.06 + 0.03 = 0.42
        let aging: f64 = weights.0.iter()
            .zip(states.iter())
            .map(|(w, s)| w * s.value)
            .sum();
        assert!((aging - 0.42).abs() < 1e-12, "aging_rate={} expected 0.42", aging);
    }

    /// Coupling matrix Γ — null hypothesis: γ_ij = 0 default per CORRECTIONS-2026-04-22.
    /// influence() with zero gamma returns 0 regardless of states.
    #[test]
    fn null_gamma_yields_zero_influence() {
        let gamma = Gamma([[0.0; N_COUNTERS]; N_COUNTERS]);
        let states = [
            CounterState { value: 0.5 },
            CounterState { value: 0.3 },
            CounterState { value: 0.6 },
            CounterState { value: 0.4 },
            CounterState { value: 0.2 },
        ];
        for &c in &Counter::ALL {
            let inf = gamma.influence(c, &states);
            assert!(inf.abs() < 1e-12, "{:?} influence under null gamma should be 0, got {}", c, inf);
        }
    }

    /// Identity gamma (γ_ii = 1, off-diagonal = 0): influence equals own value.
    #[test]
    fn identity_gamma_yields_self_value() {
        let mut gamma_arr = [[0.0; N_COUNTERS]; N_COUNTERS];
        for i in 0..N_COUNTERS {
            gamma_arr[i][i] = 1.0;
        }
        let gamma = Gamma(gamma_arr);
        let states = [
            CounterState { value: 0.5 },
            CounterState { value: 0.3 },
            CounterState { value: 0.6 },
            CounterState { value: 0.4 },
            CounterState { value: 0.2 },
        ];
        for (idx, &c) in Counter::ALL.iter().enumerate() {
            let inf = gamma.influence(c, &states);
            let expected = states[idx].value;
            assert!((inf - expected).abs() < 1e-12,
                "{:?} self-influence: got {} expected {}", c, inf, expected);
        }
    }
}
