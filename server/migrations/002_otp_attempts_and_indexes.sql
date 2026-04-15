-- Migration 002: OTP attempt tracking + performance indexes

-- OTP brute-force protection
ALTER TABLE users ADD COLUMN IF NOT EXISTS otp_attempts INTEGER DEFAULT 0;

-- Performance indexes missing from 001_initial.sql

-- ze_samples: time-range queries (trend endpoint)
CREATE INDEX IF NOT EXISTS idx_ze_samples_recorded_at
    ON ze_samples(recorded_at DESC);

-- ze_samples: cohort percentile query (join on birth_year)
CREATE INDEX IF NOT EXISTS idx_users_birth_year
    ON users(birth_year) WHERE deleted_at IS NULL;

-- ze_samples: verified samples by time (most common access pattern)
CREATE INDEX IF NOT EXISTS idx_ze_samples_user_verified_time
    ON ze_samples(user_id, is_verified, recorded_at DESC);

-- posts: thread queries (parent_id)
CREATE INDEX IF NOT EXISTS idx_posts_parent
    ON posts(parent_id) WHERE deleted_at IS NULL AND parent_id IS NOT NULL;

-- post_reactions: count reactions per post (feed query)
CREATE INDEX IF NOT EXISTS idx_post_reactions_post
    ON post_reactions(post_id, type);

-- study_enrollments: user's enrollments
CREATE INDEX IF NOT EXISTS idx_study_enrollments_user
    ON study_enrollments(user_id, status);

-- ze_guide_logs: session grouping
CREATE INDEX IF NOT EXISTS idx_ze_guide_logs_session
    ON ze_guide_logs(session_id);

-- interventions: type filter + time
CREATE INDEX IF NOT EXISTS idx_interventions_user_type
    ON interventions(user_id, type, recorded_at DESC);
