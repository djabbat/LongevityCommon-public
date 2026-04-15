// Ontogenesis v4.2 — src/lib.rs
// Etagenesis platform: lifespan simulation 0–120 years
// Stack: Rust/WASM (core algorithms) + Three.js/WebGL (3D render)
//
// v4.2 changes:
// - 3 modules → 5 phases (Nature Comm 2025)
// - 4 domains: Morphology · Physiology · Psychology · Sociology
// - CV/Range → LCS (Latent Change Score)
// - FDR Benjamini-Hochberg (primary) + Bonferroni (sensitivity)
// - Metamorphosis detection (cross-domain synchronous transitions)
// - Expected ~12 metamorphoses per lifespan (empirical)

pub mod data;
pub mod analysis;
pub mod params;
pub mod metamorphosis;

pub use data::ingestion::{DataRecord, DataIngestion, DataType};
pub use data::normalization::AgeGrid;
pub use analysis::transition_detection::{TransitionDetector, Transition, TransitionType};
pub use analysis::lcs::{
    LcsParams, DualLcsParams, LongitudinalSeries, Observation,
    LcsTestResult, CouplingResult, CouplingDirection,
    estimate_lcs, estimate_dual_lcs, lcs_individual_tests, analyze_coupling,
    normal_cdf, LCS_MIN_POINTS, COUPLING_THRESHOLD,
};
pub use params::OntogenesisParams;
pub use metamorphosis::{
    Domain, Phase, Metamorphosis, MetamorphosisStrength,
    DomainTransition, CrossDomainResult,
    detect_metamorphoses, cross_domain_trigger, fdr_bh,
    WINDOW_MONTHS, MIN_DOMAINS, FDR_Q, BONFERRONI_ALPHA,
};
