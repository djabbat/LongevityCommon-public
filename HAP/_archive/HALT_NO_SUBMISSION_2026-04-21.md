# ⛔ HAP — SUBMISSION HALT (2026-04-21)

**Status:** 🔴 DO NOT SUBMIT to any journal, grant, or external collaborator until EVIDENCE.md is rebuilt from verified PubMed records.

## Why

Deep audit 2026-04-21 found:

1. **10/10 PMIDs in EVIDENCE.md §1.1–1.2 are fabricated** — every cited PMID resolves to a completely unrelated paper (trypanosome mitochondria, lung PET/CT, hematology case reports, etc.). Combined with 12 prior mismatches in `docs/TRUE_MISMATCHES_HAP.md`, the entire citation layer is untrusted.
2. **56-taxon meta-analysis is self-referential** — `internal_data/hap_taxa_analysis_2026-04-15.csv` and `audit/2026-04-20_hap_taxa.pdf` **do not exist in the repo**. The 100% correlation and p<0.0001 figure cannot be independently reconstructed.
3. **"Accepted to *Biological Reviews*" is unsupported** — no acceptance letter, no submission ID, no correspondence file.
4. **BHCA recalc: 9.5/27 (Class III), not the claimed 20/27.** Below the 18/27 threshold required for Class II evidence.
5. **Format mismatch with *Biological Reviews*.** HAP is a ~4k-word hypothesis paper; Biol Rev expects 15-25k-word systematic reviews with 200+ refs and PGLS. Correct initial venue when rebuilt: *BioEssays* or *Medical Hypotheses*.

## Concrete counterexamples already on the board (not addressed in current CONCEPT)

- Decapod crustacean nociception (F_s+F_b organ absent)
- *C. elegans* state-dependent behavior (Cermak et al. 2020 *eLife*)
- Planarian aversive learning
- Earthworm chloragogen detoxification function
Each passes ≥3 Def.1 criteria without HAP's central organ.

## Rebuild checklist (no deadline — multi-week)

1. [ ] Rebuild `EVIDENCE.md` from verified PubMed records; every PMID must be cross-checked via esummary API within 24h of write
2. [ ] Commit `internal_data/hap_taxa_analysis_*.csv` OR delete all taxa-count / correlation / p-value claims from CONCEPT.md
3. [ ] Add PGLS (phylogenetically independent contrasts) analysis to substantiate taxa-level claims
4. [ ] Address competing axes (gut-brain, HPA, inflammation, interoception) — currently ignored
5. [ ] Add cite-by-context: PMID 38518778 (Li 2024 TGR5 hypothalamic GABA→depression); PMID 36634820 (Ntona 2023 NAFLD-bile-acids-depression); PMID 28554773 (DopEcR Drosophila); PMID 24679535 (Anderson & Adolphs 2014); PMID 21636277 (Bateson 2011 bees). These are genuine supporting papers.
6. [ ] Delete sentence "Принята к публикации (Biological Reviews)" from all files
7. [ ] Recompute BHCA using verified evidence — target ≥18/27 Class II before any submission
8. [ ] Re-target initial venue: *BioEssays* or *Medical Hypotheses* (not Biol Rev)

## Rationale for halt

**Given shared ecosystem infrastructure with CDATA / MCOA / EIC / PhD submissions, a research-integrity flag triggered by a fabricated-PMID submission would be disproportionately damaging** across all those channels. One fabrication caught by a reviewer destroys credibility for the PI across all parallel submissions.

## Governance

- **Audit file:** `~/Desktop/CommonHealth/HAP/DEEP_AUDIT_2026-04-21.md`
- **Pattern source:** `feedback_deepseek_no_citations` (DeepSeek hallucinated citations; never use DeepSeek for literature search — validated across HAP, Ontogenesis, MCOA, CDATA, Ze)
- **Halt effective:** 2026-04-21 06:45 Tbilisi
- **Halt lifted when:** EVIDENCE.md rebuild + taxon CSV committed + BHCA recomputed ≥18/27 + one external peer review independently confirms ≥2.5/5 on Biol Rev rubric
