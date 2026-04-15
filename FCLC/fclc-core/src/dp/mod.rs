pub mod renyi;
pub use renyi::{RdpAccountant, RdpError, rdp_gaussian, rdp_to_dp};

use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use thiserror::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DpConfig {
    pub epsilon: f64,
    pub delta: f64,
    pub sensitivity: f64,
}

impl Default for DpConfig {
    fn default() -> Self {
        Self {
            epsilon: 2.0,
            delta: 1e-5,
            sensitivity: 1.0,
        }
    }
}

#[derive(Debug, Error)]
pub enum DpError {
    #[error("DP budget exhausted: requested {requested:.4}, remaining {remaining:.4}")]
    BudgetExhausted { requested: f64, remaining: f64 },
    #[error("Invalid DP parameters: {0}")]
    InvalidParams(String),
}

/// Epsilon budget accountant for (ε, δ)-DP via **basic (linear) composition**.
///
/// Tracks cumulative epsilon expenditure against a total budget by simple addition.
/// This is the CONSERVATIVE worst-case bound: ε_total = Σ ε_i over all rounds.
///
/// For tighter bounds use `RdpAccountant` (renyi.rs), which exploits the Rényi DP
/// curve of the Gaussian mechanism and subsampling to reduce effective ε by ~30–40×.
///
/// # Naming note
/// Linear composition DP budget accountant for (ε, δ)-DP.
///
/// Tracks cumulative epsilon expenditure by simple addition (basic composition).
/// This is the conservative worst-case bound: ε_total = Σ ε_i over all rounds.
/// For tighter bounds use `RdpAccountant` in `dp::renyi`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinearDpAccountant {
    pub total_epsilon: f64,
    budget: f64,
}

impl LinearDpAccountant {
    pub fn new(budget: f64) -> Self {
        Self {
            total_epsilon: 0.0,
            budget,
        }
    }

    /// Attempt to spend `epsilon` from the budget.
    /// Returns Err if the budget would be exceeded.
    pub fn spend(&mut self, epsilon: f64) -> Result<(), DpError> {
        if epsilon <= 0.0 {
            return Err(DpError::InvalidParams(format!(
                "epsilon must be positive, got {epsilon}"
            )));
        }
        let new_total = self.total_epsilon + epsilon;
        if new_total > self.budget {
            Err(DpError::BudgetExhausted {
                requested: epsilon,
                remaining: self.remaining(),
            })
        } else {
            self.total_epsilon = new_total;
            Ok(())
        }
    }

    /// Remaining epsilon budget.
    pub fn remaining(&self) -> f64 {
        (self.budget - self.total_epsilon).max(0.0)
    }

    /// Fraction of budget consumed (0.0 – 1.0).
    pub fn fraction_consumed(&self) -> f64 {
        (self.total_epsilon / self.budget).min(1.0)
    }

    /// Project cumulative ε after `rounds` additional rounds, each costing `eps_per_round`.
    ///
    /// R6 concern: with ε=2.0/round and 100 rounds, linear composition gives ε_total=200.
    /// Use this to surface the warning in dashboards / logs before budget is committed.
    ///
    /// Returns (projected_total, budget_exceeded).
    pub fn epsilon_projection(&self, rounds: u32, eps_per_round: f64) -> (f64, bool) {
        let projected = self.total_epsilon + eps_per_round * rounds as f64;
        (projected, projected > self.budget)
    }
}

/// Compute the standard deviation of Gaussian noise satisfying (ε, δ)-DP
/// for a query with given L2 sensitivity.
///
/// σ = sensitivity * sqrt(2 * ln(1.25 / δ)) / ε
pub fn gaussian_noise_sigma(sensitivity: f64, epsilon: f64, delta: f64) -> f64 {
    sensitivity * (2.0 * (1.25_f64 / delta).ln()).sqrt() / epsilon
}

/// Sample a single Gaussian noise value with the appropriate σ for (ε, δ)-DP.
pub fn gaussian_noise(sensitivity: f64, epsilon: f64, delta: f64) -> f64 {
    let sigma = gaussian_noise_sigma(sensitivity, epsilon, delta);
    let normal = Normal::new(0.0, sigma).expect("valid sigma");
    normal.sample(&mut thread_rng())
}

/// Clip gradient to have L2 norm ≤ max_norm (in-place).
pub fn clip_gradient(gradient: &mut Vec<f32>, max_norm: f32) {
    if max_norm <= 0.0 {
        return;
    }
    let norm: f32 = gradient.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > max_norm {
        let scale = max_norm / norm;
        for g in gradient.iter_mut() {
            *g *= scale;
        }
    }
}

/// Add calibrated Gaussian noise to every element of `gradient` (in-place).
/// Applies (ε, δ)-DP with the given sensitivity.
pub fn add_noise_to_gradient(gradient: &mut Vec<f32>, config: &DpConfig) {
    let sigma = gaussian_noise_sigma(config.sensitivity, config.epsilon, config.delta);
    let normal = Normal::new(0.0f64, sigma).expect("valid sigma");
    let mut rng = thread_rng();
    for g in gradient.iter_mut() {
        *g += normal.sample(&mut rng) as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accountant_spend() {
        let mut acc = LinearDpAccountant::new(10.0);
        assert!(acc.spend(2.0).is_ok());
        assert!(acc.spend(2.0).is_ok());
        assert_eq!(acc.total_epsilon, 4.0);
        assert_eq!(acc.remaining(), 6.0);
    }

    #[test]
    fn test_accountant_exhausted() {
        let mut acc = LinearDpAccountant::new(5.0);
        assert!(acc.spend(4.0).is_ok());
        assert!(acc.spend(2.0).is_err());
    }

    #[test]
    fn test_clip_gradient() {
        let mut g = vec![3.0f32, 4.0];
        clip_gradient(&mut g, 1.0);
        let norm: f32 = g.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_add_noise() {
        let original = vec![1.0f32; 100];
        let mut g = original.clone();
        let config = DpConfig::default();
        add_noise_to_gradient(&mut g, &config);
        // After noise, gradient should differ from original
        let changed = g.iter().zip(original.iter()).any(|(a, b)| (a - b).abs() > 1e-9);
        assert!(changed);
    }
}
