pub mod dp;
pub mod model;
pub mod scoring;
pub mod aggregation;
pub mod schema;
pub mod privacy;
pub mod mobile_node;

pub use dp::{DpConfig, LinearDpAccountant, DpError, gaussian_noise_sigma};
pub use dp::renyi::{RdpAccountant, RdpError, rdp_gaussian, rdp_gaussian_subsampled, rdp_to_dp};
pub use scoring::ShapleyScorer;
pub use aggregation::{
    fedprox_aggregate, krum_select,
    // SecAgg+ v2 — cryptographic pairwise mask cancellation
    NodeKeypair, ShamirShare,
    secagg_apply_masks, secagg_aggregate,
    expand_seed_to_mask, chacha20_pairwise_mask,
    shamir_split_gf257, shamir_reconstruct_gf257,
};
pub use model::{FederatedModel, LogisticRegressionModel, GradientUpdate,
                SamplePrediction, SubgroupMetrics, FairnessAgeGroup, FairnessSex, FairnessReport,
                evaluate_age_group_fairness, evaluate_sex_fairness, PateConfig,
                PhiDForm, phi_d_selector, PateVsDpSgdComparison,
                NonIidSimConfig, DpSensitivityBudget,
                DpCompositionSummary, PrivacyDefenseInDepth, privacy_defense_stack,
                MembershipInferenceAudit, MiaAttackType,
                IrbStatus, DatasetEthicsStatus, dataset_ethics_catalogue,
                // v10: ISO/IEC 27559 compliance audit, EEG preprocessing, IUS
                DpPrivacyStandard, DpComplianceAudit,
                EegPreprocessingSpec, IntendedUseStatement,
                // v11: ISO-compliant DP config, χ_Ze Phase 2 validation protocol
                DpIsoCompliantConfig, BiomarkerPhase, ChiZeValidationStudy};
pub use schema::{OmopRecord, anonymize_record};
pub use privacy::{DeidentConfig, deidentify_batch};
