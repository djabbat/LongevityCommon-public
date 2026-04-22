# LiveCellMicroscopy — Retrofitted Zeiss IM 35 Imaging Core

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

This subproject defines the **physical imaging platform** on which the entire Cytogenetic Tree experiment runs. A retrofitted Zeiss IM 35 inverted microscope — selected for its optical quality, modularity, and accessibility at an academic Georgian-lab budget — is equipped with a 100× / 1.4 NA oil objective, Piezo Z stage, environmental chamber, and dual-laser fluorescence path. The platform must image ≥ 72 h of BJ-hTERT-RITE lineages without photobleaching or focus drift.

## §2 Scientific basis / mechanism

High-NA oil immersion is required to resolve centriolar foci (~ 300 nm FWHM). Piezo-Z (sub-10 nm repeatability) enables thin Z-stacks without mechanical wear. Environmental chamber (37 °C, 5 % CO₂, > 95 % RH) preserves physiology over multi-day runs. Fluorescence is delivered by solid-state lasers (488 + 561 nm) through TIRF / epi selector; detection via high-QE sCMOS on two independent cameras for simultaneous red/green (split with dichroic).

## §3 Current state of the art

- Pitrone PG et al. 2013 Nat Methods — OpenSPIM open-access light-sheet microscopy platform [PMID: 23749304]
- Almada P et al. 2019 Nat Commun — automating multimodal microscopy with NanoJ-Fluidics [PMID: 30874553]
- Schott GmbH technical docs — IM 35 optical specs [DOC-PENDING]

## §4 Integration with other CytogeneticTree technologies

- **FluorescentCameras** — defines the sensors on this microscope
- **MicroscopeController** — PyMMCore-Plus drives focus, stage, lasers, cameras
- **LaserAblation_405** — shares objective and stage; adds dichroic on ablation port
- **CellPose_Segmentation** — consumes the image streams this microscope produces
- **RITE_Centriole** — biological samples imaged here
- **AICoordinator** — can trigger adaptive protocols on this platform

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Most academic scopes sold new cost > €100 k; a retrofit delivers equivalent live-cell performance at ≤ €25 k
2. Long-term (> 48 h) imaging stability is non-trivial — requires perfect focus system or PID Z-tracking
3. Two-camera simultaneous acquisition requires careful alignment and triggering

**Deliverables (Phase A):**
- Operational retrofitted Zeiss IM 35 with spec-compliant 100× path
- 72 h demonstration run (BJ-hTERT, RITE, dual-channel)
- Z-drift ≤ 100 nm over 24 h (demonstrated)
- Laser stability ≤ 1 % CV over 24 h
- Open-hardware BOM + alignment protocol published on Zenodo
