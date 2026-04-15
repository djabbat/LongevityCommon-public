# FCLC — Federated Clinical Learning Cooperative

A privacy-preserving federated learning platform for clinical AI, built in Rust and Elixir.

> **Status:** v0.1.0-alpha · All 13 API endpoints verified live · 38/38 tests pass · **Pilot ready** (2026-04-06)
> **Grant status:** CONCEPT v6.0 finalised — **ГОТОВО К ПОДАЧЕ** EIC Pathfinder Open (deadline 12 May 2026).
> PI: Jaba Tkemaladze (ORCID: 0000-0001-8651-7243) · Co-PI: Giorgi Tsomaia (WP2+WP4)

## What it does

FCLC enables hospitals and clinics to collaboratively train AI models on patient data **without sharing any raw records**. Each institution trains locally; only privacy-protected gradient updates leave the clinic.

**Privacy guarantees (5-layer stack):**
- **L1** Direct identifier removal (name, MRN, exact DOB, address)
- **L2** Quasi-identifier generalization (age bins, decade dates, HbA1c rounding)
- **L3** k-anonymity (k≥5) per (age_group, sex) cell
- **L4** Rényi DP-SGD (ε=2.0/round, δ=1e-5) — **Rényi accounting active**: ~1.985ε saved/round (~30–40 rounds vs 5 with linear)
- **L5** SecAgg+ additive masking — orchestrator sees only aggregate

**Fairness:** Shapley value scoring (Monte Carlo, M=150) rewards each node proportionally to its actual contribution to model quality.

**Robustness:** Krum algorithm tolerates up to 25% Byzantine (malicious or faulty) nodes per round.

---

## Architecture

```
fclc-core/        Rust library — DP engine, Shapley, FedProx/Krum, OMOP schema
fclc-node/        Rust binary + egui GUI — local clinic node (de-id preview, retry logic)
fclc-server/      Rust binary + Axum REST API — central orchestrator
fclc-web/         Elixir/Phoenix LiveView — web dashboard
```

See [MAP.md](MAP.md) for full component interaction diagram and data flow.

---

## Quick Start

### Prerequisites

