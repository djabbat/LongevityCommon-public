#!/usr/bin/env bash
# BioSense — Main launcher
# Usage: ./biosense.sh [command]
#
# Commands:
#   demo        — Run Ze demo (synthetic data, no dataset needed)
#   cuban       — Analyze Cuban Normative EEG lifespan curve (N=196)
#   dortmund    — Analyze Dortmund young vs old (N=60)
#   lemon       — Analyze MPI-LEMON alpha peak (N=30)
#   eceo        — Analyze EC vs EO within-subject (Zenodo 3875159)
#   [no args]   — Interactive menu

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$SCRIPT_DIR/src"

# Check Python
if ! command -v python3 &>/dev/null; then
    echo "ERROR: python3 not found"
    exit 1
fi

# Check dependencies
check_deps() {
    python3 -c "import mne, numpy, scipy, matplotlib" 2>/dev/null || {
        echo "Installing dependencies..."
        pip install -r "$SRC_DIR/requirements.txt"
    }
}

run_demo() {
    echo "=== BioSense Ze Demo (synthetic data) ==="
    python3 "$SRC_DIR/eeg_ze_processor.py" --demo
}

run_cuban() {
    echo "=== Cuban Normative EEG — Lifespan Curve (N=196) ==="
    if [ -z "$ZE_CUBAN_DIR" ]; then
        ZE_CUBAN_DIR="$SCRIPT_DIR/data/cuban/oldgandalf-FirstWaveCubanHumanNormativeEEGProject-3783da7/EyesClose"
    fi
    python3 "$SRC_DIR/ze_cuban_analysis.py"
}

run_dortmund() {
    echo "=== Dortmund Vital Study — Young vs Old (N=60) ==="
    if [ -z "$ZE_DORTMUND_DIR" ]; then
        echo "Set ZE_DORTMUND_DIR to the Dortmund BIDS root directory"
        exit 1
    fi
    python3 "$SRC_DIR/ze_dortmund_pipeline.py"
}

run_lemon() {
    echo "=== MPI-LEMON — Alpha Peak Ze (N=30) ==="
    if [ -z "$ZE_LEMON_DIR" ]; then
        ZE_LEMON_DIR="$SCRIPT_DIR/data/lemon"
    fi
    python3 "$SRC_DIR/ze_alpha_peak.py"
}

run_eceo() {
    echo "=== Zenodo 3875159 — EC vs EO Within-Subject ==="
    if [ -z "$ZE_ZENODO_VHDR" ]; then
        ZE_ZENODO_VHDR="$SCRIPT_DIR/data/zenodo/360.vhdr"
    fi
    python3 "$SRC_DIR/ze_ec_eo_analysis.py"
}

show_menu() {
    echo ""
    echo "╔══════════════════════════════════════════════╗"
    echo "║          BioSense — Ze EEG Analysis          ║"
    echo "╠══════════════════════════════════════════════╣"
    echo "║  1. Demo (synthetic, no data needed)         ║"
    echo "║  2. Cuban lifespan curve (N=196)             ║"
    echo "║  3. Dortmund young vs old (N=60)             ║"
    echo "║  4. MPI-LEMON alpha peak (N=30)              ║"
    echo "║  5. Zenodo EC vs EO (within-subject)         ║"
    echo "║  q. Quit                                     ║"
    echo "╚══════════════════════════════════════════════╝"
    echo ""
    read -rp "Select option: " choice
    case "$choice" in
        1) run_demo ;;
        2) run_cuban ;;
        3) run_dortmund ;;
        4) run_lemon ;;
        5) run_eceo ;;
        q|Q) echo "Bye."; exit 0 ;;
        *) echo "Unknown option: $choice"; exit 1 ;;
    esac
}

check_deps

case "${1:-}" in
    demo)     run_demo ;;
    cuban)    run_cuban ;;
    dortmund) run_dortmund ;;
    lemon)    run_lemon ;;
    eceo)     run_eceo ;;
    "")       show_menu ;;
    *)
        echo "Unknown command: $1"
        echo "Usage: $0 [demo|cuban|dortmund|lemon|eceo]"
        exit 1
        ;;
esac
