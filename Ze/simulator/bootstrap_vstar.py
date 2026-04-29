#!/usr/bin/env python3
"""
Bootstrap 95% CI for v*_active — PER-DATASET MODE (BUG-v9-3 fix)
==================================================================
Computes BCa bootstrap confidence interval for v*_active SEPARATELY
per dataset. Pooling across datasets was INVALID (I²=90.3%, Cochran
Q=20.613, p<0.0001 — documented 2026-04-11).

Datasets:
  Cuban EEG Dataset     (Zenodo 4244765)     N=88   d=1.694
  Dortmund HRV Study    (OpenNeuro ds005385)  N=60   d=0.732
  MPI-LEMON EEG         (Babayan 2019)        N=50   d=0.110

Usage:
    # Per-dataset (DEFAULT — recommended):
    python3 bootstrap_vstar.py --mode per_dataset \\
            --data_cuban cuban_v_values.npy \\
            --data_dortmund dortmund_v_values.npy \\
            --data_lemon mpi_lemon_v_values.npy

    # Pooled (DEPRECATED — only if Cochran Q passes homogeneity):
    python3 bootstrap_vstar.py --mode pooled \\
            --data_cuban cuban_v_values.npy \\
            --data_dortmund dortmund_v_values.npy

    # Permutation test: compare v*_cuban vs v*_dortmund:
    python3 bootstrap_vstar.py --mode permutation \\
            --data_cuban cuban_v_values.npy \\
            --data_dortmund dortmund_v_values.npy

Data format:
    Each .npy file = 1D array of per-subject optimal v values (float64)
    v_i = v at which chi_Ze is maximised for subject i
    Expected shape: (N_subjects,)

Output:
    bootstrap_vstar_results.json

Author: Ze Vectors Theory / Tkemaladze 2026
BUG-v9-3: Added per-dataset mode, permutation test, Cochran Q guard.
"""

import numpy as np
import json
import argparse
from pathlib import Path

# ──────────────────────────────────────────────────────────────────
# BCa BOOTSTRAP
# ──────────────────────────────────────────────────────────────────

def bca_ci(data: np.ndarray, stat_fn, B: int = 10000,
           alpha: float = 0.05, seed: int = 42) -> dict:
    """
    Bias-corrected and accelerated (BCa) bootstrap CI.
    Returns: dict with keys: estimate, ci_lo_95, ci_hi_95, ci_width, se,
             n_bootstrap, n_subjects, z0_hat, a_hat
    """
    from scipy import stats as scipy_stats

    rng = np.random.default_rng(seed)
    n = len(data)
    observed = float(stat_fn(data))

    # Bootstrap replicates
    boot = np.array([
        stat_fn(rng.choice(data, size=n, replace=True))
        for _ in range(B)
    ])

    # Bias correction z0_hat
    z0_hat = float(scipy_stats.norm.ppf(np.mean(boot < observed)))

    # Acceleration a_hat: jackknife
    jack = np.array([stat_fn(np.delete(data, i)) for i in range(n)])
    jack_mean = float(np.mean(jack))
    num = float(np.sum((jack_mean - jack) ** 3))
    den = float(6.0 * (np.sum((jack_mean - jack) ** 2) ** 1.5))
    a_hat = num / den if den != 0 else 0.0

    # Adjusted percentiles
    z_lo = float(scipy_stats.norm.ppf(alpha / 2))
    z_hi = float(scipy_stats.norm.ppf(1 - alpha / 2))

    def adj_pct(z_in):
        inner = z0_hat + z_in
        return float(scipy_stats.norm.cdf(z0_hat + inner / (1.0 - a_hat * inner))) * 100.0

    pct_lo = adj_pct(z_lo)
    pct_hi = adj_pct(z_hi)

    ci_lo = float(np.percentile(boot, pct_lo))
    ci_hi = float(np.percentile(boot, pct_hi))

    return {
        "estimate": observed,
        "ci_lo_95": ci_lo,
        "ci_hi_95": ci_hi,
        "ci_width": ci_hi - ci_lo,
        "se": float(np.std(boot)),
        "n_bootstrap": B,
        "n_subjects": n,
        "z0_hat": z0_hat,
        "a_hat": a_hat,
    }


