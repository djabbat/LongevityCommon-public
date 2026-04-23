# DEEP AUDIT — Ontogenesis (strict independent review)

**Date:** 2026-04-21
**Auditor role:** Independent (CommonHealth ecosystem Claude agent)
**Scope:** `~/Desktop/CommonHealth/Ontogenesis/` — CONCEPT v4.2 + THEORY/EVIDENCE/OPEN_PROBLEMS/PARAMETERS/DESIGN/README
**Verification methods:** NCBI E-utilities (esummary), Nature/Oxford Academic direct DOI resolution, targeted 2020–2026 PubMed / Nature / NIH literature searches
**Outcome:** **MAJOR REVISION REQUIRED** before any external submission (grant, preprint, peer-reviewed article). Multiple fabricated PMIDs in KNOWLEDGE.md, one factual mis-statement in CONCEPT.md §9.2, and significant competitive-analysis + regulatory gaps.

---

## §1. Core scientific claims inventory (top 10) and verification status

Claims extracted from `CONCEPT.md`, `THEORY.md`, `EVIDENCE.md` with their stated PMID/DOI backing, cross-checked against NCBI E-utilities and publisher metadata on 2026-04-21.

| # | Claim | Stated citation | Verified? | Notes |
|---|-------|-----------------|-----------|-------|
| C1 | Etagenesis is the umbrella developmental process zygote→death (Frolkis 1999) | PMID 10394081, DOI 10.1159/000022092, *Gerontology* 45:227–232 | **YES** — title/author/journal/year match exactly (Frolkis VV, "Aging, antiaging, ontogenesis and periods of age development", *Gerontology* 1999) | The CONCEPT §14 line item is mis-stated as "Gerontology 45:227–232"; the actual Frolkis 1999 paper in *Gerontology* is Vol. 45, but title given in EVIDENCE.md 1.1 ("Aging and aging-related diseases … *Interdisciplinary Topics in Gerontology*, 1999, Vol. 28, pp. 1–22") is a **DIFFERENT work**. The actual PMID 10394081 paper is NOT in *Interdisciplinary Topics in Gerontology* — EVIDENCE.md §1.1 confuses two separate Frolkis papers. **Fix: align EVIDENCE.md citation text with PubMed record.** |
| C2 | Frolkis's syndromes-of-aging / stress-age syndrome framework (1992) | PMID 1612465, DOI 10.1159/000213310 | **YES** — Frolkis VV "Syndromes of aging", *Gerontology* 1992 | Correct. |
| C3 | Hayflick limit ≈ 120 y theoretical (Hayflick & Moorhead 1961) | PMID 13905658 | **YES** — Hayflick L, Moorhead PS "The serial cultivation of human diploid cell strains", *Exp Cell Res* 1961 | Correct — but CONCEPT §2.2 conflates Hayflick's ~50-division fibroblast limit with a "~120 years" organism-level extrapolation. Hayflick 1961 never claims 120 years. **Fix: call this "Hayflick-limit extrapolation" or cite secondary source explicitly.** |
| C4 | Demographic lifespan cap ~115–125 y (Dong et al. 2016) | DOI 10.1038/nature19793 | **YES** paper exists and says ~115 y | BUT: CONCEPT §2.2 fails to mention that the Dong paper was rebutted by FIVE separate *Nature* Brief Communications in 2017 (Hughes & Hekimi; de Beer; Rozing; Lenart & Vaupel; Brown et al.) contesting the statistical analysis. Currently only Barbi 2018 is cited as counter-evidence. **Gap: cite the 2017 rebuttals or remove the "~115–125 years" line as empirically settled.** |
| C5 | DOSI biomarker → lifespan limit 120–150 y (Pyrkov et al. 2021) | PMID 34035236, DOI 10.1038/s41467-021-23014-1 | **YES** — Pyrkov TV et al. *Nat Commun* 2021 | Correct citation. Caveat about extrapolation vs measurement is appropriately flagged in CONCEPT §2.2. |
| C6 | Latent Change Score methodology for developmental cognitive neuroscience (Kievit et al. 2018) | PMID 29325701, DOI 10.1016/j.dcn.2017.11.007 | **YES** — Kievit RA et al. "Developmental cognitive neuroscience using latent change score models: A tutorial and applications", *Dev Cogn Neurosci* 2018 | Correct. Methodological fit to 24-parameter × 5-phase model is reasonable but see §2 gap analysis. |
| C7 | Five neurobiological epochs with turning points at ~9, ~32, ~66, ~83 years | DOI 10.1038/s41467-025-65974-8 (*Nat Commun* 2025) | **YES** — verified via Nature and Cambridge press release. Full citation: Mousley A, Bethlehem RAI, Yeh FC, Astle DE (2025). "Topological turning points across the human lifespan". *Nat Commun* 16:10055 | Correct. **But: the platform elevates this to "primary temporal grid (v4.2)" — this is one study (N=4,216, diffusion imaging only); using it as the primary periodization and labeling its discovery dates empirically certain turning points over-weights a single not-yet-replicated finding.** |
| C8 | British Birth Cohorts — **n = 27,432** with 6.7 M SNPs (CONCEPT §9.2) | DOI 10.1093/ije/dyaf141 (Shireby et al. 2025, IJE) | **PARTIAL** — The paper exists (Shireby et al. "Data Resource Profile: Genomic data in multiple British birth cohorts (1946–2001)…", *IJE* 54(5), 2025). **BUT: the actual n = 36,603, NOT 27,432. 24,192 = cohort members only; remainder includes MCS parents.** | **FACTUAL ERROR in CONCEPT §9.2.** Must be corrected to "n = 36,603 (of whom 24,192 cohort members)". |
| C9 | ~12 expected "age metamorphoses" across 0–120 y lifespan (THEORY §4.2, CONCEPT §2.6) | No citation (marked "hypothesis") | **No external support.** Twelve candidate timestamps listed in CONCEPT §2.6 are informal synthesis. | Appropriately labeled hypothetical, but empirical detector has not been run; OP-03 acknowledges this. Fine as presented, but must NEVER be cited as an established finding. |
| C10 | Ontogenesis provides D_i,0 initial damage values for MCOA counters at t=25 y | Internal cross-link to MCOA | **Internal consistency check only** — no external validation exists yet. Appendix A (CONCEPT) and THEORY §6 identify f_i as unknown. OP-02 (P1) flags this as open. | Acceptable as research direction, but MCOA integration claims in README.md ("Ontogenesis establishes initial conditions D_i,0") must be softened to "is intended to establish" until OP-02 is resolved. |

