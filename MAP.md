# BioSense — Component Map

## Architecture Overview

```
BioSense Platform
│
├── Module 1: EEG [ACTIVE]
│   ├── Core library:         src/eeg_ze_processor.py
│   │   ├── ze_cheating_index()     — compute χ_Ze from binary sequence
│   │   ├── alpha_peak_ze()         — proxy method (PSD peak → v → χ_Ze)
│   │   ├── narrowband_ze()         — narrowband binarization method
│   │   ├── load_cuban_mcr()        — Cuban .mat cross-spectral loader
│   │   └── group_statistics()      — t-test, Cohen's d, CI, AUC, ANCOVA
│   │
│   ├── Dataset analyses:
│   │   ├── ze_ec_eo_analysis.py    — Zenodo 3875159: EC vs EO (1 subj)
│   │   ├── ze_lemon_analysis.py    — MPI-LEMON: broadband Ze
│   │   ├── ze_bandwise.py          — MPI-LEMON: per-band Ze
│   │   ├── ze_alpha_peak.py        — MPI-LEMON: alpha peak → χ_Ze (N=30)
│   │   ├── ze_batch_pipeline.py    — MPI-LEMON: batch download + analysis
│   │   ├── ze_dortmund_pipeline.py — Dortmund: young vs old (N=60)
│   │   └── ze_cuban_analysis.py    — Cuban: lifespan curve (N=196)
│   │
│   └── Data:
│       ├── data/cuban/             — Cuban Normative EEG (.mat)
│       ├── data/lemon/             — MPI-LEMON (.set, EC condition)
│       └── data/zenodo/            — Zenodo 3875159 (BrainVision)
│
├── Module 2: HRV [PLANNED]
│   └── ze_ecg.py (→ AIM integration point)
│       ├── RR-interval Ze velocity
│       ├── χ_Ze cardiac signal
│       └── RMSSD + autonomic profiling
│
└── Module 3: Olfaction [PLANNED]
    ├── Turin theory: tunneling electron spectroscopy
    ├── VOC sensor interface
    └── Disease fingerprint classifier
```

---

## Data Flow: EEG Module

```
Raw EEG (EDF/BrainVision/EEGLAB)
        │
        ▼
[eeg_ze_processor.py: load + resample to 128 Hz]
        │
        ├──── Proxy method ─────────────────────────────────────────────────
        │     PSD computation (Welch) → find alpha peak f_peak
        │     v_peak = 2 × f_peak / fs
        │     χ_Ze = 1 − |v_peak − v*| / max(v*, 1−v*)
        │
        └──── Narrowband Ze method ─────────────────────────────────────────
              Bandpass filter (8–12 Hz)
              Binarize: x_k = 1 if sample > median else 0
              v = N_switches / (N − 1)
              χ_Ze = 1 − |v − v*| / max(v*, 1−v*)
                        │
                        ▼
              Group statistics:
              t-test + Cohen's d + 95% CI + AUC + ANCOVA (sex-adjusted)
                        │
                        ▼
              Results: JSON + PNG → results/
```

---

## Feedback Loops

```
EEG results → KNOWLEDGE.md (validated findings accumulate)
      ↓
Paper writing (Ze.docx → peer review → publication)
      ↓
New hypotheses → new datasets → validate → loop
      ↓
AIM integration: χ_Ze patient biomarker → clinical use
```

---

## Central Nodes (highest connectivity)

1. **`eeg_ze_processor.py`** — imported by all analysis scripts; core Ze math
2. **Ze Theory (v*, χ_Ze formula)** — shared across EEG, HRV, Olfaction modules
3. **AIM `ze_ecg.py`** — bridge between BioSense and patient care system
4. **KNOWLEDGE.md** — accumulates validated results for paper writing

---

## Cross-Module Dependencies

```
BioSense EEG ←──────── Ze Theory ──────────→ BioSense HRV
                             │
                             ▼
                      ZeAnastasis (theoretical)
                             │
                             ▼
                    AIM patient HRV analysis
                             │
                             ▼
                    Regenesis protocols
```

---

## External Repository

`ze_eeg_validation/` — git submodule / separate repo (djabbat/ze-eeg-validation)
Contains the full EEG validation codebase with its own README and git history.

---

## File Placement Rules

| Category | Location |
|----------|----------|
| Python source | `src/` |
| Dataset analysis scripts | `src/` |
| Raw data (not committed) | `data/` |
| Analysis outputs (JSON/PNG) | `results/` |
| Reference papers / .docx | `Materials/` |
| Core project docs (9 files) | root |
| Launcher script | root (`biosense.sh`) |

---

_Last updated: 2026-03-28_
