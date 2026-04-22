# GenealogyReconstruction: Conceptual Framework

## §1 Purpose
GenealogyReconstruction is the algorithmic core of the CytogeneticTree project, responsible for converting raw cellular division history into a formal, computable Directed Acyclic Graph (DAG). Its primary purpose is to algorithmically reconstruct the complete genealogical tree of cellular differentiation, tracing all lineages from the zygote to every terminally differentiated cell. This is achieved by leveraging a key biological proxy: the inheritance pattern of **centriole age**. By tracking which centriole (older or younger) is inherited by each daughter cell during asymmetric cell divisions—a process linked to cell fate decisions—the algorithm infers mother-daughter relationships and lineage bifurcations. This reconstruction provides the essential topological scaffold upon which subsequent cytogenetic and epigenetic data can be mapped, transforming a list of events into a testable model of developmental history.

## §2 Mechanism / Basis
The algorithm operates on two primary input streams:
1.  **Division-Event Log:** A chronologically ordered record of every cell division, containing cell identifiers, timestamps, and division type (symmetric/asymmetric).
2.  **Centriole-Inheritance Decisions:** For each asymmetric division, a record of which daughter cell inherited the older ("mother") centriole and which inherited the younger ("daughter") centriole, based on imaging or inference pipelines.

The core logic builds a NetworkX DiGraph (DAG) where nodes represent cell states (with metadata: ID, generation, centriole age) and directed edges represent "is-parent-of" relationships. The algorithm parses the event log sequentially:
*   **Symmetric Division:** Creates two child nodes from one parent, with edges indicating lineage split but no fate bias from centriole inheritance.
*   **Asymmetric Division:** Creates two child nodes. The edge to the cell inheriting the older centriole is tagged with a fate bias property (e.g., `progenitor_fate: likely`), based on the established correlation between older mother centriole inheritance and stem/progenitor fate (Yamashita et al. 2007 Science, [PMID: 17255513]; Royall et al. 2023, [PMID: 37882444]).
*   **Terminal Event:** Marks a node as a leaf (no outgoing edges).

Critical logic handles biological noise: **Focus Drift** (temporary loss of tracking, resolved via temporal gap-closing), **Mixed Centriole** inheritance (ambiguous signals trigger a probabilistic branch point), and **Out-of-Plane Division** (3D spatial data is used to validate or correct inferred 2D lineage connections).

## §3 State of the Art (≤3 Key Refs)
Current lineage reconstruction predominantly relies on fluorescent labeling (e.g., Brainbow), live imaging, or single-cell DNA barcoding. These methods have limitations in temporal resolution, scalability, or ability to function retrospectively in fixed tissues. The use of endogenous, structurally inherited organelles as lineage recorders is a nascent but powerful paradigm.
1.  **Centriole as a Determinant of Cell Fate:** Foundational work established the non-random inheritance of the older mother centriole during asymmetric division in *Drosophila* male germline stem cells (Yamashita et al. 2007 Science, [PMID: 17255513]) and human neural progenitor cells (Royall et al. 2023, [PMID: 37882444]), linking it to the retention of stem cell properties.
2.  **Computational Lineage Tracing:** Advances in single-cell phylogenetics and algorithms for reconstructing trees from CRISPR-Cas9 mutation patterns (GESTALT, McKenna et al. 2016 Science, [PMID: 27229144]; Chan et al. 2019 Nature, [PMID: 31086336]) provide a relevant computational framework for building trees from sparse, noisy data.
3.  **Integrative Morphodynamic Analysis:** Recent methods combining live-cell imaging with transcriptional-landscape mapping (LARRY lentiviral barcoding, Weinreb et al. 2020 Science, [PMID: 31974159]) represent the state-of-the-art in high-fidelity lineage extraction, setting a benchmark for accuracy that this project aims to achieve via a fixed-tissue-compatible method.

## §4 Integration with Other CytogeneticTree Technologies
GenealogyReconstruction is a central integration layer:
*   **Input From:** `../CentrioleDating/` provides the critical "centriole age" attribute for each cell. `../LineageImaging/` (or simulated data) provides the raw division-event log.
*   **Output To:** The produced cytogenetic tree DAG is the primary input for `../EpigeneticMapping/`, which overlays chromatin state data onto each node. It also feeds into `../TreeAnalysis/` for topological quantification (branching asymmetry, depth analysis) and visualization modules.
*   **Shared Data Structure:** All subprojects adhere to a common node/edge schema (using `attrs` in NetworkX) to ensure interoperability, containing fields for Cell_ID, Generation_Num, Centriole_Age, Timestamp, and Fate_Bias_Score.

## §5 Gaps & What to Build
Existing gaps this subproject must address:
1.  **Algorithmic Gap:** No open-source tool exists that uses centriole inheritance rules as the primary engine for tree reconstruction. We must build this logic from the ground up in Python/NetworkX.
2.  **Noise-Handling Gap:** Published studies often ignore real-world imaging artifacts. We must implement robust modules for handling focus drift, ambiguous centriole signals, and 3D validation.
3.  **Validation Gap:** The algorithm requires a simulation framework (`../LineageImaging/Simulator`) to generate ground-truth trees with introduced noise, against which reconstruction accuracy can be rigorously tested.

**What to Build:** A Python package `genealogy_reconstructor` containing: a core `TreeBuilder` class, submodules for `noise_resolution` (drift, mixed), `io_handlers` for log parsing, `validation` metrics (edge accuracy, topology similarity), and export functions to standard formats (JSON, GraphML).

== END
