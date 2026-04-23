# Skeletons Status — 2026-04-22

## Что сгенерировано

**11 Rust Axum backends** (`<project>/backend/`):
- Cargo.toml + src/{main.rs, routes.rs, models.rs, db.rs, error.rs, config.rs}
- migrations/001_initial.sql
- Dockerfile + README
- Сумма: ~130 файлов

**11 Phoenix LiveView frontends** (`<project>/frontend/`):
- mix.exs + config/{config,dev,prod,runtime}.exs
- lib/<app>/{application.ex, <app>_web.ex, endpoint.ex, router.ex, telemetry.ex}
- lib/<app>_web/live/{dashboard,detail}_live.ex + .html.heex
- lib/<app>_web/components/{core_components, layouts}.ex
- lib/<app>_web/clients/backend_client.ex
- Сумма: ~201 файл

**docker-compose-all.yml** — координированный запуск всех 22 сервисов + postgres.

## Качество скелетов: архитектурный черновик

**⚠️ Скелеты НЕ готовы к compile без ручной правки.**

Типичные проблемы в Rust backends (обнаружены при `cargo check` MCOA):

1. **Устаревший axum API:** DeepSeek использует `axum::Server::bind().serve()` — удалено в axum 0.7; правильно: `axum::serve(listener, app)`
2. **Некорректные self-imports:** `use mcoa_backend::...` ссылается на бинарник, должно быть через `mod` или `pub use` в main.rs
3. **Type annotations:** некоторые переменные требуют explicit type hints для async context
4. **sqlx macros:** `sqlx::query!` требует либо live DB для macro check, либо offline-data — не работает при первом cargo check без DATABASE_URL

## Что делать дальше

### Вариант A — Use as architectural blueprint (рекомендация)
Скелеты служат как **документация архитектуры и контрактов**:
- Какие endpoints у backend (routes.rs → REST API contract)
- Какие entities (models.rs → domain model)
- Какие миграции (migrations/ → database schema)
- Какие LiveViews (lib/.../live/ → UI structure)

Реальная реализация делается вручную Rust/Phoenix разработчиком (или Claude Code в interactive session с live compile feedback).

### Вариант B — Fix via iterative compile-repair (overnight workflow)
Запустить ещё один orchestrator который:
1. `cargo check` для каждого backend
2. Ошибки → DeepSeek с контекстом файла → fix
3. Повтор до success (max 5 iterations per project)
4. То же для Phoenix: `mix deps.get && mix compile`

Оценка: ~3-5 часов overnight для всех 22 сервисов.

### Вариант C — Manual fixup key projects first
Ручная доводка 3 приоритетных backends (FCLC, MCOA, CDATA) для EIC demo:
- 2-4 часа работы на каждый через Claude Code в interactive session
- Остальные 8 — позже или при необходимости

## Что НЕ делать
- Не запускать `cargo build` или `mix compile` на всех 22 сразу — загрузит диск несколько GB зависимостей без guarantee что код хотя бы компилируется
- Не удалять скелеты — они ценны как architecture reference
- Не пытаться coordinated docker-compose up пока backends не скомпилированы

## Рекомендация

Для EIC submission 2026-05-12 скелеты достаточны как **architectural documentation**. Реальная реализация backends не требуется для grant submission (это WP2-WP4 deliverables, не входят в proposal stage).

После получения финансирования (если получим) — Вариант B или C в первые 3 месяца проекта.

## Файлы Statistics

| Проект | Rust backend files | Phoenix frontend files |
|--------|-------------------|------------------------|
| FCLC | 4 | 16 |
| MCOA | 9 | 20 |
| CDATA | 11 | 16 |
| BioSense | 11 | 19 |
| Telomere | 10 | 18 |
| MitoROS | 11 | 18 |
| EpigeneticDrift | 11 | 20 |
| Proteostasis | 11 | 18 |
| Ze | 11 | 19 |
| HAP | 23 | 19 |
| Ontogenesis | 8 | 18 |

Порты:
- Backends: 3001-3011
- Frontends: 4001-4011
- Postgres: 5432 (single instance, 11 databases)