**Additional PMIDs cited in KNOWLEDGE.md (per `docs/REFERENCE_AUDIT_Ontogenesis.md`) — all verified against PubMed:**

| PMID as cited | Claim attached | Actual PubMed record | Status |
|---------------|----------------|----------------------|--------|
| 18971494 | Knickmeyer 2008 — "brain reaches 80% adult volume by age 3" | Kelly & LaMont 2008, *NEJM* — "*Clostridium difficile* — more difficult than ever" | **WRONG PMID.** Correct Knickmeyer 2008 PMID = **19020011** (Knickmeyer RC et al. "A Structural MRI Study of Human Brain Development from Birth to 2 Years", *J Neurosci* 28(47):12176–82) |
| 22306083 | Lebel 2012 — PFC myelination to ~24–25 y | Chimento et al. 2012, *Mol Cell Endocrinol* — estradiol / apoptosis in spermatocytes | **WRONG PMID.** Needs corrected Lebel reference (e.g. Lebel & Beaulieu 2011, *J Neurosci* 31:10937–47, PMID 21795544 — longitudinal DTI study) |
| 9262477 | Juul 1997 — "IGF-1 peak at Tanner III–IV" | Beltramo 1997, *Science* — anandamide transport | **WRONG PMID.** Correct Juul candidate: PMID 8126152 (Juul A et al. 1994, *JCEM* 78:744–52, IGF-I in 1030 healthy children) or the Juul 1997 *JCEM* paper — researcher must verify. |
| 5785179 | Marshall & Tanner 1969 — pubertal staging standard | Marshall & Tanner 1969, *Arch Dis Child* — "Variations in pattern of pubertal changes in girls" | **CORRECT PMID, but for the GIRLS paper.** The matched boys paper is Marshall & Tanner 1970 (*Arch Dis Child* 45:13–23, PMID 5440182). Clarify which is meant or cite both. |
| 12803352 | "NK cell efficiency at age 70 = 50%" | Margolis 2003, *Int J Qual Health Care* — primary-care satisfaction in UAE | **FABRICATED LINKAGE.** No plausible overlap; must be removed or replaced with a real citation (e.g. Gounder et al. 2018, PMID 29540975, on NK-cell senescence). |
| 28792876 | Jaiswal 2017 — CHIP acceleration after 25 y | Shi H et al. 2017, *NEJM* — NAD deficiency and niacin | **WRONG PMID.** Correct Jaiswal 2017 PMID = **28636844** (*NEJM* 377:111–121, "Clonal Hematopoiesis and Risk of Atherosclerotic Cardiovascular Disease"). |

