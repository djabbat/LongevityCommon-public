#!/usr/bin/env bash
# Build ze-runner and install to Phoenix priv/ directory
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PHOENIX_PRIV="$SCRIPT_DIR/../website/ze_sim/priv"

echo "==> Building Ze simulator (Rust)..."
cd "$SCRIPT_DIR"

# Add cargo to PATH if installed via rustup
export PATH="$HOME/.cargo/bin:$PATH"

cargo build --release -p ze-runner

echo "==> Installing ze-runner to Phoenix priv/..."
mkdir -p "$PHOENIX_PRIV"
cp "$SCRIPT_DIR/target/release/ze-runner" "$PHOENIX_PRIV/ze-runner"

echo "==> Done. Binary: $PHOENIX_PRIV/ze-runner"
