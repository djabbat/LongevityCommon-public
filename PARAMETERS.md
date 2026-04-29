# BioSense — Parameters

## Ze Theory Constants

| Parameter | Value | Description |
|-----------|-------|-------------|
| v* | 0.45631 | Ze fixed point (maximum materialization, theoretical) |
| f_opt @ 128 Hz | 29.2 Hz | Ze-optimal frequency at standard resample rate |
| f_opt @ 100 Hz | 22.8 Hz | Ze-optimal frequency (Cuban dataset native rate) |
| f_opt @ 250 Hz | 57.0 Hz | Ze-optimal frequency (MPI-LEMON native rate) |

Formula: **f_opt = v* × fs / 2**

---

## EEG Processing Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Default resample | 128 Hz | Standard for Ze analysis; balances resolution and cost |
| Alpha band | 8–12 Hz | Standard neurophysiology |
| Ze-sensitive band | 25–35 Hz | Around f_opt; maximum χ_Ze sensitivity |
| Beta band | 13–30 Hz | Partially overlaps Ze-sensitive range |
| Gamma band | 30–45 Hz | Highest χ_Ze values in band-wise analysis |

---

## Dataset Parameters

### Cuban Normative EEG (Zenodo 4244765)
- Channels: 19 (10–20 system)
- Native sampling rate: 100 Hz
- Format: MATLAB .mat (cross-spectral matrix, averaged reference)
- N: 198 subjects, ages 5–97

### MPI-LEMON (Babayan et al. 2019)
- Channels: 62
- Native sampling rate: 250 Hz → resample to 128 Hz
- Format: EEGLAB .set
- N: 228 subjects (30 analyzed: 15 young 20–30, 15 old 65–75)

### Dortmund Vital Study (ds005385)
- Channels: standard EEG cap
- Native sampling rate: resampled to 128 Hz
- Format: EDF/BIDS
- N: 608 total (60 analyzed: 30 young 20–30, 30 old 63–70)

### Zenodo 3875159 (Jabès et al. 2021)
- Channels: 128
- Native sampling rate: 512 Hz → resample to 128 Hz
- Format: BrainVision (.vhdr + .vmrk + .eeg)
- N: 1 subject (within-subject EC vs EO validation)

---

## Statistical Thresholds

| Metric | Threshold | Interpretation |
|--------|-----------|----------------|
| Cohen's d | < 0.2 | Negligible |
| Cohen's d | 0.2–0.5 | Small |
| Cohen's d | 0.5–0.8 | Medium |
| Cohen's d | > 0.8 | Large |
| AUC | > 0.7 | Acceptable biomarker |
| AUC | > 0.8 | Good biomarker |
| p-value | < 0.05 | Significant |

---

## Age Group Definitions (BioSense standard)

| Group | Age range | Rationale |
|-------|-----------|-----------|
| Children | 5–12 yr | Pre-adolescent brain development |
| Teens | 12–18 yr | Adolescent |
| Young adults | 18–35 yr | Peak χ_Ze range |
| Middle-aged | 35–60 yr | Post-peak decline |
| Older adults | 60–80 yr | Age-related EEG slowing |
| Oldest | 80+ yr | Extreme aging |

Ze-peak predicted age: **36.5 years** (confirmed quadratic model, Cuban dataset)

---

## HRV Parameters (Planned)

| Parameter | Standard value | Description |
|-----------|---------------|-------------|
| RMSSD | ms | Root mean square of successive RR differences |
| v (HRV Ze) | — | Ze velocity of RR binarized sequence |
| χ_Ze (HRV) | — | Cheating index of cardiac signal |
| Recording length | ≥ 5 min | Minimum for stable HRV metrics |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| ZE_CUBAN_DIR | Path to Cuban EyesClose/ folder |
| ZE_DORTMUND_DIR | Path to Dortmund BIDS root |
| ZE_LEMON_DIR | Path to MPI-LEMON preprocessed data |
| ZE_ZENODO_VHDR | Path to Zenodo 3875159 .vhdr file |

---

_Last updated: 2026-03-28_
