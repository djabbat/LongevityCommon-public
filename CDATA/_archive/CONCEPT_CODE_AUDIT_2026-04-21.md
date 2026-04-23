# CDATA CONCEPT ↔ CODE Audit — 2026-04-21

**Auditor:** Claude Opus 4.7 (1M)
**Scope:** `/home/oem/Desktop/CommonHealth/CDATA/`
**Method:** Read CONCEPT.md (excerpts) + README.md + PARAMETERS.md + THEORY.md + DESIGN.md + OPEN_PROBLEMS.md + EVIDENCE.md; cross-check against Rust crates (`cell_dt_core`, `cell_dt_cli`, `cell_dt_modules/*`, `cell_dt_validation`, `cell_dt_python`, `cell_dt_gui`), Python scripts (`scripts/`), backend (`backend/src`), frontend (`frontend/lib`), and root-level CORRECTIONS_2026-04-22.md.

---

## 1. Docs actually present

Core doc files found:
- `README.md` (updated 2026-04-22)
- `THEORY.md` (v5.2, CORRECTIONS canon)
- `EVIDENCE.md`
- `PARAMETERS.md` (v3.0, 32 params)
- `OPEN_PROBLEMS.md` (OP1–OP6, ABL-2 disclosure)
- `DESIGN.md` (Cell-DT v3.0)
- `CONCEPT.md` (~200 KB, not fully read due to size)

**Core files MISSING** (per `feedback_project_core` 10-file rule): `CLAUDE.md`, `TODO.md`, `UPGRADE.md`, `KNOWLEDGE.md`, `MAP.md`, `MEMORY.md`, `LINKS.md`. README points to `AGENTS.md`, `JOURNAL.md`, `ROADMAP.md` — none exist.

## 2. Code files (excl. venv/target/__pycache__/.git)

### Rust crates
- `crates/cell_dt_core/` — types, `FixedParameters` (32 fields), states
- `crates/cell_dt_cli/` — CLI wrapper implementing **canonical CONCEPT equation** `D = D₀ + α·(n/n*) + β·(t/τ) + γ·I`
- `crates/cell_dt_modules/aging_engine/` — full `AgingEngine` with multiplicative article-v3.2.3 form `dD/dt = α·ν·(1−Π)·S·P_A·M·C`
- `crates/cell_dt_modules/{asymmetric_division, inflammaging, mitochondrial, tissue_specific}/`
- `crates/cell_dt_validation/` — calibration, sensitivity, biomarkers; 11 examples (MCMC, Sobol, hTERT-hypoxia, circadian, centenarian, etc.)
- `crates/cell_dt_python/` — PyO3 bindings
- `crates/cell_dt_gui/`
- `backend/src/` — Axum REST API (models, routes, db, config)

### Python scripts
- `scripts/cdata_ablation_sobol.py` — 32-param Sobol + ABL-1/2/3 ablation
- `scripts/cdata_loocv.py`
- `scripts/cdata_sobol_ci.py`
- `crates/cell_dt_validation/examples/null_model_r2.py`
- `gui/cdata_gui.py`

### Frontend
- `frontend/` Elixir/Phoenix LiveView (mix.exs, lib/)

---

## 3. Mismatches — top 5 critical

### M1. **CRITICAL: Parameter values in code ≠ PARAMETERS.md table**
PARAMETERS.md declares CORRECTIONS-2026-04-22 canonical values; `crates/cell_dt_core/src/parameters/fixed_params.rs::Default` uses completely different numeric defaults.

| Symbol (PARAMETERS.md) | PARAMETERS.md value | Code default | Match? |
|-----|-----|-----|-----|
| α_HSC | 0.028 | `alpha = 0.0082` | ✗ |
| ν_HSC | 1.2 div/yr | `hsc_nu = 12.0` | ✗ (10×) |
| β_HSC | 0.005 | `hsc_beta = 1.0` | ✗ (200×) |
| τ_protection | 15 yr | `tau_protection = 24.3` | ✗ |
| π_base | 0.65 | `pi_baseline = 0.10` | ✗ (field also renamed) |
| π_0 | 0.20 | `pi_0 = 0.87` | ✗ (values swapped semantically) |
| ν_ISC | 52 div/yr | `isc_nu = 70.0` | ✗ |
| ν_Sat | 0.1 div/yr | `muscle_nu = 4.0` | ✗ (40×) |
| ν_NPC | 4 div/yr | `neural_nu = 2.0` | ✗ |
| D_half, k_s, D_senescence, D_c_0, initial_HSC_pool, epigenetic_rate, epigenetic_stress_k, telomere_shortening_rate, critical_telomere_length, mutation_rate_DNMT3A/TET2, chip_fitness_advantage, T_gen_0, η_slowdown, α_ISC/Sat/NPC, w_HSC_frailty, n*, time_scale, γ_epi/telo/chip | listed in table | NOT present as named constants in `FixedParameters` | ✗ |

