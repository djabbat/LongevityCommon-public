# UPGRADE — CytogeneticTree

## v1.0 (2026-04-21, current)

Initial scaffolding:
- Umbrella CONCEPT + 10 core files
- 12 technology sub-subprojects with own 5-file cores each
- Literature landscape (KNOWLEDGE.md) pending agent completion
- Connection to Impetus LOI 2026-04-25 (Phase A MVCT)

## v1.1 (Phase 0 refinement, ~2026-05)

- [ ] Populate KNOWLEDGE.md with verified literature (all PMIDs checked via PubMed API)
- [x] Review 12 sub-subproject CONCEPTs for scientific accuracy; replace `[PMID-PENDING-VERIFY]` placeholders — 2026-04-22 (19 PMIDs verified via PubMed; unresolvable refs marked "pending dedicated verification")
- [ ] MAP.md dependency graph finalized after KNOWLEDGE.md contents known
- [ ] Integration test: connect Ilia Zheleznov HSC simulator (https://github.com/zhelilyan-create/hematopoiesis-simulator) as computational pre-validation tool
- [ ] Add explicit data-release plan (Zenodo DOI + image metadata standard)

## v2.0 (Phase 1 kickoff — contingent on Impetus Go, ~2026-06)

- [ ] RITE-Centriolin construct: synthesize via Twist Bio → clone into pLenti-Cre-ERT2 backbone → package + validate
- [ ] AutomatedMicroscopy platform assembled (per Impetus LOI budget)
- [ ] AI `/overnight` agent PROMPT.md validated in 48-h pilot (confirms ablation accuracy + decision latency)
- [ ] First BJ-hTERT lineage tracked for 2 weeks (pilot demo)
- [ ] Division-event log → DAG proof-of-concept

## v3.0 (Phase 1 complete, ~2026-12)

- [ ] 6-month continuous tracking of 6 parallel arms × 3 clonal replicates = 18 lineages
- [ ] DAG reconstruction across ~50-100 population-doublings per lineage
- [ ] polyGlu quantification at all lineage endpoints
- [ ] Statistical analysis (log-rank + MCMC) complete
- [ ] Manuscript: *Cytogenetic Tree of Fibroblast Differentiation* — submitted Nature Methods or Cell Reports Methods
- [ ] Dataset release on Zenodo (DOI)

## v4.0 (Phase 2 — mouse HSCs, ~2027-2028)

- [ ] RITE-Centriolin construct validated in mouse HSCs (LSK cells)
- [ ] Serial bone marrow transplantation with RITE-tagged HSCs
- [ ] Cross-validation between fibroblast tree (Phase 1) and HSC tree (Phase 2) — conserved topology?
- [ ] Paper: *Centriolar Lineage Tracking Across Mammalian Stem Cell Compartments*

## v5.0 (Phase 3 — vertebrate embryo, ~2028-2029)

- [ ] Zebrafish or mouse zygote microinjection
- [ ] Full embryonic imaging through gastrulation
- [ ] First complete DAG from zygote → early somatic lineages
- [ ] Landmark paper: *The Full Cytogenetic Tree of Vertebrate Development*

## v6.0 (Long-term, 2030+)

- [ ] Cross-species Cytogenetic Tree comparison (mammals + fish + amphibian)
- [ ] Platform release for other labs (Addgene plasmids, Micro-Manager plugin, DAG-reconstruction library)
- [ ] Clinical translation: human PSC-derived organoid Cytogenetic Trees for disease modeling
- [ ] Integration with MCOA full multi-counter framework for per-lineage aging prediction

---

## Known blockers / decision points

| Blocker | Decision date | If resolved | If blocked |
|---------|---------------|-------------|------------|
| Impetus funding | 2026-05-15 (decision) | Proceed Phase 1 Jun-Dec 2026 | Re-submit Hevolution/NIH R21; delay 6-12 mo |
| RITE-Centriolin construct works | Month 2 of Phase 1 | Continue to Aim A.5 | Fall back to Dendra2-Centrin photoconversion |
| 405-nm phototoxicity | Month 2 of Phase 1 | Stay with CW Cobolt | Upgrade to fs-IR (+$15k) or switch target organelle |
| Geiger lab Phase B | Month 6 | Ulm collaboration active | Passegué / Goodell fallback |
