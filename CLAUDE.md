# CLAUDE.md — CommonHealth Ecosystem

> ⚠️ **См. [_archive/audits/CORRECTIONS_2026-04-22.md](_archive/audits/CORRECTIONS_2026-04-22.md)** — некоторые утверждения в этом файле могут быть отозваны. Каноны обновлены 2026-04-22.


## Project Identity

**CommonHealth** — центральная платформа экосистемы долголетия + **umbrella для EIC Pathfinder заявки**.
**Подпроекты (подпапки):** MCOA · FCLC · Ze · CDATA · BioSense · Ontogenesis · HAP (+ Aqtivirebuli из Iqalto как WP5). **MCOA** (добавлен 2026-04-21) — мета-теоретический фундамент; остальные подпроекты встраиваются в MCOA как счётчики или измерительные слои.
**Версия:** CONCEPT v4.0 (Ecosystem Edition) | **Status: CONCEPT APPROVED**
**Location:** `~/Desktop/CommonHealth/`

**EIC Pathfinder — ⏸ DEFERRED TO 2027 (per 2026-04-21 deep audit):**

Blocker summary (cannot be fixed in 21 days before 2026-05-12):
1. **0 signed EU LoIs.** EIC requires ≥2 independent EU-MS partners in 2 different Member States with signed commitment letters. DFKI cold-contacted 2026-04-01; realistic LoI turnaround 4-8 weeks.
2. **ε_total=10 without working PATE demo** — Reviewer C scored this REJECT in internal peer review v10 (1.86/5).
3. **CDATA ABL-2 paradox** (Sobol S1 inverted vs central claim) unresolved.
4. **Subproject audit failures:** Ontogenesis 6/6 KNOWLEDGE.md PMIDs fabricated; HAP 10/10 EVIDENCE.md PMIDs fabricated (both halted 2026-04-21).

**Canonical next submission target:** Horizon Europe Pathfinder Open 2027 (Q1 call). Timeline:
- 2026-05 → 2026-09: rebuild HAP EVIDENCE.md + Ontogenesis KNOWLEDGE.md from verified PubMed
- 2026-06 → 2026-08: acquire ≥2 signed EU-MS LoIs (DFKI, INRIA, ETH, or equivalent)
- 2026-09 → 2026-12: PATE demo implementation (ε≈0.63 path) with working code + benchmark
- 2026-10: resolve CDATA ABL-2 Sobol paradox via extended global sensitivity analysis
- 2027-Q1: EIC Pathfinder Open resubmission

*Historical (frozen):* Variant C structure (WP1 FCLC €0.6M + WP2 Ze €0.5M + WP3 CDATA €0.8M + WP4 BioSense €0.6M + WP5 Aqtivirebuli €0.5M = €3.0M / 36 mo, host NGO Georgia Longevity Alliance) preserved for 2027 resubmission scaffolding but **not the submission plan for 2026-05-12**.

*Deep audit file:* `~/Desktop/CommonHealth/FCLC/DEEP_AUDIT_2026-04-21.md`
*Canonical deferral record:* `~/Desktop/CommonHealth/_archive/EIC_CONSORTIUM_STRUCTURE_2026-04-21.md`

---

## Ecosystem Structure

```
CommonHealth/           ← этот проект (социальный слой)
├── MCOA/               ← Multi-Counter Architecture of Organismal Aging — мета-теория (добавлен 2026-04-21)
├── FCLC/               ← федеративное обучение, DP-инфраструктура
├── Ze/                 ← Counter "S" MCOA: плазма/SASP-петля, χ_Ze и PAG (переработан 2026-04-23; стек: Rust simulator+backend / Phoenix LiveView)
├── CDATA/              ← теория повреждения центриолей, MCAI (Counter #2 в MCOA)
├── BioSense/           ← EEG+HRV+обоняние (измерительный слой MCOA)
├── Ontogenesis/        ← платформа онтогенеза 0–25 лет
├── HAP/                ← Hepato-Affective Primacy Theory (нейро-гепатология)
├── server/             ← Rust/Axum REST API
├── web/                ← React TypeScript PWA
└── realtime/           ← Elixir/Phoenix Channels
```

**Правило:** CommonHealth — thin social layer over FCLC+Ze+CDATA+BioSense+Ontogenesis. Никакой новой науки, никакой новой privacy-инфраструктуры. Новое: UX сообщества, ранжирование ленты, Ze·Guide AI.

---

## Source of Truth

**CONCEPT.md is the authoritative document.**
Все подпроекты имеют собственные CONCEPT.md — авторитет на уровне подпроекта.
При конфликте: CommonHealth CONCEPT.md > субпроект CONCEPT.md.

