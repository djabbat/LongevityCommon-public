# CDATA Parameter Reconciliation Analysis — 2026-04-21

**Auditor:** Claude Opus 4.7 (1M)
**Scope:** Docs ↔ Code divergence for 5 parameters flagged in 2026-04-21 audit.
**Canonical doc:** `/home/oem/Desktop/CommonHealth/CDATA/PARAMETERS.md` (v3.0, 32 params, CORRECTIONS-2026-04-22 canon).
**Canonical code:** `/home/oem/Desktop/CommonHealth/CDATA/crates/cell_dt_core/src/parameters/fixed_params.rs` (`FixedParameters::default()`).

Cross-verified via:
- `crates/cell_dt_validation/src/calibration.rs` — MCMC Round-7 posterior documentation
- `crates/cell_dt_modules/aging_engine/src/lib.rs` — running simulator (multiplicative `dD/dt` form)
- `crates/cell_dt_modules/tissue_specific/src/tissue_params.rs` — tissue-level `base_division_rate`
- `crates/cell_dt_cli/src/lib.rs` — CONCEPT-canonical additive form `D = d₀ + α·(n/n*) + β·(t/τ) + γ·I`
- `scripts/cdata_ablation_sobol.py`, `scripts/cdata_loocv.py`, `gui/cdata_gui.py`

---

## §1. Executive Summary — TL;DR per parameter

| # | Param | Docs | Code | Root cause | Recommendation |
|---|-------|------|------|------------|----------------|
| 1 | α (α_HSC) | 0.028 | 0.0082 | **Stale docs.** Code value is MCMC-post-Round-7 posterior; calibration.rs explicitly cites PMID 36583780 and documents collinearity with τ (r=0.858). | **(b) Fix docs → code** |
| 2 | ν_HSC | 1.2 /yr | 12.0 | **Unit divergence inside code** (not /decade). `tissue_params.rs` comment L46: "40 bp/yr ÷ 12 div/yr". Code genuinely treats ν_HSC = 12 div/yr, which is ~10× the Wilson 2008 literature estimate. | **(b) Fix docs → code** + **(c) add note** explaining discrepancy vs Wilson 2008; OR **(a)** if docs value is preferred for biological fidelity |
| 3 | β_HSC | 0.005 /yr | 1.0 | **Dead parameter in engine.** `hsc_beta` is declared in `FixedParameters` but has **zero downstream references**; the multiplicative `AgingEngine::step()` does not multiply by it. β is only used in `cell_dt_cli::compute_damage()` (additive form), where it is a separate `beta: 0.15` (not 1.0). | **(b) Fix docs → code** AND remove `hsc_beta` from `FixedParameters` OR document it as reserved |
| 4 | π_base / π_0 | π_base=0.65, π_0=0.20 | pi_0=0.87, pi_baseline=0.10 | **Semantic inversion + rename.** Docs: π_base = max (asymptote top), π_0 = min (floor). Code: `pi_0` = amplitude coefficient (max-min), `pi_baseline` = floor (asymptote bottom). Different decomposition of same biological quantity. | **(a) Fix code → docs semantics** (rename `pi_baseline` → `pi_base`, reinterpret `pi_0` as floor); OR **(b) rewrite docs** to match code's "amplitude + baseline" form. L2 in TODO.md tracks the rename. |
| 5 | τ_prot | 15 yr | 24.3 yr | **Stale docs.** Code value is MCMC Round-7 posterior mean, 95% CI [19.1, 29.7] per `calibration.rs` L71. Docs value 15 yr is pre-calibration literature prior. | **(b) Fix docs → code** |

**Overall:** 4 of 5 divergences are **stale docs vs calibrated code**, not bugs. Single real bug: `hsc_beta = 1.0` is an unused field that misleads readers. PARAMETERS.md should be rewritten to reflect post-Round-7 MCMC values as the canonical specification; no simulations need to be rerun.

---

## §2. Per-parameter analysis with code-location references

### 2.1 α (α_HSC) — damage per division

