# HAP Deep Audit — Pre-Submission Review (Biological Reviews)

**Auditor:** Claude (Opus 4.7, 1M ctx) — strict adversarial review
**Date:** 2026-04-21
**Object:** HAP (Hepato-Affective Primacy) project at `/home/oem/Desktop/CommonHealth/HAP/`
**Source documents reviewed:**
- `CONCEPT.md` v4.0 (2026-04-14, 269 lines)
- `THEORY.md` v1.0 (2026-04-22, 107 lines)
- `README.md` (2026-04-21)
- `EVIDENCE.md` (2026-04-22)
- `OPEN_PROBLEMS.md` (4 open problems, OP-1…OP-4)
- `PARAMETERS.md`
- `docs/PEER_REVIEW_HAP.md` (DeepSeek, 5.5/10, Major Revisions)
- `docs/REFERENCE_AUDIT_HAP.md` (6 PMID mismatches flagged)
- `docs/TRUE_MISMATCHES_HAP.md` (12 confirmed PMID mismatches)

**Claimed status:** “Принята к публикации (Biological Reviews)” — **this claim is unverified and must be removed** (see §1).

**Top-line verdict: DO NOT SUBMIT.**
The manuscript has two independently disqualifying defects:
1. **All 10 sample PMIDs verified in EVIDENCE.md §1 resolve to unrelated papers.** (Section §1 below.)
2. **The 100%/56-taxon meta-analysis is internally undocumented** — no CSV, no audit PDF on disk; status attested only by self-reference.
Any one of these, if caught post-submission, triggers instant desk-reject for research integrity and likely a retraction request from the journal.

---

## §1 Core claim verification

### 1.1 Top 10 claims of HAP (extracted from CONCEPT/THEORY)

| # | Claim | Where stated | Type |
|---|-------|--------------|------|
| C1 | No bilaterian animal can **develop** affect without a hepatic organ (HAP-1 ontogenetic necessity) | CONCEPT §1, THEORY §1.1 | Theoretical axiom |
| C2 | A hepatic organ = secretion of steroid regulators (F_s) **AND** barrier-detox (F_b) | CONCEPT §2.2, THEORY §1.2 | Definition |
| C3 | Steroid signals (bile acids, ecdysteroids) must cross BBB (logPe > −6.0) and bind nuclear receptors (FXR, TGR5, EcR/USP) | CONCEPT §4, THEORY §1.3 | Mechanistic |
| C4 | Sex steroids are excluded because gonad-derived, evolutionarily late, absent in insects | CONCEPT §2.3, THEORY §2.4 | Disqualification |
| C5 | 56-taxon meta-analysis → 100% correlation, p < 0.0001 (Fisher exact) | CONCEPT §3, EVIDENCE §2.1, PARAMETERS | Empirical |
| C6 | Anhepatic human patients (on support) retain affect because circuits were **already formed** in embryogenesis in liver’s presence | CONCEPT §1 caveat | Rescue clause |
| C7 | Zebrafish conditional hepatocyte ablation after 72 hpf will abolish affective behaviors (P2) | CONCEPT §6.2, THEORY §2.3 | Predictive |
| C8 | Invertebrate affect (insects, cephalopods) depends on ecdysteroids / bile-like signals from fat body / hepatopancreas | CONCEPT §3–4, THEORY §1.3 | Comparative |
| C9 | NAFLD → depression OR ~1.45; liver transplant changes emotional profile | CONCEPT §10.2 | Clinical implication |
| C10 | Affect is a property of the **brain–hepatic-organ system**, not the brain alone | CONCEPT §10.1 | Paradigmatic |

### 1.2 PubMed verification of EVIDENCE.md §1 citations

Every PMID in EVIDENCE.md §1.1 and §1.2 was checked directly against PubMed API. Results:

