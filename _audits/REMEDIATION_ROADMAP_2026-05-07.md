# LongevityCommon — remediation roadmap

**Trigger:** deep audit 2026-05-07 (`AUDIT_DEEP_2026-05-07.md`)
**Approved scope:** P0+P1 items #1 (social layer), #3 (missing CLAUDE.md), #4 (BioSense backend), #5 (counter-modules plan), #6 (nginx audit), #7 (v* unify). #2 (HAP/Ontogenesis cleanup) deferred to "future" list.

**Status snapshot (2026-05-07 22:00 +04):**
- ✅ #3 — 7 CLAUDE.md созданы (MCOA, CDATA, AutomatedMicroscopy, Telomere, MitoROS, EpigeneticDrift, Proteostasis)
- ✅ #6 — nginx audit + 7 `.bak` файлов вычищены, broken-symlink finding отозван (biosense-web Phoenix живёт на :4501)
- ✅ Структурная ремедиация (pre-requisite per user 2026-05-07): stale `/home/jaba/web/longevitycommon/` (184 MB) удалён, AUTH bypass committed, deploy convention документирован
- ⏳ #1 #4 #5 #7 — этот roadmap

---

## Audit corrections (после server probe)

| Audit finding | Корректировка |
|---|---|
| Lens D P0 #4: «BioSense backend not running, χ_Ze biomarker не работает» | **Частично false-positive.** `biosense-web.service` (Phoenix LiveView, :4501) активен; `biosense.longevity.ge` отдаёт 200, title "BioSense Simulator". Что **отсутствует** — Rust backend для **реального** χ_Ze (CONCEPT.md упоминает :4101). Текущий Phoenix dashboard вероятно показывает demo data. |
| Lens B P1 #4: «FCLC локальная копия пуста» | **Confirmed false-positive** (см. audit § "False-positive corrections"). FCLC server-resident, отдельный repo `djabbat/FCLC`. |
| Lens C P0 #2: «API-handoffs не реализованы» | **Confirmed**, частично. FCLC ↔ AIM работают; MCOA/CDATA/Ze/BioSense — нет cross-talk. |

---

## Roadmap по приоритетам

### Phase 1 — counter-modules integration plan (#5) — **2 недели, blocking #4**

**Goal:** определить судьбу 4 concept-stage counter'ов (Telomere, MitoROS, EpigeneticDrift, Proteostasis) до начала работы над BioSense backend и social layer.

| Step | Action | Output | Risk |
|---|---|---|---|
| 1.1 | **MCOA numbering reconciliation** — решить P0 audit Lens C #1: CDATA в MCOA = Counter #1 или #2? Telomere = #2? Один документ-decision-record + sweep по всем CONCEPT.md. | `MCOA/docs/COUNTER_NUMBERING_DECISION.md` + 5 patches | low (decision-only) |
| 1.2 | **Determine Phase для каждого counter:** active-development / future-work / dropped. Активные = реализовать Rust kinetics + tests; future = заморозить с datestamp; dropped = `_archive/`. | `MCOA/docs/COUNTER_ROADMAP.md` | medium (deletion?) |
| 1.3 | **For each active counter:** скаффолд `<counter>/backend/` Rust crate с D_i kinetics function + 5 unit tests + integration со смежным MCOA workspace. | 1-4 new Rust crates | low |
| 1.4 | **MCOA orchestrator API** — единая ручка `/v1/counters/<id>/D` возвращающая текущее состояние counter'а. Docs в `MCOA/CONCEPT.md`. | `mcoa-orchestrator` Rust binary | low |

**Deliverable:** активные counter'ы имеют рабочий Rust backend с kinetics + единый API. Это разблокирует BioSense backend (#4) и social layer (#1) — у них появятся реальные источники данных для χ_Ze и UI.

---

### Phase 2 — v* unification (#7) — **3 дня, parallel с Phase 1**

**Goal:** устранить путаницу `v* = 0.45631 (Python)` vs `-0.087 (Article)` в `PARAMETERS.md`. Сейчас Ze и BioSense могут трактовать `v*` по-разному при обмене данными через API.

| Step | Action | Output |
|---|---|---|
| 2.1 | **Decision** какая конвенция канонична (Python? Article? новая?). Запись в `PARAMETERS.md` § "v* convention". | 1 commit to PARAMETERS.md |
| 2.2 | **Sweep** всех CONCEPT/THEORY/PARAMETERS файлов на упоминания v* — обновить под канон. | ~5-10 file edits |
| 2.3 | **CI gate** — простой `scripts/check_v_star.sh` который грепает v* в CONCEPT/THEORY и алертит при mismatch с PARAMETERS канонической секцией. | shellcheck-clean script + GitHub Actions step |
| 2.4 | **API contract** — Ze и BioSense API должны возвращать `v_star` в канонической convention; добавить comment в каждой ручке. | code edits in respective backends |

**Deliverable:** единая конвенция, проверяемая CI, документированная.

---

### Phase 3 — BioSense backend (#4) — **2 недели**

**Prerequisite:** Phase 1 (counter modules с реальными D_i, чтобы BioSense backend имел что агрегировать в χ_Ze).

**Goal:** реализовать Rust backend `:4101` (или :4502 — он уже port-mapped в nginx), считающий **реальный** χ_Ze биомаркер вместо текущей "BioSense Simulator" demo dashboard на :4501.