**Scale of problem:** 6 of 6 KNOWLEDGE.md PMIDs are wrong. This is not a parsing artifact — the numeric PMIDs themselves are fabricated / hallucinated. Any grant reviewer who spot-checks will conclude the project uses invented citations. **All KNOWLEDGE.md PMIDs must be re-verified line-by-line against PubMed before the file is distributed.**

Self-cites `PMID 36583780` (Tkemaladze 2023, *Mol Biol Rep*) and `PMID 20480236` (Lezhava 2011, *Biogerontology*) are themselves real, but the REFERENCE_AUDIT flagged them as LOW_SCORE because the CONCEPT.md surrounding context (planarian regeneration, centriolar aging) does not substantively relate to 0–120 y developmental trajectory modeling. **Verdict: self-citation inclusion is forced and will appear padded to any serious reviewer.** The CLAUDE.md self-citation rule (≤15 % of references, topically relevant) is being **violated on topical-relevance** — neither paper directly supports any of claims C1–C10.

---

## §2. Developmental biology literature gaps (PubMed/Nature 2020–2026)

Mandatory citations that a developmental-platform paper reviewing 0–25 y (or 0–120 y) cannot omit in 2026:

### §2.1. Growth standards / anthropometrics — **gap**
- **WHO Multicentre Growth Reference Study (MGRS).** de Onis et al. 2004 (PMID 15069916) — methodology paper; 6-country, N≈8,500, 0–71 months. Only "WHO Growth Charts" is cited in CONCEPT §9.1 without source paper or methodology, which is insufficient for peer review.
- **CDC Extended BMI-for-age charts (Dec 2022).** Not cited. Critical for 2–20 y BMI categorization above 97th percentile (BMI up to 60). Reference: Hampl SE et al. 2023 *Pediatrics* "CDC Extended BMI-for-Age Percentiles Versus Percent of the 95th Percentile" (PMC 11074997). **Must be added** if platform claims contemporary BMI handling.

### §2.2. Pediatric brain development — **major gap**
- **Bethlehem RAI, Seidlitz J et al. (2022). "Brain charts for the human lifespan." *Nature* 604:525–533 (PMID 35388223).** N = 101,457 participants, 123,984 MRI scans, 115 days post-conception to 100 years. This is the SINGLE most important reference for any lifespan neuroimaging-normalization platform. **Not cited anywhere in Ontogenesis.** Its omission is disqualifying for any peer review that touches brain volume, because every reviewer knows about brainchart.io.
- The Mousley 2025 *Nat Commun* paper (used as "5-phase basis") explicitly builds on and cites Bethlehem 2022. Using Mousley without Bethlehem is citation laundering and reviewers will notice.

### §2.3. Pediatric epigenetic clocks — **gap**
- **PedBE clock (McEwen LM et al. 2020, PNAS 117:23329–23335).** 94-CpG pediatric buccal clock, median absolute error 0.35 y. Not cited despite platform's claim to track 0–25 y biological aging.
- Raffington L et al. 2024 systematic review of pediatric epigenetic clocks (PMC 10964791) — mandatory for any "childhood biological age" discussion.
- Shireby et al. 2020 "cortical DNA methylation clock" and Horvath clocks applied to pediatric cohorts are also missing.

### §2.4. Adolescent brain development — **gap**
- **ABCD Study publications (N=11,880, ages 9–10 at baseline, 21 US sites).** Ample primary refs: Volkow ND et al. 2018 *Dev Cogn Neurosci* (design paper); Casey BJ et al. 2018; Garavan H et al. 2018. None cited. For a platform targeting 0–25 y this is equivalent to publishing a gerontology paper without citing the Framingham or Rotterdam cohorts.
- Lebel & Deoni 2018 "Development of the myelinated brain" *Neuroimage* 182:207–218 — definitive longitudinal DTI myelination reference.

