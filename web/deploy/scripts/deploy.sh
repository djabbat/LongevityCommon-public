#!/bin/bash
# deploy.sh — build web SPA and rsync to /var/www/longevitycommon-web/.
# nginx serves static files; /api/ proxied to social-server :8080;
# /realtime/ proxied to realtime :4500. See deploy/nginx/app.longevity.ge.conf.
#
# Per DEPLOY_CONVENTION.md.

set -euo pipefail

REPO_ROOT="${REPO_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
WEB_DIR="$REPO_ROOT/web"
TARGET_DIR="${TARGET_DIR:-/var/www/longevitycommon-web}"

echo "[1/3] building SPA…"
cd "$WEB_DIR"
npm install --no-audit --no-fund
npm run build

echo "[2/3] rsync dist/ → $TARGET_DIR"
sudo install -d -o www-data -g www-data -m 0755 "$TARGET_DIR"
sudo rsync -a --delete dist/ "$TARGET_DIR/"
sudo chown -R www-data:www-data "$TARGET_DIR"

echo "[3/3] reload nginx"
sudo /usr/sbin/nginx -t
sudo systemctl reload nginx

echo "✓ web deployed. Visit https://app.longevity.ge/"