---

## Language Defaults

- Backend API: **Rust** (Axum, sqlx)
- Frontend: **React + TypeScript** (Vite, PWA)
- Realtime: **Elixir/Phoenix** (Channels, LiveView)
- Subproject specifics: см. CLAUDE.md каждого подпроекта

---

## Critical Rules

### Четыре фактора здоровья (обязательно в UI и API)
Здоровье = ОРГАНИЗМ + ПСИХИКА + СОЗНАНИЕ + СОЦИУМ
- Ze·Profile отображает все 4 фактора
- Ze·Guide отвечает на вопросы по всем 4 доменам
- Таблица `health_factors` хранит психика/сознание/социум (организм — в ze_samples)
- ~~Интегральный Health Score: `0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social`~~ **УДАЛЕНО 2026-04-22** — веса не имели вывода из MCOA L_tissue; используется напрямую L_tissue с tissue-specific w_i (см. CONCEPT.md §A.2)

### Ze·Guide
1. **Disclaimer перед КАЖДЫМ ответом** — без исключений
2. **Логировать ВСЁ** в `ze_guide_logs` (disclaimer_sent = true)
3. **Не давать медицинских советов** — только научный контекст
4. **Цитировать источники** — DOI, файлы, датасеты

### Биологический возраст
- Всегда: point estimate + 95% CI + stability label
- Никогда: «Ваш возраст улучшился на 2 года за ночь»
- stability: high (<3y CI) / medium (<7y) / low

### База данных
- Схема: `server/migrations/001_initial.sql`
- ORM: sqlx (compile-time queries)
- Параметры: `$1, $2, ...` — никогда строковая интерполяция
- GDPR: soft delete через `deleted_at`, экспорт через `GET /api/data/export`

### Антифрод
- DOI → verify через Crossref API при создании поста
- Неверный DOI → `rank_penalty += 2.0` (не блокировать пост)

### API responses
```rust
// Успех: Json(value)
// Ошибка: (StatusCode::XXX, String)
// Никогда: .unwrap() в handlers
```

---

## Приоритеты разработки

1. **Безопасность** — no SQL injection, параметры везде
2. **Корректность** — Ze compute с CI
3. **Юридическая защита** — Ze·Guide logs, consent, GDPR export
4. **Производительность** — индексы на ze_samples, posts; pagination

---

## DeepSeek Rule

**Код — Claude. Всё остальное (статьи, тексты, переводы, гранты) — DeepSeek API.**
Ключ: `~/.aim_env → DEEPSEEK_API_KEY`
Модели: `deepseek-chat` (быстро) · `deepseek-reasoner` (научные рассуждения)

---

## Core .md Files

Все .md кроме README.md — файлы ядра.
Генерируются из CONCEPT.md. Обновляются при каждом значимом изменении.
ARCHITECTURE не существует отдельно — его содержимое в CONCEPT.md.

**Файлы ядра (полный список — в .gitignore для public):**
`CONCEPT.md` · `KNOWLEDGE.md` · `PARAMETERS.md` · `MAP.md` · `MEMORY.md` · `LINKS.md` · `UPGRADE.md` · `TODO.md` · `CLAUDE.md` · `STRATEGY.md` · `REMINDER.md`

**`STRATEGY.md`** — гибридная грантовая стратегия (5 треков: FCLC/CDATA/Ze/BioSense/Ontogenesis).
Читать первым делом в каждой сессии перед работой с любым подпроектом.

**Git (монорепозиторий):**
- **Единый репозиторий:** `djabbat/CommonHealth` (объединяет CommonHealth + FCLC + Ze + CDATA + BioSense + Ontogenesis)
- Private: все файлы включая .md ядра
- Public: только код + README (core .md в .gitignore)

---

## Subproject References

| Подпроект | CLAUDE.md | Авторитетный документ |
|-----------|-----------|----------------------|
| FCLC | `FCLC/CLAUDE.md` | `FCLC/CONCEPT.md` |
| Ze | `Ze/CLAUDE.md` | `Ze/CONCEPT.md` |
| CDATA | `CDATA/CLAUDE.md` | `CDATA/CONCEPT.md` |
| BioSense | `BioSense/CLAUDE.md` | `BioSense/CONCEPT.md` |
| Ontogenesis | `Ontogenesis/CLAUDE.md` | `Ontogenesis/CONCEPT.md` |
| HAP | `HAP/CLAUDE.md` | `HAP/CONCEPT.md` |