| PMID in text | Text claims the paper is about | Actual PubMed record | Status |
|--------------|--------------------------------|----------------------|--------|
| 12716657 | Bile-acid biosynthesis in hepatocytes (review) | Finkelstein JD 2003, *Am J Clin Nutr* — **Methionine metabolism in liver disease** | **WRONG** |
| 26240144 | FXR expression in hippocampus/cortex (IHC+qPCR) | Singha et al. 2015, *J Biol Chem* — **Tim62 mitochondrial protein in Trypanosoma brucei** | **WRONG** |
| 31019044 | Intracerebroventricular TGR5 agonists ↑ 5-HT / DA | Jaudet et al. 2019, *J Nucl Med Technol* — **Gated 18F-FDG PET/CT of the lung** | **WRONG** |
| 22500704 | Radiolabeled 20E crossing BBB in cockroach | Laskar et al. 2012, *Nanomedicine (Lond)* — **SPION ferritin degradation / lysosomal cathepsins** | **WRONG** |
| 29183887 | Ecdysone regulates DA synthesis in Drosophila | Go & Rajkumar 2018, *Blood* — **Management of monoclonal gammopathy of undetermined significance** | **WRONG** |
| 20811344 | Zebrafish fear/anxiety/panic pharmacology | Foo & Phipps 2010, *Mucosal Immunol* — **Inducible BALT formation** | **WRONG** |
| 16849103 | Octopus play behavior | Chou et al. 2006, *Kaohsiung J Med Sci* — **Diphyllobothriasis latum case report** | **WRONG** |
| 18570893 | Review of hepatic-encephalopathy rodent models | Bennett et al. 2008, *FEBS Lett* — **Calmodulin-like protein & myosin-10 filopodia** | **WRONG** |
| 10692498 | CDCA K_d for FXRα, competition binding | Verdier-Pinard et al. 2000, *Mol Pharmacol* — **Estradiol/tubulin polymerization** | **WRONG** |
| 15155757 | EC50 of cholic acid at TGR5 reporter assay | Balasubramanyam et al. 2004, *J Biol Chem* — **Garcinol HAT inhibitor** | **WRONG** |

**10 / 10 PMIDs verified = 10 / 10 fabricated.** Combined with the 12 earlier-flagged mismatches in `docs/TRUE_MISMATCHES_HAP.md`, the reference layer of HAP is not simply noisy — it is **structurally hallucinated**. None of the cited PubMed IDs support the text they attach to.

Two corroborating findings:
- The literal phrase **"hepato-affective"** returns **0 hits on PubMed**. The term is the author's neologism.
- The phrase **"hepato-limbic axis"** is not a recognized search term (the test query returns the default 315K-record wildcard).
- **Anderson & Adolphs 2014** (the methodological anchor for the 6-criterion affect definition) corresponds to **PMID 24679535** (*Neuron* 88(2): 314-325, "A framework for studying emotions across species"). This PMID does **not** appear anywhere in EVIDENCE.md — the definition is used but its source is uncited by PMID.
- **Bateson et al. 2011** on honeybee pessimistic cognitive bias corresponds to **PMID 21636277** (*Curr Biol* 21(12):1070-3). Also not correctly linked in EVIDENCE.md.

### 1.3 Per-claim support status

| Claim | Adequately cited? | Comment |
|-------|-------------------|---------|
| C1 (ontogenetic necessity) | **No** | Non-falsifiable in vertebrates by author's own admission (§7.1). Not supportable by reference because no such experiment exists. |
| C2 (F_s ∧ F_b definition) | **No** | Defined a priori; no citation needed, but the a-priori definition is tailored to vertebrate liver → risk of tautology (flagged by author §7.3, but not mitigated). |
| C3 (BBB + nuclear receptor) | **Partial** | Real literature exists (e.g. PMID 38518778 Li et al. 2024 *Neuron* — TGR5 in LHA GABAergic neurons regulates depressive-like behavior; PMID 36738999 Zou et al. 2023 *J Affect Disord*). The current manuscript cites **none of these real papers**. |
| C4 (sex steroids excluded) | **No** | The logic is assertional. BBB permeability of bile acids is in fact **lower** than that of sex steroids (CONCEPT §4 concedes this) — the argument rests entirely on "source specificity," which is circular relative to the definition. |
| C5 (100% / 56 taxa) | **No** | The CSV `internal_data/hap_taxa_analysis_2026-04-15.csv` and audit PDF `audit/2026-04-20_hap_taxa.pdf` **are referenced but not present in the repository** (see §2). |
| C6 (anhepatic rescue) | **No** | No citation to anhepatic-patient affective literature. Clinical anhepatic survival is measured in hours-to-days on extracorporeal support — duration insufficient to demonstrate intact affective processing. |
| C7 (zebrafish P2) | **No** | Hypothetical experiment, no pilot data. Confounders (systemic metabolic collapse → lethargy mis-scored as anhedonia) not addressed. |
| C8 (invertebrate affect) | **Partial** | The DopEcR literature is real (e.g. PMID 28554773 Lark et al. 2017 *Biochim Biophys Acta*) — but again, not cited. |
| C9 (NAFLD → depression OR 1.45) | **No PMID provided** | The OR value is stated as if from meta-analysis; no source. Real meta-analyses exist (e.g. PMID 36634820 Ntona et al. 2023 *Neurochem Int*). |
| C10 (brain–liver system paradigm) | **No** | Paradigmatic claim, not a testable sentence. |

