#!/bin/bash
# deploy.sh — build mix release and (re)install systemd unit.
# Per DEPLOY_CONVENTION.md.

set -euo pipefail

REPO_ROOT="${REPO_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
RT_DIR="$REPO_ROOT/realtime"
UNIT_NAME="longevitycommon-realtime.service"
UNIT_SRC="$RT_DIR/deploy/systemd/$UNIT_NAME"
UNIT_DST="/etc/systemd/system/$UNIT_NAME"

echo "[1/4] building mix release…"
cd "$RT_DIR"
MIX_ENV=prod mix deps.get --only prod
MIX_ENV=prod mix compile
MIX_ENV=prod mix release --overwrite

echo "[2/4] installing systemd unit"
sudo install -m 0644 "$UNIT_SRC" "$UNIT_DST"
sudo systemctl daemon-reload

echo "[3/4] enable + (re)start"
sudo systemctl enable "$UNIT_NAME"
sudo systemctl restart "$UNIT_NAME"

echo "[4/4] smoke /healthz"
sleep 3
if curl -sf http://127.0.0.1:4500/healthz > /dev/null; then
    echo "  ✓ realtime healthy on 127.0.0.1:4500"
else
    echo "  ✗ healthz FAILED — check 'journalctl -u $UNIT_NAME -n 60'" >&2
    exit 1
fi
