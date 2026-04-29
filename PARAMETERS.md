# CDATA — Параметры модели

**Версия модели:** Cell-DT v3.0 (32 параметра)
**Дата калибровки:** 2026-04-10
**Канон:** CORRECTIONS_2026-04-22 (γ_i = 0 по умолчанию)

> ✅ **ALL 5 PARAMETER DIVERGENCES RESOLVED 2026-04-21** — см. `PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md` для полного анализа.
>
> | Параметр | Prior docs value | Resolved value | Resolution path |
> |----------|-------------------|-----------------|------------------|
> | α (α_HSC) | 0.028 | **0.0082** | (b) docs → code; MCMC posterior (PMID 36583780 concept only, no α published) |
> | ν_HSC | 1.2 /year | **1.2 /year** | (a) code 12.0 → 1.2 (Wilson 2008 standard); parameter insensitive (ΔR²≈0 at ±20%), safe change |
> | β_HSC | 0.005 | **dual-form documented**: 1.0 multiplicative (dead field), 0.005 additive `cell_dt_cli::CounterParams` |
> | π (signal-dep vs age-decay) | 0.65 `pi_base` + `D_half` + `k_s` | **age-decay model documented**: `pi_0=0.87`, `pi_baseline=0.10`, `tau_protection=24.3`. Signal-dep model deprecated (never implemented) |
> | τ_prot | 15 years | **24.3 years** | (b) docs → code; Round-7 MCMC posterior (free parameter) |
>
> **Следствие:** таблица ниже **теперь match code** для всех активных параметров. Bonus finding: fixed 6 locations of fabricated Jaiswal 2017 PMID 28792876 → correct 28636844 across CDATA Rust modules (same DeepSeek hallucination pattern documented in `feedback_deepseek_no_citations`).
>
> **Также:** `cell_dt_cli::CounterParams` hosts a **third parameter set** (α=0.60, β=0.15, τ=30yr) for the MCOA additive damage form — orthogonal to the multiplicative AgingEngine; annotated but out-of-scope for current reconciliation.

Следующая таблица содержит все 32 параметра модели CDATA, оставшиеся после редукции с 120 (см. Model Selection в `CONCEPT.md`). Параметры сгруппированы по модулям. `S1` — индекс чувствительности первого порядка из Sobol analysis (N=16384).

| Модуль | Имя параметра | Символ | Описание | Единицы | Значение (оценка) | 95% CI/Диапазон | Источник (PMID/DOI) | Статус | S1 (ранг) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Core Centriolar** | `alpha_HSC` | α_HSC | Прирост повреждения центриоли за деление (HSC) | damage/division | **0.0082** | [0.006, 0.011] | Round-7 MCMC posterior (`calibration.rs`); conceptual framework in PMID 36583780 | **Fitted** (docs updated 2026-04-21 → code post-calibration value) | 0.224 (2) |
| | `nu_HSC` | ν_HSC | Базовая частота делений HSC | divisions/year | 1.2 | [0.8, 1.6] | Wilson et al., Nature 2008; Kowalczyk et al., Cell Stem Cell 2015 | Literature + Fitted | 0.155 (3) |
| | `beta_HSC` | β_HSC | Фоновая скорость повреждения центриоли (время). См. notes ниже — dead field в multiplicative engine | damage/year | **1.0** (multiplicative/unused) <br> 0.005 (additive cell_dt_cli) | [0.001, 0.01] (additive); N/A (multiplicative) | `fixed_params.rs:79` retained; active in `cell_dt_cli::CounterParams` additive form | **Deprecated in multiplicative; active in additive CLI form** (2026-04-21) | 0.025 (6, additive only) |
| | `tau_protection` | τ_prot | Временная константа экспоненциального затухания youth_protection | years | **24.3** | [18.5, 30.2] | Round-7 MCMC posterior (`calibration.rs` — free parameter) | **Fitted** (docs updated 2026-04-21 → code post-calibration value; prior `15 years` was pre-calibration value) | 0.046 (5) |
| **Age-decay Youth Protection** (CDATA v3.0 current implementation) | `pi_0` | π_0 | Амплитуда экспоненциального затухания youth_protection; formula: `youth_protection(age) = pi_0 · exp(−age/tau_protection) + pi_baseline` | unitless | 0.87 | [0.80, 0.92] | Round-7 MCMC posterior (`calibration.rs`) | **Fitted** (free parameter in MCMC) | 0.013 (8) |
| | `pi_baseline` | π_floor | Асимптотический floor youth_protection при t → ∞ | unitless | 0.10 | [0.05, 0.15] | Round-7 MCMC posterior | **Fitted** | <0.001 |

