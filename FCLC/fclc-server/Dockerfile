# ── Build stage ───────────────────────────────────────────────────────────────
FROM rust:1.78-slim-bookworm AS builder

WORKDIR /app

# Cache dependencies — copy manifests first
COPY Cargo.toml Cargo.lock ./
COPY fclc-core/Cargo.toml fclc-core/
COPY fclc-node/Cargo.toml fclc-node/
COPY fclc-server/Cargo.toml fclc-server/

# Create stub lib/main files so cargo can resolve the workspace
RUN mkdir -p fclc-core/src fclc-node/src fclc-server/src && \
    echo "fn main() {}" > fclc-server/src/main.rs && \
    echo "" > fclc-core/src/lib.rs && \
    echo "fn main() {}" > fclc-node/src/main.rs

RUN cargo build --release --package fclc-server 2>/dev/null; true

# Now copy actual source and build for real
COPY fclc-core/ fclc-core/
COPY fclc-server/ fclc-server/

RUN touch fclc-server/src/main.rs && \
    cargo build --release --package fclc-server

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/fclc-server /app/fclc-server
COPY fclc-server/migrations/ /app/migrations/

EXPOSE 3000

CMD ["/app/fclc-server"]
