# CommonHealth

> First social network where your biological age is your profile — measured in real time, improved with the community, grounded in peer-reviewed science.

**Author:** Jaba Tkemaladze  
**Status:** MVP v1 — in development  
**Stack:** Rust (Axum) · React/TypeScript PWA · PostgreSQL · Phoenix (realtime, post-MVP)

---

## What is CommonHealth?

CommonHealth is the public face of the Ze / FCLC / BioSense / CDATA ecosystem — a thin social layer that turns four research repositories into a global longevity movement.

Every user gets a **Ze·Profile**: a live biological dashboard driven by χ_Ze (Ze complexity index) and D_norm (bridge equation to biological age). Data flows through the FCLC 5-layer privacy pipeline. Scientists recruit real participants. Users become co-authors of longevity studies.

---

## Repository layout

```
CommonHealth/
├── CONCEPT.md          — approved product concept (v2.0)
├── ARCHITECTURE.md     — technical spec: DB schema, API, data models
├── README.md           — this file
├── docs/
│   ├── API.md          — OpenAPI reference (Swagger)
│   └── DATABASE.md     — full schema with ERD notes
├── server/             — Rust / Axum REST API (MVP backend)
│   ├── Cargo.toml
│   ├── migrations/     — SQL migrations (sqlx)
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       ├── db/
│       ├── models/
│       ├── handlers/
│       ├── middleware/
│       ├── services/
│       └── routes.rs
├── web/                — React + TypeScript PWA (Vite)
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── App.tsx
│       ├── components/
│       ├── pages/
│       ├── hooks/
│       ├── store/
│       └── types/
└── realtime/           — Phoenix / Elixir (WebSocket, post-MVP)
    ├── mix.exs
    └── lib/
```

---

## Quick start (development)

### Prerequisites

- Rust 1.77+ (`rustup update stable`)
- Node.js 20+ / pnpm 9+
- PostgreSQL 16+
- Elixir 1.16+ / Erlang 26+ (for realtime, optional in MVP)

### 1. Database

```bash
psql -U postgres -c "CREATE DATABASE commonhealth;"
cd server
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

### 2. Backend

```bash
cd server
cp .env.example .env   # edit DATABASE_URL, DEEPSEEK_API_KEY, JWT_SECRET
cargo run
# → http://localhost:3000
```

### 3. Frontend

```bash
cd web
pnpm install
pnpm dev
# → http://localhost:5173
```

### 4. Realtime (post-MVP)

```bash
cd realtime
mix deps.get
mix phx.server
# → http://localhost:4000
```

---

## Core concepts

| Term | Meaning |
|------|---------|
| **χ_Ze** | Ze complexity index — primary biomarker (0–1, higher = younger biology) |
| **D_norm** | Normalized biological distance — bridge to chronological age |
| **Ze·Profile** | User's live biological dashboard with 95% CI on bio age |
| **Ze·Guide** | AI assistant (DeepSeek + Llama 3 fallback) for scientific Q&A |
| **FCLC node** | Federated Citizen Longevity Computing node — privacy-preserving data contributor |
| **Lab study** | Citizen science experiment — hypothesis → protocol → data → publication |

---

## Architecture overview

```
┌─────────────────────────────────────────────────────────┐
│  Web PWA (React/TS)          Mobile (React Native, v2)  │
└──────────────────┬──────────────────────────────────────┘
                   │ REST (MVP) / WebSocket (post-MVP)
┌──────────────────▼──────────────────────────────────────┐
│  Rust / Axum API (server/)                               │
│  ┌───────────┐  ┌───────────┐  ┌────────────────────┐  │
│  │  Auth     │  │  Posts    │  │  Ze compute engine │  │
│  │ passkeys  │  │  Feed     │  │  χ_Ze / D_norm     │  │
│  │ email OTP │  │  Ranking  │  │  CI intervals      │  │
│  └───────────┘  └───────────┘  └────────────────────┘  │
│  ┌───────────┐  ┌───────────┐  ┌────────────────────┐  │
│  │ Ze·Guide  │  │  Studies  │  │  Data import       │  │
│  │ DeepSeek  │  │  Lab      │  │  JSON / CSV        │  │
│  │ + Llama3  │  │  Consent  │  │  BioSense/Oura/... │  │
│  └───────────┘  └───────────┘  └────────────────────┘  │
└──────────────────┬──────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────┐
│  PostgreSQL 16   (OMOP CDM compatible schema)            │
└─────────────────────────────────────────────────────────┘
                   │ post-MVP
┌──────────────────▼──────────────────────────────────────┐
│  Phoenix / Elixir  (realtime channels, PubSub)           │
└─────────────────────────────────────────────────────────┘
```

---

## API (summary)

Full spec: `docs/API.md`

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/auth/register` | Register with email OTP |
| POST | `/api/auth/login` | Login, returns JWT |
| GET | `/api/users/:id` | Public Ze·Profile |
| GET | `/api/feed` | Ranked post feed |
| POST | `/api/posts` | Create post (with DOI validation) |
| GET | `/api/dashboard` | Personal χ_Ze dashboard |
| POST | `/api/data/import` | Upload JSON data (BioSense/Oura/Garmin) |
| GET | `/api/data/export` | Download all personal data (GDPR) |
| GET | `/api/studies` | List open studies |
| POST | `/api/studies/:id/join` | Join study (generates consent record) |
| POST | `/api/ze-guide/ask` | Ask Ze·Guide (logged, with disclaimer) |

---

## Legal

χ_Ze and D_norm are **research metrics**, not medical devices.  
Ze·Guide is **not a physician**. Every response includes a mandatory legal disclaimer.  
All user data can be exported or deleted at any time (GDPR Art. 17).

---

## License

MIT — see `LICENSE`

*CommonHealth v1.0-dev — 2026-04-08*