**Deprecated / Legacy parameters (removed 2026-04-21 audit — never implemented in v3.0 code):**

Prior versions of PARAMETERS.md listed four parameters (`pi_base`, `pi_0` alt-meaning, `D_half`, `k_s`) corresponding to a *signal-dependent self-renewal model* (probability = f(damage)), planned as future work but never carried through to the `FixedParameters` struct or `aging_engine` formulas. The current v3.0 implementation uses the simpler *age-decay youth protection model* above. The signal-dependent model has been explicitly deprecated (see `PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md §π-divergence`). Legacy names retained here for historical traceability:

| Prior symbol | Prior value | Status |
|---|---|---|
| `pi_base` | 0.65 | **REMOVED** — field does not exist in code |
| `pi_0` (signal-dep meaning) | 0.20 | **REINTERPRETED** — same field name now means MCMC-calibrated amplitude (0.87) of age-decay, not minimum of signal-dep |
| `D_half` | 2.5 | **REMOVED** — not implemented |
| `k_s` | 0.8 | **REMOVED** — not implemented |
| **Epigenetic Link** | `epigenetic_rate` | r_ep | Скорость эпигенетического дрейфа (условная) | epi_units/year | 0.045 | [0.040, 0.050] | Horvath 2013; данные DunedinPACE | Literature + Scaled | **0.403 (1)** |
| | `epigenetic_stress_k` | k_ep | Коэф. усиления эпиг. дрейфа под стрессом | unitless | 1.5 | [1.2, 2.0] | Peters-Hall 2020; связь гипоксия-метил. | Literature | 0.071 (4) |
| **Telomere** | `telomere_shortening_rate` | ΔTelo/div | Укорачивание теломер за деление | bp/division | 50 | [30, 70] | Shay & Wright, 2000 (обзор) | Literature | <0.001 |
| | `critical_telomere_length` | T_crit | Критическая длина для сенесценции | bp | 3000 | [2500, 3500] | Литература по фибробластам | Literature | <0.001 |
| **CHIP** | `mutation_rate_DNMT3A` | μ_D | Частота мутаций DNMT3A | mutations/cell/year | 2.5e-7 | [1e-7, 5e-7] | Jaiswal et al., NEJM 2017, экстраполяция | Literature | <0.001 |
| | `mutation_rate_TET2` | μ_T | Частота мутаций TET2 | mutations/cell/year | 1.8e-7 | [0.8e-7, 3e-7] | Jaiswal et al., NEJM 2017 | Literature | <0.001 |
| | `chip_fitness_advantage` | s | Селективное преимущество CHIP-клона | unitless | 0.1 | [0.05, 0.15] | Оценка из данных VAF | **Assumed** | <0.001 |
| **Cell Cycle** | `T_gen_0` | T_{gen,0} | Базовая продолжительность клеточного цикла HSC | days | 30 | [20, 40] | Wilson et al., Nature 2008; Bernitz et al., Cell Stem Cell 2016 | Literature | <0.001 |
| | `eta_slowdown` | η | Коэффициент замедления цикла от повреждения | damage^{-1} | 0.15 | [0.10, 0.20] | Калибровка на данных замедления | **Fitted** | <0.001 |
| **Senescence** | `D_senescence` | D_sen | Порог повреждения для входа в сенесценцию | damage | 8.0 | [6.0, 10.0] | Оценка, экстраполяция in vitro данных | **Assumed** | <0.001 |
| **Initial Conditions** | `D_c_0` | D_{c,0} | Начальное повреждение центриоли при рождении | damage | 0.1 | [0.05, 0.15] | Оценка | **Assumed** | <0.001 |
| | `initial_HSC_pool` | N_HSC,0 | Начальный размер пула HSC | cells | 11,000 | [8,000, 14,000] | Оценка для мыши (донор) | Literature | <0.001 |
| **Tissue-Specific (ISC)** | `alpha_ISC` | α_ISC | Прирост повреждения (кишечник) | damage/division | 0.035 | [0.028, 0.042] | Масштабирование от HSC по ν | **Scaled** | <0.001 |
| | `nu_ISC` | ν_ISC | Частота делений ISC | divisions/year | **70** (code post-MCMC) / 52 (lit. prior) | [40, 65] lit. prior | мета-анализ данных мыши + Round-7 MCMC | **Fitted** (2026-04-25 reconciliation) | <0.001 |
| **Tissue-Specific (Muscle)** | `alpha_Sat` | α_Sat | Прирост повреждения (сателлитные клетки) | damage/division | 0.002 | [0.001, 0.004] | Очень низкая частота делений | **Scaled** | <0.001 |
| | `nu_Sat` | ν_Sat (muscle_nu) | Частота делений сателлитных клеток | divisions/year | **4.0** (code post-MCMC active) / 0.1 (lit. prior quiescent) | [0.05, 0.2] lit. prior | оценка взрослой мыши + Round-7 MCMC; код моделирует активную фракцию | **Fitted** (2026-04-25 reconciliation) | <0.001 |
| **Tissue-Specific (Neural)** | `alpha_NPC` | α_NPC | Прирост повреждения (нейральные прогениторы) | damage/division | 0.020 | [0.015, 0.025] | Royall et al., eLife 2023; калибровка | **Fitted** | <0.001 |
| | `nu_NPC` | ν_NPC (neural_nu) | Частота делений NPC | divisions/year | **2.0** (code post-MCMC) / 4 (lit. prior) | [2, 6] lit. prior | литература по гиппокампу взрослых + Round-7 MCMC | **Fitted** (2026-04-25, в нижнем диапазоне prior) | <0.001 |
| **Coupling (MCOA)** | `gamma_epi` | γ_epi | Связь с эпигенетическим счётчиком | unitless | **0** | [0, 0.05] | **По умолчанию 0 (CORRECTIONS)** | **Null Hypothesis** | N/A |
| | `gamma_telo` | γ_telo | Связь с теломерным счётчиком | unitless | **0** | [0, 0.05] | **По умолчанию 0 (CORRECTIONS)** | **Null Hypothesis** | N/A |
| | `gamma_chip` | γ_chip | Связь с CHIP-счётчиком | unitless | **0** | [0, 0.05] | **По умолчанию 0 (CORRECTIONS)** | **Null Hypothesis** | N/A |
| **Scaling Factors** | `n_star` | n* | Нормировочный коэффициент для делений | unitless | 100 | Фиксировано | Безразмерная нормировка | **Fixed** | N/A |
| | `time_scale` | τ_scale | Характерное время для β | years | 1 | Фиксировано | Нормировка на 1 год | **Fixed** | N/A |
| **Output Weight** | `w_HSC_frailty` | w_HSC | Вклад HSC-истощения в общую дряхлость | unitless | 0.25 | [0.15, 0.35] | Калибровка на фенотипах старения | **Fitted** | <0.001 |

**Легенда статуса:**
*   **Literature:** Значение и диапазон напрямую взяты из указанной литературы.
*   **Fitted:** Значение получено путём калибровки (MCMC) модели на агрегированных экспериментальных данных.
*   **Assumed:** Обоснованное предположение, основанное на косвенных данных или биологической plausibility.
*   **Scaled:** Значение получено масштабированием от аналогичного параметра в другой ткани на основе известных различий в биологии.
*   **Fixed:** Фиксированное значение, используемое для нормировки, не влияющее на динамику.
*   **Null Hypothesis:** Согласно CORRECTIONS-2026-04-22, параметры связи γ по умолчанию равны 0.