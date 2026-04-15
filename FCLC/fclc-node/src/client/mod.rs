use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Registration request sent to orchestrator when node comes online.
#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub node_name: String,  // matches server models.rs RegisterRequest.node_name
    pub node_id: Uuid,
}

/// Response from orchestrator after registration.
#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    pub node_id: Uuid,
    pub status: String,  // matches server models.rs RegisterResponse.status
}

/// Model update payload sent to orchestrator after local training.
/// Field names and types match server models.rs UpdatePayload exactly.
#[derive(Debug, Serialize)]
pub struct ModelUpdatePayload {
    pub node_id: Uuid,
    pub round_id: u32,
    pub gradient: Vec<f64>,           // server expects f64
    pub epsilon_spent: f64,           // was: dp_epsilon_spent
    pub loss: f64,                    // was: train_loss: f32
    pub auc: f64,                     // was: val_auc: f32
    pub record_count: usize,
    pub sigma: Option<f64>,           // Gaussian σ for Rényi DP accounting on server
    pub sampling_rate: Option<f64>,   // q = batch_size/dataset_size for Rényi DP
}

/// Global model weights received from orchestrator.
/// Matches server models.rs GlobalModelResponse exactly.
#[derive(Debug, Deserialize)]
pub struct GlobalModelResponse {
    pub round: u64,
    pub weights: Vec<f64>,
    pub version: String,
}

/// Shapley score for this node from the orchestrator.
#[derive(Debug, Deserialize)]
pub struct ShapleyScoreResponse {
    pub node_id: Uuid,
    pub score: f64,
    pub normalised_score: f64,
}

/// HTTP client that communicates with the FCLC orchestrator.
pub struct OrchestratorClient {
    base_url: String,
    node_id: Uuid,
    client: reqwest::blocking::Client,
}

impl OrchestratorClient {
    pub fn new(base_url: &str, node_id: Uuid) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            node_id,
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// Register this node with the orchestrator.
    pub fn register(&self, name: &str) -> Result<RegisterResponse> {
        let url = format!("{}/api/nodes/register", self.base_url);
        let payload = RegisterRequest {
            node_name: name.to_string(),
            node_id: self.node_id,
        };
        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .context("Failed to connect to orchestrator")?;

        resp.json::<RegisterResponse>()
            .context("Failed to parse registration response")
    }

    /// Submit a model update to the orchestrator.
    ///
    /// Retries up to `MAX_SUBMIT_RETRIES` times on network errors (connection
    /// refused, timeout) with exponential backoff. HTTP 4xx/5xx errors are NOT
    /// retried — they indicate a protocol or server-side problem.
    pub fn submit_update(&self, payload: ModelUpdatePayload) -> Result<()> {
        const MAX_SUBMIT_RETRIES: u32 = 3;
        const BASE_BACKOFF_MS: u64 = 1_000;

        let url = format!("{}/api/nodes/{}/update", self.base_url, self.node_id);
        let mut last_err = anyhow::anyhow!("submit_update: no attempts made");

        for attempt in 0..=MAX_SUBMIT_RETRIES {
            if attempt > 0 {
                let wait_ms = BASE_BACKOFF_MS * (1 << (attempt - 1)); // 1s, 2s, 4s
                std::thread::sleep(std::time::Duration::from_millis(wait_ms));
            }

            match self.client.post(&url).json(&payload).send() {
                Ok(resp) if resp.status().is_success() => return Ok(()),
                Ok(resp) => {
                    // HTTP error — do not retry (4xx = client bug, 5xx = server fault).
                    return Err(anyhow::anyhow!(
                        "Orchestrator returned HTTP {}", resp.status()
                    ));
                }
                Err(e) => {
                    // Network-level error (timeout, connection refused) — retry.
                    last_err = anyhow::anyhow!(
                        "Network error on attempt {}/{}: {}", attempt + 1, MAX_SUBMIT_RETRIES + 1, e
                    );
                }
            }
        }

        Err(last_err.context("submit_update failed after all retries"))
    }

    /// Download the current global model from the orchestrator.
    pub fn get_global_model(&self) -> Result<GlobalModelResponse> {
        let url = format!("{}/api/model/current", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .context("Failed to fetch global model")?;

        resp.json::<GlobalModelResponse>()
            .context("Failed to parse global model response")
    }

    /// Fetch Shapley score for this node.
    pub fn get_shapley_score(&self) -> Result<ShapleyScoreResponse> {
        let url = format!("{}/api/nodes/{}/score", self.base_url, self.node_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .context("Failed to fetch Shapley score")?;

        resp.json::<ShapleyScoreResponse>()
            .context("Failed to parse Shapley score response")
    }

    /// Check connectivity to the orchestrator (returns true if reachable).
    pub fn ping(&self) -> bool {
        let url = format!("{}/api/model/current", self.base_url);
        self.client.get(&url).send().is_ok()
    }

    pub fn node_id(&self) -> Uuid {
        self.node_id
    }
}
