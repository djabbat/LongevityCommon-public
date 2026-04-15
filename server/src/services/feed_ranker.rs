/// Feed ranking algorithm
/// score(post) = base_score × quality_factor × recency_decay - penalty
///
/// base_score      = support×1 + replicate×3 + cite×2 + challenge×1
/// quality_factor  = 1.0 + (0.5 if doi_verified) + (0.3 if code_url) + (0.2 if data_url)
/// recency_decay   = exp(-hours / 48)

use chrono::{DateTime, Utc};

pub struct PostScoreInput {
    pub reactions_support: i64,
    pub reactions_replicate: i64,
    pub reactions_challenge: i64,
    pub reactions_cite: i64,
    pub doi_verified: bool,
    pub has_code_url: bool,
    pub has_data_url: bool,
    pub created_at: DateTime<Utc>,
    pub rank_penalty: f64,
}

pub fn compute_score(input: &PostScoreInput) -> f64 {
    let base_score = (input.reactions_support as f64 * 1.0)
        + (input.reactions_replicate as f64 * 3.0)
        + (input.reactions_cite as f64 * 2.0)
        + (input.reactions_challenge as f64 * 1.0);

    let quality_factor = 1.0
        + if input.doi_verified { 0.5 } else { 0.0 }
        + if input.has_code_url { 0.3 } else { 0.0 }
        + if input.has_data_url { 0.2 } else { 0.0 };

    let hours_old = (Utc::now() - input.created_at).num_minutes() as f64 / 60.0;
    let recency_decay = (-hours_old / 48.0_f64).exp();

    (base_score * quality_factor * recency_decay) - input.rank_penalty
}
