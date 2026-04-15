# CommonHealth — TODO

_Обновлено: 2026-04-10_

---

## ⚡ ПЕРВЫМ ДЕЛОМ В КАЖДОЙ СЕССИИ

> **Прочитать [`STRATEGY.md`](STRATEGY.md)** — гибридная грантовая стратегия по подпроектам.  
> Определить активный трек по дедлайнам, затем работать по TODO нужного подпроекта.
> - Трек 1 → `FCLC/TODO.md` + `FCLC/REMINDER.md`
> - Трек 2 → `CDATA/` (peer review как самостоятельная заявка)
> - Трек 3 → `Ze/` (публикация + пилот)
> - Трек 4 → `BioSense/` (прототип TRL 3)

---

## ПЛАН РАЗВИТИЯ (записан 2026-04-09)

### ЭТАП 1 — Инструмент измерения (BioSense)
- [ ] Standalone web-инструмент: загрузи CSV/JSON → χ_Ze (без CommonHealth)
- [x] **Стандартизированный формат `BioSenseExport` JSON (схема)** — `server/src/models/biosense.rs` — 2026-04-09 ✅
- [x] **Документированный API endpoint: `POST /api/biosense/compute` → `{chi_ze_eeg, chi_ze_hrv, bio_age, ci_95}`** — публичный, без авторизации — 2026-04-09 ✅
- [x] **Тест совместимости: Python pipeline → CommonHealth JSON import** — `BioSense/src/test_biosense_export.py` (4/4 тестов) — 2026-04-09 ✅

### ЭТАП 2 — FCLC (privacy-инфраструктура)
- [ ] `cargo build` + `mix deps.get` — проверить сборку
- [ ] Интеграционный тест: fclc-node → fclc-server (реальный round)
- [ ] ChaCha20+Poly1305 SecAgg+ (сейчас LCG placeholder)
- [ ] FCLC ECDSA подпись χ_Ze записей (CommonHealth ждёт)
- [ ] EIC Pathfinder Part B — подача май 2026

### ЭТАП 3 — CommonHealth MVP (v1.0)
- [x] Схема БД: users, ze_samples, posts, studies, ze_guide_logs — готово
- [x] **Миграция 003: `health_factors` таблица** (психика, сознание, социум) — 2026-04-09
- [x] **Обновить `ZeProfile` модель** — 4-факторный Health Score — 2026-04-09
- [x] **Обновить `ze_compute.rs`** — `compute_health_factors()`, `compute_profile()` — 2026-04-09
- [x] **Обновить `dashboard.rs`** — health_factors в ответе + POST /api/health-factors — 2026-04-09
- [x] **Обновить TypeScript типы** — HealthFactorSummary, CreateHealthFactorRequest, ZeProfile — 2026-04-09
- [x] **Обновить `ZeProfileCard.tsx`** — HealthFactorsPanel с 4 факторами + Health Score — 2026-04-09
- [x] **Обновить `Dashboard.tsx`** — HealthFactorForm (mood, stress, mindful, purpose, support, isolation) — 2026-04-09
- [x] Cargo check без ошибок — 2026-04-09 ✅
- [x] **GDPR export: добавить `health_factors` в `export_data()`** — 2026-04-09 ✅
- [x] **FeedChannel: авторизация приватных комнат** (`user_id` проверяется через socket.assigns) — 2026-04-09 ✅
- [ ] Деплой: Hetzner + Cloudflare + Neon.tech

### ЭТАП 4 — Рост и наука (v2.0)
- [ ] Первое исследование CommonHealth → публикация (IF>5)
- [ ] BLE синхронизация BioSense
- [ ] React Native мобильное приложение
- [ ] ORCID верификация
- [ ] Ze·Clock автоматический (cron)
- [ ] Статья: «4-факторная модель здоровья и χ_Ze»

### ЭТАП 5 — Масштаб (v3.0)
- [ ] Institutional подписка (университеты, клиники)
- [ ] Researcher API (DP-защищённые Ze-данные)
- [ ] Региональные Ze·Clock
- [ ] Федеративные мобильные узлы FCLC
- [ ] White-label Ze·Profile для партнёров

---

## Экосистема — реструктуризация (2026-04-08) ✅

- [x] Переместить FCLC → `CommonHealth/FCLC/`
- [x] Переместить Ze → `CommonHealth/Ze/`
- [x] Переместить CDATA → `CommonHealth/CDATA/`
- [x] Переместить BioSense → `CommonHealth/BioSense/`
- [x] Переписать `CONCEPT.md` → v4.0 (Ecosystem Edition)
- [x] Обновить `CLAUDE.md` → ecosystem rules + subproject references
- [x] Обновить `MAP.md` → структура с подпапками
- [x] Добавить Ecosystem Context в FCLC/CLAUDE.md, Ze/CLAUDE.md, CDATA/CLAUDE.md, BioSense/CLAUDE.md
- [x] Обновить `MEMORY.md` → решение о реструктуризации
- [ ] Обновить git remote пути (если изменились) для FCLC, CDATA
- [ ] Создать `CommonHealth-private` repo, push всего экосистемного кода

