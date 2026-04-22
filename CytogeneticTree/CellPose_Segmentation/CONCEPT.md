# CellPose_Segmentation — AI Live-Cell Segmentation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

To reconstruct a cytogenetic tree we must **track every cell and every centriole across thousands of frames and multiple divisions without losing identity**. Manual segmentation is intractable at this scale. CellPose provides a pre-trained, generalist AI segmentation model that operates directly on live-cell fluorescence / brightfield images, producing instance masks per cell and per sub-cellular foci (centrioles). This subproject adapts and fine-tunes CellPose for the RITE-centriole imaging stream that drives the parent project.

## §2 Scientific basis / mechanism

CellPose (Stringer et al. 2021 Nat Methods) uses a U-Net with a topographical flow representation: each pixel is assigned a vector pointing toward the center of its cell, and mask instances are recovered by following these flows to their sinks. The network is trained on a large generalist corpus including phase-contrast, fluorescence, and brightfield data. CellPose 2.0 adds user-in-the-loop fine-tuning; CellPose 3.0 adds image restoration. For centriole foci (1–2 pixel punctae), we pair CellPose cell-level masks with a diffraction-limited spot detector (e.g., Trackpy, spotiflow) gated by the cell mask.

## §3 Current state of the art

- Stringer C et al. 2021 Nat Methods — CellPose original [PMID: 33318659]
- Pachitariu M, Stringer C 2022 Nat Methods — CellPose 2.0 human-in-the-loop [PMID: 36344832]
- Dohmen E et al. 2024 — spotiflow sub-pixel spot localization (bioRxiv, not yet in PubMed)

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** + **FluorescentCameras** — produce the raw image streams
- **RITE_Centriole** — provides the red/green centriole channels to segment
- **MicroscopeController** — can trigger on-the-fly segmentation for adaptive acquisition
- **AICoordinator** — interprets segmentation results to decide laser ablation targets
- **ImageAnalysis** — downstream quantification uses the masks produced here
- **GenealogyReconstruction** — tracks link masks across frames into lineages

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Default CellPose models trained on fixed images; live-cell mitotic shapes are under-represented
2. Centriolar foci (< 500 nm) below standard CellPose training resolution
3. Two-channel (red/green) centriole tracking across mitosis is a specialized task

**Deliverables (Phase A):**
- Fine-tuned CellPose model for BJ-hTERT dividing cells
- Integrated pipeline: CellPose cell masks + spotiflow centriole detection within masks
- Benchmark: ≥ 95 % cell-level F1, ≥ 90 % centriole F1 vs hand-annotated ground truth
- Open dataset + trained weights on Zenodo
