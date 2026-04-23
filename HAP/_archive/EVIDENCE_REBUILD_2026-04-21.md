# HAP Evidence — Rebuild v1 (2026-04-21 audit)

**Status:** Phase A draft. Only verified PMIDs admitted. Quarantined file `EVIDENCE.md.QUARANTINED_2026-04-21` is NOT used as a source.
**Verification date:** 2026-04-21 (all 5 anchor PMIDs checked via NCBI E-utilities: esummary + efetch, text+JSON).
**Target venue after rebuild:** *BioEssays* (hypothesis paper, ~4k words).
**BHCA target:** ≥18/27 (Class II). Current pre-rebuild: 9.5/27 (Class III).

---

## 1. Supporting evidence (verified PMIDs only)

| # | Claim | Key PMID | Full citation (verified) | Abstract excerpt (1 sentence, verbatim) | Verification |
|---|-------|----------|--------------------------|------------------------------------------|--------------|
| 1 | A bile-acid receptor (TGR5) in specific hypothalamic GABAergic neurons causally regulates depressive-like behavior in mammals — a concrete liver-derived signal → CNS affective circuit route. | **38518778** | Li XY, Zhang SY, Hong YZ, et al. TGR5-mediated lateral hypothalamus–dCA3–dorsolateral septum circuit regulates depressive-like behavior in male mice. *Neuron* 2024;112(11):1795-1814.e10. doi:10.1016/j.neuron.2024.02.019 | "Upregulation of TGR5 or inhibition of GABAergic excitability in LHA markedly alleviated depressive-like behavior, whereas down-regulation of TGR5 or enhancement of GABAergic excitability facilitated stress-induced depressive-like behavior." | 2026-04-21 (esummary+efetch) |
| 2 | Liver pathology (NAFLD/MAFLD) is pathophysiologically linked to depression via gut-dysbiosis → bile acids / SCFA / LPS → monoaminergic and BDNF pathways — a published integrative review supporting the gut-liver-brain axis framing of HAP. | **36634820** | Ntona S, Papaefthymiou A, Kountouras J, et al. Impact of nonalcoholic fatty liver disease-related metabolic state on depression. *Neurochem Int* 2023;163:105484. doi:10.1016/j.neuint.2023.105484 | "NAFLD-related gut dysbiosis … stimulates mediators including lipopolysaccharides, short-chain fatty acids and bile acids, which play significant role in depression." | 2026-04-21 |
| 3 | Cross-species precedent for a single receptor integrating a steroid-like hormone with a classical neuromodulator to shape behaviorally relevant CNS activity (Drosophila DopEcR: ecdysone+dopamine). Supports the cross-phylum "steroid → CNS affect-adjacent circuit" motif, without requiring vertebrate bile acids. | **28554773** | Lark A, Kitamoto T, Martin JR. Modulation of neuronal activity in the Drosophila mushroom body by DopEcR, a unique dual receptor for ecdysone and dopamine. *Biochim Biophys Acta Mol Cell Res* 2017;1864(10):1578-1588. doi:10.1016/j.bbamcr.2017.05.015 | "Drosophila DopEcR is a GPCR that responds to both ecdysone (the major steroid hormone in insects) and dopamine, regulating multiple second messenger systems … DopEcR is preferentially expressed in the nervous system and involved in behavioral regulation." | 2026-04-21 |
| 4 | Conceptual framework endorsing cross-species "emotion primitives" (internal states with scalability, valence, persistence, generalization) as a legitimate scientific construct — provides the methodological backbone for HAP's operational definition of affect. | **24679535** | Anderson DJ, Adolphs R. A framework for studying emotions across species. *Cell* 2014;157(1):187-200. doi:10.1016/j.cell.2014.03.003 | "Emotion states exhibit certain general functional and adaptive properties that apply across any specific human emotions like fear or anger, as well as across phylogeny." | 2026-04-21 |
| 5 | Empirical demonstration of invertebrate emotion-like state (pessimistic cognitive bias + hemolymph monoamine shift after stressor) — supports HAP's claim that visceral/humoral → CNS affective-state modulation is conserved into insects. | **21636277** | Bateson M, Desire S, Gartside SE, Wright GA. Agitated honeybees exhibit pessimistic cognitive biases. *Curr Biol* 2011;21(12):1070-1073. doi:10.1016/j.cub.2011.05.017 | "Shaken bees also have lower levels of hemolymph dopamine, octopamine, and serotonin … the bees' response to a negatively valenced event has more in common with that of vertebrates than previously thought." | 2026-04-21 |

**All 5 verified.** No replacements required at this stage.

