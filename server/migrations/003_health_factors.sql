-- Migration 003: Health Factors (4-factor health model)
-- Organism (χ_Ze) is already tracked in ze_samples.
-- This table covers the other 3 factors: psyche, consciousness, social.

CREATE TABLE health_factors (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    recorded_at     TIMESTAMPTZ NOT NULL,
    -- Psyche: emotional state [0.0, 1.0]
    psyche_score    DOUBLE PRECISION CHECK (psyche_score BETWEEN 0.0 AND 1.0),
    psyche_mood     TEXT CHECK (psyche_mood IN ('very_good','good','neutral','bad','very_bad')),
    psyche_stress   DOUBLE PRECISION CHECK (psyche_stress BETWEEN 0.0 AND 1.0),
    psyche_notes    TEXT,
    -- Consciousness: mindfulness, goals, meaning [0.0, 1.0]
    consciousness_score      DOUBLE PRECISION CHECK (consciousness_score BETWEEN 0.0 AND 1.0),
    consciousness_mindful    DOUBLE PRECISION CHECK (consciousness_mindful BETWEEN 0.0 AND 1.0),
    consciousness_purpose    DOUBLE PRECISION CHECK (consciousness_purpose BETWEEN 0.0 AND 1.0),
    consciousness_notes      TEXT,
    -- Social: quality of social connections [0.0, 1.0]
    social_score     DOUBLE PRECISION CHECK (social_score BETWEEN 0.0 AND 1.0),
    social_support   DOUBLE PRECISION CHECK (social_support BETWEEN 0.0 AND 1.0),
    social_isolation DOUBLE PRECISION CHECK (social_isolation BETWEEN 0.0 AND 1.0),
    social_notes     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_health_factors_user_time ON health_factors(user_id, recorded_at DESC);

-- Integrated health score view (materialized on read, not stored)
-- health_score = 0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social
-- organism comes from ze_samples.chi_ze_combined (latest 90d average)