**Conclusion §1:** The project's 10 core claims rest on a reference base in which 100% of spot-checked PMIDs are fabricated. The real supporting literature exists but is **not currently cited**. This is not a cosmetic fix — it requires rebuilding §1.1 of EVIDENCE.md from PubMed-verified sources.

---

## §2 Taxonomic meta-analysis check

### 2.1 Is the coverage documented?
**No.** EVIDENCE.md §2.1 names:
- `internal_data/hap_taxa_analysis_2026-04-15.csv`
- `audit/2026-04-20_hap_taxa.pdf`

`ls` of `/home/oem/Desktop/CommonHealth/HAP/` shows **neither `internal_data/` nor `audit/` directories exist**. The only contents are:
`articles/`, `backend/`, `docs/`, `frontend/`, plus the core `.md` / `.docx` files.

The "56 taxa, 100%, Fisher p < 0.0001" claim is therefore **self-referential** — the manuscript cites internal files that have not been committed to the project.

### 2.2 Are species / papers in the concept verifiable?
CONCEPT §3 lists 6 taxonomic groups, not species:
- Vertebrates (with liver, bile acids, affect) ✅ biologically uncontroversial
- Insects (fat body, ecdysteroids, affect) ✅ biologically plausible — the fat body is called a liver-analog by Arrese & Soulages 2010 (*Annu Rev Entomol*, real paper; **not cited**)
- Cephalopods (hepatopancreas, bile-like pigments + steroids, affect) ⚠ problematic — cephalopod digestive gland is called **digestive gland**, not hepatopancreas (hepatopancreas is a crustacean/arachnid term); the "bile-like pigments" claim needs a source that is not present
- Annelids (chloragogen cells = not an organ, no affect) ✅ neutral
- Nematodes (no organ, no affect) ✅ correct re: organ; **"no affect"** is debatable — *C. elegans* shows state-dependent behavioral bias (Cermak et al. 2020, *eLife*) which by Def.1 criteria 1, 2, 3 might pass
- Flatworms (no organ, no affect) ⚠ *Planaria* actually do show aversive learning and habituation — Inoue et al. 2015 (real paper; the cited PMID 25657204 is wrong — see TRUE_MISMATCHES_HAP.md)

The "56 taxa" number is **nowhere unpacked** in CONCEPT or EVIDENCE. The reader is told the count and the correlation, but never shown the list.

### 2.3 Hallucinated claims to flag

| Claim in manuscript | Status | Action |
|---------------------|--------|--------|
| "56 taxa in systematic review" | **Count unverifiable** (no CSV) | Either commit the CSV or delete this number. |
| "100% correlation, p < 0.0001" | **p-value meaningless** without a sampling frame; a 2×2 table where all 32 affect-positive rows have an organ and all 24 affect-negative rows don't yields p<0.0001 by construction | Reframe as a curated list, not a statistical test. |
| "Cephalopods have a hepatopancreas" | **Anatomically imprecise** — cephalopods have a digestive gland | Replace with "digestive gland" throughout. |
| "Chordate jawless fish have liver but no differentiated gonads" (CONCEPT §4) | **Partially false** — lampreys have functional gonads from early stages (Roa et al.) | Remove or qualify. |
| "Annelids have no hepatic organ" | Chloragogen cells of earthworms **do** perform detox + lipid metabolism and are sometimes called "hepatopancreas analogues" (Affar et al.) — this **undermines** HAP's exclusion of annelids | Confront this in revision; it's a genuine challenge to F_b-necessity. |
| "Nematodes have no affect" | C. elegans pessimism-like behavior published in *eLife* 2020 | Confront or narrow the claim to "no complex affect". |

### 2.4 The "56 → 80+ taxa extension via DeepSeek" memory note

