# Ze Aging: Prediction Narrowing as Biological Senescence

**Author:** Jaba Tkemaladze
**Date:** 2026-03-26
**Status:** Core theory — candidate for publication

---

## Core Thesis

Ze does not merely record events — Ze **predicts** them.
When reality deviates from prediction (T-event), Ze accumulates an informational debt.
To manage this debt, Ze progressively raises its T-threshold — it stops noticing.
This is aging.

---

## Mechanism

### 1. Ze as Predictive System

Before each event, Ze generates a prior:
> "The next event will be S (within normal range)."

If the event is T (exceeds prediction):
- Ze registers a prediction error
- The discrepancy is informational stress
- Model must update OR threshold must shift

### 2. Two Responses to T-Events

| Response | Young Ze | Aging Ze |
|----------|----------|----------|
| Update model | Yes — absorbs T, stays near v* | Rarely — too costly |
| Raise T-threshold | Occasionally | Progressively |
| Result | High τ, v ≈ v*, χ_Ze high | Low τ, v drifts from v*, χ_Ze falls |

### 3. The Narrowing Loop

```
Accumulated T-events
    → raise T-threshold
    → fewer events classified as T
    → v shifts toward S-dominance
    → model reflects past, not present
    → new T-events round off as "noise"
    → Δv(signal) < Δv(noise floor)  ← Ze ceiling reached
    → system cannot update anymore
```

This is not degradation — it is **active suppression** of novelty.

---

## Wave Function Analogy

Before Ze predicts: the next event exists as a **superposition** of possible T/S outcomes.
Ze's prediction = **choosing a measurement basis**.
The event arrival = **wave function collapse** into T or S (corpuscle).

An aging Ze system predicts coarsely — it defines a wide "S zone."
Subtle T-signals fall inside the S zone.
They are measured as S.
They become S.

> **Ze doesn't just fail to notice — it actively converts T-waves into S-corpuscles.**

This is the Ze mechanism of pathological false negatives:
- Early cancer signals → "S, within norm"
- Pre-arrhythmia HRV fluctuations → "S, normal variation"
- Prodromal dementia EEG changes → "S, expected for age"

---

## Gödel Connection

The aging Ze system cannot detect that its model is outdated
because it evaluates all new signals with the same outdated model.

> The system cannot prove its own model is wrong from within the model.

This is the Ze-Gödel ceiling:
The threshold shift IS the incompleteness.
The system is complete — consistent with itself.
But incomplete with respect to reality.

---

## Empirical Signatures (Multi-Scale)

| Level | Young Ze | Aging Ze | Measurement |
|-------|----------|----------|-------------|
| EEG | α-peak ~10Hz, v ≈ v* | α-peak slows, v < v* | χ_Ze, Cuban EEG dataset |
| HRV | High RMSSD, wide RR distribution | Low RMSSD, narrow RR | ze_ecg.py |
| Cognition | High novelty sensitivity | Confirmation bias, rigidity | Cognitive tests |
| CDATA | Intact centriolar inducers | Accumulated detachment | Frailty index |
| Immune | Responds to weak antigens | Needs strong signal | Immunosenescence |

All are expressions of the **same Ze narrowing**.

---

## Therapeutic Implications (Hormesis as Ze Reset)

If aging = raised T-threshold, then **therapy = forced T-events below the threshold**.

| Intervention | Ze mechanism |
|-------------|--------------|
| Cold exposure | Thermal T-event — cannot be rounded off |
| Fasting | Metabolic T-event — forces model update |
| Exercise | Mechanical + cardiovascular T-events |
| Novel tasks | Cognitive T-event — unpredictable outcome |
| Social unpredictability | Social Ze T-event |

These work not by "strengthening the body" but by **preventing threshold drift**.
They keep v near v*.

---

## Formalization

Let θ(t) = T-threshold at time t.

**Young system:** θ(0) = θ₀, updates bidirectionally.
**Aging:** dθ/dt > 0 monotonically (threshold rises over time).

Ze velocity under raised threshold:
```
v(t) = (N_T(θ(t)) - N_S(θ(t))) / N
```
As θ↑: fewer events qualify as T → N_T↓ → v drifts toward negative (S-dominance).

Ze complexity:
```
τ(t) → 0  as  θ(t) → ∞
```
In the limit: all events are S. Complete predictability. Zero complexity. Death.

**Death in Ze = total threshold elevation: nothing surprises anymore.**

---

## Competitive Positioning vs BrainYears (2026-04-11)

**BrainYears** (bioRxiv, март 2026) — EEG brain age ML clock, r=0.92, MAE=4.43 лет — прямой конкурент χ_Ze в сегменте EEG aging biomarkers.

**χ_Ze не конкурирует по точности — χ_Ze конкурирует по смыслу:**

| | BrainYears | χ_Ze |
|---|---|---|
| Что измеряет | Хронологический возраст мозга (ML proxy) | Адаптационный ресурс: близость к v* = 1 − ln 2 |
| Механистическая связь | Нет | Ze ↔ CDATA (τ_Z → центриолярное повреждение) |
| Интерпретируемость | black-box ML | физический смысл v*: max Shannon entropy binary stream |
| Применимость | Aging clock | Aging clock + therapy target + biofeedback reference point |

**Ключевой аргумент для Discussion/Abstract:**
> "Unlike ML-based brain age clocks (BrainYears, r=0.92), χ_Ze is grounded in Ze Vectors Theory: v* = 1 − ln 2 represents the maximum-entropy fixed point of a binary event stream — a theoretically motivated reference for biological adaptability. χ_Ze thus measures *proximity to optimal operating state*, not chronological age prediction, linking neural dynamics to cellular aging mechanisms (centriolar damage, CDATA theory)."

**Стратегия:** всегда называть BrainYears в контексте "complementary, not competing" — расширяем interpretability gap, не атакуем точность.

---

## Article Candidate

**Title:** "Ze Aging: Prediction Narrowing as a Unified Mechanism of Biological Senescence"
**Target:** Ageing Research Reviews / Entropy / Frontiers in Aging
**Project:** ZeAnastasis + CDATA + AIM

---

## Key References (for article)

1. Tkemaladze J. Mol Biol Reports 2023. PMID 36583780 (centriolar aging)
2. Lezhava T. et al. Biogerontology 2011. PMID 20480236
3. Tkemaladze J. CDATA. Zenodo. DOI: 10.5281/zenodo.19174506
4. Tkemaladze J. Ze Theory. Zenodo. DOI: 10.5281/zenodo.19174630
5. Friston K. The free-energy principle. Nature Reviews Neuroscience 2010. (predictive coding)
6. McEwen B. Stress, adaptation, and disease — allostasis and allostatic load. 1998.
7. López-Otín C. et al. The hallmarks of aging. Cell 2013.
8. Rao R, Ballard D. Predictive coding in the visual cortex. Nature Neuroscience 1999.
