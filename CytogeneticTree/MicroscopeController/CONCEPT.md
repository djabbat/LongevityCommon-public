# MicroscopeController — Micro-Manager 2.0 + PyMMCore-Plus Automation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

A multi-day, adaptive lineage experiment cannot be driven by a human at the eyepiece. This subproject builds the **software brain** that synchronizes stage, Z-Piezo, lasers, shutters, two cameras, environmental chamber, and laser-ablation galvo under one coherent Python API. It is the glue that turns independent hardware into a programmable experiment, and it is the layer through which the AICoordinator issues real-time instructions.

## §2 Scientific basis / mechanism

Micro-Manager 2.0 is the de-facto open-source microscope control standard, with device drivers for ~ 200 vendors. PyMMCore-Plus wraps the C++ MMCore in modern Python (async, type hints) and integrates with napari, useq-schema, and OME-NGFF. Our controller layer exposes high-level primitives (`acquire_zstack`, `pulse_laser`, `move_stage`, `set_well`) and low-level callbacks that run during acquisition (e.g., real-time CellPose inference to re-focus or re-target).

## §3 Current state of the art

- Edelstein A et al. 2014 J Biol Methods — advanced microscope control using µManager [PMID: 25606571]
- PyMMCore-Plus GitHub docs (pymmcore-plus.github.io) [URL-VERIFY]
- useq-schema — YAML-based acquisition description [URL-VERIFY]

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** — underlying hardware driven by this layer
- **FluorescentCameras** — acquisition + trigger control
- **LaserAblation_405** — dispatches ablation events
- **CellPose_Segmentation** — inline callback during acquisition
- **AICoordinator** — issues high-level commands to this layer
- **GenealogyReconstruction** — consumes acquisition metadata + event logs

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Few published reference pipelines for long-duration adaptive microscopy
2. Event-driven architecture mixing hardware triggers + Python callbacks is brittle
3. Reliable recovery from transient hardware faults (laser blip, camera USB reset) is rarely documented

**Deliverables (Phase A):**
- Working µManager 2.0 config for the retrofitted IM 35
- Python package `cytotree-control` wrapping PyMMCore-Plus with project-specific primitives
- YAML-driven experiment descriptions (useq-schema extension)
- Robust 72 h acquisition with automated fault recovery
- Open-source on GitHub (MIT)