---

## Фаза 0 — Настройка (сделано ✅)

- [x] CONCEPT.md v2.0 — утверждён
- [x] ARCHITECTURE → интегрирована в CONCEPT (v3.0)
- [x] README.md — публичный
- [x] Файлы ядра (KNOWLEDGE, MAP, PARAMETERS, LINKS, TODO, UPGRADE, MEMORY, CLAUDE)
- [x] server/ — Rust/Axum: модели, хендлеры, сервисы, маршруты
- [x] server/migrations/001_initial.sql — полная схема PostgreSQL
- [x] web/ — React PWA: Feed, Dashboard, Studies, Login, ZeGuide
- [x] realtime/ — Phoenix: FeedChannel, ZeClockChannel, StudyChannel

---

## Фаза 1 — MVP v1 (месяц 1–3)

### Backend (Rust)
- [ ] Настроить PostgreSQL, запустить миграции
- [ ] Проверить cargo build (добавить `regex-lite` в Cargo.toml — нужен для ai_guide.rs)
- [ ] Email OTP: подключить SMTP (SendGrid или Postfix)
- [ ] Тесты: auth flow, Ze compute, feed ranking
- [ ] `.env` с реальными ключами (DeepSeek, JWT_SECRET, DATABASE_URL)

### Frontend (React)
- [ ] `pnpm install && pnpm build` — проверить сборку
- [x] Страница Settings: импорт JSON (drag & drop)
- [x] Страница Profile: публичный Ze·Profile по username
- [x] Share-карточка Ze·Profile (виральный механик — PNG export)
- [x] PWA: иконки 192px и 512px (SVG + gen-icons скрипт)

### Инфраструктура
- [ ] Зарегистрировать домен (commonhealth.io или commonhealth.app)
- [ ] Cloudflare Pages: деплой web/
- [ ] VPS (Hetzner CX21): деплой server/
- [ ] SSL через Cloudflare

---

## Фаза 2 — Посев (месяц 1–3, параллельно)

- [ ] Написать announcement пост для arXiv/bioRxiv
- [ ] Холодные письма 50 аккаунтам longevity в Twitter/X
- [ ] Инвайт-коды для первых 500 пользователей
- [ ] Первое исследование Lab: гипотеза + протокол (цель: Δχ_Ze ≥ 0.03)

---

## Фаза 3 — Стабилизация (месяц 3–6)

- [ ] Ze·Clock: автоматический еженедельный пост (cron)
- [ ] Shapley-взвешенное авторство для исследований
- [ ] ORCID iD API верификация учёных
- [ ] Дебаты: интерфейс голосования арбитра
- [ ] Интеграция импорта Apple Health (XML → JSON конвертер)
- [ ] Интеграция Oura Ring API (OAuth2)

---

## Фаза 4 — Post-MVP (месяц 6–9)

- [ ] Phoenix WebSocket: FeedChannel live updates
- [ ] React Native мобильное приложение (iOS + Android)
- [ ] BLE синхронизация с BioSense браслетом
- [ ] Полный GDPR аудит fclc-core юристами
- [ ] Публикация первого исследования CommonHealth (IF>5)

---

## Технический долг

- [x] Добавить `regex-lite = "0.1"` в server/Cargo.toml
- [x] Rate limiting: tower-governor на auth (5/мин), ze-guide (20/мин), API (120/мин)
- [x] CORS: AllowOrigin::list из ALLOWED_ORIGINS env, убрать Any
- [x] OTP: SendGrid интеграция + lockout после 5 попыток
- [x] CI расчёт: Якобиан вместо эвристики
- [x] Cohort percentile: исправлена инвертированная логика
- [x] Аномалии: tolerance band (std < 0.001) вместо exact equality
- [x] Ze·Guide disclaimer: блокирующий баннер до начала чата
- [x] FCLC подпись: явный TODO вместо молчаливой заглушки
- [x] Миграция 002: otp_attempts + 8 недостающих индексов
- [x] JWT_SECRET: проверка минимальной длины 32 символа
- [x] Убрать неиспользуемый webauthn-rs из Cargo.toml
- [ ] Passkeys (WebAuthn): регистрация и аутентификация (v2)
- [x] Admin handler: GET /api/admin/stats
- [x] Integration tests (tokio-test + sqlx test transactions)
- [ ] FCLC ECDSA signature verification (v2 — требует FCLC public key)
- [x] BUG-F3: `RdpAccountant` уже используется в fclc-server/state.rs — 2026-04-09 ✅
- [x] **Ze·Guide: история сессии** — последние 6 turns из ze_guide_logs → ConversationTurn → ai_guide::ask — 2026-04-09 ✅
- [x] **Ze·Guide: DOI-инъекция** — CDATA/Ze DOIs + формулы в ZE_SYSTEM_PROMPT — 2026-04-09 ✅
- [ ] Admin: заменить `is_pro` proxy → явный `is_admin` флаг (v2)
- [x] BioSense: стандартизированный `BioSenseExport` JSON schema — 2026-04-09 ✅
- [x] BioSense: standalone POST /api/biosense/compute endpoint — 2026-04-09 ✅