### §2.5. Pediatric / child-specific biomarkers for aging — **gap**
- Belsky DW et al. 2020 *eLife* "Quantification of the pace of biological aging in humans through a blood test" (DunedinPoAm, Dunedin cohort) — highly relevant as LCS exemplar on real longitudinal data; not cited.
- Elliott ML et al. 2021 *Nat Aging* (DunedinPACE) — 2022 updated pace-of-aging clock; not cited.

### §2.6. Child-development longitudinal methodology — **gap**
- ALSPAC (Avon Longitudinal Study of Parents and Children) — Boyd A et al. 2013 *IJE*. One of the world's three biggest pediatric birth cohorts; not cited despite direct methodological relevance.
- Millennium Cohort Study (MCS, UK) — Connelly R, Platt L 2014 *IJE*. Mentioned indirectly via British Birth Cohorts paper but not cited directly.

**Summary §2:** Ontogenesis currently cites **zero** of Bethlehem 2022, ABCD design papers, DunedinPACE, PedBE, ALSPAC, and the CDC 2022 extended BMI charts. Any developmental-biology reviewer will flag this as a preparation-level deficiency.

---

## §3. Competitive landscape — differentiation from established platforms

### §3.1. All of Us (NIH)
- **Scope:** 1 M+ adults; as of Aug 2024 began limited pediatric enrollment (ages 0–4 at 5 sites; ~1,600 enrolled by late 2025); planned expansion to ages 0–12. Source: NIH Record, NIH All of Us announcement (allofus.nih.gov).
- **What it does:** nationwide, EHR-linked, biospecimen-backed, US-diverse.
- **Overlap with Ontogenesis:** Generates the kind of real longitudinal cohort data Ontogenesis needs to calibrate phases IV–V (and eventually I–II once the pediatric arm matures). **Ontogenesis is a simulator, not a cohort — not a direct competitor, but All of Us provides the ground truth Ontogenesis still lacks.**

### §3.2. UK Biobank
- **Scope:** 500,000 adults enrolled 2006–2010 at ages 40–69. **No pediatric arm exists as of April 2026.** The CONCEPT.md's implicit reference to a "UK Biobank pediatric arm" is incorrect — no such arm exists. (The British Birth Cohorts used in CONCEPT §9.2 are administered by UCL Centre for Longitudinal Studies, not UK Biobank. These are distinct datasets.)
- **Verdict:** **Terminology error to fix.** Also: the *actual* comparator for 0–25 y is MCS / BCS70 / NCDS / NSHD / Next Steps, not "UK Biobank pediatric arm".

### §3.3. Generation R (Rotterdam, NL)
- **Scope:** 9,778 mothers enrolled April 2002 – Jan 2006; prospective from fetal life; multidisciplinary physical, ultrasound, behavioral, and biological sampling. Source: Jaddoe VWV et al. 2017 *Eur J Epidemiol* (PMID 28070760), 2010 update PMC 2991548.
- **Overlap:** Deep phenotyping across morphology + physiology + psychology + sociology from fetal life → direct competitor of the Ontogenesis *coverage* claim. **Differentiation must be explicit:** Ontogenesis is a simulation platform, not a cohort; Generation R has the data Ontogenesis simulates.

### §3.4. ABCD Study
- **Scope:** N = 11,880 US children born 2006–2008, ages 9–10 at baseline, 21 sites, baseline data release 2018; domains include brain imaging, mental health, biospecimens, neurocognition, substance use, culture. Source: Volkow ND et al. 2018 *Dev Cogn Neurosci*.
- **Overlap:** Covers the ABCD age range (9–14+ now) that is ~Phase II in Ontogenesis v4.2. ABCD data release v5.1 (2024) is the canonical source for that age band.

### §3.5. Differentiation — is Ontogenesis defensible?

| Criterion | All of Us | UK Biobank | Generation R | ABCD | Ontogenesis v4.2 |
|-----------|-----------|-----------|--------------|------|------------------|
| Type | Real cohort + EHR | Real cohort | Real cohort | Real cohort | **Simulator** |
| Size | 1 M adults, ~1.6 k children | 500 k adults | ~10 k children | ~12 k children | Synthetic (N=1,000 sample) |
| Age range | 0–adult (expanding) | 40–69 at entry | fetal–~20+ | 9–19 | 0–120 y (simulated) |
| Real biospecimens | Yes | Yes | Yes | Yes | **No** |
| MCOA-counter output | No | No | No | No | **Yes (proposed)** |
| Theoretical integration (Frolkis/MCOA) | No | No | No | No | **Yes** |
| Data-use cost to researcher | Application + DCC | Application + £ | Application | Application (DEAP/NDA) | Open-source Rust/WASM (planned) |

