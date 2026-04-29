# BioSense — Project Memory

## Decisions Made

### 2026-03-26: Project renamed EEG → BioSense
- **Decision:** Expand scope from EEG-only to multisensor platform (EEG + HRV + Olfaction)
- **Rationale:** Ze theory applies to any biosignal; wearable bracelet concept requires multiple sensors
- **Impact:** TODO.md, CLAUDE.md updated; ze_eeg_validation subfolder retained as-is

### 2026-03-24: Cuban dataset chosen as primary lifespan validation
- **Decision:** Cuban Normative EEG (Zenodo 4244765, N=198, ages 5–97) selected as strongest validation
- **Rationale:** Largest N, full lifespan, confirmed inverted-U curve, d=1.694
- **Result:** χ_Ze peak at 36.5 years confirmed; validates Ze aging hypothesis

### 2026-03: Proxy method adopted as primary Ze metric for resting EEG
- **Decision:** Use alpha peak frequency → v_proxy rather than broadband binarization
- **Rationale:** Resting EEG alpha (~10 Hz) is far from v* (0.45631); broadband Ze gives
  noisy results. Proxy method is equivalent to standard alpha peak analysis but expresses
  results in Ze units.
- **Limitation:** χ_Ze is a monotonic transformation of f_peak at fixed fs; no independent
  information beyond alpha peak frequency.

### 2026-03: Resting-state EEG identified as low-sensitivity context for Ze
- **Decision:** Shift focus to task-based EEG (cognitive load) and narrow 25–35 Hz band
- **Rationale:** At alpha peak (~10 Hz), Δχ_Ze per Hz is tiny (~0.005); for 80% power
  (d=0.11, LEMON result) need N≈1289/group — not achievable
- **Next target:** n-back / working memory datasets where beta/gamma dominate

### 2025: ze_eeg_validation/ created as separate git repo
- **Decision:** Keep EEG validation code in separate repo (djabbat/ze-eeg-validation)
- **Rationale:** Public scientific code should be independently citable and reproducible
- **Structure:** ze_eeg_validation/ is a subfolder within BioSense but has own git history

---

## Lessons Learned

### EEG analysis
1. **ICA preprocessing smooths individual differences** — preprocessed LEMON data showed
   smaller effects than Dortmund (less preprocessing). Less filtering = more Ze signal.
2. **Cross-spectral matrix (Cuban) gives cleaner f_peak** than PSD on raw EEG —
   averaging reference + Laplacian improves signal-to-noise.
3. **EC vs EO transition is the strongest Ze effect** (d≈0.6–1.0) — much larger than
   young vs old resting state (d≈0.1). Within-subject designs are more powerful.
4. **Band-specific Ze in gamma (30–45 Hz) shows correct direction** but needs larger N.
   Beta/gamma bands are where Ze theory has theoretical advantage.

### Statistical
5. **ANCOVA (sex-adjusted) is necessary** for cross-sectional age comparisons —
   sex effects on alpha frequency confound Ze group differences.
6. **AUC > 0.7 = acceptable biomarker** — Dortmund AUC=0.715 meets this threshold.
7. **Quadratic model for lifespan** — inverted-U better than linear (χ_Ze peaks ~36.5 yr).

### Project management
8. **Keep raw data OUT of git** — data/ folder should be in .gitignore
9. **Results (JSON/PNG) can be committed** — small size, useful for quick review
10. **ze_eeg_validation/ has its own git** — do not double-commit files in subdirectory

---

## History / Milestones

| Date | Milestone |
|------|-----------|
| ~2025 | EEG project started; Ze theory applied to EEG |
| 2025 | Zenodo 3875159 EC vs EO analysis — within-subject validation |
| 2025 | MPI-LEMON analysis — null result, underpowered (d=0.11) |
| 2026-03 | Dortmund ds005385 — significant result (p=0.006, d=0.732, N=60) |
| 2026-03 | Cuban Normative EEG — lifespan curve confirmed (d=1.694, N=196) |
| 2026-03 | ze_eeg_validation/ repo structured; README written; ready for submission |
| 2026-03-26 | Project renamed EEG → BioSense; scope expanded |
| 2026-03-28 | Full project initialization: 9 core files created; git push to djabbat/BioSense |

---

## Open Questions

1. Why exactly is v* = 0.45631? (theoretical derivation vs empirical calibration)
2. How to interpret χ_Ze in units of neurophysiology (Hz equivalents)?
3. Why EC→EO transition (d≈0.6–1.0) >> young→old resting (d≈0.1)?
4. Will task-based EEG (n-back, working memory) show larger Ze aging effect?
5. Does Ze theory work for HRV with the same v*?

---

_Last updated: 2026-03-28_
