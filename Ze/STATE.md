# STATE — Ze Theory

**Назначение:** волатильное состояние проекта. Активные задачи, текущий статус, лог решений, milestones.
**Конвенция:** новые записи в `Decision Log` сверху с датой `## YYYY-MM-DD`.

---

## Current status (2026-04-25)

- **Канон:** `Ze Theory.pdf` + `Ze Теория.pdf` в корне
- **Симулятор:** Rust workspace, 3 модуля работают (impedance / chsh / autowaves), F1-F6 unit tests проходят
- **Backend:** axum REST на `127.0.0.1:4001` — working
- **Frontend:** Phoenix LiveView на `127.0.0.1:4000` — working
- **Покрытие книги кодом:** Foundations + ключевые блоки Physics + Cheating; не реализованы GR-blocks (гл. 9-11), neural correlates (гл. 15), philosophical chapters (гл. 22-24)

---

## Active TODOs

### P0 — Sync code ↔ книга

- [ ] Каждая формула в `simulator/src/lib.rs` имеет комментарий `// Ze Theory.pdf §X.Y`
- [ ] Glossary в CONCEPT.md дополнен (Appendix D книги)

### P1 — Расширение симулятора

- [ ] 2D autowaves (гл. 17): спиральные волны
- [ ] Quantum damping на произвольной плотности (гл. 8.2): не только singlet
- [ ] Космологический solver (гл. 9-10): через cobaya/MontePython
- [ ] Релятивистский блок (гл. 9): GR-интегратор
- [ ] Квантование импеданса (гл. 11): полевая инфраструктура

### P2 — Эксперименты T1-T8

См. OPEN_PROBLEMS.md §Validation gaps.

### P3 — Frontend

- [ ] LiveView 3 вкладки с Chart.js
- [ ] Мобильная адаптация
- [ ] Экспорт CSV/PNG
- [ ] Параметры через UI sliders

### P4 — Публикация

- [ ] arXiv preprint «Ze Theory: Foundations + Physics» (гл. 1-11, ~80 стр)
- [ ] Phys. Rev. X / npj QI: «Ze-Deformation of Bell» (гл. 7-8)
- [ ] Phys. Rev. D / JCAP: «Cosmology of Impedance» (гл. 10)
- [ ] Полная книга на Zenodo как монография
- [ ] Перевод книги на грузинский через DeepSeek

---

## Milestones (✅ ставится немедленно после выполнения)

### v2.0 — Книги как канон ✅ 2026-04-25

- [x] PDF книги перенесены в корень
- [x] Старые THEORY/EVIDENCE/PARAMETERS/README → `_archive/articles_2026-04-23/`
- [x] CONCEPT.md создан (мост книга↔код)
- [x] LongevityCommon/CLAUDE.md строка про Ze исправлена
- [x] Сверка формул: точное совпадение с книгой

### v2.1 — Идеальная core schema ✅ 2026-04-25

- [x] Архивирована старая 10-файловая схема (TODO/MEMORY/UPGRADE/LINKS/KNOWLEDGE/MAP) → `_archive/core_v2_2026-04-25/`
- [x] Создана новая 9-файловая схема: CONCEPT/README/CLAUDE/THEORY/DESIGN/EVIDENCE/PARAMETERS/STATE/OPEN_PROBLEMS
- [x] Схема сохранена в memory как правило для будущих проектов

### v2.2 — Code redesign + correspondence audit ✅ 2026-04-25

- [x] Все ссылки `5.md` / старый THEORY → `Ze Theory.pdf §X.Y` в комментариях
- [x] Добавлен модуль `hierarchy` (гл. 5): проекция K/C/t_phys/Φ_Ze + dim(Z) growth
- [x] Добавлен модуль `cosmology` (гл. 10): `Ï + 3HÏ + m²I = 3(ä/a)/Λ_Ze` + bounce
- [x] Тесты F1-F6 → F1-F8 (добавлен F7 K+I=0 invariant; F8 cosmology bounce)
- [x] 9/9 тестов pass (cargo test --release)
- [x] CONCEPT §6 «Карта книга↔код» обновлена под новые модули
- [x] Correspondence audit: все формулы в CONCEPT/THEORY/PARAMETERS совпадают с кодом

### v1.0 — Ребрендинг под TOE ✅ 2026-04-23

- [x] Переход с биорегуляторного use case на TOE
- [x] Rust workspace + Phoenix frontend
- [x] 3 модуля симулятора

---

## Decision Log

### 2026-04-25 — Идеальная core schema

Решено: 5 обязательных файлов (CONCEPT/README/CLAUDE/STATE/OPEN_PROBLEMS) + 4 опциональных (THEORY/DESIGN/EVIDENCE/PARAMETERS) вместо старых 10. Причины: дублирование (timeline в 4 местах), TODO≈UPGRADE, MEMORY≈часть CLAUDE. Подпроекты CDATA/MCOA/HAP/Ontogenesis уже использовали более чистую схему — синтез с моими принципами DRY и stable-vs-volatile.

### 2026-04-25 — Книги как канон

Решено: 2 PDF книги пользователя — единственный источник истины теории. Все .md ядра — навигаторы по книге, не дубликаты. THEORY.md больше не создаётся (был бы дубликатом книги).

### 2026-04-25 — LongevityCommon/CLAUDE.md исправлен

Старая строка про Ze ("Counter S MCOA: плазма/SASP-петля, χ_Ze и PAG") противоречила книге. Заменено на корректное физическое описание. χ_Ze в MCOA остаётся отдельным биомаркером — разные use cases.

### 2026-04-23 — Ребрендинг под TOE

Проект переработан с биорегуляторного use case на физическо-математическую TOE на базе `~/Desktop/5.md`. Объясняет конфликт с LongevityCommon/CLAUDE.md, который держал старое описание до 2026-04-25.

---

## Активные вопросы (open questions)

- [ ] Нужен ли отдельный DESIGN.md для архитектуры Rust workspace? (сейчас читается из README §стек + Cargo.toml)
- [ ] Перевод книги на грузинский — приоритет или low-priority?
- [ ] Найти лабораторию для T1 (CHSH с энтропийным модулятором)

---

## Что НЕ делать

- Не воссоздавать архивированные THEORY/KNOWLEDGE — канон теперь книги
- Не редактировать PDF (read-only)
- Не путать с χ_Ze в MCOA
- Не пушить `_archive/` в public
- Не возвращать старую 10-файловую схему

## Startup checklist (каждая новая сессия)

1. Прочитать `CONCEPT.md` + последние записи Decision Log
2. Если задача про теорию → открыть нужную главу `Ze Theory.pdf`
3. Если про код → глава + `simulator/src/lib.rs`
4. Спросить пользователя: что делаем сегодня?