**Defensible differentiation:** Ontogenesis occupies a legitimately empty niche — it is a **theory-driven open-source simulator** that (a) unifies 4 domains in a single state vector, (b) maps 0–25 y trajectories onto MCOA initial conditions, (c) is runnable without a data-access committee. **This is defensible IF and only IF:**
1. Every quantitative prediction is honestly labeled as "LCS-model-synthetic" rather than "observed";
2. The platform cites the real-cohort alternatives and frames itself as complementary, not competitive;
3. The 5-phase periodization is softened from "primary grid" to "one of several candidate grids, based on a single 2025 *Nat Commun* study pending replication".

**Currently READMEs and CONCEPT.md violate all three conditions to some degree.** (e.g. "empirically выведенная модель пяти фаз" overstates the strength of Mousley 2025; CONCEPT §9.2 mis-reports the BBC n; none of All of Us / Generation R / ABCD is mentioned.)

---

## §4. Ethical / regulatory audit for 0–25 y platform

Ontogenesis §11 addresses this in ~6 bullet lines. This is **vastly insufficient** for any grant review touching human subjects.

### §4.1. IRB / human-subjects (US regulatory)
- Because the platform currently runs on **synthetic + aggregated public data only**, it qualifies for "not human subjects research" under 45 CFR 46 — but **only if** no identifiable private information is ever ingested. The §11 bullet "IRB при использовании для исследований с персональными данными" is correct but insufficient:
  - When integrated with AIM/BioSense patient data (CONCEPT §10), Ontogenesis becomes a human-subjects instrument. An IRB protocol is required **before** this integration, not after.
  - 45 CFR 46 Subpart D (Additional Protections for Children) applies whenever a subject is <18 y. Research must be classified as §46.404 (minimal risk) / §46.405 (minor increment over minimal risk with prospect of direct benefit) / §46.406 / §46.407. Ontogenesis v4.2 CONCEPT does not address this.
  - Amended regulation: 89 FR 84822 (Oct 2024) — Subpart D revisions. Must be reviewed.

### §4.2. GDPR (EU subjects)
- **GDPR Art. 8:** personal data of a child <16 y is lawful only with parental/guardian consent; Member States may lower age to 13 (DE/HU/LT/LU/SK/NL keep 16; AT = 14; others vary). Not addressed in CONCEPT §11.
- If British Birth Cohorts data are used after approval (CONCEPT §9.2) the data-controller status (UCL CLS) and the researcher's data-processing role must be documented in a DPIA (Data Protection Impact Assessment). Not mentioned.
- Cross-border transfer clauses for EU → Georgia (where the platform is developed): SCCs (Standard Contractual Clauses) or adequacy decision. Not mentioned.

### §4.3. Pediatric consent / assent
- 45 CFR 46.408: (i) parental permission + (ii) child assent when age-appropriate. "Assent" is not mere non-objection. Required assent script for platform-derived reports about a minor.
- ASEBA / CBCL licensing (CONCEPT §9.1 mentions "CBCL — лицензия") — OK at norms level, but any individual-scoring use requires a separately licensed end-user.
- WISC/WAIS licensing (Pearson) — similar; norms-only use is allowed, individual scoring is not.

### §4.4. "Not a medical device" framing
- CONCEPT §11.1 says "Research Use Only" (RUO). For US FDA this means no Clinical Decision Support claim may be made. CONCEPT §10.3 ("AIM/treatment_recommender.py — коэффициенты регенерации из Ontogenesis для коррекции протоколов") **crosses the RUO boundary** — using Ontogenesis-derived coefficients to modulate treatment in a clinical setting is CDS-as-medical-device under the 21st Century Cures Act carve-out rules (IMDRF / FDA CDS final guidance 2022). If AIM is used in a clinical practice, this integration triggers device classification analysis. **Not addressed.**
- EU MDR 2017/745 is stricter: CDS is explicitly within the definition of "medical device software" (Class IIa typically). Offering integrative-medicine "коэффициенты" that influence treatment to EU patients requires CE-mark.

