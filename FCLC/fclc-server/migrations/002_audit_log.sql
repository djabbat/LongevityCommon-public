-- FCLC Server — audit log (hash-chain)
-- APPEND-ONLY: no UPDATE or DELETE permitted by application logic.
-- Each entry commits to the previous via prev_hash → tamper-evident chain.
-- Genesis prev_hash = '0' × 64 (zero SHA-256 string).

CREATE TABLE IF NOT EXISTS audit_log (
    entry_id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    round_id       UUID NOT NULL REFERENCES rounds(round_id),
    round_number   BIGINT NOT NULL,
    -- SHA-256 of the aggregated model weights after this round (hex string)
    gradient_hash  TEXT NOT NULL,
    mean_auc       DOUBLE PRECISION NOT NULL,
    participating  INT NOT NULL,
    -- SHA-256 of previous entry's entry_hash; genesis = '0' × 64
    prev_hash      TEXT NOT NULL,
    -- SHA-256(round_id || round_number || gradient_hash || prev_hash)
    entry_hash     TEXT NOT NULL UNIQUE,
    recorded_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_round    ON audit_log(round_number);
CREATE INDEX IF NOT EXISTS idx_audit_recorded ON audit_log(recorded_at);
