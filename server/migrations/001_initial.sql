-- LongevityCommon initial schema
-- Compatible with FCLC OMOP CDM conventions

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username        TEXT UNIQUE NOT NULL,
    email           TEXT UNIQUE NOT NULL,
    email_verified  BOOLEAN NOT NULL DEFAULT FALSE,
    otp_code        TEXT,
    otp_expires_at  TIMESTAMPTZ,
    birth_year      INTEGER,
    country_code    CHAR(2),
    orcid_id        TEXT,
    degree_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_pro          BOOLEAN NOT NULL DEFAULT FALSE,
    fclc_node_id    TEXT,
    fclc_node_active BOOLEAN NOT NULL DEFAULT FALSE,
    passkey_cred    JSONB,
    consent_given   BOOLEAN NOT NULL DEFAULT FALSE,
    consent_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_username ON users(username) WHERE deleted_at IS NULL;

-- Ze samples (core biomarker data)
CREATE TABLE ze_samples (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    recorded_at     TIMESTAMPTZ NOT NULL,
    source          TEXT NOT NULL CHECK (source IN ('biosense','apple_health','oura','garmin','manual')),
    chi_ze_eeg      DOUBLE PRECISION,
    chi_ze_hrv      DOUBLE PRECISION,
    chi_ze_combined DOUBLE PRECISION,
    d_norm          DOUBLE PRECISION,
    bio_age_est     DOUBLE PRECISION,
    bio_age_ci_low  DOUBLE PRECISION,
    bio_age_ci_high DOUBLE PRECISION,
    ci_stability    TEXT CHECK (ci_stability IN ('high','medium','low')),
    fclc_signature  TEXT,
    is_verified     BOOLEAN NOT NULL DEFAULT TRUE,
    raw_payload     JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ze_samples_user_time ON ze_samples(user_id, recorded_at DESC);
CREATE INDEX idx_ze_samples_verified ON ze_samples(user_id, is_verified, recorded_at DESC);

-- Interventions log
CREATE TABLE interventions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    recorded_at TIMESTAMPTZ NOT NULL,
    type        TEXT NOT NULL CHECK (type IN ('sleep','exercise','fasting','supplement','other')),
    value       JSONB NOT NULL,
    notes       TEXT,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_interventions_user_time ON interventions(user_id, recorded_at DESC);

-- Posts (social feed)
CREATE TABLE posts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id       UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type            TEXT NOT NULL CHECK (type IN ('ze_log','science_thread','study_invite','debate')),
    content         TEXT NOT NULL CHECK (length(content) >= 10),
    doi             TEXT,
    doi_verified    BOOLEAN NOT NULL DEFAULT FALSE,
    doi_checked_at  TIMESTAMPTZ,
    code_url        TEXT,
    data_url        TEXT,
    score           DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    rank_penalty    DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    parent_id       UUID REFERENCES posts(id),
    study_id        UUID,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited_at       TIMESTAMPTZ,
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_posts_feed ON posts(score DESC, created_at DESC) WHERE deleted_at IS NULL AND parent_id IS NULL;
CREATE INDEX idx_posts_author ON posts(author_id, created_at DESC) WHERE deleted_at IS NULL;

-- Post reactions
CREATE TABLE post_reactions (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id    UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type       TEXT NOT NULL CHECK (type IN ('support','replicate','challenge','cite')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (post_id, user_id, type)
);

-- Studies (citizen science Lab)
CREATE TABLE studies (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creator_id      UUID NOT NULL REFERENCES users(id),
    title           TEXT NOT NULL,
    hypothesis      TEXT NOT NULL,
    protocol        JSONB NOT NULL,
    target_n        INTEGER NOT NULL CHECK (target_n > 0),
    enrolled_n      INTEGER DEFAULT 0,
    duration_days   INTEGER NOT NULL CHECK (duration_days > 0),
    status          TEXT DEFAULT 'recruiting' CHECK (status IN ('draft','recruiting','active','completed','published')),
    dua_template_id TEXT,
    result_doi      TEXT,
    arbiter_id      UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    starts_at       TIMESTAMPTZ,
    ends_at         TIMESTAMPTZ
);

CREATE INDEX idx_studies_status ON studies(status, created_at DESC);

-- Study enrollments (consent records)
CREATE TABLE study_enrollments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    study_id        UUID NOT NULL REFERENCES studies(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    consent_text    TEXT NOT NULL,
    consented_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status          TEXT DEFAULT 'active' CHECK (status IN ('active','withdrawn','completed')),
    shapley_weight  DOUBLE PRECISION DEFAULT 1.0,
    UNIQUE (study_id, user_id)
);

-- Ze·Guide logs (mandatory for legal protection)
CREATE TABLE ze_guide_logs (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id      UUID NOT NULL,
    prompt          TEXT NOT NULL,
    response        TEXT NOT NULL,
    model_used      TEXT NOT NULL,
    cited_dois      TEXT[] DEFAULT '{}',
    cited_files     TEXT[] DEFAULT '{}',
    disclaimer_sent BOOLEAN NOT NULL DEFAULT TRUE,
    latency_ms      INTEGER,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_ze_guide_logs_user ON ze_guide_logs(user_id, created_at DESC);

-- Debates
CREATE TABLE debates (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id     UUID NOT NULL REFERENCES posts(id),
    arbiter_id  UUID REFERENCES users(id),
    status      TEXT DEFAULT 'open' CHECK (status IN ('open','closed','resolved')),
    resolution  TEXT,
    resolved_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE debate_votes (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    debate_id  UUID NOT NULL REFERENCES debates(id),
    voter_id   UUID NOT NULL REFERENCES users(id),
    criterion  TEXT NOT NULL CHECK (criterion IN ('has_data','reproducible','valid_stats')),
    value      BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (debate_id, voter_id, criterion)
);
