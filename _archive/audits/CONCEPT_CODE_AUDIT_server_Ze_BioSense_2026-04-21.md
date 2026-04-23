# CONCEPT ↔ Code Audit — server · Ze · BioSense triangle

**Date:** 2026-04-21 (retry after token-limit abort)
**Scope:** `CommonHealth/server/src/`, `CommonHealth/Ze/*.md`, `CommonHealth/BioSense/*.md`
**Excluded:** `target/`, `.git/`, `node_modules/`, `MEGA_AUDIT_*` archives
**Context:** morning-pass fixes already applied (R²=0.84 retraction in `ze_compute.rs` / `biosense.rs`; Minkowski DOI → `hqm2c554`; BioSense null results honest)

---

## 1. Server code inventory

### Handlers (10)
`admin.rs`, `auth.rs`, `biosense.rs`, `dashboard.rs`, `data.rs`, `posts.rs`, `studies.rs`, `users.rs`, `ze_guide.rs`, plus `mod.rs`.

### Services (5)
`ai_guide.rs`, `doi_validator.rs`, `feed_ranker.rs`, `ze_compute.rs`, `mod.rs`.

### Models (8)
`biosense.rs`, `intervention.rs`, `post.rs`, `study.rs`, `user.rs`, `ze_guide.rs`, `ze_profile.rs`, `mod.rs`.

### Routes (`routes.rs`) — total 28 endpoints
Public: `GET /api/users/:id`, `GET /api/users/by-username/:username`, `GET /api/feed`, `GET /api/studies`, `GET /api/studies/:id`, `POST /api/biosense/compute`, `GET /health`.
Auth: `POST /api/auth/register`, `POST /api/auth/verify-otp`.
Protected (Ze·Guide bucket): `POST /api/ze-guide/ask`, `GET /api/ze-guide/history`.
Protected (general bucket): `PATCH /api/users/me`, `DELETE /api/users/me`, `POST /api/posts`, `DELETE /api/posts/:id`, `POST /api/posts/:id/react`, `GET /api/dashboard`, `GET /api/dashboard/trend`, `POST /api/interventions`, `POST /api/health-factors`, `POST /api/data/import`, `GET /api/data/export`, `POST /api/studies`, `POST /api/studies/:id/join`, `DELETE /api/studies/:id/leave`, `GET /api/admin/stats`.

---

## 2. Endpoints declared in `CommonHealth/CONCEPT.md` (line 328-341)

All 13 listed paths are implemented. **No broken promises from root CONCEPT.**

## 3. Endpoints implemented but not listed in CONCEPT.md

(Not a defect — CONCEPT lists the "core 13"; these are reasonable extensions.)
- `POST /api/biosense/compute` — present in BioSense CONCEPT §API (v3.2).
- `POST /api/health-factors` — implicit in CONCEPT §Слой 2.
- `GET /api/ze-guide/history` — UX companion of `/ask`.
- `GET /api/studies/:id`, `POST /api/studies`, `DELETE /api/studies/:id/leave` — implicit in Lab description.
- `PATCH/DELETE /api/users/me`, `DELETE /api/posts/:id`, `GET /api/users/by-username/:username`, `GET /api/admin/stats`, `GET /health` — infra.

**Recommendation (FIX LATER):** append a 1-line enumeration in `CommonHealth/CONCEPT.md` API section so ops docs match; not blocking.

---

## 4. Formula alignment (post morning-fix)

| Formula | Ze/THEORY.md | Server code | Aligned |
|---|---|---|---|
| `v = N_S/(N−1)` | §3.2 (canonical) | `ai_guide.rs:32` (system prompt) | ✅ |
| `χ_Ze = 1 − |v−v*|/max(v*,1−v*)` | §3.3 | `ai_guide.rs:33` | ✅ |
| `v*_passive = 1 − ln 2 ≈ 0.3069` | §3.2 analytic | `ai_guide.rs:34` | ✅ |
| `v*_active ≈ 0.456` | §3.2 marked "preliminary hypothesis, I²=90.3%" | `ai_guide.rs:35` marked DEPRECATED as universal constant | ✅ |
| χ_Ze status | §3.3 "theoretical abstract, NOT a validated biomarker" | `ai_guide.rs:36`, `ze_compute.rs:5`, `biosense.rs:13` all say "research-mode heuristic, NOT validated" | ✅ |
| Bridge: `bio_age = chrono_age × (1 − 1.2·(1−χ_Ze)·K)` | implicit from §3.3 bridge | `ze_compute.rs:143-148`, `biosense.rs:112` | ✅ |
| Minkowski DOI `10.65649/hqm2c554` | §3.4 | `ai_guide.rs:24` | ✅ |

**No formula drift post morning-fix.**

---

