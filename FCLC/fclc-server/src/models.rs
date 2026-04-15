use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Registration ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub node_name: String,
    pub node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub node_id: Uuid,
    pub status: String,
}

// ── Update submission ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayload {
    /// Gradient vector (f64 so JSON round-trips cleanly)
    pub gradient: Vec<f64>,
    /// Epsilon consumed by DP noise on this update (linear accounting, kept for display).
    pub epsilon_spent: f64,
    pub loss: f64,
    pub auc: f64,
    pub record_count: usize,
    /// Gaussian noise multiplier σ used by the node for DP-SGD.
    /// When present, the server uses Rényi DP accounting (tighter bounds).
    /// If absent, falls back to linear (basic) composition.
    pub sigma: Option<f64>,
    /// Batch sampling rate q = batch_size / dataset_size (for Rényi subsampled Gaussian).
    /// Required for accurate Rényi DP accounting; defaults to 0.01 if absent.
    pub sampling_rate: Option<f64>,
}

// ── Global model ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalModelResponse {
    pub weights: Vec<f64>,
    pub round: u64,
    pub version: String,
}

// ── Round metadata ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub round_id: Uuid,
    pub round_number: u64,
    pub auc: f64,
    pub participating_nodes: usize,
    pub timestamp: String,
}

// ── Shapley score ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeScore {
    pub node_id: Uuid,
    pub shapley_score: f64,
    pub round: u64,
}

// ── Dashboard metrics ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub current_round: u64,
    pub node_count: usize,
    pub auc_history: Vec<f64>,
    pub avg_shapley: f64,
    /// Total epsilon saved across all nodes: Σ (linear_ε - Rényi_ε).
    /// Zero when sigma/sampling_rate were not submitted by nodes.
    pub rdp_epsilon_savings: f64,
    /// Number of nodes with effective epsilon > 80% of EPSILON_TOTAL (at-risk).
    pub nodes_near_budget: usize,
}

// ── Node listing ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: Uuid,
    pub node_name: String,
    pub epsilon_spent: f64,
    pub registered_at: String,
}

// ── Audit log (hash-chain) ────────────────────────────────────────────────────

/// One entry in the tamper-evident hash-chain audit log.
/// Each entry commits to the previous via `prev_hash`.
/// Genesis entry has prev_hash = '0' × 64.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: Uuid,
    pub round_id: Uuid,
    pub round_number: u64,
    /// SHA-256 hex of aggregated model weights after this round.
    pub gradient_hash: String,
    pub mean_auc: f64,
    pub participating: usize,
    /// SHA-256 hex of the previous entry's `entry_hash` (chain link).
    pub prev_hash: String,
    /// SHA-256 hex of (round_id ‖ round_number ‖ gradient_hash ‖ prev_hash).
    pub entry_hash: String,
    pub recorded_at: String,
}

// ── Error responses ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { error: msg.into() }
    }
}
