# BioSense — Knowledge System

## Ze Theory: Core Definitions

### Fundamental formulas
```
Binary sequence:  x_k = 1  if  sample > median, else 0
Ze velocity:      v = N_S / (N − 1)           [N_S = number of binary switches]
Fixed point:      v* = 0.45631                 [theoretical maximum materialization]
Cheating index:   χ_Ze = 1 − |v − v*| / max(v*, 1−v*)    ∈ [0, 1]
Ze-optimal freq:  f_opt = v* × fs / 2
```

### Properties
- χ_Ze = 1 when v = v* (optimal)
- χ_Ze → 0 as v → 0 (no switching) or v → 1 (constant switching)
- At 128 Hz: f_opt = 29.2 Hz (beta/gamma boundary)
- At 100 Hz: f_opt = 22.8 Hz
- α-band proxy: v_proxy = 2 × f_peak / fs (monotonic transformation of f_peak)

---

## Validated Empirical Facts (EEG)

### Fact 1: Inverted-U lifespan curve (Cuban, N=196)
- χ_Ze peaks at ~36.5 years of age
- χ_Ze(young 18–35) = 0.5287 ± 0.036 — highest group
- χ_Ze(old 60–80) = 0.4895 ± 0.036
- Young vs Old: t=5.847, p<0.0001, d=1.694 [1.147, 2.487], AUC=0.715
- Quadratic model R² = 0.153
- f_opt prediction (Ze): ~22.8 Hz at 100 Hz sampling
- Observation: alpha peak in this dataset ~9–10 Hz (far from f_opt)

### Fact 2: Within-subject EC vs EO effect (Zenodo 3875159)
- Δχ_Ze(EO − EC) = +0.064 (large effect)
- Stable across 3 repeated EC/EO pairs
- Mechanism: EO causes alpha desynchronization → f shifts upward → v closer to v*
- This is the largest reproducible Ze effect observed in EEG

### Fact 3: Cross-sectional young vs old resting EEG (Dortmund, N=60)
- Proxy method: Young χ_Ze=0.449 vs Old χ_Ze=0.429; p=0.006, d=0.732; AUC=0.715
- Narrowband Ze: Young=0.450 vs Old=0.444; p=0.028, d=0.584
- ANCOVA (sex-adjusted): F(1,57)=4.56, p=0.037
- Sex × group interaction: p=0.442 (effect is not sex-specific)

### Fact 4: MPI-LEMON null result (N=30)
- Young (20–30): χ_Ze = 0.4345 ± 0.044; Old (65–75): χ_Ze = 0.4299 ± 0.035
- t=0.302, p=0.765, d=0.110 — NOT significant
- Required N for 80% power: ~1289/group (not achievable)
- Reason: ICA preprocessing + narrow age range + small N

---

## Theoretical Interpretation

### Why resting-state alpha is a weak Ze context
- Alpha peak at ~10 Hz → v_proxy ≈ 0.156 (at 128 Hz) — far from v*=0.456
- Δf_peak ≈ 0.2 Hz per decade → Δv ≈ 0.003 → Δχ_Ze ≈ 0.005 (tiny)
- Individual variability ≈ 1–2 Hz >> age effect

### Why EC→EO transition is a strong Ze context
- Alpha desynchronization shifts f from ~10 Hz toward ~12–15 Hz
- This moves v closer to v* (toward 29.2 Hz at 128 Hz)
- Large Δf → measurable Δχ_Ze

### Where Ze theory should work best (unvalidated)
1. Cognitive tasks (n-back, working memory): beta/gamma activation → v closer to v*
2. Narrow band 25–35 Hz (around f_opt): maximum χ_Ze sensitivity per Hz
3. Sleep stage transitions: major frequency shifts

---

## Turin Olfaction Theory (Knowledge Base)

### Core claim
Olfactory receptors function as molecular spectrometers via inelastic electron tunneling,
not via lock-and-key shape matching (classical theory).

### Mechanism
- Electron tunnels from donor to acceptor site in receptor
- Inelastic tunneling: electron loses energy = phonon emitted = vibrational mode activated
- Molecule is detected by its vibrational spectrum (like IR spectroscopy)
- Explains: why molecules with same shape but different deuterium content smell different

### Evidence for Turin theory
- Drosophila behavioral studies: deuterated compounds smell different despite same shape
- Human psychophysics: some evidence for spectral discrimination
- Counter-evidence: some exceptions not explained by vibration theory

### Relevance to BioSense
- VOC sensors could exploit vibrational fingerprints
- Disease states alter VOC composition → measurable spectral signatures
- Aging changes VOC profile (2-nonenal, dimethyl sulfide, etc.)

---

## HRV Ze Theory (Planned Knowledge)

### Hypothesis
- Heart rate variability = RR interval sequence
- Apply Ze binarization: v_HRV = N_switches / (N_RR − 1)
- χ_Ze_HRV as autonomic nervous system health index
- Pre-disease states → reduced HRV complexity → v moves away from v*

### Known HRV facts (standard)
- RMSSD: root mean square successive differences — parasympathetic marker
- SDNN: standard deviation of NN intervals — overall HRV
- HF power (0.15–0.4 Hz): vagal tone
- LF power (0.04–0.15 Hz): sympathetic + vagal
- LF/HF ratio: sympathovagal balance
- HRV decreases with age (consistent with Ze aging hypothesis)

---

## Key Datasets — Summary

| Dataset | Zenodo ID / Source | N | Age | Modality | Ze result |
|---------|---------------------|---|-----|----------|-----------|
| Cuban Normative EEG | 4244765 | 198 | 5–97 | EEG 19ch MAT | d=1.694 *** |
| Zenodo Jabès 2021 | 3875159 | 1 | — | EEG 128ch BrainVision | Δ=+0.064 |
| Dortmund Vital | ds005385/OpenNeuro | 60 | 20–70 | EEG EDF/BIDS | d=0.732 ** |
| MPI-LEMON | Babayan 2019 | 30 | 22–72 | EEG 62ch EEGLAB | d=0.110 ns |
| PhysioNet EEG-MMI | physionet.org | 109 | 20–89 | EEG EDF | Not analyzed |

---

## Key References

- **Ze Theory:** Tkemaladze J. Mol Biol Reports 2023. PMID 36583780
- **Aging biology:** Lezhava T. et al. Biogerontology 2011. PMID 20480236
- **MPI-LEMON:** Babayan A. et al. Sci Data 2019; 6:308
- **Cuban EEG:** Valdés-Sosa P.A. et al. NeuroImage 2021
- **Turin olfaction:** Turin L. Chem Senses 1996; 21(6):773–91
- **Ze EEG paper:** Tkemaladze J. [Manuscript under review, 2026]

---

_Last updated: 2026-03-28_
