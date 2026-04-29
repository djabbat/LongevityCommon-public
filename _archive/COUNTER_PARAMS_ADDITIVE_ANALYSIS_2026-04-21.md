# CounterParams (cell_dt_cli) — Third Parameter Set Analysis

**Date:** 2026-04-21
**Scope:** Audit of `cell_dt_cli::CounterParams` additive form relative to MCOA manuscript and Impetus LOI.
**Files of interest:**
- `/home/oem/Desktop/CommonHealth/CDATA/crates/cell_dt_cli/src/lib.rs` (lines 28–47)
- `/home/oem/Desktop/CommonHealth/CDATA/PARAMETERS.md` (line 19 footnote, line 27)
- `/home/oem/Documents/Submissions/2026-04-25_NatureAging_MCOA/MCOA_v5_NatureAging_2026-04-21.md` (Box 1, §3, §6)
- `/home/oem/Documents/Grants/CommonHealth/CDATA/docs/IMPETUS_2026-04-25/LOI_Impetus_v25-1_AI_2026-04-21.md` (§Background, Aim A.1)

---

## 1. Confirmed Defaults (lib.rs:28–40)

```rust
d0=0.0, alpha=0.60, beta=0.15, gamma=0.0,
n_star=50.0, tau_days=10950.0 (=30 yr), d_critical=0.65
```

Tissue scalars (lib.rs:55–72): Neuron/Cardiomyocyte α×0.05 β×1.5; IntestinalCrypt α×1.5 β×0.8; HSC α×1.2.

## 2. Model Form — CRITICAL SEMANTIC DIFFERENCE

`compute_damage` (lib.rs:42–47) implements the **normalized** additive form:

> `D = D₀ + α·(n/n*) + β·(t/τ) + γ·coupling`

where α and β are **dimensionless fractions of d_critical** at the reference scales n* and τ. This is NOT the raw form used in MCOA v5 Box 1 and Impetus LOI:

- MCOA v5: `D_i = D_{i,0} + k_i^div · n + k_i^time · t + Σ c_{ij}·D_j` — k's carry units [D_i]/div and [D_i]/time.
- Impetus LOI §Background: `P(n,t) = P₀ + α·n + β·t` (de Grey refinement) — raw additive.

So α=0.60, β=0.15 in `CounterParams` are **not directly comparable** to the multiplicative α_HSC=0.0082 (PARAMETERS.md line 25) nor to any numeric α/β cited in MCOA/Impetus. They implicitly encode: "at 50 divisions, division-driven damage fills 60% of d_critical; at 30 yr of pure quiescence, time-driven damage fills 15% of d_critical."

## 3. Calibration Status

**Placeholders, not calibrated.** Lib.rs header claims values are "calibrated from CDATA meta-analysis," but:
- No MCMC/bootstrap posterior CI given (unlike α_HSC=0.0082 in PARAMETERS.md).
- Values are round numbers (0.60, 0.15, 50, 30).
- No citation with those numerics anywhere in MCOA v5 or Impetus LOI.
- PARAMETERS.md line 19 explicitly marks them "annotated but out-of-scope for current reconciliation."

## 4. Publication / Figure Cross-References

- MCOA v5: symbolic only (α, β, k_i^div, k_i^time). No figure uses α=0.60 / β=0.15.
- Impetus LOI: symbolic only. Aim A.1 *aims to measure* α and β — i.e., these are the unknowns the grant will fit, not inputs.
- Aubrey de Grey correspondence (2026-04-19) established the symbolic `P(n,t) = P₀ + α·n + β·t`; no numeric instantiation.

## 5. Inconsistencies Identified

1. **Docstring overstatement.** lib.rs:3 says "Parameters calibrated from CDATA meta-analysis." They are not calibrated — they are illustrative defaults. Fix: rephrase to "illustrative defaults consistent with MCOA tissue ordering."
2. **Form mismatch with MCOA/Impetus.** `compute_damage` uses normalized `(n/n*)` and `(t/τ)`; both papers use raw `n` and `t`. These are equivalent modulo reparametrization (k^div = α/n*, k^time = β/τ), but the relationship must be documented to prevent a reviewer concluding α_HSC=0.0082 (multiplicative engine) and α=0.60 (CLI) contradict each other.
3. **β_HSC in dual-form section of PARAMETERS.md line 27** says "0.005 additive cell_dt_cli"; actual code has β=0.15 (global default). The 0.005 figure likely came from an earlier unnormalized draft. Needs reconciliation.

## 6. Recommendation

**Add a separate §3b "MCOA additive CLI form" subsection to PARAMETERS.md** documenting:

- Source file + struct name (`cell_dt_cli::CounterParams`)
- Model equation (normalized form, with explicit n* and τ)
- All 7 defaults with status = **Illustrative (not MCMC-fitted)**
- Mapping to raw MCOA form: `k_i^div = α / n*`, `k_i^time = β / τ`
- Explicit note: these are **not** the α_HSC used by the multiplicative AgingEngine; the CLI is a standalone MCOA Counter-#1 emitter and does not participate in Round-7 calibration.
- Fix the β=0.005 legacy figure on line 27 → β=0.15 (or clarify it is the pre-normalization equivalent 0.15/30yr ≈ 0.005/yr).
- Correct lib.rs:3 docstring from "calibrated" to "illustrative defaults, pending MCOA-joint MCMC."

No code change needed beyond the docstring. No manuscript change needed (MCOA and Impetus are symbolic-only and remain consistent).