# ──────────────────────────────────────────────────────────────────
# COCHRAN'S Q HETEROGENEITY TEST
# ──────────────────────────────────────────────────────────────────

def cochran_q_test(estimates: list, variances: list) -> dict:
    """
    Cochran's Q test for homogeneity across k estimates.
    H0: all estimates come from same population.

    Parameters:
        estimates: list of k point estimates (e.g. v*_cuban, v*_dort, v*_lemon)
        variances: list of k variance estimates (SE²)

    Returns:
        dict with Q, df, p_value, I2, pooled_estimate, heterogeneity_verdict
    """
    from scipy import stats as scipy_stats

    k = len(estimates)
    w = [1.0 / v for v in variances]        # inverse-variance weights
    w_total = sum(w)
    theta_bar = sum(wi * ti for wi, ti in zip(w, estimates)) / w_total  # weighted mean

    Q = sum(wi * (ti - theta_bar) ** 2 for wi, ti in zip(w, estimates))
    df = k - 1
    p_value = float(1.0 - scipy_stats.chi2.cdf(Q, df))

    I2 = max(0.0, (Q - df) / Q * 100.0) if Q > 0 else 0.0

    if I2 >= 75:
        verdict = "HIGH heterogeneity — pooling INVALID"
    elif I2 >= 50:
        verdict = "MODERATE heterogeneity — pooling PROBLEMATIC"
    elif I2 >= 25:
        verdict = "LOW heterogeneity — pooling ACCEPTABLE with caution"
    else:
        verdict = "MINIMAL heterogeneity — pooling ACCEPTABLE"

    return {
        "Q": float(Q),
        "df": df,
        "p_value": p_value,
        "I2_pct": float(I2),
        "pooled_estimate": float(theta_bar),
        "heterogeneity_verdict": verdict,
    }


# ──────────────────────────────────────────────────────────────────
# TWO-SAMPLE PERMUTATION TEST
# ──────────────────────────────────────────────────────────────────

def permutation_test_median_diff(a: np.ndarray, b: np.ndarray,
                                  n_perms: int = 10000,
                                  seed: int = 42) -> dict:
    """
    Two-sided permutation test: H0: median(a) == median(b)
    Test statistic: |median(a) - median(b)|
    """
    rng = np.random.default_rng(seed)
    observed_diff = float(abs(np.median(a) - np.median(b)))
    combined = np.concatenate([a, b])
    na = len(a)

    count = 0
    for _ in range(n_perms):
        perm = rng.permutation(combined)
        diff = abs(np.median(perm[:na]) - np.median(perm[na:]))
        if diff >= observed_diff:
            count += 1

    p_value = float(count / n_perms)
    return {
        "observed_median_diff": observed_diff,
        "p_value": p_value,
        "n_permutations": n_perms,
        "interpretation": (
            "SIGNIFICANT: v*_cuban ≠ v*_dortmund — pooling INVALID"
            if p_value < 0.05 else
            "NOT significant: v*_cuban ≈ v*_dortmund (pooling may be considered)"
        ),
    }


# ──────────────────────────────────────────────────────────────────
# DATA LOADING WITH SIMULATION FALLBACK
# ──────────────────────────────────────────────────────────────────

def load_or_simulate(path: str, n: int, mean: float, sd: float,
                     label: str, rng: np.random.Generator) -> tuple:
    """
    Load .npy file if exists; otherwise simulate with warning.
    Returns (array, simulated: bool)
    """
    p = Path(path)
    if p.exists():
        arr = np.load(str(p))
        print(f"  ✓ {label}: N={len(arr)} (real data)")
        return arr, False
    else:
        arr = rng.normal(mean, sd, n).clip(0.01, 0.99)
        print(f"  ⚠ {label}: N={n} (SIMULATED — replace with real data)")
        return arr, True


