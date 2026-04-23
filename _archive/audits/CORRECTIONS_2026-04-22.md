# CORRECTIONS 2026-04-22 — Master Document

**Этот документ — единый источник истины по всем отозванным утверждениям и обновлённым канонам CommonHealth экосистемы после мега-аудита и peer review 2026-04-22.**

Применяется ко ВСЕМ подпроектам и всем core файлам. При конфликте между этим документом и любым другим .md файлом — этот документ имеет приоритет.

---

## 1. ОТОЗВАННЫЕ УТВЕРЖДЕНИЯ

### 1.1. Health Score формула — УДАЛЕНА
- ~~`Health Score = 0.40·organism + 0.25·psyche + 0.20·consciousness + 0.15·social`~~
- **Причина:** веса не имели математического вывода из MCOA L_tissue; декларативны без обоснования
- **Взамен:** используется напрямую `L_tissue(n, t) = Σ_i w_i(tissue) · f_i(D_i(n, t))` из MCOA; веса `w_i(tissue)` определяются калибровкой из данных, не a priori
- **Статус в коде:** компонент Health Score удаляется из frontend/PWA до пересмотра

### 1.2. χ_Ze как валидированный биомаркер — ОТОЗВАНО (уточнено 2026-04-23)
- ~~"χ_Ze predicts biological age with R²=0.84"~~
- ~~"χ_Ze = mean(χ_Ze_eeg, χ_Ze_hrv) is validated clinical biomarker"~~
- **Причина:** R²=0.84 получено на синтетических данных (`null_model_r2.py`), не реальных; χ_Ze_eeg не прошёл pre-registered тесты в MPI-LEMON, Dortmund Vital, Cuban когортах
- **Новое определение (2026-04-23):** χ_Ze — **исследовательский индекс MCOA Counter "S"**, описывающий системную синхронизацию старения через плазму/SASP-петлю (Argentieri 2024 / Jeon 2022 basis). См. `Ze/CONCEPT.md §4`, `Ze/THEORY.md §2.6`. **Полностью переработан; прежняя «Ze Vector Theory» и связанные артефакты (v*_active/passive, Theorem 5.1, DESI Z10) — отозваны и не являются частью текущего Ze.**
- **Применение:** только как research metric. НЕ клинический биомаркер, НЕ валидирован.

### 1.3. MCOA Test 2 как источник γ_i — ОТОЗВАНО (циклическая зависимость)
- ~~"Coupling scalar γ_i measured in MCOA Test 2"~~
- **Причина:** MCOA Test 2 — будущий тест для измерения связей между работающими счётчиками; не может одновременно быть источником параметра γ_i и использовать γ_i
- **Новое правило:** по умолчанию `γ_i = 0` (null independence hypothesis); ненулевые γ_i появляются только если post-hoc statistical analysis отвергает independence на данных; никогда не ссылаться на "MCOA Test 2" как источник априорных значений

### 1.4. Старая EIC структура (5 WP: FCLC/Ze/CDATA/BioSense/Aqtivirebuli) — ОТМЕНЕНА
- **Новая структура EIC Part B v3 (Variant B):**
  - WP1 MCOA Framework (€0.3M, M1-M12)
  - WP2 CDATA Experimental Validation (€0.9M, M6-M36)
  - WP3 CDATA Computational & ABL-2 Resolution (€0.4M, M1-M30)
  - WP4 FCLC Platform (€0.5M, M1-M24)
  - Total: €2.1M + €0.1M management = **€2.2M**
- **Убрано из EIC:**
  - Ze/χ_Ze как WP (χ_Ze имеет 3 определения, v имеет 2 формулы)
  - BioSense как отдельное WP (переосмыслен как sensor-only для FCLC в рамках WP4)
  - Aqtivirebuli nutritional pilot (не вписывается в CDATA-ядро)
  - Health Score + социальный слой (продуктовый, не научный)

### 1.5. Двойственное определение Ze-скорости `v` — КАНОНИЗИРОВАНО
- **Каноническая формула:** `v = N_S / (N − 1)` ∈ [0, 1] — доля синхронных интервалов
- **Устаревшие (не использовать):** `v = N_T / (N_T + N_S)` (в Ze/CONCEPT.md:103), `v = (N_T − N_S) / (N_T + N_S)` (в Ze/README.md:21)
- Полная запись: `Ze/CANONICAL_DEFINITIONS.md`