---

## 2. Claim-by-claim triage (CONCEPT v4.0 + THEORY v1.0)

Taxonomy: **SAVE** = supported by ≥1 verified anchor or established literature; **DOWNGRADE** = partially supported, needs weaker wording; **DISCARD** = no credible evidence now, remove from rebuild.

### 2.1 SAVE (8)
| # | Claim | Anchor(s) |
|---|-------|-----------|
| S1 | Bile acid → TGR5 → CNS → depressive-like behavior route exists in mammals. | 38518778 |
| S2 | NAFLD/MAFLD liver pathology is comorbid with depression via gut-liver-brain axis. | 36634820 |
| S3 | Non-mammalian bilaterians (insects, bees) exhibit emotion-like internal states measurable by cognitive/behavioral assays. | 21636277, 24679535 |
| S4 | A unified cross-species "emotion primitives" framework is a legitimate research program. | 24679535 |
| S5 | Steroid-family hormones (ecdysone in insects) modulate CNS circuits relevant to behavior via receptors co-responsive with classical neuromodulators. | 28554773 |
| S6 | Hemolymph monoamine (DA/5-HT/OA) shifts accompany emotion-like states in invertebrates. | 21636277 |
| S7 | Hepatic organ in vertebrates secretes bile acids that can act as signaling molecules beyond digestion. | 38518778 + 36634820 |
| S8 | Operational definition of affect via Anderson–Adolphs criteria (persistence, generalization, valence asymmetry, etc.) is appropriate for HAP. | 24679535 |

### 2.2 DOWNGRADE (7)
| # | Original strong claim | Weaker rewording |
|---|-----------------------|------------------|
| D1 | "Affect cannot develop in any bilaterian lacking a hepatic organ" (ontogenetic necessity, 100% taxonomic correlation) | "Current evidence is consistent with hepatic/hepatic-analog organs being strong facilitators — possibly necessary — of full affective-state repertoires; a hard biconditional requires taxonomic sampling beyond present data." |
| D2 | "56 taxa, 100% correlation (p<0.0001), no counterexamples" | **Table must be rebuilt from sourced taxon reports; current numbers have no verified references.** Rewrite as "a survey of reviewed taxa (TBD n, cited individually) finds enrichment of affect-like indicators in species with hepatic-analog organs." |
| D3 | "Insects' fat body + pericardial cells = hepatic organ equivalent at full functional parity" | "Insect fat body + pericardial cells share selected hepatic functions (steroid synthesis, xenobiotic metabolism, phagocytosis) and provide the nearest functional analog; full functional equivalence remains an open empirical question." |
| D4 | "Bile acids cross BBB with moderate-high permeability" | "Select bile acids can access CNS regions under physiological and especially pathological (inflamed BBB) conditions; the effective in-vivo concentration range in humans remains debated." Needs fresh PubMed sweep in Phase B. |
| D5 | "Liver transplantation changes recipient's emotional profile" | REMOVE from Evidence (no anchor yet) until a verified PubMed source is added. |
| D6 | "NAFLD → depression OR ~1.45" | Remove the specific effect size pending verified meta-analysis retrieval (Phase B task). Keep qualitative comorbidity claim (S2). |
| D7 | MCOA integration claim `D_HAP(n,t) = D_0 + α·(n/n*) + β·(t/τ) + γ·I(other)` as formal framework | Keep as *proposed* formalism flagged "candidate counter pending empirical parameter estimation"; do not state as established. |

### 2.3 DISCARD (10)
| # | Claim | Reason |
|---|-------|--------|
| X1 | "Accepted to Biological Reviews (IF ~10)" (CONCEPT v4.0 header line) | Factually false; paper not submitted. Remove. |
| X2 | "Peer-reviewed by R1–R8" framing | Internal reviews, not journal peer review; remove from public-facing wording. |
| X3 | "Cephalopod hepatopancreas secretes bile pigments + steroid hormones → affect" | No verified PMID supporting a hepatopancreas-affect causal pathway. |
| X4 | "Annelids/nematodes/planaria: no affect" (as settled fact) | Direct counterexamples exist in the literature (see §3). Cannot be stated as established. |
| X5 | Specific numerical BHCA subscores (2, 3, 1.5, ...) totaling ≈1.9 | Hand-assigned without audit; recompute in Phase B against verified PMIDs only. |
| X6 | "logPe > -6.0 or extraction > 1%" as a universal HAP threshold for steroid BBB permeability | Arbitrary threshold with no source; drop or justify in Phase B. |
| X7 | "Embryo without liver dies before behavior forms" used to explain non-falsifiability in vertebrates | Logical framing OK, but presented as experimental fact — rewrite as methodological limitation, not empirical claim. |
| X8 | "Sterile mice have normal affect, therefore microbiota not necessary" (alternative-hypothesis rebuttal) | Needs sourcing; germ-free mouse behavioral literature is nuanced and partly contradicts. Remove in current form. |
| X9 | "Prefrontal cortex lesions are a consequence of liver-steatosis-associated hyperinflammation" as a HAP-specific mechanism | Ntona 2023 presents this as one hypothesis among several; HAP cannot use it as settled mechanism. |
| X10 | "Chloragogen cells in annelids are not an organ" as a reason to exclude annelids | Functional-organ status is a gradable matter; HAP cannot dismiss by definition. |

