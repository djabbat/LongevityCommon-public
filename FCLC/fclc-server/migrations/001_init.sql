-- FCLC Server — initial schema
-- Run with: sqlx migrate run

-- Registered federated nodes
CREATE TABLE IF NOT EXISTS nodes (
    node_id       UUID PRIMARY KEY,
    node_name     TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    epsilon_spent DOUBLE PRECISION NOT NULL DEFAULT 0.0
);

-- Aggregation rounds
CREATE TABLE IF NOT EXISTS rounds (
    round_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    round_number        BIGINT NOT NULL,
    auc                 DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    participating_nodes INT NOT NULL DEFAULT 0,
    completed_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Per-node updates submitted per round
CREATE TABLE IF NOT EXISTS updates (
    update_id    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id      UUID NOT NULL REFERENCES nodes(node_id),
    round_number BIGINT NOT NULL,
    epsilon_spent DOUBLE PRECISION NOT NULL,
    loss          DOUBLE PRECISION NOT NULL,
    auc           DOUBLE PRECISION NOT NULL,
    record_count  BIGINT NOT NULL,
    submitted_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Shapley contribution scores per node per round
CREATE TABLE IF NOT EXISTS shapley_scores (
    score_id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id       UUID NOT NULL REFERENCES nodes(node_id),
    round_number  BIGINT NOT NULL,
    shapley_score DOUBLE PRECISION NOT NULL,
    computed_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_updates_node   ON updates(node_id);
CREATE INDEX IF NOT EXISTS idx_updates_round  ON updates(round_number);
CREATE INDEX IF NOT EXISTS idx_shapley_node   ON shapley_scores(node_id);
CREATE INDEX IF NOT EXISTS idx_shapley_round  ON shapley_scores(round_number);
CREATE INDEX IF NOT EXISTS idx_rounds_number  ON rounds(round_number);
