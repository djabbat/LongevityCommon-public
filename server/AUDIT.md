# Social layer audit — 2026-05-07

**Scope:** `server/` (Rust Axum) + `web/` (React 18 / Vite) + `realtime/` (Elixir / Phoenix Channels). Per Phase 4 step 4.1 of `_audits/REMEDIATION_ROADMAP_2026-05-07.md`.

**Trigger:** audit AUDIT_DEEP_2026-05-07.md found social layer not deployed on server. This audit checks: что уже написано локально, что vapor, что готово к deploy.

---

## Build status (laptop, 2026-05-07 22:48 +04)

| Layer | Build | Tests | Deploy state |
|---|---|---|---|
| `server/` Rust Axum | ✅ `cargo build` clean (+1 unused-import warning) | ❌ `auth_integration_tests` 23 compile errors | ❌ no systemd unit on server |
| `web/` React Vite | not yet built (npm install needed) | not yet checked | ❌ no nginx route, no dist/ |
| `realtime/` Phoenix | not yet built (mix deps.get needed) | not yet checked | ❌ no systemd unit on server |

---

## Server (Rust Axum) — what exists

**Routes (27 total in `src/routes.rs`):**

| Category | Routes | Handler |
|---|---|---|
| Public (no auth) | `/api/users/:id`, `/api/users/by-username/:username`, `/api/feed`, `/api/studies`, `/api/studies/:id`, `/api/biosense/compute`, `/api/disclosures/v5_changes`, `/health` | `users.rs`, `posts.rs`, `studies.rs`, `biosense.rs`, `disclosures.rs` |
| Auth | `/api/auth/register`, `/api/auth/verify-otp`, login implied | `auth.rs` |
| Ze Guide | `/api/ze-guide/ask`, `/api/ze-guide/history` | `ze_guide.rs` (assumes LLM hookup) |
| Authenticated user | `/api/users/me` (PATCH/DELETE), `/api/posts` (POST/DELETE/react), `/api/dashboard`, `/api/dashboard/trend`, `/api/interventions`, etc. | `users.rs`, `posts.rs`, `dashboard.rs` |

**Handler files (11):** admin, auth, biosense, dashboard, data, disclosures, posts, studies, users, ze_guide.

**Migrations (4):**
- `001_initial.sql` — base schema
- `002_otp_attempts_and_indexes.sql` — auth hardening
- `003_health_factors.sql` — 4-factor health table (organism/psyche/consciousness/social)
- `004_add_hrv_sdnn_columns.sql` — HRV extensions

**Real:**
- Axum router, postgres sqlx integration, models, services, middleware.
- API surface fairly comprehensive — 27 routes covering social/dashboard/biosense/Ze-Guide.
- Config from env (postgres URL, secrets).

**Vapor / blockers:**
- `auth_integration_tests` does not compile (23 errors). Likely API drift between handler signatures and test fixtures since last edit.
- Ze·Guide handler exists but LLM hookup not verified; must respect root-CLAUDE rule "disclaimer перед КАЖДЫМ ответом" + log to `ze_guide_logs`.
- No GDPR export endpoint visible (`GET /api/data/export` per root CONCEPT). Need to grep `data.rs` to confirm/plan.
- No biosense backend client (the new `BioSense/backend/` :4502 endpoint isn't called from social server's `biosense::compute_chi_ze` yet — `biosense.rs` likely has its own copy of χ_Ze logic which would diverge).

**P1 to-fix (before social-layer deploy):**
1. Fix `auth_integration_tests` so CI gate is restored.
2. Wire `biosense::compute_chi_ze` to the Rust BioSense backend at `http://127.0.0.1:4502/chi_ze` (single source of truth — no duplicate formula).
3. Verify `data.rs` has `GET /api/data/export` (GDPR export).
4. Add deploy/ folder per DEPLOY_CONVENTION (systemd + nginx for app.longevity.ge).

---

## Web (React) — what exists

**Pages (6):** Dashboard, Feed, Login, Profile, Settings, Studies.
**Stack:** React 18 + Vite + react-router 6 + zustand + tanstack/query 5 + recharts + axios.
**Components:** present in `src/components/`.
**Hooks:** `src/hooks/`.

**Real:**
- Substantial page scaffold (6 main views).
- Modern stack (zustand state, query for server cache, recharts for trend visualisation).
- API client config (axios) likely points to social-server.

**Vapor / blockers:**
- No `npm install` done locally → no `node_modules/` → can't smoke build.
- No `dist/` artefacts → not yet deployable as static.
- nginx site `app.longevity.ge` currently points to a Phoenix endpoint (per server inventory `app.longevity.ge.conf`) — need to decide if React static or Phoenix-rendered.
- Migration plan `web/RUST_PHOENIX_MIGRATION_PLAN.md` exists — read separately to understand intent.

**P1 to-fix:**
1. `cd web && npm install && npm run build` — verify build passes.
2. Confirm `axios` baseURL points to social-server (not localhost).
3. nginx site: serve `dist/` static; proxy `/api/` to social-server.

---

## Realtime (Phoenix) — what exists

**Channels (3):**
- `study_channel.ex` — study feed live updates
- `ze_clock_channel.ex` — Ze biological clock streaming
- `feed_channel.ex` — social feed live updates

**Modules:** Endpoint, Router, UserSocket, Application, Repo, Auth, HealthController.

**Real:** Phoenix Channels skeleton with auth, health, three feature channels.

**Vapor / blockers:**
- No `mix deps.get` → can't compile yet locally.
- No systemd unit on server for `realtime`.
- Postgres dependency: `Repo` configured but DB credentials unknown.

