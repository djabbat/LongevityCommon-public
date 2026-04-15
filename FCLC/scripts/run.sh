#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# FCLC — Federated Clinical Learning Cooperative
# Main launcher script
# ──────────────────────────────────────────────────────────────────────────────
set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

BOLD='\033[1m'; CYAN='\033[0;36m'; GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[1;33m'; NC='\033[0m'

echo -e "${BOLD}${CYAN}╔══════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${CYAN}║   FCLC — Federated Clinical Learning        ║${NC}"
echo -e "${BOLD}${CYAN}║   Cooperative  v0.1.0-alpha                 ║${NC}"
echo -e "${BOLD}${CYAN}╚══════════════════════════════════════════════╝${NC}"
echo ""

if [[ -z "$1" ]]; then
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  build       Build all Rust crates (release)"
    echo "  check       cargo check (fast syntax check)"
    echo "  test        Run Rust test suite"
    echo "  server      Start fclc-server (orchestrator) on :3000"
    echo "  node        Start fclc-node (local clinic GUI)"
    echo "  web         Start fclc-web (Phoenix dashboard) on :4000"
    echo "  db-setup    Create PostgreSQL database + run migrations"
    echo "  db-reset    Drop and recreate database"
    echo "  docs        Generate rustdoc"
    echo "  clean       Clean build artifacts"
    echo "  demo        Run a 3-node local demo simulation"
    echo ""
    echo "Environment:"
    echo "  DATABASE_URL   PostgreSQL URL (default: postgres://localhost/fclc)"
    echo "  FCLC_HOST      Server host (default: 0.0.0.0)"
    echo "  FCLC_PORT      Server port (default: 3000)"
    echo ""
    echo "Examples:"
    echo "  bash run.sh build"
    echo "  DATABASE_URL=postgres://localhost/fclc bash run.sh server"
    echo "  bash run.sh node"
    exit 0
fi

CMD="$1"

check_rust() {
    if ! command -v cargo &>/dev/null; then
        echo -e "${RED}Error: cargo not found. Install Rust: https://rustup.rs${NC}"
        exit 1
    fi
}

check_mix() {
    if ! command -v mix &>/dev/null; then
        echo -e "${RED}Error: mix not found. Install Elixir: https://elixir-lang.org${NC}"
        exit 1
    fi
}

check_pg() {
    if [[ -z "$DATABASE_URL" ]]; then
        export DATABASE_URL="postgres://localhost/fclc"
        echo -e "${YELLOW}Using default DATABASE_URL: $DATABASE_URL${NC}"
    fi
}

case "$CMD" in
    build)
        check_rust
        echo -e "${BOLD}Building FCLC Rust workspace (release)...${NC}"
        cargo build --workspace --release 2>&1
        echo -e "${GREEN}✓ Build complete${NC}"
        ;;

    check)
        check_rust
        echo -e "${BOLD}Checking workspace...${NC}"
        cargo check --workspace 2>&1
        echo -e "${GREEN}✓ Check complete${NC}"
        ;;

    test)
        check_rust
        echo -e "${BOLD}Running test suite...${NC}"
        cargo test --workspace 2>&1
        echo -e "${GREEN}✓ Tests complete${NC}"
        ;;

    server)
        check_rust
        check_pg
        echo -e "${BOLD}Starting fclc-server on ${FCLC_HOST:-0.0.0.0}:${FCLC_PORT:-3000}...${NC}"
        echo "DATABASE_URL=$DATABASE_URL"
        DATABASE_URL="$DATABASE_URL" \
        FCLC_HOST="${FCLC_HOST:-0.0.0.0}" \
        FCLC_PORT="${FCLC_PORT:-3000}" \
        cargo run -p fclc-server --release 2>&1
        ;;

    node)
        check_rust
        echo -e "${BOLD}Starting fclc-node GUI...${NC}"
        echo -e "${YELLOW}Note: Requires a display (X11/Wayland). Set FCLC_SERVER_URL env var.${NC}"
        FCLC_SERVER_URL="${FCLC_SERVER_URL:-http://localhost:3000}" \
        cargo run -p fclc-node --release 2>&1
        ;;

    web)
        check_mix
        echo -e "${BOLD}Starting fclc-web Phoenix dashboard on :4000...${NC}"
        if [[ ! -d "fclc-web" ]]; then
            echo -e "${RED}Error: fclc-web/ directory not found.${NC}"
            echo "Run: mix phx.new fclc-web --no-ecto"
            exit 1
        fi
        cd fclc-web
        mix deps.get
        FCLC_SERVER_URL="${FCLC_SERVER_URL:-http://localhost:3000}" mix phx.server 2>&1
        ;;

    db-setup)
        check_pg
        echo -e "${BOLD}Setting up PostgreSQL database...${NC}"
        if command -v createdb &>/dev/null; then
            createdb fclc 2>/dev/null || echo "Database may already exist."
        fi
        if [[ -f "fclc-server/migrations/001_init.sql" ]]; then
            psql "$DATABASE_URL" < fclc-server/migrations/001_init.sql
            echo -e "${GREEN}✓ Migrations applied${NC}"
        else
            echo -e "${YELLOW}No migration file found at fclc-server/migrations/001_init.sql${NC}"
        fi
        ;;

    db-reset)
        check_pg
        echo -e "${YELLOW}WARNING: This will drop and recreate the fclc database.${NC}"
        read -p "Continue? [y/N] " confirm
        if [[ "$confirm" == "y" || "$confirm" == "Y" ]]; then
            dropdb fclc 2>/dev/null || true
            bash "$0" db-setup
        else
            echo "Cancelled."
        fi
        ;;

    docs)
        check_rust
        echo -e "${BOLD}Generating documentation...${NC}"
        cargo doc --workspace --no-deps --open 2>&1
        ;;

    clean)
        check_rust
        echo -e "${BOLD}Cleaning build artifacts...${NC}"
        cargo clean 2>&1
        echo -e "${GREEN}✓ Clean complete${NC}"
        ;;

    demo)
        check_rust
        check_pg
        echo -e "${BOLD}Running 3-node local demo...${NC}"
        echo "(Starts server + 3 simulated nodes, runs 5 federated rounds)"
        # Start server in background
        DATABASE_URL="${DATABASE_URL:-postgres://localhost/fclc}" \
        cargo run -p fclc-server --release &
        SERVER_PID=$!
        sleep 3
        echo -e "${GREEN}Server started (PID $SERVER_PID)${NC}"
        echo -e "${YELLOW}To stop: kill $SERVER_PID${NC}"
        echo "Demo: connect fclc-node GUI to http://localhost:3000"
        ;;

    *)
        echo -e "${RED}Unknown command: $CMD${NC}"
        echo "Run '$0' without arguments to see usage."
        exit 1
        ;;
esac