### 1.6. Декларации о "доказанности" CDATA — ОГРАНИЧЕНЫ
- Нельзя утверждать "CDATA is the unique causal driver of aging" до завершения WP2 in vivo тестов
- Нельзя игнорировать ABL-2 парадокс: Sobol S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224; зануление epigenetic улучшает R²
- Честное disclosure в `CDATA/CONCEPT.md Appendix B`; 4 возможных научных исхода все публикуемы

---

## 2. НОВЫЕ КАНОНЫ

### 2.1. MCOA — единственная рамка интеграции
- `L_tissue(n, t) = Σ_i w_i(tissue) · f_i(D_i(n, t))` — единственная формула агрегации повреждений
- Аксиомы M1-M4 определяются в MCOA Framework WP1 проекта
- Никаких "4 доменов" (organism/psyche/consciousness/social) — работа на уровне тканей

### 2.2. CDATA — Appendix B honest disclosure
- Три аксиомы CDATA сохранены как рабочая гипотеза
- ABL-2 парадокс зафиксирован как центральная научная задача WP3
- 4 возможных исхода (Validated / Correlational / Downstream / Null) все публикабельны

### 2.3. Scaffold-подпроекты (Telomere, MitoROS, EpigeneticDrift, Proteostasis)
- Разморожены (FROZEN.md удалены)
- Работа продолжается как активные подпроекты
- В EIC грантах НЕ упоминаются
- При разработке: `γ_i = 0` by default; формула `f_i` конкретизируется в MCOA Framework WP1; активная разработка после получения финансирования (2028-2029)

### 2.4. Ze — отдельный трек теоретических публикаций
- Активен как теоретическая работа
- В грантах НЕ упоминается
- Целевые журналы: Foundations of Physics, Phys Rev D, Journal of Number Theory
- Канон: `Ze/CANONICAL_DEFINITIONS.md`, `Ze/PUBLICATIONS_TRACK.md`

### 2.5. BioSense — sensor-only
- Raw EEG/HRV data collection для FCLC
- БЕЗ на-борту χ_Ze computation
- БЕЗ медицинских claims
- Не CE-marked, не FDA-cleared (research-grade only)
- Полные заметки: `BioSense/SCOPE_NOTES_2026-04-22.md`

### 2.6. Верификация ссылок (мега-аудит 2026-04-22)
- PMID: 365/365 подтверждены (100%) — нет фабрикации
- DOI: 144/164 после очистки парсинга (88%) — остальные 20 это парсинг-артефакты, self-cite Longevity Horizon (10.65649/*, не в Crossref), ZooKeys figure-DOI, preprints с будущими датами
- Все ссылки в core файлах считаются валидными

---

## 3. ГДЕ ПРИМЕНЯТЬ

Этот документ применяется ко ВСЕМ файлам в `~/Desktop/CommonHealth/` и подпроектах. При подготовке любого документа (LOI, грант, статья, публичная презентация):
- Если файл устарел — заменить на соответствующее из этого документа
- Если нет явной отсылки — добавить строку в начало: `> См. CORRECTIONS_2026-04-22.md для обновлённых канонов.`
- При новом написании — использовать только каноны из §2

## 4. КАК ОТСЛЕЖИВАТЬ СООТВЕТСТВИЕ

Файл `COMPLIANCE_REPORT_2026-04-22.md` (генерируется автоматически) показывает какие .md файлы всё ещё содержат устаревшие утверждения.

Команда для проверки:
```bash
cd ~/Desktop/CommonHealth
grep -l -E "Health Score|0\.40.*organism|MCOA Test 2|R²=0\.84|WP5.*Aqtivirebuli" --include="*.md" -r . | grep -v MEGA_AUDIT
```

## 5. ДАТА И ИСТОЧНИК

- Дата: 2026-04-22
- Источник решений: пользователь Jaba Tkemaladze (Host: Georgia Longevity Alliance) + мега-аудит ecosystem + peer reviews EIC v2 (rejection trajectory → v3 Variant B)
- Контекст: подготовка EIC Pathfinder Open 2026-05-12 submission