## 5. Retracted-claim scan — `R²=0.84`, `N=196`, `d=1.694`, "χ_Ze validated", Health Score

### Server src/ — all residual mentions are explicit retractions:
- `handlers/biosense.rs:13` — "prior … retracted 2026-04-22 — synthetic-data artefact"
- `services/ze_compute.rs:5,17` — "claim retracted"
- `services/ai_guide.rs:36,39` — "retracted (synthetic-data artefact)", Health Score explicitly flagged REMOVED

### `models/ze_profile.rs` — **FIXED NOW** ✅
Line 57 previously said `Integrated health score: 0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social` as a positive definition. Rewrote to clearly mark formula as DEPRECATED per CONCEPT §A.2, retained only as transitional placeholder, slated for L_tissue replacement.

### `CommonHealth/CONCEPT.md` — **FIXED NOW** (5 edits) ✅
Root concept still advertised the retracted validation in five places — direct contradiction with its own §A.2 (which removes Health Score) and with `CORRECTIONS_2026-04-22.md`:
- Line 106 (ASCII diagram): `│R²=0.84│` → `│MCAI/Ze│`
- Line 118 (table row CDATA): replaced with R²(MCAI)=0.745 + retraction pointer
- Line 133 (competitive table): `Да (R²=0.84)` → `Да (MCOA framework)`
- Line 217 (bio_age formula): removed naked "(R²=0.84)" marker, added research-path disclaimer
- Line 224 (Валидация): replaced "Cuban EEG N=196 + Dortmund" with honest theoretical-construct status + null results + I²=90.3% pooling caveat

### `Ze/` and `BioSense/` .md layer
Clean as of morning audit (retracted-claim references now only in `DEEP_AUDIT_Ze_BioSense_2026-04-21.md`, `CANONICAL_DEFINITIONS.md`, and historical `CONCEPT.md §8` statistical-plan discussion, all appropriately tagged).

### `BioSense/src/*.py` and `Ze/backend/`
No live "R²=0.84" / "Health Score" strings. Legacy separate backends appear unused since monorepo; nothing to fix today.

---

## 6. Top-5 critical mismatches found (with classification)

| # | Mismatch | Severity | Action |
|---|---|---|---|
| 1 | `CommonHealth/CONCEPT.md` advertised `R²=0.84` for χ_Ze bio_age in 5 separate places despite own §A.2 retraction | HIGH | **FIX NOW** ✅ (5 edits applied) |
| 2 | `models/ze_profile.rs:57` docstring still defined Health Score formula as positive fact | MEDIUM | **FIX NOW** ✅ (docstring rewritten) |
| 3 | `CONCEPT.md` API block (§328–341) is authoritative list but omits ~13 implemented endpoints (biosense/compute, ze-guide/history, health-factors, …) | LOW | **FIX LATER** — add section to `CommonHealth/TODO.md` |
| 4 | `W_ORGANISM … W_SOCIAL` constants still live in `ze_profile.rs` and participate in `compute_health_factors`; formula is called `health_score` but wrapped with disclaimer. CONCEPT §A.2 says "удалить Health Score компонент" | MEDIUM | **FIX LATER** — architectural refactor (L_tissue replacement) belongs to UPGRADE.md, not a 20-LoC edit |
| 5 | None of the server handlers implement MCOA `L_tissue` output yet, though CONCEPT v5.1 declares it the user-facing metric (`Ze·Profile UI: показывать L_tissue …`) | MEDIUM | **FIX LATER** — requires MCOA calibration data; add to `CommonHealth/TODO.md` |

No further `d=1.694`, `N=196`, or "χ_Ze validated" claims remain in live code/docs within audit scope.

---

## 7. Writes issued today

1. `server/src/models/ze_profile.rs` — field docstring of `health_score` (4 lines).
2. `CommonHealth/CONCEPT.md` — 5 line-level edits (diagram + 2 tables + bridge-formula note + validation paragraph).
3. `CommonHealth/CONCEPT_CODE_AUDIT_server_Ze_BioSense_2026-04-21.md` — this log.

**FIX LATER items queued** (to be written to `CommonHealth/TODO.md` in a follow-up pass; not done this session due to token budget):
- [ ] A. Extend CONCEPT §API block to list all 28 routes actually implemented (or link to `routes.rs`).
- [ ] B. Replace `compute_health_factors` + W_* constants with tissue-specific L_tissue surface per MCOA; coordinate with `MCOA/CONCEPT.md` and `models/ze_profile.rs` schema changes.
- [ ] C. Implement `/api/l-tissue/:user_id` (or extend `/api/dashboard`) to surface L_tissue(HSC, brain, muscle) per CONCEPT §A.2.

---

**End of audit.** Status: core .md ↔ code drift resolved for retracted-claim layer; remaining items are architectural (MCOA → L_tissue refactor) and scheduled via TODO.