**Triage counts:** SAVE = 8, DOWNGRADE = 7, DISCARD = 10. Total claims audited = 25.

---

## 3. Counterexamples to address (must be handled in Phase C manuscript)

These are inherited from the prior DEEP_AUDIT and remain active liabilities:

1. **Decapod crustacean nociception / emotion-like responses** — multiple published studies argue for pain-like states in crabs/lobsters; crustacean hepatopancreas is a digestive gland, not a bile-acid secreting liver-analog in the HAP sense. HAP must either expand F_s definition or acknowledge partial counterexample.
2. **C. elegans state-dependent behavior** (Cermak et al., 2020, *eLife*, PMID verification required in Phase B): arousal/satiety-dependent behavior without any hepatic organ — direct challenge to HAP biconditional.
3. **Planarian aversive learning** (multiple reports): associative avoidance in flatworms lacking any hepatic analog.
4. **Earthworm chloragogen cells and nociceptive behavior** — possession of chloragogen tissue *plus* nociceptive responses muddies the "no liver → no affect" line.

**Decision rule for rebuild:** if any one of these is reconfirmed with verified PMIDs in Phase B, the HAP biconditional is falsified as stated and must be restructured as a graded/quantitative hypothesis (DOWNGRADE path for D1).

---

## 4. Open questions — prioritized PubMed sweep for Phase B

Ordered by criticality to BHCA ≥18 target:

1. **Quantitative bile-acid BBB permeability in vivo** (human + rodent). Needed to support D4.
2. **C. elegans arousal/emotion-like states** — verify Cermak 2020 eLife PMID; read abstract; decide if counterexample survives scrutiny.
3. **Decapod crustacean nociception / welfare literature** (Elwood and others); verify at least 3 PMIDs.
4. **Liver transplantation and affective-profile change** — is there *any* verified peer-reviewed human study? If no, permanently retire D5.
5. **Germ-free mouse behavioral phenotypes** — depression/anxiety literature, for correct treatment of "microbiota alternative hypothesis."
6. **FXR/TGR5 CNS expression atlases** — verified anatomy refs to replace fabricated anatomical density claims.
7. **Ecdysone / DopEcR and insect affect-like behavior beyond the single 2017 paper** — broaden evidence for S5.
8. **NAFLD ↔ depression meta-analyses** — retrieve a *verified* OR/RR number to reinstate D6 if desired.
9. **Insect fat body as hepatic analog** — review papers on functional equivalence for D3.
10. **Cephalopod hepatopancreas neuroendocrine output** — decide if X3 can be rescued into DOWNGRADE.

Phase B protocol: every retrieved PMID passes (a) esummary resolves, (b) title/author/year match, (c) abstract genuinely supports claim, (d) entry recorded here with verification date. **No DeepSeek, no unverified DOI**, per `feedback_deepseek_no_citations` and `feedback_verify_references`.

---

## 5. Audit trail

| Field | Value |
|-------|-------|
| Timestamp | 2026-04-21 |
| Auditor | Claude (Phase A, REBUILD_PLAN_2026-04-21 §4) |
| PMIDs verified (E-utilities esummary + efetch) | 5 / 5 (38518778, 36634820, 28554773, 24679535, 21636277) |
| PMIDs failing verification | 0 |
| PMIDs flagged for replacement | 0 |
| Claims triaged | 25 (SAVE 8 / DOWNGRADE 7 / DISCARD 10) |
| Source files audited | `CONCEPT.md` (v4.0), `THEORY.md` (v1.0) |
| Not consulted (per rule) | `EVIDENCE.md.QUARANTINED_2026-04-21` (do not restore) |
| Next phase | Phase B — BHCA recomputation against verified evidence only; §4 PubMed sweep |
| Gate for Phase B exit | ≥80% of SAVE/DOWNGRADE claims have ≥2 verified PMIDs each |