| Step | Action | Output |
|---|---|---|
| 3.1 | **Decide port:** оставить :4101 (per CONCEPT) или переиспользовать :4502 (уже nginx-mapped). Записать в `BioSense/CLAUDE.md`. | 1-line decision |
| 3.2 | **Rust crate `biosense-backend`** — implements χ_Ze computation per `BioSense/CONCEPT.md § "χ_Ze formula"`. Pulls D_i values from MCOA orchestrator (Phase 1.4). | Rust crate + tests |
| 3.3 | **Integration:** Phoenix `biosense-web` (:4501) переключается с demo на реальный backend. Endpoint `/api/chi_ze` возвращает реальные значения. | Phoenix patch |
| 3.4 | **Deploy:** `BioSense/deploy/systemd/biosense-backend.service` + deploy.sh per DEPLOY_CONVENTION. | systemd unit + script |
| 3.5 | **Smoke:** end-to-end test `curl https://biosense.longevity.ge/api/chi_ze` → реальное число с CI. | E2E in CI |

**Deliverable:** χ_Ze выходит из vapour-status, есть рабочий API для social layer (Phase 4).

---

### Phase 4 — social layer (#1) — **4 недели, MAJOR**

**Prerequisite:** Phase 1 + 3 (BioSense χ_Ze работает, counter modules реализованы).

**Goal:** запустить `server/` (Rust/Axum) + `web/` (React/TS) + `realtime/` (Elixir/Phoenix Channels) на сервере. Это центральная UX платформа — регистрация, Ze·Profile (4 фактора), Ze·Guide, лента исследований.

| Step | Action | Output |
|---|---|---|
| 4.1 | **Audit existing code** в `server/`, `web/`, `realtime/` — что уже написано (CONCEPT референсит много функций), что vapour. | `server/AUDIT.md` |
| 4.2 | **Minimal viable schema** — `server/migrations/001_initial.sql` уже существует; убедиться что migrations runnable + seed data. | working migration chain |
| 4.3 | **Ze·Guide MVP** — 1 endpoint `/api/ze_guide` + frontend chat UI; обязательный disclaimer + полный logging в `ze_guide_logs` (per root CLAUDE rule). | Phase 1 endpoint working |
| 4.4 | **Ze·Profile** — endpoint `/api/profile` возвращает 4 фактора (organism / psyche / consciousness / social). Каждый из organism подтягивается из BioSense backend (Phase 3). | endpoint + frontend page |
| 4.5 | **Realtime feed** — Phoenix Channel для live updates; Elixir работает. | working realtime |
| 4.6 | **Deploy:** `server/deploy/`, `web/deploy/`, `realtime/deploy/` per convention. nginx site `app.longevity.ge` (уже есть конфиг — проверить target). | 3 systemd units, 1 nginx site |
| 4.7 | **GDPR**: data export (`GET /api/data/export`) + soft delete (`deleted_at`). | code + tests |

**Deliverable:** реальная социальная платформа доступна на `app.longevity.ge`. EIC заявка может ссылаться на работающий MVP.

---

## Cross-cutting requirements

### Migration to deploy/ convention

Применить per `DEPLOY_CONVENTION.md`:
- MCOA → создать `MCOA/deploy/` с landing-page deploy script (P2; cosmetic)
- CDATA → то же
- Ze → найти, откуда сейчас стартует beam.smp на :4400, перенести под `Ze/deploy/`
- BioSense → создать в Phase 3
- server/web/realtime → создать в Phase 4

### Server-side `git pull` automation

systemd timer на сервере, каждые N минут:
- `cd /home/jaba/web/aim && git pull --ff-only`
- `cd /home/jaba/web/fclc && git pull --ff-only`

Это не запущено сейчас — каждое обновление вручную.

---

## Timeline + sequencing

```
Week 1       Week 2       Week 3       Week 4       Week 5       Week 6       Week 7   ...
└─Phase1──────────────┐
└─Phase2 (parallel)─┐ │
                    │ └─Phase3 BioSense──────────┐
                    │                            │
                    │                            └─Phase4 social layer (4w)──────────────┐
                    │                                                                     │
                    └ ready for Phase4                                                    │
                                                                                          ▼
                                                                                       MVP @ app.longevity.ge
```

**Total:** ~7 weeks к полноценному social layer MVP, при условии нормальной нагрузки. Параллельная Phase 2 экономит неделю.

---

## Open decisions (для пользователя)

1. **Counter numbering reconciliation:** CDATA = #1 или #2? Это P0 finding и блокирует Phase 1.1. Канонический выбор — за пользователем.
2. **v\* canonical convention:** Python value (0.45631) или Article value (-0.087)? Phase 2.1.
3. **BioSense backend port:** :4101 (CONCEPT) или :4502 (already nginx-mapped)? Phase 3.1.
4. **Counter module phase classification:** какие из 4 counter'ов (Telomere/MitoROS/EpigeneticDrift/Proteostasis) идут в active-development, а какие во future-work? Phase 1.2.
5. **#2 deferred:** TOXIC HAP/Ontogenesis — когда возвращаемся? Текущий план — отложить до решения по EIC заявке (там уже выработано — не использовать).

После ответа на эти 5 вопросов можно начинать Phase 1 без задержек.

---

## File output из этой сессии

- `_audits/AUDIT_DEEP_2026-05-07.md` — полный аудит (201 LoC)
- `_audits/REMEDIATION_ROADMAP_2026-05-07.md` — этот документ
- `docs/DEPLOY_CONVENTION.md` — single source of truth для deploy patterns
- `MCOA/CLAUDE.md`, `CDATA/CLAUDE.md`, `AutomatedMicroscopy/CLAUDE.md`, `Telomere/CLAUDE.md`, `MitoROS/CLAUDE.md`, `EpigeneticDrift/CLAUDE.md`, `Proteostasis/CLAUDE.md` — 7 missing CLAUDE.md восстановлены
- `AIM/AI/queen_deploy/` — un-archived, AUTH bypass committed
- Server: stale `/home/jaba/web/longevitycommon/` удалён (backup tarball остался), 7 `.bak` nginx файлов удалены