- Rust 1.77+ (`rustup update`)
- PostgreSQL 15+
- Elixir 1.16+ / Phoenix 1.7+ (for fclc-web)
- Python 3.10+ (for data tools and OMOP scripts — see [Python venv setup](#python-venv-setup) below)

### Build

```bash
# Clone
git clone https://github.com/djabbat/FCLC.git
cd FCLC

# Build Rust workspace
cargo build --workspace --release

# Run fclc-server (orchestrator)
DATABASE_URL=postgres://localhost/fclc cargo run -p fclc-server

# Run fclc-node (local clinic GUI)
cargo run -p fclc-node
```

### Database setup

```bash
# Create database + user
createdb fclc
psql -c "CREATE USER fclc WITH PASSWORD 'fclc';"
psql -c "ALTER DATABASE fclc OWNER TO fclc;"

# Or via Docker (existing postgres container):
docker exec postgres psql -U postgres -c "CREATE USER fclc WITH PASSWORD 'fclc';"
docker exec postgres psql -U postgres -c "CREATE DATABASE fclc OWNER fclc;"

# Migrations run automatically on server startup (sqlx::migrate!)
# Or manually:
psql postgres://fclc:fclc@localhost:5432/fclc < fclc-server/migrations/001_init.sql
psql postgres://fclc:fclc@localhost:5432/fclc < fclc-server/migrations/002_audit_log.sql
```

### Generate demo data (no real patients needed)

```bash
cd FCLC
python3 scripts/generate_demo_data.py --nodes 3 --records 500 --seed 42 --out data/
# Creates: data/clinic_node1_demo.csv, clinic_node2_demo.csv, clinic_node3_demo.csv
# Load via fclc-node GUI: Data tab → CSV path
```

### Web dashboard (fclc-web)

```bash
cd fclc-web
mix deps.get
mix phx.server
# Open http://localhost:4000
```

### Docker Compose (server + web + PostgreSQL)

```bash
# Build and start all services
docker compose up --build

# Services:
#   fclc-server  → http://localhost:3000
#   fclc-web     → http://localhost:4000
#   PostgreSQL   → localhost:5432

# Environment variables (set in .env or shell):
DATABASE_URL=postgres://fclc:fclc@db:5432/fclc
FCLC_API_TOKEN=your-secret-token
SECRET_KEY_BASE=...   # Phoenix: mix phx.gen.secret
```

### Python venv setup

Python используется для вспомогательных инструментов: OMOP ETL-скрипты, валидация датасетов, peer review пайплайны (DeepSeek API), анализ результатов федерации.

```bash
# Создать venv (один раз)
python3 -m venv venv

# Активировать
source venv/bin/activate          # Linux/macOS
# venv\Scripts\activate           # Windows

# Установить зависимости
pip install -r requirements-python.txt
```

**Что должно быть в `venv` (файл `requirements-python.txt`):**

| Пакет | Версия | Назначение |
|-------|--------|-----------|
| `openai` | ≥1.14.0 | DeepSeek API (OpenAI-compatible) — peer review, анализ |
| `python-dotenv` | ≥1.0.0 | Загрузка `~/.aim_env` (DEEPSEEK_API_KEY) |
| `pandas` | ≥2.0.0 | OMOP CDM: обработка таблиц person, condition_occurrence, measurement |
| `sqlalchemy` | ≥2.0.0 | Подключение к PostgreSQL для OMOP валидации |
| `psycopg2-binary` | ≥2.9.0 | PostgreSQL драйвер |
| `pydantic` | ≥2.0.0 | Валидация OMOP-схем, конфигурация |
| `requests` | ≥2.31.0 | HTTP-клиент для REST API fclc-server |
| `numpy` | ≥1.26.0 | Обработка градиентов, Shapley-аппроксимации |
| `scikit-learn` | ≥1.4.0 | AUC/ROC расчёт для MVP-валидации (T2DM AUC>0.75) |
| `matplotlib` | ≥3.8.0 | Визуализация результатов раундов |
| `pdfplumber` | ≥0.10.0 | Извлечение данных из медицинских PDF |
| `pytest` | ≥8.0.0 | Тесты Python-скриптов |

> **Примечание:** `venv/` добавлен в `.gitignore`. Коммитить `requirements-python.txt`, не сам `venv/`. После клонирования репо: `python3 -m venv venv && source venv/bin/activate && pip install -r requirements-python.txt`.

---

## Configuration

Key parameters are documented in [PARAMS.md](PARAMS.md). The most important:

| Parameter | Default | File |
|-----------|---------|------|
| DP ε per round | 2.0 | `fclc-server/config.toml` |
| DP δ | 1e-5 | `fclc-server/config.toml` |
| FedProx μ | 0.1 | `fclc-server/config.toml` |
| Krum Byzantine fraction | 0.25 | `fclc-server/config.toml` |
| Shapley MC samples | 150 | `fclc-server/config.toml` |
| k-anonymity k | 5 | `fclc-node/config.toml` |

---

## REST API (fclc-server)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/nodes/register` | Register a new clinic node |
| POST | `/api/nodes/{id}/update` | Submit gradient update for current round |
| GET | `/api/model/current` | Download current global model weights |
| GET | `/api/rounds/{id}` | Get round metadata and results |
| GET | `/api/nodes/{id}/score` | Get Shapley score history for a node |
| GET | `/api/metrics` | Aggregated training metrics JSON (for fclc-web) |
| GET | `/metrics` | Prometheus scrape endpoint (text/plain; version=0.0.4) |
| GET | `/api/audit` | Hash-chain audit log (tamper-evident round history) |

Full OpenAPI spec: `fclc-server/openapi.yaml` (TODO)

---

## Crate Structure

```
fclc-core/src/
├── dp/           Differential privacy: Gaussian mechanism, Rényi accountant
├── scoring/      Shapley value estimation (Monte Carlo)
├── aggregation/  FedProx weighted average + Krum robust selection
├── schema/       OMOP CDM structs (OmopRecord, LabResult, Medication…)
└── privacy/      De-identification, k-anonymity, quasi-identifier generalization

fclc-node/src/
├── app.rs        egui application loop (3 tabs: Dashboard / Data / Training)
├── pipeline/     De-identification + OMOP normalization + local training
├── connector/    CSV and FHIR JSON importers
└── client/       reqwest HTTP client → fclc-server

fclc-server/src/
├── main.rs       Axum router, tokio runtime
├── routes/       REST endpoint handlers
├── orchestrator/ Round logic: collect → Krum → FedProx → Shapley → persist
└── db/           sqlx queries against PostgreSQL
```

---

## Data Flow (summary)

1. **Clinic node** imports CSV/FHIR → de-identification preview (user confirms) → normalizes to OMOP → trains locally with DP-SGD
2. **SecAgg+ masked update** → POST to orchestrator (automatic retry ×3, exponential backoff 1 s / 2 s / 4 s on network errors)
3. **Orchestrator** runs Krum (reject Byzantine) → FedProx aggregation → Shapley scoring → saves to PostgreSQL
4. **New global model** distributed to all nodes
5. **fclc-web** dashboard reads metrics via REST → displays in LiveView (Shapley bar chart with colour coding per node)

Full diagram: [MAP.md](MAP.md)

---

## Privacy Model

Layer 1: Direct identifier removal (name, MRN, address, exact DOB)
Layer 2: Quasi-identifier generalization (age → 5-yr bin, rare Dx → "other")
Layer 3: k-anonymity (k≥5; groups below threshold suppressed)
Layer 4: DP-SGD (Gaussian noise, ε=2.0/round, δ=1e-5)
Layer 5: SecAgg+ (orchestrator sees only the masked sum)

---

## Legal & Compliance

- **Georgian PDPL (2023):** Personal data processing requires explicit consent + data minimization
- **GDPR Article 9:** Special category health data; requires DPA agreement between nodes and orchestrator
- **DUA template:** Required before any node joins; stored in `legal/DUA_template.docx` (TODO)

---

## References

- McMahan et al. (2017). Communication-efficient learning of deep networks from decentralized data. *AISTATS.*
- Bonawitz et al. (2022). Federated learning and privacy. *CACM.*
- Wang et al. (2020). Principled evaluation of fairness metrics for federated learning. *AAAI.*
- Li et al. (2020). FedProx: Federated optimization for heterogeneous networks. *MLSys.*
- Blanchard et al. (2017). Machine learning with adversaries: Byzantine tolerant gradient descent. *NIPS.*
- Abadi et al. (2016). Deep learning with differential privacy. *CCS.*

---

## License

Apache 2.0 — see `LICENSE`.

---

## Contact

Georgia Longevity Alliance · djabbat@gmail.com
Project public page: https://github.com/djabbat/FCLC-public