### §4.5. Summary §4
Current CONCEPT §11 is a **6-line disclaimer paragraph, not a compliance framework**. For EIC Pathfinder (2026-05-12) or any IF 5+ journal touching human data, the following are **minimum required additions**:
1. Explicit IRB protocol template (even if "not human subjects" claim is made);
2. DPIA template under GDPR Art. 35;
3. Consent/assent template for minors with age-bracketed language;
4. Licensing audit for ASEBA, Pearson, WHO (ToU on growth charts);
5. Clear delineation of RUO vs CDS: either pull the AIM/treatment_recommender integration out of Ontogenesis scope or accept device-classification path.

---

## §5. Top 5 reviewer attack points (major-grant submission)

Ranked by severity; these are what a *Nature Aging* / *Developmental Science* / EIC reviewer will spot within the first pass:

1. **Fabricated PMIDs in KNOWLEDGE.md (6/6 wrong).** This is career-level reputational risk. A single attentive reviewer who opens PubMed on one PMID will conclude the whole document is hallucinated. Must be fixed before ANY external submission. *Severity: blocker.*

2. **Factual error: BBC n = 27,432 vs actual 36,603.** CONCEPT §9.2 states 27,432 participants with 6.7 M SNPs; Shireby et al. 2025 *IJE* (dyaf141) actually reports 36,603. Reviewers who access the source paper will catch this. *Severity: blocker for trust.*

3. **Missing Bethlehem 2022 "Brain charts for the human lifespan" (*Nature* 604:525–533).** The single most-cited recent paper in lifespan neuroimaging is not in any Ontogenesis file. Reviewers will assume the authors are unfamiliar with the current state of the field. Similar gap for ABCD, ALSPAC, PedBE, DunedinPACE. *Severity: field-competency blocker.*

4. **Over-reliance on a single 2025 paper (Mousley et al. *Nat Commun*) for the primary temporal grid (5 phases at 9/32/66/83 y).** Elevated from "one alternative periodization" (CONCEPT §2.3) to "основная" (§2.4) without independent replication. The Nat Commun paper is based on diffusion imaging of N=4,216 from public archives — not a definitive developmental periodization. Reviewers will ask: "Why is a single 2025 paper the basis for the whole 5-phase architecture?". *Severity: conceptual-foundation weakness.*

5. **Regulatory / ethics section is 6 bullet points where a complete compliance framework is needed.** GDPR Art. 8 (child consent), 45 CFR 46 Subpart D (minors), FDA/EU-MDR CDS classification for AIM integration, DPIA under GDPR Art. 35 — none addressed. Any EIC reviewer touching human-data workflows will ask for a full DPIA + IRB protocol. *Severity: submission-readiness blocker for EIC Pathfinder 2026-05-12.*

**Secondary concerns (not top-5 but will be raised):**
- The "12 age metamorphoses" hypothesis (CONCEPT §2.6) has no empirical test yet and is labeled speculative, but will still attract methodological scrutiny ("how are you avoiding confirmation bias?").
- The self-citation to `PMID 36583780` and `PMID 20480236` is forced — neither paper concerns developmental trajectories. The CLAUDE.md rule specifies "contextually relevant"; neither passes that test here.
- The Hayflick limit → 120 y mapping (CONCEPT §2.2) is a logical leap that Hayflick himself never made; reviewers of aging biology will note this.
- The Dong 2016 *Nature* paper is cited for the 115–125 y cap without acknowledging the 2017 Brief Communications rebuttals.

---

## §6. Required revisions (concrete checklist)

### §6.1. Immediate (before any external artifact is produced)
- [ ] **KNOWLEDGE.md PMID audit:** open each PMID in PubMed, replace wrong ones. Known fixes:
  - Knickmeyer 2008 → PMID **19020011**
  - Jaiswal 2017 → PMID **28636844**
  - Marshall & Tanner 1969 (girls) = PMID 5785179 ✓; add (boys) Marshall & Tanner 1970, PMID **5440182**
  - Lebel 2012 → verify actual intended paper; candidate Lebel & Beaulieu 2011, PMID **21795544**
  - Juul 1997 IGF-1 → verify; candidate Juul 1994, PMID **8126152**
  - NK-cell efficiency claim: replace PMID 12803352 with a real senescence citation or drop the claim