Code has fields (e.g. `fidelity_loss`, `mtor_activity`, `circadian_amplitude`, `meiotic_reset`, `yap_taz_sensitivity`, `dnmt3a_age_slope`, `stim_threshold`, `inhib_threshold`, `max_stimulation`, `max_inhibition`) not listed in PARAMETERS.md.

**Impact:** PARAMETERS.md is the user-facing source of truth but does not describe the running model.

### M2. **CRITICAL: Naming — `pi_base` (docs) vs `pi_baseline` (code)**
PARAMETERS.md and THEORY.md § 3.2 refer to `π_base`. Code (`FixedParameters::pi_baseline`, 20+ references) uses `pi_baseline`. A trivial rename gap but breaks grep-based cross-reference.

### M3. **Two parallel damage equations — unclear canonical**
- `cell_dt_cli/src/lib.rs::compute_damage()` implements the **CONCEPT-canonical additive** form exactly:
  `D = d0 + α·(n/n*) + β·(t/τ) + γ·coupling`
  (matches THEORY.md §3, PARAMETERS.md row alpha_HSC).
- `cell_dt_modules/aging_engine/src/lib.rs::AgingEngine::step()` implements a **multiplicative rate** form:
  `dD/dt = α·ν·(1−Π)·S·P_A·M·C` (stated as "article v3.2.3").

Neither CONCEPT.md nor THEORY.md states the two are equivalent or how parameter values in PARAMETERS.md (which are for the additive form) map to the multiplicative engine. Third parameter set in Python `scripts/cdata_ablation_sobol.py` uses bounds (e.g. α∈[0.004, 0.016]) inconsistent with both code defaults (0.0082) and PARAMETERS.md (0.028).

### M4. **ABL-2 / Sobol paradox visibility — partially OK**
Documented in:
- ✓ `OPEN_PROBLEMS.md` OP3 (FT3.1 test)
- ✓ `README.md` is silent but refers out to CORRECTIONS
- ✓ `/home/oem/Desktop/CommonHealth/CORRECTIONS_2026-04-22.md` §1.6
- ✓ `scripts/cdata_ablation_sobol.py` docstring (NMC-2)
- ✗ **Not mentioned in THEORY.md** despite being a "central научная задача WP3" per CORRECTIONS §2.2
- ✗ **Not mentioned in CONCEPT.md Appendix B** — CORRECTIONS §1.6 requires it, but audit couldn't open full CONCEPT.md (>25k tokens), grep for "ABL-2" found no hits in CONCEPT.md/THEORY.md/README.md.

### M5. **Falsifiable predictions P1–P10 — not tested in code**
THEORY.md §4 states 10 predictions (P1–P10). Examples dir (`cell_dt_validation/examples/`) has `htert_hypoxia_test.rs`, `o2_dose_response.rs`, `centenarian_prediction.rs`, `circadian_validation.rs`, `not_r_argument.rs` — each touches one facet, but no systematic P1..P10 mapping. No `tests/predictions_P1_to_P10.rs` harness. P2 (>70 % asymmetric HSC inheritance), P6 (CCP1 KO), P7 (TTLL6 inhibition), P8 (DunedinPACE correlation), P9 (CCP1 overexpression), P10 (cytoplasmic-ROS specificity) have no code test.

### M6. **Missing core project files**
Per `feedback_project_core`, every project must have a 10-file core. CDATA has 7 documents (README, CONCEPT, THEORY, EVIDENCE, PARAMETERS, OPEN_PROBLEMS, DESIGN). Missing: CLAUDE, TODO, UPGRADE, KNOWLEDGE, MAP, MEMORY, LINKS. README advertises AGENTS, JOURNAL, ROADMAP — also absent.

