-- Migration 004: Add validated HRV organism biomarkers to ze_samples
--
-- REASON: BioSense CONCEPT v3.2 (2026-04-12) — χ_Ze_eeg and χ_Ze_hrv failed
-- empirical validation (4 consecutive nulls). Interim organism score is now
-- SDNN + RMSSD, validated on PhysioNet Fantasia N=40 (d=0.724, p=0.028, BCa).
-- Reference: DOI 10.65649/a184qf96 (null series) + CONCEPT.md §ze_samples.

ALTER TABLE ze_samples
    ADD COLUMN IF NOT EXISTS sdnn_ms         DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS rmssd_ms        DOUBLE PRECISION,
    ADD COLUMN IF NOT EXISTS organism_sdnn   DOUBLE PRECISION;

-- organism_sdnn: normalised SDNN score ∈ [0, 1] for 4-factor health model
-- Normalisation reference range: SDNN 10–180 ms (clinical literature);
-- organism_sdnn = clamp((sdnn_ms - 10) / 170, 0, 1)
-- Stored pre-computed to avoid re-normalisation at query time.

COMMENT ON COLUMN ze_samples.sdnn_ms IS
    'SDNN (ms) — validated HRV aging biomarker (Fantasia N=40, d=0.724, p=0.028). '
    'Replaces chi_ze_eeg as interim organism score per CONCEPT v3.2.';

COMMENT ON COLUMN ze_samples.rmssd_ms IS
    'RMSSD (ms) — parasympathetic HRV metric; secondary organism indicator.';

COMMENT ON COLUMN ze_samples.organism_sdnn IS
    'Normalised organism score from SDNN: clamp((sdnn_ms - 10) / 170, 0, 1). '
    'Used as W_ORGANISM component in 4-factor health score when chi_ze_combined is NULL.';