# ──────────────────────────────────────────────────────────────────
# MAIN
# ──────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="Bootstrap v*_active CI — per-dataset or pooled"
    )
    parser.add_argument(
        "--mode",
        choices=["per_dataset", "pooled", "permutation"],
        default="per_dataset",
        help=(
            "per_dataset (default, recommended): bootstrap each dataset "
            "separately. pooled: DEPRECATED — only use after Cochran Q check. "
            "permutation: two-sample test comparing Cuban vs Dortmund medians."
        ),
    )
    parser.add_argument("--data_cuban", type=str, default="cuban_v_values.npy")
    parser.add_argument("--data_dortmund", type=str, default="dortmund_v_values.npy")
    parser.add_argument("--data_lemon", type=str, default="mpi_lemon_v_values.npy",
                        help="MPI-LEMON v values (N≈50)")
    parser.add_argument("--B", type=int, default=10000)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--out", type=str, default="bootstrap_vstar_results.json")
    args = parser.parse_args()

    rng = np.random.default_rng(args.seed)

    print("=" * 65)
    print("  bootstrap_vstar.py — v*_active bootstrap CI")
    print(f"  Mode: {args.mode.upper()}")
    print("=" * 65)

    # Load datasets
    # Cuban EEG: mean≈0.456, SD≈0.04 (d=1.694 → large effect vs passive)
    # Dortmund HRV: mean≈0.43, SD≈0.06 (d=0.732 → moderate)
    # MPI-LEMON: mean≈0.32, SD≈0.07 (d=0.110 → near passive, null)
    cuban_v, cuban_sim = load_or_simulate(
        args.data_cuban, 88, 0.456, 0.04, "Cuban EEG (N=88)", rng
    )
    dort_v, dort_sim = load_or_simulate(
        args.data_dortmund, 60, 0.430, 0.06, "Dortmund HRV (N=60)", rng
    )
    lemon_v, lemon_sim = load_or_simulate(
        args.data_lemon, 50, 0.320, 0.07, "MPI-LEMON EEG (N=50)", rng
    )
    any_simulated = cuban_sim or dort_sim or lemon_sim

    v_passive = float(1.0 - np.log(2))  # ≈ 0.3069

    results = {
        "mode": args.mode,
        "v_star_passive": v_passive,
        "simulated_data": any_simulated,
        "simulation_note": (
            "SIMULATED DATA — results for illustration only. "
            "Replace .npy files with real per-subject v values."
            if any_simulated else "Real data used."
        ),
    }

    # ── MODE: PER_DATASET ─────────────────────────────────────────
    if args.mode == "per_dataset":
        print(f"\nRunning BCa bootstrap per dataset (B={args.B})...\n")

        per = {}
        for label, arr in [("cuban", cuban_v), ("dortmund", dort_v), ("lemon", lemon_v)]:
            res = bca_ci(arr, np.median, B=args.B, seed=args.seed)
            per[label] = res
            v_est = res["estimate"]
            ci_lo = res["ci_lo_95"]
            ci_hi = res["ci_hi_95"]
            w = res["ci_width"]
            cohens_d = (v_est - v_passive) / res["se"] if res["se"] > 0 else float("nan")
            print(f"  {label.upper():10s}: v* = {v_est:.5f}  "
                  f"95% BCa CI [{ci_lo:.5f}, {ci_hi:.5f}]  "
                  f"width={w:.4f}  d={cohens_d:.3f}")
            per[label]["cohens_d_vs_passive"] = round(cohens_d, 4)

        results["per_dataset"] = per

        # Cochran's Q across 3 datasets
        ests = [per[k]["estimate"] for k in ["cuban", "dortmund", "lemon"]]
        sqs  = [per[k]["se"] ** 2 for k in ["cuban", "dortmund", "lemon"]]
        # Guard against zero variance
        sqs = [max(s, 1e-10) for s in sqs]
        cochran = cochran_q_test(ests, sqs)
        results["cochran_q"] = cochran

        print(f"\n  Cochran Q = {cochran['Q']:.3f} (df={cochran['df']}, "
              f"p={cochran['p_value']:.4f})")
        print(f"  I² = {cochran['I2_pct']:.1f}% — {cochran['heterogeneity_verdict']}")

        if cochran["I2_pct"] >= 50:
            print("\n  🔴 POOLING INVALID — report datasets separately.")
            print(f"     Pooled estimate ({cochran['pooled_estimate']:.5f}) shown "
                  "for reference ONLY — DO NOT USE IN PAPERS.")
        else:
            print(f"\n  ✅ Pooling acceptable (I²<50%). "
                  f"Pooled estimate = {cochran['pooled_estimate']:.5f}")

        results["recommendation"] = (
            "Report 3 datasets separately (Cuban / Dortmund / MPI-LEMON). "
            "I²=90.3% from prior computation — pooling is statistically invalid. "
            "Any claim about universal v*_active requires independent replication."
        )

    # ── MODE: PERMUTATION ─────────────────────────────────────────
    elif args.mode == "permutation":
        print(f"\nTwo-sample permutation test: Cuban vs Dortmund "
              f"(n_perms={args.B})\n")
        perm = permutation_test_median_diff(
            cuban_v, dort_v, n_perms=args.B, seed=args.seed
        )
        results["permutation_test"] = perm
        print(f"  Observed |median diff| = {perm['observed_median_diff']:.5f}")
        print(f"  p-value = {perm['p_value']:.4f}")
        print(f"  → {perm['interpretation']}")

        print(f"\nTwo-sample permutation test: Cuban vs MPI-LEMON\n")
        perm2 = permutation_test_median_diff(
            cuban_v, lemon_v, n_perms=args.B, seed=args.seed
        )
        results["permutation_test_cuban_vs_lemon"] = perm2
        print(f"  Observed |median diff| = {perm2['observed_median_diff']:.5f}")
        print(f"  p-value = {perm2['p_value']:.4f}")
        print(f"  → {perm2['interpretation']}")

    # ── MODE: POOLED (DEPRECATED) ─────────────────────────────────
    elif args.mode == "pooled":
        print("\n" + "!" * 65)
        print("  ⚠️  POOLED MODE — DEPRECATED")
        print("  I²=90.3% across 3 datasets (Cochran Q=20.613, p<0.0001)")
        print("  Pooling N=196 is STATISTICALLY INVALID.")
        print("  Use --mode per_dataset for publishable results.")
        print("!" * 65 + "\n")

        combined = np.concatenate([cuban_v, dort_v])
        print(f"  Combined N={len(combined)} (Cuban+Dortmund only, as in v1)")
        print(f"  Running BCa bootstrap (B={args.B})...")

        res = bca_ci(combined, np.median, B=args.B, seed=args.seed)
        res["deprecation_warning"] = (
            "DEPRECATED: pooling invalid (I²=90.3%). "
            "For publication use --mode per_dataset."
        )
        results["pooled"] = res

        print(f"\n  v*_active (pooled, DEPRECATED) = {res['estimate']:.5f}")
        print(f"  95% BCa CI: [{res['ci_lo_95']:.5f}, {res['ci_hi_95']:.5f}]")
        print(f"  ⚠️  DO NOT cite this value in papers (high heterogeneity).")

    # ── SUMMARY ───────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print(f"  v*_passive (analytical) = {v_passive:.5f}  (= 1 − ln 2)")
    print("  v*_active: see per-dataset estimates above")
    print("=" * 65)

    if any_simulated:
        print("\n⚠️  WARNING: Some or all data is SIMULATED.")
        print("   Provide real .npy files for publishable results.")

    # Save JSON
    out_path = Path(args.out)
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nResults saved → {out_path}")


if __name__ == "__main__":
    main()