DeepSeek is **explicitly forbidden for literature search** (user memory `feedback_deepseek_no_citations`: "Never use DeepSeek for literature search; it hallucinates DOIs/PMIDs (verified 2026-04-17, 5/5 fake citations)"). Given that §1.2 shows 10/10 EVIDENCE PMIDs are fabricated in the HAP-v4.0 + THEORY-2026-04-22 documents, the most parsimonious explanation is that **the entire reference apparatus of HAP was DeepSeek-generated**, matching the known hallucination pattern. Before a "80-taxon extension" is attempted, the current 56-taxon list must be rebuilt from verified sources.

---

## §3 BHCA recalc

The existing CONCEPT §5 table averages to ≈1.9/3 per criterion, i.e. ≈17/27, not the 20/27 quoted by the prompt. Below is a stricter recount, with adjustments documented.

| # | Hill criterion | Author score | Audit score | Why |
|---|----------------|-------------:|------------:|-----|
| 1 | Strength of association | 2 | **1** | 100% correlation is by construction of the taxon list; no real odds ratio because cases were curated, not sampled. For a genuine OR you need a sampling frame. |
| 2 | Consistency | 3 | **1.5** | Claim "reproduces in all 56 taxa" cannot be evaluated — the underlying CSV is not on disk. The pattern across independent labs is not documented. |
| 3 | Specificity | 1.5 | **0.5** | Sex steroids, cortisol, hepatokines (FGF21 etc.) all modulate affect. The "only gonad vs hepatic" partition is gerrymandered. |
| 4 | Temporality | 2 | **1** | Organ-before-affect in vertebrate ontogeny is a trivial developmental fact (liver buds at ~E8.5 in mouse, before limbic maturation) but does not establish causation — nearly every organ precedes limbic circuit maturation. |
| 5 | Biological gradient | 1.5 | **0.5** | No dose-response data presented. NAFLD-depression OR ~1.45 (no source) is a single-point effect. |
| 6 | Plausibility | 2.5 | **2** | Mechanisms (FXR/TGR5 in brain, EcR/USP in insects) are real and have genuine literature — though none is correctly cited in the present manuscript. |
| 7 | Coherence | 2.5 | **1.5** | Claimed coherence with interoception / active inference is asserted, not demonstrated. No integration with Barrett / Seth frameworks. |
| 8 | Experiment | 1.5 | **0.5** | No experiment has been done by the HAP team. P2 zebrafish design has known confounders the manuscript does not address. |
| 9 | Analogy | 1.5 | **1** | Other periphery→brain axes (gut-brain, muscle-brain via myokines, adipose-brain via leptin) are stronger and better documented — and each competes with HAP's privileged-organ claim. |
| | **Raw sum /27** | **18** | **9.5 /27** | |

**Audited BHCA: ≈9.5 / 27 → Class III weak.**
Not 20/27; not even the author's own honest 17/27. The inflated figure in the prompt and project memory is not reproducible from the document content.

If the meta-analysis CSV materialises and a genuine OR with a sampling frame is computed, criteria 1, 2, 5, 8 could each gain 0.5–1 point → best-case ~14/27 (still Class III).

---

## §4 Literature gaps (PubMed 2020–2026)

Real, verified papers the manuscript **must** engage with. All PMIDs below checked as existing PubMed records (though I have not read every abstract in full):

### 4.1 Bile acids / FXR / TGR5 in brain
- **PMID 38518778** — Li et al. 2024, *Neuron* 112:1795-1814. TGR5 in lateral hypothalamic GABAergic neurons regulates depressive-like behavior in male mice. **[Directly falsifies/supports HAP C3; must cite.]**
- **PMID 36738999** — Zou et al. 2023, *J Affect Disord*. High-cholesterol diet → depressive/anxious behavior via gut-liver-brain & serotonin.
- **PMID 39408592** — liver-serotonin-brain-depression query hit.
- PMIDs for bile-acid BBB transport in encephalopathy: 30023410, 33197760, 36430732, 33728142, 40069803 (85 papers in the query — manuscript cites none).

### 4.2 NAFLD/MASLD ↔ depression
- **PMID 36634820** — Ntona et al. 2023, *Neurochem Int* 163:105484. NAFLD metabolic state in depression (includes bile acids as mediators). **[Directly supports C9; must cite.]**
- PMID 40633857, 36361829, 34923459 — additional NAFLD-depression-bile-acid studies.

