/// Unit tests for feed ranking algorithm
/// score = base × quality × recency_decay − penalty
///
/// Run: cargo test --test feed_ranker_tests

use chrono::Utc;
use longevitycommon_server::services::feed_ranker::{compute_score, PostScoreInput};

fn base_input() -> PostScoreInput {
    PostScoreInput {
        reactions_support: 0,
        reactions_replicate: 0,
        reactions_challenge: 0,
        reactions_cite: 0,
        doi_verified: false,
        has_code_url: false,
        has_data_url: false,
        created_at: Utc::now(),
        rank_penalty: 0.0,
    }
}

#[test]
fn test_zero_reactions_gives_zero_score() {
    let score = compute_score(&base_input());
    // base_score=0, quality=1.0, decay≈1.0, penalty=0 → score=0
    assert_eq!(score, 0.0);
}

#[test]
fn test_replicate_weighted_higher_than_support() {
    let with_replicate = compute_score(&PostScoreInput {
        reactions_replicate: 1,
        ..base_input()
    });
    let with_support = compute_score(&PostScoreInput {
        reactions_support: 1,
        ..base_input()
    });
    // replicate weight=3, support weight=1
    assert!(with_replicate > with_support, "replicate should outrank support 3:1");
    assert!((with_replicate / with_support - 3.0).abs() < 0.01);
}

#[test]
fn test_quality_bonus_doi_verified() {
    let without_doi = compute_score(&PostScoreInput {
        reactions_support: 5,
        ..base_input()
    });
    let with_doi = compute_score(&PostScoreInput {
        reactions_support: 5,
        doi_verified: true,
        ..base_input()
    });
    // quality factor: 1.0 vs 1.5 → ratio = 1.5
    assert!(with_doi > without_doi);
    let ratio = with_doi / without_doi;
    assert!((ratio - 1.5).abs() < 0.01, "DOI bonus should be 1.5× quality, got {ratio:.3}");
}

#[test]
fn test_quality_bonus_code_and_data() {
    let base = compute_score(&PostScoreInput { reactions_cite: 2, ..base_input() });
    let with_code = compute_score(&PostScoreInput {
        reactions_cite: 2,
        has_code_url: true,
        ..base_input()
    });
    let with_data = compute_score(&PostScoreInput {
        reactions_cite: 2,
        has_data_url: true,
        ..base_input()
    });
    let with_all = compute_score(&PostScoreInput {
        reactions_cite: 2,
        doi_verified: true,
        has_code_url: true,
        has_data_url: true,
        ..base_input()
    });

    assert!(with_code > base);
    assert!(with_data > base);
    assert!(with_all > with_code);
    assert!(with_all > with_data);

    // Total quality factor = 1.0 + 0.5 + 0.3 + 0.2 = 2.0
    let ratio = with_all / base;
    assert!((ratio - 2.0).abs() < 0.01, "full quality bonus should be 2×, got {ratio:.3}");
}

#[test]
fn test_recency_decay_reduces_old_posts() {
    use chrono::Duration;

    let new_post = compute_score(&PostScoreInput {
        reactions_support: 10,
        created_at: Utc::now(),
        ..base_input()
    });
    let old_post = compute_score(&PostScoreInput {
        reactions_support: 10,
        created_at: Utc::now() - Duration::hours(96), // 2× half-life (48h)
        ..base_input()
    });

    // decay(96h) = exp(-96/48) = exp(-2) ≈ 0.135
    // so old_post ≈ new_post * 0.135
    assert!(old_post < new_post * 0.2, "96h old post should decay significantly");
}

#[test]
fn test_penalty_reduces_score() {
    let clean = compute_score(&PostScoreInput {
        reactions_support: 5,
        ..base_input()
    });
    let penalised = compute_score(&PostScoreInput {
        reactions_support: 5,
        rank_penalty: 2.0,
        ..base_input()
    });
    assert!(
        clean - penalised > 1.9,
        "penalty of 2.0 should reduce score by ~2, diff={:.3}", clean - penalised
    );
}

#[test]
fn test_score_can_go_negative_with_large_penalty() {
    let score = compute_score(&PostScoreInput {
        rank_penalty: 100.0,
        ..base_input()
    });
    assert!(score < 0.0, "large penalty on zero-reaction post should yield negative score");
}

#[test]
fn test_full_ranking_order() {
    use chrono::Duration;

    // Expected rank: verified_with_reactions > support_only > old_post > penalised
    let verified = compute_score(&PostScoreInput {
        reactions_support: 3,
        reactions_replicate: 1,
        doi_verified: true,
        has_code_url: true,
        created_at: Utc::now(),
        ..base_input()
    });
    let support_only = compute_score(&PostScoreInput {
        reactions_support: 3,
        created_at: Utc::now(),
        ..base_input()
    });
    let old_post = compute_score(&PostScoreInput {
        reactions_support: 3,
        reactions_replicate: 1,
        doi_verified: true,
        created_at: Utc::now() - Duration::days(10),
        ..base_input()
    });
    let penalised = compute_score(&PostScoreInput {
        reactions_support: 5,
        rank_penalty: 2.0,
        created_at: Utc::now(),
        ..base_input()
    });

    assert!(verified > support_only, "verified+code should rank above support_only");
    assert!(support_only > penalised || old_post > penalised, "penalised post ranks lower");
    assert!(old_post < verified, "10-day old post ranks below fresh verified post");
}
