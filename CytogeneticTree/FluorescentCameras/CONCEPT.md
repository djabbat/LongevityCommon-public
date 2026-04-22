# FluorescentCameras — Scientific CMOS Detectors for Live-Cell Fluorescence

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

The imaging sensor is the bottleneck that sets the SNR, temporal resolution, and photon-efficiency of the whole Cytogenetic Tree readout. This subproject selects, qualifies, and integrates **industrial-grade scientific CMOS cameras** (FLIR Blackfly S, Hikrobot MV-CH050-10UM, and Basler ace classes) as cost-effective alternatives to traditional sCMOS (Hamamatsu Orca-Flash, Photometrics Prime BSI). The target is ≥ 70 % QE at 510 and 610 nm, ≤ 2 e⁻ read noise, global shutter, mono, 3–5 MP, at ≤ €2 k per unit.

## §2 Scientific basis / mechanism

High-sensitivity live-cell fluorescence imaging demands:

1. **High quantum efficiency** (photons → electrons): 60–80 % in visible band
2. **Low read noise**: < 3 e⁻ per pixel to resolve dim centriole foci
3. **Global shutter**: avoids rolling-shutter artifacts on moving mitotic cells
4. **Mono, cooled**: color filters halve QE; cooling reduces dark current during long exposures

Industrial cameras based on Sony Pregius / Starvis sensors (IMX250, IMX264, IMX428) now rival scientific cameras at ~ 10× lower cost. The main concession is smaller well depth (~ 10 k e⁻) and less mature drivers, both acceptable for our application.

## §3 Current state of the art

- Sony IMX Pregius datasheet (IMX250 / IMX264) — industrial global-shutter CMOS specs
- Mandracchia B et al. 2020 — low-noise industrial camera characterization for microscopy (reference pending dedicated PubMed verification)
- Photometrics / Hamamatsu application notes — sCMOS reference [DOC-PENDING]

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** — host platform; cameras mount on C-mount ports
- **MicroscopeController** — PyMMCore-Plus drivers for Genicam / Pylon / Spinnaker SDKs
- **CellPose_Segmentation** — consumes camera streams; benefits from low read noise
- **RITE_Centriole** — dual-channel red/green demands two synchronized cameras
- **ImageAnalysis** — flat-field / dark-field corrections calibrated here

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Industrial cameras rarely characterized for microscopy QE in peer-reviewed literature
2. Synchronization of two independent cameras requires precise hardware trigger
3. Long-term cooling needs thermoelectric + fan (not provided on most industrial models)

**Deliverables (Phase A):**
- Two-camera rig: identical sCMOS units for red/green synchronized acquisition
- QE curve + read-noise measurement per unit
- Hardware trigger sync < 1 µs jitter
- Open-source characterization notebook + dataset on Zenodo
