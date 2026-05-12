# THEORY — AutomatedMicroscopy

## Формальная теоретическая основа

### 1. Проблема — human shift overhead в live-cell microscopy

Traditional time-lapse microscopy требует continuous human oversight: ручная focus adjustment, manual field-of-view selection, visual inspection of cultures, media changes, intervention at anomalies.

В condiciях single-PI labs (as Georgia Longevity Alliance), 24/7 continuous supervision невозможно. Результат: **либо эксперименты ограничены рабочими часами (8-12h/day), либо закупается дорогое автоматизированное оборудование ($25-50k+)**.

### 2. Гипотеза

**Теза:** Low-cost retrofit ($4,500) + AI agent в роли "night-shift lab technician" позволяет достичь industrial-grade 24/7 imaging без capital-intensive hardware.

**Формальная аксиома M1 (Feasibility):**
> Для class CDATA-type experiments (time-lapse polyGlu intensity measurement on mother centrioles в BJ-hTERT fibroblasts), а AI agent (Claude Code в `/overnight` режиме) может выполнять supervisory function eq. quality к trained technician, при условиях:
> - **Well-defined PROMPT** (natural-language protocol)
> - **Bounded autonomy** (pre-authorized routine actions + require-human-approval для strategic decisions)
> - **Full journaling** (every decision logged с rationale, reproducible after-the-fact)

### 3. Prompt-driven supervision model

Formalization of experimenter-AI interaction:

```
PROMPT: natural-language description of experiment goals
 ↓ parsed by Claude Code
CRITERIA: concrete thresholds, metrics, conditions
 ↓ continuous monitoring every 30 min
OBSERVATIONS: image data, environmental sensors
 ↓ comparison to CRITERIA
SIGNAL: INFO / WARN / CRIT → human
 OR continue_schedule autonomously
```

Bayesian decision-theoretic framing:

```
P(action | observation, prompt) ∝ P(observation | action, prompt) · P(action | prompt)
```

где:
- **prior P(action|prompt)** = "what would a trained technician do here"
- **likelihood P(observation|action, prompt)** = expected outcome given protocol compliance
- Decision: select action maximizing expected reward (experiment success ∩ biosafety ∩ human trust)

### 4. Аксиомы subproject

**M1 (Feasibility):** AI-operated microscopy achieves ≥80% of trained-technician supervision quality для routine protocols, at <20% cost.

**M2 (Interpretability):** Every AI decision must link к explicit PROMPT.md line + measurable observations. No "black-box" автономных actions без traceable rationale.

**M3 (Bounded autonomy):** AI acts only within `auto_allow` policy list; `require_human_approval` gates preserve human strategic control; `forbidden` gates preserve biosafety.

**M4 (Reproducibility):** Complete journals (decisions + rationale + observations) enable post-hoc audit of any experimental run by human reviewer.

### 5. Scope

**In scope:**
- Live-cell fluorescence imaging (BF + FITC + TRITC + DAPI channels)
- Z-stack acquisition (up to 20 μm range, 2 μm steps)
- Environmental chamber monitoring (37°C + 5% CO₂ + humidity)
- Autonomous autofocus, channel switching, stage positioning
- Image analysis pipeline (CellPose segmentation, ImageJ measurements)
- Signal generation to human experimenter per PROMPT.md

**Out of scope (for Phase A):**
- Physical cell manipulation (no liquid handling robot в Phase A)
- Chamber opening для media change (manual, human task)
- Novel imaging modalities (only standard epifluorescence)
- Cross-lab federated coordination (that's FCLC scope)
- Therapeutic intervention decisions (outside AI policy)

### 6. Interfaces с другими подпроектами LongevityCommon

| Subproject | Interface |
|------------|-----------|
| **CDATA** | Primary user — Phase A experiments run on this platform |
| **FCLC** | Future: anonymized imaging data contribution to federated learning pool |
| **MCOA** | Future: multi-counter experiments (Telomere, MitoROS) reuse same infrastructure |
| **BioSense** | Potential: shared signal-processing pipelines (cross-domain aging markers) |

### 7. Predictions

1. **Data yield:** 6 months `/overnight` operation → ~900 GB imaging data, ~40 decisions/night journaled = 7,200 logged decisions total
2. **Efficiency:** experiments complete 2-3× faster than with 9-5 human oversight (continuous vs 40-hour weeks)
3. **Cost per experiment:** ~$5k equipment amortization + ~$20 AI subscription per 6-month run = ~$5,020 per experimental cycle
4. **Reliability:** 95%+ uptime target (UPS + redundant sensors + fail-safe policies)

### 8. Falsification conditions

Platform is **falsified / not-suitable** если:
- Claude Code decisions deviate from trained-technician judgment >20% случаев (measured post-hoc blind review by independent scientist)
- Hardware uptime <80% over first 60 days
- Contamination rate >10% per experimental run (vs typical 1-3% in standard microscopy)
- User (Jaba) abandons autonomous mode after 1 month (too stressful, too much supervision needed)

### 9. Связь с MCOA framework

AutomatedMicroscopy — **instrumental layer** не theoretical counter. Но сам факт его существования enables MCOA framework operationally: без 24/7 imaging infrastructure невозможно собрать данные для temporal dynamics D_i(n, t) разных counter'ов.

Без AutomatedMicroscopy → MCOA остаётся теоретической абстракцией.
С AutomatedMicroscopy → MCOA получает эмпирический substrate.
