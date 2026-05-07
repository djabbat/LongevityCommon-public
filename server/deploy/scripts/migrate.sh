#!/bin/bash
# migrate.sh — idempotent migration runner for longevitycommon_social.
#
# Reads DATABASE_URL from env (or /etc/aim/lc_social.env on server).
# Each migration in server/migrations/*.sql is applied in lex order;
# already-applied migrations are skipped via a sentinel table
# `_migrations(name TEXT PRIMARY KEY, applied_at TIMESTAMPTZ DEFAULT NOW())`.
#
# Per DEPLOY_CONVENTION.md (~/Desktop/LongevityCommon/docs/).

set -euo pipefail

REPO_ROOT="${REPO_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
MIG_DIR="$REPO_ROOT/migrations"
DB_URL="${DATABASE_URL:-}"

if [ -z "$DB_URL" ]; then
    if [ -f /etc/aim/lc_social.env ]; then
        # shellcheck disable=SC1091
        . /etc/aim/lc_social.env
        DB_URL="${DATABASE_URL:-}"
    fi
fi

if [ -z "$DB_URL" ]; then
    echo "FAIL: DATABASE_URL not set, and /etc/aim/lc_social.env missing or no DATABASE_URL inside." >&2
    exit 1
fi

echo "Migration target: ${DB_URL//:*@/:****@}"

# 0. Ensure sentinel table exists.
psql "$DB_URL" -v ON_ERROR_STOP=1 <<'SQL' >/dev/null
CREATE TABLE IF NOT EXISTS _migrations (
    name        TEXT PRIMARY KEY,
    applied_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SQL

applied=0
skipped=0
for f in "$MIG_DIR"/*.sql; do
    name=$(basename "$f")
    if psql "$DB_URL" -tAc "SELECT 1 FROM _migrations WHERE name = '$name'" | grep -q 1; then
        echo "  ✓ skip (already applied): $name"
        skipped=$((skipped + 1))
        continue
    fi
    echo "  → applying: $name"
    psql "$DB_URL" -v ON_ERROR_STOP=1 -f "$f" >/dev/null
    psql "$DB_URL" -v ON_ERROR_STOP=1 \
         -c "INSERT INTO _migrations(name) VALUES('$name')" >/dev/null
    applied=$((applied + 1))
done

echo
echo "Migration complete. applied=$applied  skipped=$skipped"
