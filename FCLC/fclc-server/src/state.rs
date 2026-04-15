use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use sqlx::PgPool;
use fclc_core::RdpAccountant;

use crate::models::{RoundResult, UpdatePayload};

/// Dimension of the global logistic-regression model.
/// Must match OmopRecord::FEATURE_DIM + 1 (bias) = 9.
pub const MODEL_DIM: usize = 9;

/// Maximum cumulative (ε, δ)-DP epsilon allowed per node before exclusion.
/// With Rényi DP accounting (vs. basic composition), nodes can participate
/// in ~30–40 rounds at ε=2.0/round before hitting this limit, instead of 5.
pub const EPSILON_TOTAL: f64 = 10.0;

/// δ used for Rényi→(ε,δ)-DP conversion per node.
pub const DP_DELTA: f64 = 1e-5;

/// Minimum number of nodes that must submit updates before auto-aggregation.
pub const MIN_NODES_FOR_AGGREGATION: usize = 2;

/// Per-node DP accounting state: tracks both linear and Rényi epsilon.
#[derive(Debug)]
pub struct NodeDpState {
    /// Linear (basic composition) epsilon sum — used for display and DB persistence.
    pub epsilon_linear: f64,
    /// Rényi DP accountant — provides tighter effective epsilon estimates.
    pub rdp: RdpAccountant,
}

impl NodeDpState {
    pub fn new() -> Self {
        Self {
            epsilon_linear: 0.0,
            rdp: RdpAccountant::new(DP_DELTA),
        }
    }

    /// Effective (ε, δ)-DP epsilon: min of linear and Rényi estimates.
    /// Rényi is tighter when sigma and sampling_rate are provided.
    pub fn effective_epsilon(&self) -> f64 {
        let rdp_eps = self.rdp.current_epsilon();
        if rdp_eps.is_finite() && rdp_eps < self.epsilon_linear {
            rdp_eps
        } else {
            self.epsilon_linear
        }
    }

    /// Spend one round of DP budget.
    ///
    /// - `epsilon_spent`: linear epsilon (always updated).
    /// - `sigma` + `sampling_rate`: if provided, also updates Rényi accountant.
    pub fn spend(&mut self, epsilon_spent: f64, sigma: Option<f64>, sampling_rate: Option<f64>) {
        self.epsilon_linear += epsilon_spent;
        if let (Some(s), Some(q)) = (sigma, sampling_rate) {
            if s > 0.0 && (0.0..=1.0).contains(&q) {
                let _ = self.rdp.spend_round(s, q);
            }
        }
    }
}

/// Shared application state passed to every Axum handler via `State<Arc<AppState>>`.
pub struct AppState {
    /// PostgreSQL connection pool.
    pub pool: PgPool,

    /// Current global model weights (9-dim logistic regression: 8 features + bias).
    pub global_model: Arc<RwLock<Vec<f64>>>,

    /// Current federated round number (starts at 0, increments after each aggregation).
    pub current_round: Arc<RwLock<u64>>,

    /// Pending updates for the current round: (node_id, payload).
    /// Cleared after each successful aggregation.
    pub pending_updates: Arc<RwLock<Vec<(Uuid, UpdatePayload)>>>,

    /// Per-node DP accounting state (linear + Rényi).
    /// Key: node_id. Value: NodeDpState tracking both ε estimates.
    pub node_budgets: Arc<RwLock<HashMap<Uuid, NodeDpState>>>,

    /// History of completed rounds (in-memory mirror for fast reads).
    pub round_history: Arc<RwLock<Vec<RoundResult>>>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            global_model: Arc::new(RwLock::new(vec![0.0_f64; MODEL_DIM])),
            current_round: Arc::new(RwLock::new(0u64)),
            pending_updates: Arc::new(RwLock::new(Vec::new())),
            node_budgets: Arc::new(RwLock::new(HashMap::new())),
            round_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
