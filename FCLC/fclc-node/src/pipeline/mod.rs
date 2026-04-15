use anyhow::Result;
use fclc_core::{
    dp::{DpConfig, LinearDpAccountant, add_noise_to_gradient, clip_gradient, gaussian_noise_sigma},
    privacy::{DeidentConfig, deidentify_batch},
    schema::{OmopRecord, OmopRecord as Record},
};
use std::path::Path;

/// Results from one local training round.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrainingResult {
    /// Gradient update (model delta) ready to send to orchestrator.
    pub gradient_update: Vec<f32>,
    /// Training loss on local data.
    pub train_loss: f32,
    /// Validation AUC on local data (approximate).
    pub val_auc: f32,
    /// DP epsilon consumed this round.
    pub dp_epsilon_spent: f64,
    /// Gaussian noise multiplier σ used this round — forwarded to server for Rényi DP accounting.
    pub sigma: Option<f64>,
    /// Poisson sampling rate q = batch_size / dataset_size — forwarded for Rényi DP accounting.
    pub sampling_rate: Option<f64>,
}

/// Simple logistic regression model: weights vector (bias included as last element).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogisticModel {
    pub weights: Vec<f32>,
}

impl LogisticModel {
    /// Initialise with zeros. `dim` should match OmopRecord::FEATURE_DIM + 1 (bias).
    pub fn new(dim: usize) -> Self {
        Self {
            weights: vec![0.0f32; dim],
        }
    }

    /// Logistic sigmoid.
    fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Predict probability for a feature vector.
    pub fn predict(&self, features: &[f32]) -> f32 {
        let mut dot: f32 = self.weights[features.len()]; // bias
        for (w, x) in self.weights.iter().zip(features.iter()) {
            dot += w * x;
        }
        Self::sigmoid(dot)
    }

    /// Compute gradient of binary cross-entropy loss w.r.t. weights.
    /// Returns (loss, gradient).
    pub fn compute_gradient(
        &self,
        records: &[OmopRecord],
    ) -> (f32, Vec<f32>) {
        if records.is_empty() {
            return (0.0, vec![0.0f32; self.weights.len()]);
        }

        let mut grad = vec![0.0f32; self.weights.len()];
        let mut total_loss = 0.0f32;
        let n = records.len() as f32;

        for record in records {
            let features = record.to_features();
            let y = if record.hospitalized_next_12m { 1.0f32 } else { 0.0 };
            let y_hat = self.predict(&features);

            // Binary cross-entropy loss
            let loss = -(y * (y_hat + 1e-7).ln() + (1.0 - y) * (1.0 - y_hat + 1e-7).ln());
            total_loss += loss;

            // Gradient: (y_hat - y) * x
            let error = y_hat - y;
            for (i, &x) in features.iter().enumerate() {
                grad[i] += error * x / n;
            }
            // Bias gradient
            *grad.last_mut().unwrap() += error / n;
        }

        (total_loss / n, grad)
    }

    /// Update weights by subtracting gradient * learning_rate.
    pub fn apply_gradient(&mut self, gradient: &[f32], lr: f32) {
        for (w, g) in self.weights.iter_mut().zip(gradient.iter()) {
            *w -= lr * g;
        }
    }

    /// Compute approximate AUC on labelled data (simple rank-based).
    pub fn compute_auc(&self, records: &[OmopRecord]) -> f32 {
        if records.len() < 2 {
            return 0.5;
        }

        let mut scores: Vec<(f32, bool)> = records
            .iter()
            .map(|r| (self.predict(&r.to_features()), r.hospitalized_next_12m))
            .collect();

        scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let pos: usize = scores.iter().filter(|s| s.1).count();
        let neg = scores.len() - pos;

        if pos == 0 || neg == 0 {
            return 0.5;
        }

        // Wilcoxon statistic
        let mut rank_sum = 0usize;
        for (rank, (_, label)) in scores.iter().enumerate() {
            if *label {
                rank_sum += rank + 1;
            }
        }

        let u = rank_sum as f32 - (pos * (pos + 1) / 2) as f32;
        u / (pos as f32 * neg as f32)
    }
}