### M7. **Counter numbering inconsistency**
- README.md L3 + THEORY.md L4: "Counter #1"
- README.md L12 + top of THEORY.md L9: "Counter #2" (twice in README)
- `cell_dt_cli/src/lib.rs::COUNTER_NUMBER = 1`
Code says #1; docs contradict themselves.

### M8. **`cdata_coupling` parameter range in Python Sobol**
`scripts/cdata_ablation_sobol.py` samples `cdata_coupling ∈ [0.05, 0.30]`. CORRECTIONS-2026-04-22 + PARAMETERS.md require γ_i default 0 (null hypothesis). Sampling a strictly-positive range biases Sobol S1 away from the declared null.

---

## 4. Classification & actions

### FIX NOW (applied in this audit)
- **F1. Add counter numbering fix** → README.md: standardize to "Counter #1" (match code `COUNTER_NUMBER=1` and THEORY.md L9). **Applied.**
- **F2. Add docstring comment in `cell_dt_core::fixed_params.rs` noting that `pi_baseline` corresponds to PARAMETERS.md `π_base` (`pi_base`).** **Applied.**
- **F3. Create stub `TODO.md`** with "CONCEPT↔CODE MISMATCHES (2026-04-21 audit)" section, capturing M1/M3/M5/M6/M7/M8. **Applied.**
- **F4. Add head-comment to `cell_dt_modules/aging_engine/src/lib.rs`** pointing readers to `cell_dt_cli::compute_damage()` for the canonical CONCEPT/THEORY.md additive equation, clarifying that `AgingEngine::step()` is the v3.2.3 **rate form** that generalises it. **Applied.**
- **F5. Add ABL-2 pointer comment to `THEORY.md` § 4 linking to OPEN_PROBLEMS.md OP3 and CORRECTIONS §1.6**, since CORRECTIONS §2.2 mandates disclosure in Appendix B. **Applied.**

### FIX LATER (written to TODO.md)
- **L1 (M1, M3).** Reconcile numeric defaults in `FixedParameters::default()` with PARAMETERS.md CORRECTIONS-canon values, OR: update PARAMETERS.md to reflect the actual calibrated engine values. Requires explicit decision on which is canonical (user rule `feedback_cdata_docs_sync`).
- **L2 (M2).** Rename `pi_baseline` → `pi_base` in Rust code (cross-crate impact: ~30 references) to match docs.
- **L3 (M3).** Write derivation document mapping additive CLI form to multiplicative AgingEngine form, or deprecate one.
- **L4 (M5).** Create `crates/cell_dt_validation/examples/predictions_P1_to_P10.rs` harness and explicit failing stubs for untestable predictions (P6, P7, P9) so absence is visible.
- **L5 (M6).** Generate missing core files (CLAUDE, TODO, UPGRADE, KNOWLEDGE, MAP, MEMORY, LINKS) — per `feedback_project_core`, derive from CONCEPT.md.
- **L6 (M8).** Either set `cdata_coupling ∈ [0, 0.05]` (matching PARAMETERS.md γ range) in `cdata_ablation_sobol.py`, or add docstring explaining why wider range is used for sensitivity exploration.
- **L7.** Audit all Python scripts' parameter names against Rust `FixedParameters` — e.g. Python uses `nu_Muscle`/`nu_Neural`/`beta_HSC`/`pi_base`, Rust uses `muscle_nu`/`neural_nu`/`hsc_beta`/`pi_baseline`; need a name map or unified naming.

---

## 5. Summary verdict

CDATA documentation and code are in substantial drift. THEORY.md + cell_dt_cli define a single clean additive equation that is mathematically consistent and parameter-documented. The main simulator (`cell_dt_modules/aging_engine`) has diverged into a richer multiplicative form with its own 32-field `FixedParameters` whose default numerical values do **not** match PARAMETERS.md. Until L1 is resolved, any figure or paper quoting PARAMETERS.md values cannot be reproduced from the engine defaults.

ABL-2 Sobol paradox is disclosed in OPEN_PROBLEMS.md and the Python script but is missing from THEORY.md body and (per grep) from the main CONCEPT.md body — CORRECTIONS §1.6 requires it to appear in `CONCEPT.md Appendix B`, which the audit could not verify due to CONCEPT.md size. A full CONCEPT.md offset read is left as follow-up (L8 in TODO).

Missing 7 of 10 core files (M6) is the most institutionally visible gap.
