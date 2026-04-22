# RITE_Centriole — Recombination-Induced Tag Exchange for Centriole Age Tracking

**Parent project:** [CytogeneticTree](../CONCEPT.md) (CommonHealth umbrella)

## §1 Purpose

RITE_Centriole provides the **molecular clock** that makes the Cytogenetic Tree observable. By genetically encoding an inducible fluorescent tag swap (mCherry → GFP) on a centriolar scaffold protein (e.g., Centrin-1, SAS-6, or CEP152), we create a system in which every centriole synthesized *before* a tamoxifen pulse remains red, and every centriole synthesized *after* the pulse becomes green. This permits direct optical read-out of centriole age across multiple cell divisions — the foundational measurement required to reconstruct the lineage tree from zygote to terminally-differentiated cell.

## §2 Scientific basis / mechanism

RITE (Verzijlbergen et al. 2010 [PMID: 20018668]) uses Cre/loxP-mediated excision to irreversibly swap the C-terminal tag of a target protein. A Cre-ER^T2 driver activates only when 4-hydroxytamoxifen (4-OHT) is added, giving temporal control within minutes. Applied to centriolar proteins, the system exploits the semi-conservative mode of centriole duplication: mother centrioles retain their original (pre-pulse, red) coat, while procentrioles assembled de novo after the pulse incorporate newly-translated (green) protein. Because centriolar proteins such as Centrin are extremely stable (turnover > 24 h), the tag-swap cleanly distinguishes "old" from "new" centrioles over multiple divisions.

## §3 Current state of the art

- Verzijlbergen KF et al. 2010 — original RITE method, yeast histones [PMID: 20018668]
- Yamashita YM et al. 2007 Science — asymmetric centrosome inheritance in *Drosophila* male germline stem cells [PMID: 17255513]
- Nigg EA, Holland AJ 2018 Nat Rev Mol Cell Biol — centrosome biogenesis review [PMID: 29363672]

No published study has yet combined RITE with centriolar scaffolds in mammalian cells for lineage tracing. This gap is the core opportunity.

## §4 Integration with other CytogeneticTree technologies

- **LentiviralTools** — delivers the RITE cassette into BJ-hTERT and other cell lines
- **LiveCellMicroscopy** + **FluorescentCameras** — image two-channel (red/green) centriole signals over days
- **CellPose_Segmentation** — segments cells and centriole foci across frames
- **LaserAblation_405** — selectively kills daughter lineages to simplify tree reconstruction
- **ImageAnalysis** — quantifies red:green ratio per centriole per frame
- **GenealogyReconstruction** — consumes per-division red/green inheritance calls to assemble the tree
- **StatisticalAnalysis** — MCMC inference of centriole-age inheritance bias

## §5 Known gaps + what this subproject builds

**Gaps in literature:**
1. No validated RITE construct for mammalian centrioles exists
2. Centriolar protein choice (Centrin-1 vs SAS-6 vs CEP152) unresolved for optimal signal/noise
3. Tamoxifen pulse duration vs Cre efficiency not mapped for this application
4. Tag maturation times (mCherry ~40 min; GFP ~30 min) must be factored into timing calibration

**Deliverables (Phase A):**
- Design 3 candidate RITE-centriole plasmids (Centrin-1, SAS-6, CEP152)
- Validate tag-swap kinetics in HEK293T after 4-OHT
- Establish single-cell clonal BJ-hTERT-RITE lines
- Publish construct maps + validation data on Addgene + Zenodo

Budget line-items, pulse protocols, and imaging parameters are in PARAMETERS.md.
