# CLAUDE.md — CommonHealth Ecosystem

## Project Identity

**CommonHealth** — центральная платформа экосистемы долголетия.
**Подпроекты (подпапки):** FCLC · Ze · CDATA · BioSense · Ontogenesis
**Версия:** CONCEPT v4.0 (Ecosystem Edition) | **Status: CONCEPT APPROVED**
**Location:** `~/Desktop/CommonHealth/`

---

## Ecosystem Structure

```
CommonHealth/           ← этот проект (социальный слой)
├── FCLC/               ← федеративное обучение, DP-инфраструктура
├── Ze/                 ← Ze Vectors Theory, χ_Ze алгоритм
├── CDATA/              ← теория повреждения центриолей, MCAI
├── BioSense/           ← EEG+HRV+обоняние аппаратный слой
├── Ontogenesis/        ← платформа онтогенеза 0–25 лет
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
- Интегральный Health Score: `0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social`

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
