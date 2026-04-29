# BioSense

**Multisensor wearable platform for Ze-based biomarker analysis: EEG · HRV · Olfaction**

BioSense applies Ze Theory (Tkemaladze) to three biosignal channels for aging biomarker detection
and clinical diagnostics. The EEG module is validated across 4 public datasets (N up to 196,
lifespan ages 5–97). HRV and olfaction modules are in development.

---

## Modules

| Module | Status | Key metric |
|--------|--------|------------|
| EEG | Validated | χ_Ze aging index; Cuban d=1.694; Dortmund p=0.006 |
| HRV | Planned | χ_Ze of RR intervals, autonomic profiling |
| Olfaction | Planned | Turin tunneling theory, VOC diagnostics |

---

## Quick Start (EEG)

```bash
# Install dependencies
pip install -r src/requirements.txt

# Demo (no data needed)
python3 src/eeg_ze_processor.py --demo

# Single EEG file
python3 src/eeg_ze_processor.py --file recording.edf --age 35 --label "Subj01" --resample 128

# Cuban lifespan dataset
export ZE_CUBAN_DIR=/path/to/cuban/EyesClose
python3 src/ze_cuban_analysis.py

# Dortmund young vs old
export ZE_DORTMUND_DIR=/path/to/dortmund
python3 src/ze_dortmund_pipeline.py
```

Or use the launcher:

```bash
./biosense.sh
```

---

## Ze Theory (Core)

```
Binary sequence:  x_k = 1  if  sample > median, else 0
Ze velocity:      v = N_S / (N − 1)        [N_S = switches]
Fixed point:      v* = 0.45631
Cheating index:   χ_Ze = 1 − |v − v*| / max(v*, 1−v*)    ∈ [0, 1]
```

**Aging hypothesis:** signal slows with age → v moves away from v* → χ_Ze decreases.

Ze-optimal frequency: **f_opt = v* × fs/2** (≈ 29.2 Hz at 128 Hz sampling rate)

---

## Structure

```
BioSense/
├── CONCEPT.md          # Full project concept
├── README.md           # This file
├── CLAUDE.md           # AI assistant rules
├── TODO.md             # Task list
├── PARAMETERS.md       # Key parameters and constants
├── MAP.md              # Component and dependency map
├── MEMORY.md           # Decisions and lessons learned
├── LINKS.md            # Ecosystem connections
├── KNOWLEDGE.md        # Domain knowledge corpus
├── biosense.sh         # Main launcher
├── src/                # All source code
│   ├── eeg_ze_processor.py
│   ├── ze_cuban_analysis.py
│   ├── ze_dortmund_pipeline.py
│   ├── ze_ec_eo_analysis.py
│   ├── ze_lemon_analysis.py
│   ├── ze_bandwise.py
│   ├── ze_alpha_peak.py
│   ├── ze_batch_pipeline.py
│   └── requirements.txt
├── data/               # Datasets (not committed to git)
│   ├── cuban/
│   ├── lemon/
│   └── zenodo/
├── results/            # Analysis outputs (JSON, PNG)
└── Materials/          # Reference documents (Ze.docx, etc.)
```

---

## Validated Results

| Dataset | N | Age range | Result |
|---------|---|-----------|--------|
| Zenodo 3875159 EC vs EO | 1 subj | — | Δχ_Ze = +0.064 |
| MPI-LEMON | 30 | 22–72 yr | d=0.110, p=0.765 (underpowered) |
| Dortmund ds005385 | 60 | 20–70 yr | p=0.006, d=0.732; AUC=0.715 |
| Cuban Zenodo 4244765 | 196 | 5–97 yr | Inverted-U, peak 36.5 yr, d=1.694 |

---

## Citation

Tkemaladze, J. (2026). *Ze cheating index (χ_Ze) as a group-level index of neurodynamic aging:
Experimental EEG validation across the human lifespan.* [Manuscript under review]

Also cite:
- PMID 36583780 — Tkemaladze J. *Mol Biol Reports* 2023
- PMID 20480236 — Lezhava T. et al. *Biogerontology* 2011
