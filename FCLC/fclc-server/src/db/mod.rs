use anyhow::Result;
use chrono::DateTime;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::{AuditEntry, NodeInfo, NodeScore, RoundResult};

// ── Node CRUD ─────────────────────────────────────────────────────────────────

/// Insert a new node into the `nodes` table.
/// If the node_id already exists this is a no-op (ON CONFLICT DO NOTHING).
pub async fn insert_node(pool: &PgPool, node_id: Uuid, node_name: &str) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO nodes (node_id, node_name, registered_at, epsilon_spent)
        VALUES ($1, $2, NOW(), 0.0)
        ON CONFLICT (node_id) DO NOTHING
        "#,
    )
    .bind(node_id)
    .bind(node_name)
    .execute(pool)
    .await?;
    Ok(())
}

/// Return all registered nodes.
pub async fn list_nodes(pool: &PgPool) -> Result<Vec<NodeInfo>> {
    let rows = sqlx::query(
        r#"
        SELECT node_id, node_name, epsilon_spent,
               registered_at
        FROM nodes
        ORDER BY registered_at ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let nodes = rows
        .iter()
        .map(|r| {
            let ts: DateTime<chrono::Utc> = r.get("registered_at");
            NodeInfo {
                node_id: r.get("node_id"),
                node_name: r.get("node_name"),
                epsilon_spent: r.get("epsilon_spent"),
                registered_at: ts.to_rfc3339(),
            }
        })
        .collect();

    Ok(nodes)
}

/// Update cumulative epsilon for a node.
pub async fn update_node_epsilon(pool: &PgPool, node_id: Uuid, epsilon_spent: f64) -> Result<()> {
    sqlx::query(r#"UPDATE nodes SET epsilon_spent = $1 WHERE node_id = $2"#)
        .bind(epsilon_spent)
        .bind(node_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ── Update storage ────────────────────────────────────────────────────────────

/// Persist a node gradient update for the given round.
pub async fn insert_update(
    pool: &PgPool,
    node_id: Uuid,
    round_number: u64,
    epsilon_spent: f64,
    loss: f64,
    auc: f64,
    record_count: usize,
) -> Result<()> {
    let round_number_i64 = round_number as i64;
    let record_count_i64 = record_count as i64;
    sqlx::query(
        r#"
        INSERT INTO updates (update_id, node_id, round_number, epsilon_spent, loss, auc, record_count, submitted_at)
        VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6, NOW())
        "#,
    )
    .bind(node_id)
    .bind(round_number_i64)
    .bind(epsilon_spent)
    .bind(loss)
    .bind(auc)
    .bind(record_count_i64)
    .execute(pool)
    .await?;
    Ok(())
}

// ── Round result persistence ──────────────────────────────────────────────────

/// Persist a completed round result.
pub async fn insert_round(
    pool: &PgPool,
    round_id: Uuid,
    round_number: u64,
    auc: f64,
    participating_nodes: usize,
) -> Result<()> {
    let round_number_i64 = round_number as i64;
    let participating_nodes_i32 = participating_nodes as i32;
    sqlx::query(
        r#"
        INSERT INTO rounds (round_id, round_number, auc, participating_nodes, completed_at)
        VALUES ($1, $2, $3, $4, NOW())
        "#,
    )
    .bind(round_id)
    .bind(round_number_i64)
    .bind(auc)
    .bind(participating_nodes_i32)
    .execute(pool)
    .await?;
    Ok(())
}

/// List all rounds ordered by round_number ascending.
pub async fn list_rounds(pool: &PgPool) -> Result<Vec<RoundResult>> {
    let rows = sqlx::query(
        r#"
        SELECT round_id, round_number, auc, participating_nodes,
               completed_at
        FROM rounds
        ORDER BY round_number ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let results = rows
        .iter()
        .map(|r| {
            let ts: DateTime<chrono::Utc> = r.get("completed_at");
            RoundResult {
                round_id: r.get("round_id"),
                round_number: r.get::<i64, _>("round_number") as u64,
                auc: r.get("auc"),
                participating_nodes: r.get::<i32, _>("participating_nodes") as usize,
                timestamp: ts.to_rfc3339(),
            }
        })
        .collect();

    Ok(results)
}

/// Fetch a single round by UUID.
pub async fn get_round(pool: &PgPool, round_id: Uuid) -> Result<Option<RoundResult>> {
    let row = sqlx::query(
        r#"
        SELECT round_id, round_number, auc, participating_nodes,
               completed_at
        FROM rounds
        WHERE round_id = $1
        "#,
    )
    .bind(round_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let ts: DateTime<chrono::Utc> = r.get("completed_at");
        RoundResult {
            round_id: r.get("round_id"),
            round_number: r.get::<i64, _>("round_number") as u64,
            auc: r.get("auc"),
            participating_nodes: r.get::<i32, _>("participating_nodes") as usize,
            timestamp: ts.to_rfc3339(),
        }
    }))
}

// ── Shapley score storage ─────────────────────────────────────────────────────

/// Persist Shapley scores for all nodes in a given round.
pub async fn insert_shapley_scores(
    pool: &PgPool,
    round_number: u64,
    scores: &[(Uuid, f64)],
) -> Result<()> {
    let round_number_i64 = round_number as i64;
    for (node_id, score) in scores {
        sqlx::query(
            r#"
            INSERT INTO shapley_scores (score_id, node_id, round_number, shapley_score, computed_at)
            VALUES (gen_random_uuid(), $1, $2, $3, NOW())
            "#,
        )
        .bind(node_id)
        .bind(round_number_i64)
        .bind(score)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Fetch Shapley score history for a given node.
pub async fn get_shapley_history(pool: &PgPool, node_id: Uuid) -> Result<Vec<NodeScore>> {
    let rows = sqlx::query(
        r#"
        SELECT node_id, round_number, shapley_score
        FROM shapley_scores
        WHERE node_id = $1
        ORDER BY round_number ASC
        "#,
    )
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    let scores = rows
        .iter()
        .map(|r| NodeScore {
            node_id: r.get("node_id"),
            shapley_score: r.get("shapley_score"),
            round: r.get::<i64, _>("round_number") as u64,
        })
        .collect();

    Ok(scores)
}

/// Return average Shapley score across all nodes and rounds.
pub async fn avg_shapley(pool: &PgPool) -> Result<f64> {
    let row = sqlx::query(r#"SELECT COALESCE(AVG(shapley_score), 0.0) AS avg FROM shapley_scores"#)
        .fetch_one(pool)
        .await?;
    let avg: f64 = row.get("avg");
    Ok(avg)
}

// ── Audit log (hash-chain) ────────────────────────────────────────────────────

/// Genesis prev_hash: 64 zeroes (SHA-256 of nothing — chain anchor).
pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Return the `entry_hash` of the most recent audit entry, or GENESIS_HASH if none.
pub async fn get_latest_audit_hash(pool: &PgPool) -> Result<String> {
    let row = sqlx::query(
        r#"SELECT entry_hash FROM audit_log ORDER BY recorded_at DESC LIMIT 1"#,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.get::<String, _>("entry_hash")).unwrap_or_else(|| GENESIS_HASH.to_string()))
}

/// Append one entry to the audit log (APPEND-ONLY — no update/delete).
pub async fn insert_audit_entry(
    pool: &PgPool,
    round_id: uuid::Uuid,
    round_number: u64,
    gradient_hash: &str,
    mean_auc: f64,
    participating: usize,
    prev_hash: &str,
    entry_hash: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO audit_log
            (round_id, round_number, gradient_hash, mean_auc, participating,
             prev_hash, entry_hash, recorded_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
        "#,
    )
    .bind(round_id)
    .bind(round_number as i64)
    .bind(gradient_hash)
    .bind(mean_auc)
    .bind(participating as i32)
    .bind(prev_hash)
    .bind(entry_hash)
    .execute(pool)
    .await?;
    Ok(())
}

/// Return the full audit chain ordered by round_number ascending.
pub async fn get_audit_chain(pool: &PgPool) -> Result<Vec<AuditEntry>> {
    let rows = sqlx::query(
        r#"
        SELECT entry_id, round_id, round_number, gradient_hash, mean_auc,
               participating, prev_hash, entry_hash,
               recorded_at
        FROM audit_log
        ORDER BY round_number ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let entries = rows
        .iter()
        .map(|r| {
            let ts: DateTime<chrono::Utc> = r.get("recorded_at");
            AuditEntry {
                entry_id:      r.get("entry_id"),
                round_id:      r.get("round_id"),
                round_number:  r.get::<i64, _>("round_number") as u64,
                gradient_hash: r.get("gradient_hash"),
                mean_auc:      r.get("mean_auc"),
                participating: r.get::<i32, _>("participating") as usize,
                prev_hash:     r.get("prev_hash"),
                entry_hash:    r.get("entry_hash"),
                recorded_at:   ts.to_rfc3339(),
            }
        })
        .collect();

    Ok(entries)
}
