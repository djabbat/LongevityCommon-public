#!/usr/bin/env bash
# Ze Simulator — Phoenix web interface
# Usage: bash run.sh [setup|build|start]
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIMULATOR_DIR="$SCRIPT_DIR/../../simulator"

cmd="${1:-start}"

case "$cmd" in
  setup)
    echo "==> Installing Elixir dependencies..."
    cd "$SCRIPT_DIR" && mix deps.get && mix assets.setup
    echo "==> Building Rust simulator..."
    bash "$SIMULATOR_DIR/build.sh"
    ;;
  build)
    echo "==> Building Rust simulator..."
    bash "$SIMULATOR_DIR/build.sh"
    echo "==> Building Phoenix assets..."
    cd "$SCRIPT_DIR" && mix assets.build
    ;;
  start)
    echo "==> Starting Ze Simulator web interface..."
    echo "    URL: http://localhost:4000"
    cd "$SCRIPT_DIR" && mix phx.server
    ;;
  *)
    echo "Usage: bash run.sh [setup|build|start]"
    exit 1
    ;;
esac