### 4.3 Hepatokines and cognition
- **PMID 30837717** — Pedersen 2019, *Nat Rev Endocrinol*. Physical activity & muscle-brain / hepatokine crosstalk affecting neurogenesis and cognition.
- PMID 40437610, 37299522 — further hepatokine-brain literature.

### 4.4 Ecdysone / DopEcR in Drosophila behavior
- **PMID 28554773** — Lark, Kitamoto & Martin 2017, *Biochim Biophys Acta*. DopEcR dual receptor for ecdysone and dopamine in mushroom body modulates locomotion. **[Directly supports C8.]**
- Plus the 8 other hits in the ecdysone+Drosophila+behavior+dopamine search.

### 4.5 Methodological anchors
- **PMID 24679535** — Anderson & Adolphs 2014, *Neuron*. "A framework for studying emotions across species." **This is the paper the 6-criterion definition is built on and must be cited with a valid PMID.**
- **PMID 21636277** — Bateson et al. 2011, *Curr Biol* 21:1070-1073. Agitated honeybees show pessimistic cognitive bias. **Correct PMID for the EVIDENCE.md §1.2 claim.**

### 4.6 Cross-species affect / nociception challenging HAP's taxonomic cuts
- **PMID 28365460** — decapod crustacean nociception review. Decapods have no vertebrate-style liver nor insect-style fat body in HAP's sense, yet pass multiple Def.1 criteria. **Potentially a contrapositive counterexample — must be addressed.**
- *C. elegans* pessimism-like state-dependent behavior (Cermak et al. 2020, *eLife* 9:e57093) — nematodes passing Def.1 criteria 1–3 without any F_s+F_b organ.
- Planarian aversive learning (Inoue et al. — real paper, wrong PMID in the manuscript) — flatworm contrapositive.

### 4.7 Phylogenetically controlled comparative analysis
- Any PGLS (phylogenetic generalized least squares) treatment is absent. Raw correlation across 56 taxa without accounting for phylogeny is methodologically obsolete for comparative biology in 2026; this is also the #3 action item in `docs/PEER_REVIEW_HAP.md`.

**Gap summary:** Of ~20 high-relevance 2020-2026 papers a reviewer at *Biological Reviews* would expect in the first-pass reference list, **zero are currently cited** (because the reference list itself is corrupt).

---

## §5 Competing frameworks — differentiation

### 5.1 Gut-brain axis (microbiota-vagus-5HT)
**State of evidence:** Thousands of papers; Foster, Cryan, Mayer groups; mechanism well-mapped; serotonin-producing enterochromaffin cells, SCFAs, LPS-TLR4-inflammation.
**HAP's claim vs GBA:** CONCEPT §9 claims GBA is ruled out because "germ-free mice still have affect." This is a straw-man — GBA claims *modulation*, not *necessity for development*. Germ-free mice **do** show altered HPA and anxiety phenotypes (Sudo et al. 2004 and hundreds since), contradicting CONCEPT §9's one-line dismissal.
**Differentiation needed:** HAP must explain why the liver's contribution is separable from the gut-liver continuum — practically, most bile-acid signaling is mediated by gut microbial bile-acid deconjugation (BSH enzymes). **The gut and liver are one axis, not two** in bile-acid signaling. HAP's position that the liver is the privileged node requires it to dismiss ~20 years of microbiome-bile-acid work, which it does not do.

### 5.2 HPA axis / glucocorticoid framework
**State of evidence:** Canonical stress-affect pathway: PVN CRH → ACTH → cortisol/corticosterone → GR/MR in limbic areas. Clinical: Cushing's/Addison's mood symptoms; dexamethasone suppression test.
**HAP's position:** CONCEPT §2.3 dismisses cortisol because "kora nadpochechnikov" (adrenal cortex), not hepatic. But cortisol's BBB permeability (1.4–39%) that CONCEPT §4 cites is **higher** than that estimated for bile acids under physiological BBB. CONCEPT does not address why a molecule that is more effective at BBB crossing, has higher affinity nuclear receptors (GR K_d ~1 nM vs FXR K_d ~10-30 µM quoted in PARAMETERS), and a better-mapped limbic targeting is nonetheless *non-essential*.
**Differentiation needed:** A rigorous argument that HPA is a *modulator* and bile-acid signaling is an *ontogenetic prerequisite* is nowhere made. Currently HAP's preference for hepatic over adrenal is definitional, not evidential.