| Aspect | Value / source |
|--------|----------------|
| Docs claim | 0.028 damage/division, CI [0.022, 0.035], status **Fitted** |
| Code default | `alpha: 0.0082` — `fixed_params.rs:69` |
| Test pins value | `fixed_params.rs:190` `assert!((p.alpha - 0.0082).abs() < 1e-6)` |
| Calibration doc | `calibration.rs:61` — "fixed at literature value (PMID: 36583780); collinear with tau_protection (posterior r = 0.858)" |
| Python scripts | `cdata_ablation_sobol.py:215` comment: `# Use best-fit parameters (CDATA PARAMETERS.md: alpha=0.0082, tau=24.3, pi0=0.87)` — **scripts believe 0.0082 IS the PARAMETERS.md value.** |
| Usage in engine | `aging_engine/src/lib.rs:341` `let damage_rate = self.params.alpha * division_rate * (1.0 - protection) * …` |
| Usage in CLI | `cell_dt_cli/src/lib.rs:44` `D = d₀ + α·(n/n*) + β·(t/τ) + γ·coupling` with `alpha: 0.60` default (completely different α!) |

**Note:** `cell_dt_cli::CounterParams::default.alpha = 0.60` (`cell_dt_cli/src/lib.rs:32`) is yet another α value meant for MCOA counter-tuple reporting — per-division polyGlu rate in the additive form, not the multiplicative rate used by AgingEngine. This is a third ecosystem.

### 2.2 ν_HSC — HSC division frequency

| Aspect | Value / source |
|--------|----------------|
| Docs claim | 1.2 divisions/year, CI [0.8, 1.6], status **Literature + Fitted** (Wilson 2008; Kowalczyk 2015) |
| Code default | `hsc_nu: 12.0` — `fixed_params.rs:78` |
| Tissue mirror | `tissue_params.rs:27` `base_division_rate: 12.0` (for `TissueType::Hematopoietic`) |
| **Critical evidence for H1 test** | `aging_engine/src/lib.rs:46` comment: "HSC differentiated daughters: ~40 bp/yr ÷ ~12 div/yr ≈ 3.3 bp/div" — **explicit /year units** |
| Usage | `aging_engine/src/lib.rs:317` `division_rate = self.tissue_params.base_division_rate * age_factor * sasp_factor * regenerative_potential` |
| Calibration status | `calibration.rs:63` "insensitive: ΔR² ≈ 0 at ±20% perturbation" — excluded from MCMC, held fixed at 12.0 |
| Sobol range in Python | `cdata_ablation_sobol.py:26` `('nu_HSC', 6.0, 18.0)` — consistent with code, NOT with docs |

The H1 clean-conversion hypothesis (/decade × 10 = /year → 12 ≈ 1.2/decade) **fails**: the telomere sanity comment in `aging_engine/src/lib.rs:46` uses "/yr" explicitly. Code treats 12.0 as divisions per calendar year. This is biologically higher than Wilson 2008 (~1/yr for LT-HSC) but consistent with ST-HSC + MPP turnover (Busch 2015). The choice is modeling-motivated (total HSC compartment turnover), not a units bug.

### 2.3 β_HSC — background damage rate (time)

| Aspect | Value / source |
|--------|----------------|
| Docs claim | 0.005 damage/year, CI [0.001, 0.01], status **Assumed** |
| Code default | `hsc_beta: 1.0` — `fixed_params.rs:79` |
| **Downstream usage** | `grep -r "hsc_beta"` returns **only** the field declaration, the default value, and two assertions that it is positive (`fixed_params.rs:553`). **Not multiplied into `damage_rate` anywhere in `aging_engine`.** |
| CLI `beta` (different!) | `cell_dt_cli/src/lib.rs:33` `beta: 0.15` in `CounterParams` default — used as additive coefficient `β·(t/τ)` |
| Tissue-specific β equivalents | `fixed_params.rs:82,85,88` — `isc_beta=0.3`, `muscle_beta=1.2`, `neural_beta=1.5` — also unused (no grep hits outside self) |

**Conclusion:** `hsc_beta` (and all `*_beta` in FixedParameters) is a **ghost field**. Changing it has no effect on running simulations. This is a pure documentation-fidelity gap: the docs describe a parameter the engine does not actually use. The AgingEngine's "background damage" enters implicitly through `(1 - protection) × ros_damage_factor` with no time-linear β term.

