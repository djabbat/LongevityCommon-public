# BioSense — Ecosystem Links

## Direct Integrations

### AIM (~/Desktop/AIM/)
- **Link:** `ze_ecg.py` in AIM provides HRV analysis for patients using χ_Ze metric
- **Data flow:** BioSense Ze algorithms → AIM patient assessments
- **Shared:** Ze theory constants (v* = 0.45631), cheating index formula
- **AIM modules using Ze:** `ze_ecg.py` → patient HRV profile, RMSSD, v*-based autonomic score
- **Clinical use:** BioSense biomarkers (χ_Ze) inform integrative medicine protocols in AIM

### ZeAnastasis (~/Desktop/ZeAnastasis/)
- **Link:** Ze Theory is the mathematical foundation of both projects
- **BioSense role:** Experimental validation of Ze Theory on biological signals (EEG, HRV)
- **ZeAnastasis role:** Theoretical development of Ze framework
- **Shared:** v* constant, χ_Ze formula, Ze velocity definition
- **Publications:** Ze.docx (in BioSense/Materials/) is the primary Ze theory paper

---

## Indirect Connections

### CDATA (~/Desktop/CDATA/)
- **Link:** Statistical methods (t-test, Cohen's d, ANCOVA) overlap
- **Data:** No shared data but similar biostatistics approach
- **Publications:** May co-cite Ze theory paper from BioSense

### Regenesis (~/Desktop/Regenesis/)
- **Link:** BioSense EEG and HRV biomarkers (χ_Ze) inform anti-aging protocols
- **Future:** BioSense wearable could monitor Regenesis protocol effectiveness
- **Shared:** Aging biomarker framework

### ClinicA (~/Desktop/ClinicA/)
- **Link:** Clinical implementation of BioSense biomarkers in Dr. Tkemaladze's practice
- **Future:** BioSense wearable data integrated into patient clinical records

---

## External Resources

### Datasets
| Dataset | URL | Status |
|---------|-----|--------|
| Cuban Normative EEG | https://zenodo.org/records/4244765 | Downloaded to data/cuban/ |
| Zenodo 3875159 | https://zenodo.org/records/3875159 | In data/zenodo/ |
| MPI-LEMON | https://fcon_1000.projects.nitrc.org/indi/retro/MPI_LEMON.html | Partial (30 subj in data/lemon/) |
| Dortmund ds005385 | https://openneuro.org/datasets/ds005385 | Used in analysis |
| PhysioNet EEG-MMI | https://physionet.org/content/eegmmidb/ | Not downloaded |

### GitHub Repositories
| Repo | URL | Content |
|------|-----|---------|
| ze-eeg-validation (public) | https://github.com/djabbat/ze-eeg-validation | EEG validation codebase |
| BioSense (private) | https://github.com/djabbat/BioSense | Full project (this repo) |

### Key Literature
| Reference | Relevance |
|-----------|-----------|
| PMID 36583780 — Tkemaladze J. Mol Biol Reports 2023 | Ze Theory primary paper |
| PMID 20480236 — Lezhava T. et al. Biogerontology 2011 | Aging biology foundation |
| Babayan et al. 2019, Sci Data 6:308 | MPI-LEMON dataset paper |
| Valdés-Sosa et al. | Cuban Normative EEG dataset paper |
| Turin G. — olfactory receptor tunneling theory | Olfaction module theory |

---

## Publications Using BioSense Results

### Under review
- "Ze cheating index (χ_Ze) as a group-level index of neurodynamic aging:
  Experimental EEG validation across the human lifespan" — Tkemaladze J. (2026)

### Materials/Ze.docx
- Main Ze theory paper (located in Materials/)

---

## Notes on Ecosystem Consistency

- BioSense is listed under **AIM known projects** (CLAUDE.md Known projects table: BioSense)
- Not in AIM/Deferred/ — this is an **active** project
- AIM/NEEDTOWRITE.md contains 3 BioSense-related articles (Turin theory, HRV, EEG Ze-flow)

---

_Last updated: 2026-03-28_