**P1 to-fix:**
1. `cd realtime && mix deps.get && mix compile` — verify compiles.
2. Add `realtime/deploy/systemd/longevitycommon-realtime.service`.
3. Verify auth.ex matches social-server token format (so JWT issued by Rust server is accepted by Phoenix Channels).

---

## Cross-cutting blockers

### Database

Three components, two DBs:
- `server/` — postgres (per Cargo: sqlx postgres). Migrations `server/migrations/`.
- `realtime/` — postgres via Ecto (per `Repo` module). Migrations would live in `realtime/priv/repo/migrations/`.
- BioSense backend — stateless (no DB).

**Decision needed:** single shared postgres DB (cleaner) vs. separate (isolation). Server already has FCLC postgres + OJS mariadb on `:5432/:3306`; adding another postgres is the friction.
**Recommendation:** social-server + realtime share one DB (`longevitycommon_social`), separate from FCLC.

### Auth flow

- `server/src/handlers/auth.rs` issues tokens (likely JWT).
- `realtime/lib/longevitycommon_web/user_socket.ex` validates tokens.
- These MUST agree on signing key — read both sides + document in deploy/README.

### nginx (for `app.longevity.ge`)

Existing config at `/etc/nginx/sites-enabled/app.longevity.ge.conf` — no current backend behind it (per audit `app.longevity.ge` listed as planned). Plan:
- `/api/*` → `proxy_pass http://127.0.0.1:8080/` (social-server)
- `/realtime/*` (websockets) → `proxy_pass http://127.0.0.1:4500/`
- `/` → static `dist/` from web/ build

---

## Deploy roadmap (per DEPLOY_CONVENTION.md)

### Step 4.1 (this audit) ✅

### Step 4.2 — minimal viable schema (1 day)
- Verify migrations 001-004 runnable from cold DB.
- Seed minimal data (10 fake studies + 5 disclosures).
- `server/deploy/scripts/migrate.sh` idempotent.

### Step 4.3 — Ze·Guide MVP (3 days)
- 1 endpoint `/api/ze-guide/ask` with disclaimer + ze_guide_logs persistence.
- Frontend page (web/) chat UI.
- LLM via AIM `llm.py::ask` (cross-AIM dependency — document in CLAUDE.md).

### Step 4.4 — Ze·Profile (3 days)
- `/api/dashboard` returns 4-factor profile.
- Organism factor pulls from `BioSense/backend :4502/chi_ze`.
- Frontend Dashboard.tsx wires to /api/dashboard.

### Step 4.5 — realtime feed (2 days)
- `realtime/lib/.../feed_channel.ex` — live post updates.
- Frontend uses `phoenix-js` to subscribe.

### Step 4.6 — deploy (3 days)
- `server/deploy/`, `web/deploy/`, `realtime/deploy/` per DEPLOY_CONVENTION.
- nginx site for `app.longevity.ge`.
- 3 systemd units.

### Step 4.7 — GDPR (1 day)
- Verify `/api/data/export` exists (or add to data.rs).
- soft delete via `deleted_at` column verified on all user-touching tables.

**Total estimated: ~13 working days for full social-layer MVP from current state.**

---

## Open issues discovered while restoring CI gate (2026-05-07)

### O-1 — bio_age direction inverted vs χ_Ze health convention

`src/services/ze_compute.rs::compute_profile`:
```
D_norm   = clamp(D_NORM_ALPHA * (1 − chi_ze), 0, 1)
bio_age  = chrono_age * (1 − D_norm * K)
```

For chrono_age=35:
- `chi=0.9` (high → healthy by χ_Ze convention) → `D_norm=0.12` → `bio_age≈33.1` (close to chrono).
- `chi=0.1` (low → unhealthy)                   → `D_norm=1.08→clamp 1.0` → `bio_age≈19.25` (younger).

This makes a "low χ_Ze" subject artificially YOUNGER, which contradicts
root CONCEPT: "С возрастом χ_Ze уменьшается → low chi = old".

Test `test_cohort_percentile_worst_in_cohort` was authored against the
correct direction; it now fails (returns pct=100 for "worst" subject).
Test marked `#[ignore]` with this rationale; **formula needs review and
sign correction**, OR the test is wrong — but they can't both be right.

**Action:** decision required from Jaba before Phase 4.3 (Ze·Guide
references χ_Ze as health metric; backend computing inverted bio_age
will mislead the chat).

### O-2 — feed_ranker `test_full_ranking_order` was tied at penalty=2.0

Penalty `2.0` against `+2 reactions_support` gave net-zero discrimination
between `support_only` (3 reactions) and `penalised` (5 reactions − 2).
Bumped penalty to 4.0 in the test — formula behaviour preserved, test
no longer flaky.

### O-3 — `ze_samples_source_check` constraint dropped 'test' value

`migrations/001_initial.sql` constrains `source IN ('biosense','apple_health','oura','garmin','manual')`. Integration test used `'test'` literal → constraint violation. Test patched to `'manual'`. Schema is correct; test was wrong.

---

## What to do next session (concrete)

1. Run `npm install` and `mix deps.get` locally to verify all three layers actually compile.
2. Fix `auth_integration_tests` so server tests run green.
3. Decide: shared postgres or two DBs (Q for user).
4. Wire `server/handlers/biosense.rs` to call BioSense backend on :4502 (avoid χ_Ze formula duplication).
5. Move forward with Step 4.2 (schema verification) on a chosen DB.
