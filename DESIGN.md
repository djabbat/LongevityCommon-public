# CDATA — Архитектура и дизайн системы

**Версия ПО:** Cell-DT (Cell Destiny Tracker) v3.0
**Язык:** Rust (основная логика), Python (анализ, визуализация)
**Лицензия:** Apache 2.0
**DOI:** 10.5281/zenodo.19174506

## 1. Обзор архитектуры

Cell-DT — это дискретно-событийный симулятор, реализующий стохастическую модель CDATA на уровне популяции клеток. Архитектура следует принципам Domain-Driven Design (DDD) для чёткого разделения ответственности.
```
┌─────────────────────────────────────────────────────────────┐
│                        Application Layer                     │
│  ┌────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │   CLI      │  │   Web API    │  │   Jupyter Kernel   │  │
│  │ (clap)     │  │ (warp/axum)  │  │   (cdata_kernel)   │  │
│  └────────────┘  └──────────────┘  └────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                        Domain Layer                         │
│  ┌────────────┐  ┌────────────┐  ┌──────────────────────┐  │
│  │  Cell      │  │  Tissue    │  │  Simulation Engine   │  │
│  │ (state,    │  │ (niche,    │  │ (scheduler, event    │  │
│  │  fate)     │  │  params)   │  │   processor, RNG)    │  │
│  └────────────┘  └────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Infrastructure Layer                     │
│  ┌────────────┐  ┌────────────┐  ┌──────────────────────┐  │
│  │  Persist-  │  │   Config   │  │   Telemetry & Log   │  │
│  │   ence     │  │  (TOML)    │  │    (tracing, OTLP)   │  │
│  │ (SQLite/   │  │            │  │                      │  │
│  │   Parquet) │  │            │  │                      │  │
│  └────────────┘  └────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 2. Дерево файлов (ключевые компоненты)

```
cell-dt/
├── Cargo.toml                        # Зависимости Rust
├── README.md
├── LICENSE
├── src/
│   ├── main.rs                       # Точка входа CLI
│   ├── lib.rs                        // Корневой модуль
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── cell.rs                   // Сущность Cell: damage, state, fate decision
│   │   ├── tissue.rs                 // Сущность Tissue: parameters, niche signals
│   │   └── events.rs                 // Enum событий: Division, Differentiation, Senescence, Death
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── scheduler.rs              // Диспетчер событий (binary heap)
│   │   ├── simulator.rs              // Главный цикл симуляции
│   │   └── rng.rs                    // Обёртка над rand для детерминизма
│   ├── persistence/
│   │   ├── mod.rs
│   │   ├── repository.rs             // Trait для репозиториев
│   │   ├── cell_repo_sqlite.rs       // Реализация для SQLite
│   │   └── snapshot_parquet.rs       // Сохранение снимков в Parquet
│   └── api/
│       ├── mod.rs
│       ├── web.rs                    // Маршруты REST API (GET /simulation, POST /configure)
│       └── grpc.rs                   // (Запланировано) для стриминга событий
├── configs/
│   ├── default.toml                  // Параметры по умолчанию (32 параметра)
│   ├── hsc_focus.toml                // Конфиг с акцентом на HSC
│   └── calibration/                  // Конфиги для калибровочных прогонов
├── scripts/
│   ├── run_simulation.py             // Python-скрипт для запуска и анализа
│   ├── sensitivity_sobol.py          // Анализ чувствительности (использует SALib)
│   ├── calibration_mcmc.py           // Калибровка параметров через PyMC
│   └── visualize_population.py       // Построение графиков популяционной динамики
├── tests/
│   ├── integration/
│   │   ├── test_simulation_end_to_end.rs
│   │   └── test_persistence.rs
│   └── unit/
│       ├── test_cell_fate.rs
│       └── test_scheduler.rs
├── data/                             // .gitignore, для выходных данных
│   ├── outputs/
│   └── calibrated_params/
└── docs/
    └── api.md                        // Документация по API
```

## 3. Контракты API (REST)

### 3.1. Запуск симуляции
**`POST /api/v1/simulations`**
Запускает новую симуляцию.
*Тело запроса (JSON):*
```json
{
  "config_profile": "hsc_focus", // или inline-параметры
  "parameters_override": {
    "alpha_HSC": 0.03,
    "nu_HSC": 1.0
  },
  "max_simulated_years": 80,
  "output_format": "parquet",
  "seed": 42 // опционально, для воспроизводимости
}
```
*Ответ (JSON):*
```json
{
  "simulation_id": "sim_abc123",
  "status": "running",
  "estimated_completion": "2026-04-22T15:30:00Z",
  "links": {
    "self": "/api/v1/simulations/sim_abc123",
    "results": "/api/v1/simulations/sim_abc123/results"
  }
}
```

### 3.2. Получение статуса и результатов
**`GET /api/v1/simulations/{simulation_id}`**
Возвращает статус.
**`GET /api/v1/simulations/{simulation_id}/results`**
Возвращает результаты. Поддерживаются query-параметры:
*   `