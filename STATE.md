# STATE — LongevityCommon (Ecosystem)

---

## Current status (2026-04-25)

- **Версия:** CONCEPT v4.0 (Ecosystem Edition), CONCEPT APPROVED
- **EIC Pathfinder:** ⏸ DEFERRED TO 2027 (после deep audit 2026-04-21)
- **Подпроекты:** 7 (MCOA, CDATA, HAP, Ze, BioSense, FCLC, Ontogenesis) + Aqtivirebuli (WP5 из Iqalto)
- **Code stack:** Rust + React + Phoenix — функционирует prototype

---

## Active TODOs

### P0 — EIC 2027 Q1 rebuild path

- [ ] Acquire ≥2 signed EU-MS LoIs (DFKI, INRIA, ETH или эквивалент) — 2026-06 → 2026-08
- [ ] PATE demo для FCLC (ε≈0.63 path) — 2026-09 → 2026-12
- [ ] CDATA ABL-2 Sobol paradox resolution (extended global sensitivity) — 2026-10
- [ ] HAP EVIDENCE.md полный rebuild из верифицированных PubMed — 2026-05 → 2026-09
- [ ] Ontogenesis EVIDENCE.md аналогично — 2026-05 → 2026-09

### P1 — Code

- [ ] Auth через Keycloak OIDC
- [ ] Frontend ze_guide UX
- [ ] Realtime dashboard
- [ ] FCLC интеграция в server (REST endpoint stubs)

### P2 — UX / community

- [ ] Лента ранжирования посты + DOI верификация
- [ ] Ze·Guide AI с RAG над THEORY.md подпроектов
- [ ] Биологический возраст dashboard с CI

### P3 — Documentation

- [ ] STRATEGY.md (5-track grant strategy)
- [ ] REMINDER.md (повседневные напоминания)
- [ ] Перевод CONCEPT.md на грузинский (DeepSeek)

---

## Milestones

### v9-file core ✅ 2026-04-25
- [x] TODO.md → `_archive/core_pre_9file_2026-04-25/`
- [x] THEORY, DESIGN, EVIDENCE, STATE, OPEN_PROBLEMS созданы
- [x] CONCEPT, README, CLAUDE сохранены

### Code baseline ✅ 2026-04-25 (overnight #1 fixed)
- [x] **server build SUCCESS** после `cargo sqlx prepare`
- [x] Установлен `sqlx-cli` v0.8.6 (postgres + sqlite features)
- [x] Установлен пароль для user `longevitycommon` в локальной Postgres (`ALTER USER longevitycommon WITH PASSWORD 'longevitycommon'`)
- [x] `DATABASE_URL=postgres://longevitycommon:longevitycommon@localhost/longevitycommon cargo sqlx prepare` → `.sqlx/` cache сгенерирован
- [x] `SQLX_OFFLINE=true cargo build --release` → success (1 warning unused import; sqlx-postgres 0.7.4 future-incompat)
- [ ] TODO: `.sqlx/` директория commit в git для CI/CD без БД
- [ ] TODO: обновить sqlx 0.7.4 → 0.8 (future-incompat warning)
- [ ] web/, realtime/ — не проверены отдельно

### CONCEPT v4.0 ✅ 2026-04-21
- [x] Ecosystem Edition
- [x] CONCEPT APPROVED status
- [x] MCOA как мета-теоретический фундамент

### CORRECTIONS_2026-04-22 canon ✅
- [x] Health Score формула УДАЛЕНА
- [x] χ_Ze не валидированный biomarker
- [x] γ_i = 0 default

### Deep audit ✅ 2026-04-21
- [x] EIC blockers identified
- [x] Submission deferred to 2027

---

## Decision Log

### 2026-04-25 — 9-file core scheme migration
TODO.md архивирован. STATE.md создан с миграцией задач. THEORY/DESIGN/EVIDENCE/OPEN_PROBLEMS созданы как umbrella refs.

### 2026-04-25 — Ze description fixed in CLAUDE.md
Старая строка "Counter S MCOA: плазма/SASP/χ_Ze/PAG" → корректное физическое описание (Entropic-Geometric TOE). χ_Ze в MCOA остаётся отдельным биомаркером.

### 2026-04-22 — CORRECTIONS canon
См. `_archive/audits/CORRECTIONS_2026-04-22.md`.

### 2026-04-21 — EIC Deep audit, submission deferred
1. 0 signed EU LoIs (нужно ≥2)
2. ε_total=10 без PATE demo (Reviewer C REJECT)
3. CDATA ABL-2 paradox unresolved
4. Ontogenesis 6/6 + HAP 10/10 fabricated PMIDs

→ EIC Pathfinder 2026-05-12 deferred to 2027-Q1.

### 2026-04-21 — MCOA добавлена как мета-теория
MCOA (Multi-Counter Architecture) включена в LongevityCommon как теоретический фундамент для всех counter-проектов.

---

## Что НЕ делать

- Не подавать на EIC 2026 (deferred)
- Не использовать Health Score формулу с весами (удалено)
- Не цитировать χ_Ze как clinical biomarker
- Не игнорировать CORRECTIONS_2026-04-22

## Startup

1. CONCEPT + STATE Decision Log
2. Если работа над EIC submission → DEEP_AUDIT_2026-04-21.md (FCLC папка)
3. Спросить пользователя