- [ ] **CONCEPT §9.2:** correct `n = 27 432` → `n = 36,603 (of whom 24,192 are cohort members; remainder includes MCS parents)`. Source: Shireby et al. 2025 *IJE* 54(5).
- [ ] **EVIDENCE.md §1.1:** align citation text of Frolkis 1999 with actual PubMed record (title: "Aging, antiaging, ontogenesis and periods of age development"; journal: *Gerontology*; NOT "Interdisciplinary Topics in Gerontology").
- [ ] Remove or justify-with-topically-relevant-content the forced self-citations of PMID 36583780 + PMID 20480236 in any publication draft (CLAUDE.md rule: ≤15 % AND topically relevant).

### §6.2. Literature updates (add to EVIDENCE.md + PARAMETERS.md)
- [ ] Bethlehem RAI et al. 2022 *Nature* 604:525–533, PMID 35388223 — brain charts lifespan normative
- [ ] WHO MGRS de Onis 2004, PMID 15069916 — growth chart methodology
- [ ] CDC Extended BMI 2022 (Hampl SE et al. 2023 *Pediatrics*)
- [ ] ABCD Study design: Volkow ND et al. 2018 *Dev Cogn Neurosci*
- [ ] ALSPAC: Boyd A et al. 2013 *IJE*
- [ ] MCS: Connelly R, Platt L 2014 *IJE*
- [ ] PedBE clock: McEwen LM et al. 2020 *PNAS* 117:23329–23335
- [ ] DunedinPACE: Belsky DW et al. 2020 *eLife*; Elliott ML et al. 2021 *Nat Aging*
- [ ] Lebel & Deoni 2018 *NeuroImage* 182:207–218 — myelination development
- [ ] 2017 *Nature* Brief Communications rebuttals to Dong 2016 (Hughes & Hekimi, de Beer, Rozing, Lenart & Vaupel, Brown)

### §6.3. Framing changes (CONCEPT + README)
- [ ] Downgrade 5-phase Mousley 2025 from "primary grid" to "one candidate grid pending replication"; keep Frolkis-3 as equally weighted.
- [ ] Add explicit "Comparison to Real Cohorts" subsection differentiating Ontogenesis (simulator) from All of Us, UK Biobank (adult, not pediatric), Generation R, ABCD, ALSPAC, MCS.
- [ ] Remove the "UK Biobank pediatric arm" framing if it exists in any file (it has no pediatric arm).
- [ ] Soften MCOA initial-conditions claim in README from "establishes" to "is intended to establish, pending OP-02 resolution".

### §6.4. Ethics / regulatory (CONCEPT §11)
- [ ] Add IRB protocol template referencing 45 CFR 46 Subpart D categories (§46.404/405/406/407).
- [ ] Add GDPR Art. 8 (child digital consent ages 13–16 by Member State) and Art. 35 DPIA template.
- [ ] Clarify RUO vs CDS boundary re AIM/treatment_recommender integration (CONCEPT §10.3). Either remove clinical-modulation claim or commit to device-classification pathway (FDA CDS guidance 2022; EU MDR 2017/745 Class IIa).
- [ ] Licensing audit: ASEBA (CBCL), Pearson (WISC/WAIS) end-user terms documented.

### §6.5. Process / governance
- [ ] Run the project's own `docs/REFERENCE_AUDIT_Ontogenesis.md` script on every commit that touches a PMID/DOI.
- [ ] Introduce a pre-commit hook: no PMID/DOI added without PubMed/Crossref verification (see `~/.claude/memory/feedback_verify_references.md`).
- [ ] Until §6.1 through §6.4 are resolved: **project is not ready for EIC Pathfinder inclusion, grant submission, or journal article**. Internal development may continue.

---

**Auditor's bottom line:** Ontogenesis has a defensible conceptual niche (theory-driven open-source lifespan simulator with MCOA hand-off at t=25 y) and competent mathematical scaffolding (LCS formalism from Kievit 2018 is correctly applied). BUT the current state — 6/6 fabricated PMIDs in KNOWLEDGE.md, a factual error in the flagship dataset (BBC n), zero citation of Bethlehem 2022 / ABCD / DunedinPACE / PedBE, one-paper-basis for the primary periodization, and a 6-line ethics paragraph — means the project **cannot be submitted externally in its present form**. Estimated effort to reach submission-ready: 2–3 focused work-sessions on §6.1 + §6.2, 1 session on §6.3 framing, 2 sessions on §6.4 regulatory. Until then: internal research prototype only.
