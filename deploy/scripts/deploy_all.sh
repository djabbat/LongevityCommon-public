#!/bin/bash
# deploy_all.sh — orchestrator that runs all 3 social-layer deploy
# scripts in dependency order:
#   1. server   (Rust Axum, port 8080)        → systemd unit
#   2. realtime (Phoenix mix release, 4500)   → systemd unit
#   3. web      (React SPA static)            → /var/www + nginx reload
#
# Each sub-script is idempotent and safe to re-run.
# Per DEPLOY_CONVENTION.md.

set -euo pipefail

REPO_ROOT="${REPO_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"

echo "═══ 1/3  social-server ═══"
"$REPO_ROOT/server/deploy/scripts/migrate.sh"
sudo install -m 0644 \
    "$REPO_ROOT/server/deploy/systemd/longevitycommon-server.service" \
    /etc/systemd/system/longevitycommon-server.service
sudo systemctl daemon-reload
sudo systemctl enable --now longevitycommon-server.service
sudo systemctl restart longevitycommon-server.service
sleep 2
curl -sf http://127.0.0.1:8080/health > /dev/null \
    && echo "  ✓ server :8080 healthy" \
    || { echo "  ✗ server unhealthy" >&2; exit 1; }

echo "═══ 2/3  realtime ═══"
"$REPO_ROOT/realtime/deploy/scripts/deploy.sh"

echo "═══ 3/3  web SPA ═══"
"$REPO_ROOT/web/deploy/scripts/deploy.sh"

echo
echo "═══ nginx site (app.longevity.ge) ═══"
sudo install -m 0644 \
    "$REPO_ROOT/deploy/nginx/app.longevity.ge.conf" \
    /etc/nginx/sites-available/app.longevity.ge.conf
sudo ln -sf /etc/nginx/sites-available/app.longevity.ge.conf \
            /etc/nginx/sites-enabled/app.longevity.ge.conf
sudo /usr/sbin/nginx -t
sudo systemctl reload nginx

echo
echo "All deployed. Smoke:"
echo "  curl -I https://app.longevity.ge/"
echo "  curl -sf https://app.longevity.ge/api/health"
echo "  Browser → https://app.longevity.ge/  (SPA loads, login works)"
