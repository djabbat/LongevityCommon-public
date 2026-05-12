# PMID 36977999 — Bürgy L 2023

**Title:** CenFind: a deep-learning pipeline for efficient centriole detection in microscopy datasets.

**Journal:** BMC Bioinformatics · **Year:** 2023

**Authors (7 total, first 30 shown):**
- Bürgy L
- Weigert M
- Hatzopoulos G
- Minder M
- Journé A
- Rahi SJ
- Gönczy P

**DOI:** 10.1186/s12859-023-05214-2

**PubMed URL:** https://pubmed.ncbi.nlm.nih.gov/36977999/

**Verification stamp:** ✅ NCBI esummary 2026-05-12 — author + journal + year + title fetched live.

---

## Abstract (PubMed efetch)

```
1. BMC Bioinformatics. 2023 Mar 28;24(1):120. doi: 10.1186/s12859-023-05214-2.

CenFind: a deep-learning pipeline for efficient centriole detection in 
microscopy datasets.

Bürgy L(1), Weigert M(2), Hatzopoulos G(1), Minder M(3)(4), Journé A(1), Rahi 
SJ(3), Gönczy P(5).

Author information:
(1)Swiss Institute for Experimental Cancer Research, School of Life Sciences, 
Swiss Federal Institute of Technology Lausanne, 1015, Lausanne, Switzerland.
(2)Interschool Institute of Bioengineering, School of Life Sciences, Swiss 
Federal Institute of Technology Lausanne, 1015, Lausanne, Switzerland.
(3)Institute of Physics, Swiss Federal Institute of Technology Lausanne, 1015, 
Lausanne, Switzerland.
(4)SBB Consulting, Hilfikerstrasse 1, 3000, Bern 65, Switzerland.
(5)Swiss Institute for Experimental Cancer Research, School of Life Sciences, 
Swiss Federal Institute of Technology Lausanne, 1015, Lausanne, Switzerland. 
pierre.gonczy@epfl.ch.

BACKGROUND: High-throughput and selective detection of organelles in 
immunofluorescence images is an important but demanding task in cell biology. 
The centriole organelle is critical for fundamental cellular processes, and its 
accurate detection is key for analysing centriole function in health and 
disease. Centriole detection in human tissue culture cells has been achieved 
typically by manual determination of organelle number per cell. However, manual 
cell scoring of centrioles has a low throughput and is not reproducible. 
Published semi-automated methods tally the centrosome surrounding centrioles and 
not centrioles themselves. Furthermore, such methods rely on hard-coded 
parameters or require a multichannel input for cross-correlation. Therefore, 
there is a need for developing an efficient and versatile pipeline for the 
automatic detection of centrioles in single channel immunofluorescence datasets.
RESULTS: We developed a deep-learning pipeline termed CenFind that automatically 
scores cells for centriole numbers in immunofluorescence images of human cells. 
CenFind relies on the multi-scale convolution neural network SpotNet, which 
allows the accurate detection of sparse and minute foci in high resolution 
images. We built a dataset using different experimental settings and used it to 
train the model and evaluate existing detection methods. The resulting average 
F1-score achieved by CenFind is > 90% across the test set, demonstrating the 
robustness of the pipeline. Moreover, using the StarDist-based nucleus detector, 
we link the centrioles and procentrioles detected with CenFind to the cell 
containing them, overall enabling automatic scoring of centriole numbers per 
cell.
CONCLUSIONS: Efficient, accurate, channel-intrinsic and reproducible detection 
of centrioles is an important unmet need in the field. Existing methods are 
either not discriminative enough or focus on a fixed multi-channel input. To 
fill this methodological gap, we developed CenFind, a command line interface 
pipeline that automates cell scoring of centrioles, thereby enabling 
channel-intrinsic, accurate and reproducible detection across experimental 
modalities. Moreover, the modular nature of CenFind enables its integration in 
other pipelines. Overall, we anticipate CenFind to prove critical for 
accelerating discoveries in the field.

© 2023. The Author(s).

DOI: 10.1186/s12859-023-05214-2
PMCID: PMC10045196
PMID: 36977999 [Indexed for MEDLINE]

Conflict of interest statement: None declared.
```
