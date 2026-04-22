# LaserAblation_405 — Targeted Single-Cell Microablation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

Reconstructing the cytogenetic lineage tree quickly becomes combinatorially explosive: after 8 divisions, a single zygote-equivalent produces 256 cells, most of which drift out of the imaging field, overlap, or die. **Laser ablation** gives us the power to **selectively kill uninformative daughter lineages** and keep the field of view tractable — a "pruning shear" for the experimental tree. This subproject builds a 405 nm / fs-IR ablation capability integrated with the live-cell microscope and AI coordinator.

## §2 Scientific basis / mechanism

- **405 nm CW / pulsed diode** — kills nuclei within seconds by photolesioning DNA and generating ROS; low cost, high reliability, safe collateral.
- **fs-IR (800 nm, < 200 fs)** — non-linear multiphoton ablation with sub-micron precision; targets individual organelles (e.g., a selected centriole) without damaging neighbors.

Both are focused through the 100× / 1.4 NA oil objective (shared with imaging). Steering is galvo-based (< 1 ms per target); software gates ablation power to ROI masks coming from `CellPose_Segmentation` and decisions from `AICoordinator`.

## §3 Current state of the art

- Khodjakov A, Rieder CL 2001 J Cell Biol — centrosome requirement for fidelity of cytokinesis (laser microsurgery) [PMID: 11285289]
- Colombelli J et al. 2005 Traffic — pulsed UV laser nanosurgery for cytoskeletal dynamics in live cells [PMID: 16262721]
- Liang X et al. 2020 — automated laser ablation pipelines (reference pending dedicated PubMed verification)

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** — shares objective and stage with imaging path
- **MicroscopeController** — PyMMCore-Plus routes ablation commands
- **CellPose_Segmentation** — provides per-cell ROI for targeting
- **AICoordinator** — decides WHICH daughter to ablate based on lineage policy
- **RITE_Centriole** — in rare cases, target an individual centriole (fs-IR) to test causality of age asymmetry
- **GenealogyReconstruction** — ablations are logged as "experimental pruning" events in the tree

## §5 Known gaps + what this subproject builds

**Gaps:**
1. No turnkey ablation module priced for a retrofit academic scope
2. Automated target-selection pipelines rare in live-lineage contexts
3. Calibration of power/duration for non-damaging "mark" vs lethal "cut" is protocol-specific

**Deliverables (Phase A):**
- Integrate 405 nm diode (≥ 100 mW) via galvo path into Zeiss IM 35 retrofit
- Characterize ablation dose–response on BJ-hTERT (survival curve)
- Automated "ablate-by-mask" API callable from Python
- Proof-of-concept: ablate one daughter per division over 5 generations → simplified tree
