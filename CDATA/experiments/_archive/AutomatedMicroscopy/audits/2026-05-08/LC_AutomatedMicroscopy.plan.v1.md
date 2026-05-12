## План улучшений AutomatedMicroscopy (на основе peer review)

### P0 — Блокеры (оценка трудоёмкости + риск)

| № | Действие | Затронутые файлы | Трудоёмкость | Риск |
|---|----------|------------------|--------------|------|
| **P0.1** | Создать **Rust-демон управления микроскопом** (serial-интерфейс к Arduino, захват камеры FLIR через Spinnaker Rust bindings или Micro-Manager HTTP API, команды через Phoenix PubSub). Заменить архитектуру «Arduino + Python + Claude Code» на Rust+Phoenix. | `src/main.rs`, `src/arduino.rs`, `src/camera.rs`, `src/pubsub.rs`, `Cargo.toml`, `config/` | L (~2–3 нед.) | Высокий (отсутствие готовых Rust-драйверов для FLIR; альтернатива — Micro-Manager с Python через FFI, что нарушает правило стека) |
| **P0.2** | Заполнить **стабы** `AGENTS.md`, `DESIGN.md`, `OPEN_PROBLEMS.md`, `PARAMETERS.md` реальным содержимым — убрать «будет регенерировано DeepSeek». | `AGENTS.md`, `DESIGN.md`, `OPEN_PROBLEMS.md`, `PARAMETERS.md` | S (~2 ч.) | Низкий |
| **P0.3** | Устранить **cross-file противоречие**: `CLAUDE.md` называет `AGENTS.md` авторитетным, но он пуст. Либо наполнить `AGENTS.md`, либо изменить `CLAUDE.md`. | `CLAUDE.md`, `AGENTS.md` | S (~1 ч.) | Низкий |
| **P0.4** | Заменить **абсолютные пути `~/Desktop/...`** на относительные/переменные `$HOME/LongevityCommon` во всех .md-файлах. | `CLAUDE.md`, `README.md`, `CONCEPT.md`, `EVIDENCE.md`, `THEORY.md` | S (~1 ч.) | Низкий |

### P1 — Важно

| № | Действие | Затронутые файлы |
|---|----------|------------------|
| **P1.1** | Привести документацию к **единому языку (английский)**: исправить смесь рус/англ в `THEORY.md`. | `THEORY.md` |
| **P1.2** | Создать **образец PROMPT.md** для AI-night-shift (теперь Rust-модуль, обрабатывающий natural-language). | `docs/PROMPT_template.md` |
| **P1.3** | Добавить **план тестирования hardware**: точность XY/Z, дрейф, photobleaching, калибровка камеры, uptime. | `CLAUDE.md` (раздел "Тесты") или новый `TESTING.md` |
| **P1.4** | Добавить **traceability** от claims в `EVIDENCE.md` к решениям в `DESIGN.md`: ссылки на конкретные refs. | `DESIGN.md`, `EVIDENCE.md` |

### P2 — Nice-to-have

| № | Действие | Затронутые файлы |
|---|----------|------------------|
| **P2.1** | Настроить **CI для Rust** (lint, test, build) в GitHub Actions. | `.github/workflows/ci.yml` |
| **P2.2** | Создать **машинно-читаемый BOM** (CSV/YAML) с ценами. | `docs/BOM.csv` |
| **P2.3** | **Версионирование PROMPT-шаблонов** (semver в метаданных). | `docs/PROMPT_template.md` |
| **P2.4** | Добавить **архитектурную диаграмму** (Mermaid/PlantUML) в `DESIGN.md`. | `DESIGN.md` |