### 2.4 π_base vs pi_baseline — self-renewal probability decomposition

| Aspect | Docs (PARAMETERS.md:33-34) | Code (`fixed_params.rs:71,74`) |
|--------|------|------|
| Top value | π_base = 0.65 (max self-renewal @ max signal) | pi_0 = 0.87 (amplitude coefficient) |
| Bottom value | π_0 = 0.20 (min self-renewal @ zero signal) | pi_baseline = 0.10 (asymptotic floor) |
| Formula | (implied) π(s) = π_0 + (π_base − π_0) · σ(s, k_s, D_half) | `youth_protection(age) = pi_0 · exp(−age/τ) + pi_baseline` (`fixed_params.rs:142`) |
| Value at t=0 / max signal | π_base = 0.65 | 0.87 + 0.10 = **0.97** |
| Value at t=∞ / zero signal | π_0 = 0.20 | **0.10** |

Two different biological quantities:
- **Docs** describe self-renewal as a function of **signal strength** (π depends on damage-modulated signaling). Parameters `D_half`, `k_s` mediate the sigmoid.
- **Code** describes protection as a function of **age** (monotonic exponential decay from youth). No signal dependence; no `D_half` / `k_s` fields in `FixedParameters`.

Therefore the divergence is not just numeric — it is a **model-structural mismatch**. The "π" in docs is signal-dependent self-renewal probability; the "π" in code is age-dependent youth protection. Docs list `D_half=2.5` and `k_s=0.8` but these names do not exist anywhere in the Rust code (confirmed by grep). The code's youth-protection concept has no counterpart in PARAMETERS.md's row labels.

Test assertion `test_validate_pi_sum_exceeds_one` (`fixed_params.rs:234`) enforces `pi_0 + pi_baseline ≤ 1`, implying "pi_0 is the amplitude that starts on top of pi_baseline" — code semantics. A code value pair (0.87, 0.10) gives total protection at age 0 of 0.97, which is biologically plausible for "max youth self-renewal". But that 0.97 corresponds to docs' π_base = 0.65 **only** after re-reading docs in a signal-centric frame.

### 2.5 τ_prot — protection time constant

| Aspect | Value / source |
|--------|----------------|
| Docs claim | 15 years, CI [10, 25], **Fitted** |
| Code default | `tau_protection: 24.3` — `fixed_params.rs:73` |
| MCMC posterior | `calibration.rs:71` — "Post-Round-7 posterior mean: 24.3 yr (95% CI: 19.1–29.7)" |
| Usage | `fixed_params.rs:142` `self.pi_0 * (-age_years / self.tau_protection).exp() + self.pi_baseline` — `age_years` in years, `tau_protection` in years → dimensionless ratio ✓ |
| LOOCV prior | `cdata_loocv.py:25` `TAU_PRIOR = 24.3` |
| GUI default | `gui/cdata_gui.py:519` `tau_protection: float = 24.3` |

Units are unambiguously years; 24.3 is the calibrated posterior. The docs value 15 is a pre-calibration literature-based prior (matches CI center). This is a straightforward stale-docs case.

---

## §3. Hypothesis test table

| Hypothesis | ν_HSC | β_HSC | π (π_base vs pi_base/line) | τ_prot | α |
|------------|-------|-------|-----------------------------|--------|---|
| **H1. Unit convention (/decade etc.)** | **FAIL** — `lib.rs:46` uses "/yr" explicitly | N/A (field unused) | N/A (dimensionless) | PASS trivially (both /yr) | N/A (damage/division both) |
| **H2. Normalized vs absolute scale** | FAIL — both are absolute div/yr | N/A | Partial — different decomposition but same normalized range | PASS (both absolute yr) | FAIL — same unit |
| **H3. Calibrated posterior vs prior** | PASS — code 12.0 held fixed in MCMC per calibration.rs:63 (ΔR²≈0 insensitive); docs 1.2 is literature prior | N/A (dead) | **PASS** — code (0.87, 0.10) is MCMC posterior per calibration.rs:78; docs (0.65, 0.20) is Morrison/Kimble/Yamashita literature prior | **PASS** — code 24.3 is Round-7 posterior; docs 15 is literature prior | **PASS** — code 0.0082 fixed at PMID 36583780 literature value per calibration.rs:61; docs 0.028 is pre-Round-7 |
| **H4. Truly wrong / historical bug** | NO | **PARTIAL** — field is dead code (not a wrong number but an unused one) | NO — different decomposition is intentional | NO | NO |

