# TODO — CytogeneticTree

## Phase 0 (2026-04 scaffolding)

- [x] Generate umbrella CONCEPT.md + core files (this set) — 2026-04-21
- [x] Generate 12 technology subproject scaffoldings — 2026-04-21 (parallel agent)
- [ ] Populate KNOWLEDGE.md with verified literature (parallel agent in progress)
- [ ] MAP.md dependency graph across subprojects
- [ ] LINKS.md with Addgene plasmids, GitHub repos, Micro-Manager documentation
- [ ] Review all auto-generated subfolder CONCEPTs for scientific accuracy
- [x] Replace any `[PMID-PENDING-VERIFY]` placeholders with real verified PMIDs (feedback_verify_references rule) — 2026-04-22

## Phase 1 — Minimum Viable Cytogenetic Tree (MVCT; contingent on Impetus Go)

- [ ] De-novo RITE-Centriolin construct cloning (Twist Bio → Addgene submission → validation in BJ-hTERT). Blocker: ~6-8 weeks cloning + clonal selection timeline.
- [ ] AutomatedMicroscopy platform build (Phase A Impetus budget)
- [ ] AI `/overnight` agent orchestration (PROMPT.md validated in 48h pilot before full run)
- [ ] 6-month parallel tracking of BJ-hTERT lineages (all arms per Impetus design)
- [ ] Division-event log → DAG reconstruction (GenealogyReconstruction subproject)
- [ ] Tree annotation with polyGlu signal, Ninein co-stain, ARL13B ciliation

## Phase 2 — Mouse HSC Tree (conditional on Phase 1 success)

- [ ] Collaboration with Geiger lab (Ulm) / alternative (Passegué / Goodell)
- [ ] Serial bone marrow transplantation with RITE-Centriolin HSCs
- [ ] Competitive CD45.1/CD45.2 congenic tracking

## Phase 3 — Vertebrate Embryo (long-term, 2028-29)

- [ ] Zebrafish or mouse zygote microinjection with RITE construct
- [ ] Full embryonic imaging through blastocyst / gastrulation
- [ ] Full DAG reconstruction from zygote

## Publication plan

- [ ] Methodology paper (Nat Methods / Cell Reports Methods) — Phase 1 first
- [ ] Scientific paper (Nature / Cell / Nature Aging) — Phase 1 validation of CDATA prediction
- [ ] Data release (Zenodo with DOI) — concurrent with manuscript

## Risks / Open Questions

- [ ] RITE-Centriolin construct validation uncertain (never published for centrioles) — fallback: Dendra2-Centrin photoconversion
- [ ] 405 nm laser phototoxicity to sister centrioles may confound arm 2 (centriole-independent control)
- [ ] AI segmentation edge cases (mixed-centriole daughters, focus drift)
- [ ] Computational cost of long-running DAG on 200+ PD of data