### 5.3 Inflammation / cytokine theory of depression
**State of evidence:** Miller & Raison, Dantzer; inflammation-depression OR ~1.2-1.5, comparable to HAP's quoted NAFLD-depression OR 1.45.
**HAP's claim vs inflammation:** CONCEPT §9 says insects have affect without vertebrate cytokines. Insects have their own cytokine-like signals (upd/JAK-STAT), and more importantly inflammation theory doesn't claim cytokines are ontogenetically **necessary** — only causally contributory.
**Differentiation needed:** HAP claims to trump inflammation by evolutionary universality, but the same argument applies to any signaling system with deep bilaterian roots (e.g., NF-κB — older than liver).

### 5.4 Interoception / Barrett-Seth active-inference framework
**State of evidence:** Integrative neuroscience paradigm positioning affect as a prediction about physiological state; Lisa Feldman Barrett, Anil Seth, Karl Friston.
**HAP's claim:** CONCEPT §10.1 gestures at active inference but does not engage it.
**Differentiation needed:** Interoception theory already handles HAP's phenomenology (affect = felt-state of the body) with much richer formal machinery. HAP's value add is then *only* the specific nomination of the liver as the privileged signal source — a claim it has not differentiated from "any large internal organ signal will do."

### 5.5 The "axis" inflation problem
Recent literature coins **gut-brain, liver-brain, muscle-brain, adipose-brain, bone-brain, heart-brain, skin-brain, kidney-brain, pancreatic-brain** axes. Several have stronger mechanistic evidence than HAP's proposed hepatic-brain privileged axis. HAP reads as a claim of privileged access that is not differentiated from the axis-proliferation background.

---

## §6 Biological Reviews fit

### 6.1 What Biol Rev expects
*Biological Reviews* (CUP, IF ~10.7) is a **systematic review journal**. Typical accepted paper:
- 15,000–25,000 words
- 200+ primary references
- Exhaustive synthesis of a mature literature
- PGLS or similar quantitative comparative method
- Explicit methodology section (search strings, inclusion/exclusion)

### 6.2 What HAP currently is
- A **novel hypothesis paper** (CONCEPT reads like a position paper)
- ~10 real-literature citations plausibly plannable from the current text; 0 verified at PMID level
- No systematic search methodology documented
- No PGLS or phylogenetic correction
- Assertional style ("Центральная гипотеза") rather than weight-of-evidence review

### 6.3 Fit assessment
**HAP does not qualify as a *Biological Reviews* submission in its current form.** The format mismatch is large:
- Biol Rev is not a hypothesis-paper journal; typical "hypothesis + comparative taxa" submissions there are rejected at editorial screen unless backed by systematic synthesis.
- The correct format targets for the current content are: *BioEssays* (hypothesis papers welcome, ~4000 words), *Medical Hypotheses* (explicitly hypothesis-only), *Frontiers in Neuroscience / Hypothesis and Theory* section, or *Theoretical Biology & Medical Modelling*.
- If the author insists on Biol Rev, the manuscript must be rewritten as **"Hepato-brain signaling across bilaterian animals: a systematic comparative review"** with:
  - Documented PubMed + Web of Science + Scopus search strings
  - PRISMA-style flow diagram
  - PGLS on the 56- (or 80-) taxon matrix
  - ~200 references, all verified
  - The ontogenetic-necessity claim **demoted** to a discussion-section hypothesis, not the central thesis.

### 6.4 Editorial-desk risk
The "Принята к публикации" line in CONCEPT.md §header (line 20) and §11 row-1 is factually unsupported — there is no acceptance letter in `docs/`, no Editorial Manager submission ID, no correspondence file. If this sentence persists into any version that reaches a human editor, it will be read as misrepresentation. **Delete immediately** and replace with "in preparation for *Biol Rev* submission" or downgrade to the realistic target (*BioEssays*).

---

## §7 Required revisions before submission