**Net result:** H3 (calibrated vs prior drift) explains 3/5 (α, τ_prot, π-family numerics). Semantic rename (π) and dead field (β) explain 2/5. H1 (unit) and H2 (normalization) are **not** operative.

---

## §4. Recommended resolution paths

### Per-parameter decisions

| Param | Path | Rationale | Effort |
|-------|------|-----------|--------|
| α | **(b) Docs → code** | Code value 0.0082 is MCMC-anchored + Python/GUI/scripts already treat it as canonical | Edit PARAMETERS.md row; mark status "Fitted (Round-7 posterior)"; update 95% CI to match calibration.rs bounds |
| ν_HSC | **(b) Docs → code** + **(c) Documentation note** | Code 12.0 is total-compartment turnover; flag that this is higher than Wilson 2008 LT-HSC because it includes MPP/ST-HSC | Edit PARAMETERS.md row; add footnote explaining compartment scope |
| β_HSC | **(b) + structural cleanup** | Field is **unused**; either remove from `FixedParameters` (L-task) or retain as "reserved for additive CLI form" | Docs: mark row as "CLI-only (additive); not used by AgingEngine". Code: separate TODO to remove or document. |
| π family | **Hybrid (a) + (b)** | Requires joint docs+code rewrite: pick one decomposition convention, unify. L2 in TODO.md already tracks the rename. Recommendation: adopt **code convention** (`pi_0` as amplitude, `pi_baseline` as floor) AND rename `pi_baseline` → `pi_base` in code to match docs' preferred symbol. | (i) docs rewrite π row to "amplitude + baseline" decomposition; (ii) rename code field (~30 refs, mostly tests) |
| τ_prot | **(b) Docs → code** | Same rationale as α — MCMC posterior is canonical | Edit PARAMETERS.md row; CI [19.1, 29.7] |

### No path (a) recommended for α, ν, β, τ individually

Option (a) "fix code → docs" for any of these would require re-running the full calibration pipeline (MCMC chains, Sobol analysis N=16384, LOOCV, centenarian prediction, convergence_check, hTERT-hypoxia test), regenerating all figures, invalidating the Round-7 posterior, and potentially breaking the already-disclosed ABL-2 Sobol paradox analysis. **Not worth it.** The docs are the stale artifact; the code has been iterated on through calibration rounds.

---

## §5. Full-code vs full-docs final sync plan

Since no option (a) is recommended, the sync direction is **docs ← code** for 4/5 parameters plus a **coordinated rewrite** for π.

### §5.1 Docs-only changes (no code touched, no figure regen)

PARAMETERS.md edits (5 rows):

| Row | Change |
|-----|--------|
| `alpha_HSC` | value 0.028 → **0.0082**; CI → (literature-fixed, not MCMC-varied; see calibration.rs:61); status → **Fixed (literature, PMID 36583780)** |
| `nu_HSC` | value 1.2 → **12.0**; CI → [6, 18] (matching Sobol range); status → **Fixed (compartment-level; see footnote)**; add footnote: "ν_HSC here represents aggregated LT-HSC + ST-HSC + MPP turnover, not LT-HSC alone" |
| `beta_HSC` | value 0.005 → **N/A (CLI additive form only)**; add note: "Not used by AgingEngine multiplicative form" OR remove row entirely and list only in a 'CLI additive form' sub-table |
| `pi_base` / `pi_0` | Rewrite as: `pi_0` (amplitude) = 0.87, CI [0.82, 0.92]; `pi_baseline` = 0.10, CI [0.05, 0.15]; add formula row: `Π(age) = pi_0 · exp(−age/τ_prot) + pi_baseline`. Remove `D_half`, `k_s` rows (or mark them as "signaling form, not currently in AgingEngine"). |
| `tau_protection` | value 15 → **24.3**; CI [10, 25] → **[19.1, 29.7]**; status → **Fitted (Round-7 MCMC posterior)** |