/// Local training pipeline.
pub struct LocalPipeline {
    pub model: LogisticModel,
    pub dp_config: DpConfig,
    pub accountant: LinearDpAccountant,
    pub learning_rate: f32,
    pub local_epochs: usize,
    pub max_grad_norm: f32,
    /// FedProx proximal penalty (μ). 0.0 = standard FedAvg.
    pub fedprox_mu: f32,
    /// Last received global model weights — used for FedProx proximal term.
    global_weights: Vec<f32>,
}

impl LocalPipeline {
    pub fn new(dp_config: DpConfig, budget: f64) -> Self {
        let dim = fclc_core::schema::OmopRecord::FEATURE_DIM + 1; // +1 for bias
        Self {
            model: LogisticModel::new(dim),
            dp_config,
            accountant: LinearDpAccountant::new(budget),
            learning_rate: 0.01,
            local_epochs: 3,
            max_grad_norm: 1.0,
            fedprox_mu: 0.1, // matches CONCEPT.md §Aggregation and PARAMETERS.md
            global_weights: vec![0.0f32; fclc_core::schema::OmopRecord::FEATURE_DIM + 1],
        }
    }

    /// Load data from a CSV file, de-identify it, then run local training.
    pub fn run_from_csv(&mut self, path: &Path) -> Result<TrainingResult> {
        let mut records = crate::connector::load_csv(path)?;
        self.run_training(&mut records)
    }

    /// Run training on an in-memory batch (pre-loaded from any source).
    pub fn run_training(&mut self, records: &mut Vec<OmopRecord>) -> Result<TrainingResult> {
        // Step 1: de-identify
        let deident_config = DeidentConfig::default();
        deidentify_batch(records, &deident_config);

        if records.is_empty() {
            anyhow::bail!("No records to train on after de-identification");
        }

        // Step 2: local gradient descent for `local_epochs` epochs
        let mut final_gradient = vec![0.0f32; self.model.weights.len()];
        let mut final_loss = 0.0f32;

        for _epoch in 0..self.local_epochs {
            let (loss, mut gradient) = self.model.compute_gradient(records);
            final_loss = loss;

            // FedProx proximal correction: grad += μ * (w - w_global)
            // This penalises the local model for drifting from the global model,
            // improving convergence in non-IID settings (Li et al. 2020).
            if self.fedprox_mu > 0.0 {
                for (g, (w, wg)) in gradient.iter_mut().zip(
                    self.model.weights.iter().zip(self.global_weights.iter())
                ) {
                    *g += self.fedprox_mu * (w - wg);
                }
            }

            // DP: clip gradient
            clip_gradient(&mut gradient, self.max_grad_norm);

            // DP: add Gaussian noise
            add_noise_to_gradient(&mut gradient, &self.dp_config);

            // Update model
            self.model.apply_gradient(&gradient, self.learning_rate);
            final_gradient = gradient;
        }

        // Track DP budget: by basic composition, each of the local_epochs
        // gradient computations contributes ε, so total cost = epochs × ε.
        let epsilon_spent = self.dp_config.epsilon * self.local_epochs as f64;
        self.accountant.spend(epsilon_spent)?;

        // Compute Gaussian σ and Poisson sampling rate for Rényi DP accounting on server.
        // σ is derived from the same sensitivity/ε/δ used during noise injection.
        // sampling_rate = 1/n (each step uses the full local batch; Poisson rate ≈ 1).
        let sigma = gaussian_noise_sigma(
            self.dp_config.sensitivity,
            self.dp_config.epsilon,
            self.dp_config.delta,
        );
        let sampling_rate = 1.0_f64 / records.len().max(1) as f64;

        // Compute AUC on local data
        let val_auc = self.model.compute_auc(records);

        Ok(TrainingResult {
            gradient_update: final_gradient,
            train_loss: final_loss,
            val_auc,
            dp_epsilon_spent: epsilon_spent,
            sigma: Some(sigma),
            sampling_rate: Some(sampling_rate),
        })
    }

    /// Update model weights from global model received from orchestrator.
    /// Also saves a copy as the proximal anchor for FedProx.
    pub fn update_global_model(&mut self, global_weights: Vec<f32>) {
        if global_weights.len() == self.model.weights.len() {
            self.global_weights = global_weights.clone();
            self.model.weights = global_weights;
        }
    }

    pub fn dp_remaining(&self) -> f64 {
        self.accountant.remaining()
    }

    pub fn dp_fraction_consumed(&self) -> f64 {
        self.accountant.fraction_consumed()
    }
}
