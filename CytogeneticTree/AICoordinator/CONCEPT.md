# AICoordinator — Claude-Code Overnight Agent for Adaptive Experimentation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

A 72-hour lineage tracking experiment must make **thousands of small decisions** (which daughter to keep, when to re-focus, when to abort a branch, when to increase laser power, when to switch to mitotic burst mode). Delegating these to a human is impossible; delegating them to pure rule-based code is brittle. The AICoordinator leverages Claude Code's `/overnight` mode + a domain-specific `PROMPT.md` orchestration document to act as a domain-aware autonomous agent, issuing commands to `MicroscopeController` in real time.

## §2 Scientific basis / mechanism

The AICoordinator is not an ML model per se — it is an **LLM-as-orchestrator** that consumes:

- Segmentation output (CellPose masks + centriole positions)
- Lineage state (partial GenealogyReconstruction graph)
- Experiment policy (PROMPT.md: keep tree balanced, prune >5 siblings, prioritize red-centriole daughters, etc.)
- Live event log (errors, timings)

…and emits structured commands (`{"action": "ablate", "target_id": 42, "dose_mW": 10}`) that the controller executes. It can call DeepSeek API for heavy reasoning and Claude's `/overnight` protocol for session persistence and retry logic. Self-correcting: uses post-ablation imaging to verify effect and retries if needed.

## §3 Current state of the art

- Claude Code `/overnight` protocol (Anthropic, internal) + SESSION_STATE.md pattern
- LLM-based lab automation: Coscientist (Boiko et al. 2023 *Nature*) [PMID: 38123806]
- SmartACM / autonomous microscopy — emerging field [REF-PENDING]

## §4 Integration with other CytogeneticTree technologies

- **CellPose_Segmentation** — input stream of masks + spots
- **GenealogyReconstruction** — input: current lineage graph state
- **MicroscopeController** — receives structured commands
- **LaserAblation_405** — dispatch target
- **StatisticalAnalysis** — end-of-run consumer of decision log
- **RITE_Centriole** — decisions keyed to red/green centriole age

## §5 Known gaps + what this subproject builds

**Gaps:**
1. No standard protocol for LLM-driven lab automation at this scale
2. Safety + reversibility requires careful tool-call design (dry-run, confirmation gates)
3. Latency (LLM round-trip ~ seconds) limits decision frequency to ~ 1 per minute

**Deliverables (Phase A):**
- `PROMPT.md` orchestration spec (policies, invariants, safety rules)
- Claude Code skill that reads zarr store + emits JSON commands
- Dry-run harness (policies tested on synthetic data from GenealogyReconstruction simulator)
- Live 72 h co-driven run with human-in-the-loop oversight
- Open-source prompt + agent scaffolding on GitHub