No simulations break. No figures need regeneration.

### §5.2 Code-side changes (tracked as L-tasks in TODO.md)

| Task | Action | Tests affected |
|------|--------|----------------|
| **L2** (existing) | Rename `pi_baseline` → `pi_base` in Rust (~30 references across `fixed_params.rs`, `aging_engine/src/lib.rs:108`, `gui/cdata_gui.py:520,547`, `calibration.rs`, all test names containing `pi_baseline`). | `test_validate_pi_sum_exceeds_one`, `test_youth_protection_at_zero`, `test_youth_protection_asymptote`, `test_validate_pi_sum_at_boundary_exactly_one`, `test_pi_baseline_positive`, `test_youth_protection_at_tau` — rename only, no numeric change. |
| **New L-task** | Remove (or document as CLI-only) the dead `*_beta` fields in `FixedParameters`: `hsc_beta`, `isc_beta`, `muscle_beta`, `neural_beta`. Update `test_all_tissue_beta_positive` accordingly. | `test_all_tissue_beta_positive` — deletion or repointing |
| **New L-task** | Add module-header docstring in `fixed_params.rs` listing which fields are **live** (entered into `damage_rate` calculation) vs **reserved** (e.g. `mtor_activity`, `meiotic_reset`, `yap_taz_sensitivity` already annotated; `*_beta` fields are not). | None. |

### §5.3 Figures / validation outputs

None invalidated under the (b)-dominant plan. Only dependency: if π-field rename (L2) is done simultaneously with any docstring-regenerated figure captions, redo captions. MCMC chains, Sobol results, LOOCV, convergence_check — all unaffected since only field names change, not defaults.

### §5.4 Cross-project impact

| File | Status | Action |
|------|--------|--------|
| `gui/cdata_gui.py` | Uses `pi_0=0.87`, `pi_baseline=0.10`, `tau_protection=24.3` — **already code-aligned** | No change |
| `scripts/cdata_loocv.py` | Uses `TAU_PRIOR=24.3`, `pi_0` prior 0.87 — **already code-aligned** | No change |
| `scripts/cdata_ablation_sobol.py` | Uses `alpha_best`, `tau_best=24.3` — **already code-aligned**; but its comment L215 **wrongly claims** these are PARAMETERS.md values | Comment fix after PARAMETERS.md rewrite (then comment becomes true) |
| `backend/src/routes.rs:191,222` | Binds user-supplied `counter.alpha` to DB — no default used | No change |
| `CONCEPT.md` / `THEORY.md` | May cite old docs values in text/equations (not verified here; needs separate audit) | Follow-up grep for `0.028`, `0.005`, `pi_base = 0.65` in CONCEPT.md and THEORY.md |

---

## §6. Summary verdict

The divergence is overwhelmingly **stale documentation** (4 of 5 parameters), not a code-side bug or a unit-convention gap. The code's defaults reflect the Round-7 MCMC posterior (`calibration.rs` is the de-facto source of truth), while PARAMETERS.md was frozen at the pre-calibration literature-prior stage. One structural mismatch exists (π decomposition: docs use "max/min self-renewal given signal"; code uses "age-decaying protection as amplitude + floor"), requiring coordinated rewrite. One dead field (`hsc_beta`) should be surfaced as reserved or removed.

**Recommended action order:**
1. Rewrite PARAMETERS.md α, ν_HSC, τ_prot rows to code values (low-risk, no figure regen).
2. Rewrite π row to code's amplitude+floor form; remove `D_half`, `k_s` rows (or move to "planned future signaling form" sub-table).
3. Document `hsc_beta` / `*_beta` as reserved or schedule deletion (new L-task in TODO.md).
4. Schedule L2 rename `pi_baseline` → `pi_base` (tracked, deferred).
5. Leave calibration pipeline, figures, Sobol, LOOCV untouched.

*No simulations need to be rerun. No figures need regeneration. Publication-safety restored by Docs→Code sync alone.*
