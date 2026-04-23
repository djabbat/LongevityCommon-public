# Code Audit — Ontogenesis

## ✅ Соответствия
1. **Формулы LCS:** Реализация в `src/analysis/lcs.rs` соответствует описанию в CONCEPT.md §2.5 (уравнения однофакторной и двухфакторной LCS-моделей).
2. **Логика метаморфозов:** Алгоритм в `src/metamorphosis.rs` соответствует CONCEPT.md:
   - Окно кластеризации: 6 месяцев (`WINDOW_MONTHS = 6.0`).
   - Порог метаморфоза: ≥2 доменов (`MIN_DOMAINS = 2`).
   - Коррекция: FDR Benjamini-Hochberg (первичная) с порогом 0.05.
3. **Отсутствие устаревших утверждений:** В предоставленном коде нет упоминаний:
   - Health Score.
   - "MCOA Test 2" как источника параметров.
   - χ_Ze как валидированного биомаркера (R²=0.84).
   - Старой структуры EIC WP (Ze/BioSense/Aqtivirebuli).
4. **Периодизация:** Использование 5 нейробиологических фаз (Phase I-V) из Nature Communications 2025, как указано в CONCEPT.md §2.4, реализовано в `src/metamorphosis.rs`.

## ❌ Проблемы
**Критические:**
1. **Отсутствие параметров в коде:**
   - В `src/params.rs` структура `OntogenesisParams` содержит только **алгоритмические** константы (шаг возраста, радиус кластера).
   - **Ни один предметный параметр** из PARAMETERS.md (рост, гормоны, IQ и т.д.) не представлен в коде в виде констант, структур данных или значений по умолчанию. Код ожидает загрузки данных, но не имеет эталонных значений для симуляции/проверок.
   - **Файлы:** `src/params.rs`, отсутствующий модуль для доменных параметров.

2. **Нарушение канона γ_i = 0:**
   - CORRECTIONS §1.3 и PARAMETERS.md явно требуют: `γ_i = 0` по умолчанию (null hypothesis).
   - В коде (`src/analysis/lcs.rs`) параметр связи обозначен как `gamma`, но **нет инициализации значением 0 по умолчанию**. Это может привести к использованию неинициализированных или некорректных значений.
   - **Файл:** `src/analysis/lcs.rs` (структуры `LcsParams`, `DualLcsParams`).

3. **⚠️ Код является scaffold (нереализован):**
   - `src/analysis/lcs.rs`: Функция `estimate_lcs` обрывается на середине, не реализована.
   - `src/analysis/transition_detection.rs`: Метод `TransitionDetector::detect` неполный (обрывается на `detect_cv_peaks`).
   - `src/metamorphosis.rs`: Функция `fdr_bh` обрывается (`.co`).
   - Код задаёт структуру, но не выполняет заявленную логику. Невозможно проверить работоспособность алгоритмов.

## ⚠️ Улучшения
1. **Архитектурная согласованность:** Структура модулей (`data/`, `analysis/`, `params/`) хорошо отражает концепцию из CONCEPT.md.
2. **Документация:** Встроенные комментарии в коде ссылаются на CONCEPT.md и литературу, что облегчает аудит.
3. **Типизация:** Использование Rust `enum` для доменов (`Domain`) и фаз (`Phase`) повышает надёжность.

## 📋 Рекомендации
**Немедленные правки (готовые к применению):**
1. **Добавить модуль доменных параметров:**
   - Создать файл `src/domain_params.rs`.
   - Реализовать структуру/перечисление, хранящее все параметры из PARAMETERS.md с их значениями по умолчанию, единицами измерения и доменом.
   - Пример (для первых двух параметров):
     ```rust
     // src/domain_params.rs
     pub struct DomainParam {
         pub id: u32,
         pub name: String,
         pub domain: Domain, // из metamorphosis.rs
         pub default_value: f64,
         pub range: (f64, f64),
         pub unit: String,
     }
     pub const HEIGHT: DomainParam = DomainParam { id: 1, name: "Рост", domain: Domain::Morphology, default_value: 50.0, range: (45.0, 200.0), unit: "см" };
     pub const WEIGHT: DomainParam = DomainParam { id: 2, name: "Вес", domain: Domain::Morphology, default_value: 3.5, range: (2.5, 100.0), unit: "кг" };
     // ... все параметры из таблицы PARAMETERS.md
     ```
2. **Исправить значение γ_i по умолчанию:**
   - В `src/analysis/lcs.rs` для `DualLcsParams` установить `gamma_xy: 0.0` и `gamma_yx: 0.0` в конструкторе или реализации `Default`.
   - Добавить комментарий со ссылкой на CORRECTIONS §1.3.
     ```rust
     impl Default for DualLcsParams {
         fn default() -> Self {
             Self {
                 x: LcsParams::default(),
                 y: LcsParams::default(),
                 gamma_xy: 0.0, // CORRECTIONS §1.3: null hypothesis
                 gamma_yx: 0.0,
             }
         }
     }
     ```
3. **Завершить scaffold-функции:**
   - Реализовать заглушки для обрывков кода, возвращающие `todo!()` или стандартные значения, чтобы код хотя бы компилировался.
   - Пример для `fdr_bh` в `src/metamorphosis.rs`:
     ```rust
     pub fn fdr_bh(p_values: &[f64]) -> Vec<f64> {
         // Базовая реализация BH
         let m = p_values.len();
         let mut indexed: Vec<(usize, f64)> = p_values.iter().enumerate().map(|(i, &p)| (i, p)).collect();
         indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
         let mut q_values = vec![0.0; m];
         for (rank, &(idx, p)) in indexed.iter().enumerate() {
             q_values[idx] = (p * m as f64) / (rank + 1) as f64;
         }
         q_values
     }
     ```

**Долгосрочные рекомендации:**
- После реализации параметров провести интеграционный тест, загружающий параметры из PARAMETERS.md и сверяющий их с константами в коде.
- Полностью реализовать алгоритмы LCS и детекции метаморфозов, затем провести валидацию на синтетических данных, соответствующих British Birth Cohorts (упомянутым в CONCEPT.md).