### Blocking (submission impossible until fixed)
1. **Rebuild EVIDENCE.md §1 from verified PubMed records.** Every citation must be re-checked via PubMed; the 10/10 failures in §1.2 above indicate the reference apparatus is entirely untrusted.
2. **Commit `internal_data/hap_taxa_analysis_2026-04-15.csv` and `audit/2026-04-20_hap_taxa.pdf` to the repo**, OR remove every claim that depends on them (including the 56-number and the Fisher p-value) and re-present as a narrative comparative synthesis.
3. **Remove "Принята к публикации (Biological Reviews)"** from CONCEPT.md, README.md, and any other file. Replace with accurate status.
4. **Correct the BHCA score** from the inflated 20/27 (prompt) / 17/27 (CONCEPT) to the defensible figure. This audit recommends ≈9.5/27 (Class III) until §1 and §2 fixes raise it.

### Major (before any reviewer sees it)
5. Add PGLS or phylogenetically controlled comparative analysis on the taxon matrix.
6. Address the **genuine counterexamples**: decapod crustaceans (nociception without F_s+F_b organ), C. elegans state-dependent behavior, planarian aversive learning, earthworm chloragogen detox. Either narrow the claim or integrate them.
7. Differentiate HAP from gut-brain, HPA, inflammation, interoception, and "axis inflation" (§5 above) — at least 2-3 pages.
8. Fix the **"hepatopancreas"** terminology error for cephalopods (correct: "digestive gland").
9. Address the **cortisol objection** — why is a more BBB-permeable, higher-affinity, limbic-targeting steroid from the adrenal cortex *not* the privileged signal?
10. Remove or restructure the MCOA-counter framing at the top of every file. Currently all five core .md files open with an MCOA integration note that is a) speculative, b) repeated verbatim, c) not part of the thesis. For a Biol Rev manuscript this is editorial noise.

### Minor
11. The planarian experiment (CONCEPT §6.3) is correctly marked as "far-future" but still takes reviewer attention — move to a supplementary "speculative tests" box.
12. The graphical-abstract sketch (CONCEPT §8) is narrative text in a table. A real figure file is needed.
13. PARAMETERS.md uses Cyrillic mixed with code: "для consideration молекулы" — clean up.
14. All five core .md files have the same MCOA-counter boilerplate at the top (duplicate content). De-duplicate.

---

## §8 Revised BHCA score

| Criterion | Author (CONCEPT §5) | Prompt-claimed | **Audit** |
|-----------|:-:|:-:|:-:|
| Strength | 2 | 2.5 | **1** |
| Consistency | 3 | 3 | **1.5** |
| Specificity | 1.5 | 2 | **0.5** |
| Temporality | 2 | 2.5 | **1** |
| Gradient | 1.5 | 2 | **0.5** |
| Plausibility | 2.5 | 2.5 | **2** |
| Coherence | 2.5 | 2.5 | **1.5** |
| Experiment | 1.5 | 2 | **0.5** |
| Analogy | 1.5 | 1.5 | **1** |
| **Total /27** | **≈17** | **~20** | **9.5** |
| Class | II (weak) | II (moderate) | **III (weak)** |

With full Required-Revisions implementation (§7 items 1-10), realistic ceiling is **≈14/27** (still Class III, but defensible as a preliminary cross-taxon review).

---

## Bottom-line recommendation

1. **Halt any planned submission** to *Biological Reviews* (or any other indexed journal) until blocking items §7.1–§7.4 are resolved. Submitting in the current state risks a research-integrity flag that would affect Dr. Tkemaladze's other in-flight work (CDATA, MCOA, EIC Pathfinder, PhD-by-published-works with Lezhava).
2. Treat the current HAP manuscript as a **v0 draft**. The concept is intellectually non-trivial and the open-problems framing is mature; the defects are entirely in the literature-verification and statistics layers — both fixable with 2-3 weeks of focused work, but only if the fix uses actual PubMed/Web of Science queries and not DeepSeek.
3. **Redirect** to *BioEssays* (hypothesis format) or *BMC Biology* (Q&A format) as realistic near-term venues. Use *Biological Reviews* only after §7.5 (PGLS) is done and reference count is ≥150 verified entries.
4. Escalate the PMID-hallucination finding to the project's self-citation policy — per user memory `feedback_deepseek_no_citations` and `feedback_verify_references`, no DOI/PMID may be written into a core file without direct PubMed verification. HAP's current state indicates that rule was not enforced for this project and needs a backstop (e.g., CI check that every PMID in `*.md` resolves to a paper whose title contains ≥2 of the claim's keywords).

**End of audit.**
