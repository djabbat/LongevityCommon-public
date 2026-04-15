#!/usr/bin/env python3
"""
Ze HRV Module — Autonomic Ze Analysis from RR Intervals
=========================================================
Computes χ_Ze(HRV) — Ze cheating index from cardiac autonomic dynamics.

ALGORITHM
---------
RR-intervals encode the balance between sympathetic (LF) and parasympathetic (HF)
autonomic branches. Ze theory treats this as a binary switching process:

  State 1 (sympathetic):    LF > HF × (1 + δ)
  State 0 (parasympathetic):HF > LF × (1 + δ)
  Unchanged (hysteresis):   |LF - HF| / max(LF, HF) ≤ δ

  δ = 0.1  (10% hysteresis zone — prevents noise-driven flipping)

  Window: 300 s (5 min), overlap 50%
  v_HRV = N_S / (N_windows − 1)
  χ_Ze(HRV) = 1 − |v_HRV − v*| / max(v*, 1−v*)

CLINICAL HYPOTHESIS
-------------------
Pre-disease states (hypertension, early diabetes, chronic stress) →
autonomic rigidity → rare sympatho-vagal switching → v_HRV → 0 or 1 →
χ_Ze(HRV) decreases.

VALIDATED ON: PhysioNet CinC 2017 (planned), MIMIC-III HRV subset (planned)

REFERENCES
----------
Tkemaladze J. (2026). BioSense CONCEPT v3.0
Task Force of ESC/NASPE (1996). Heart rate variability. Circulation, 93(5), 1043–1065.
"""

import numpy as np
import json
import warnings
from pathlib import Path
from typing import Optional, Tuple, List
from dataclasses import dataclass, field
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

# ── Ze constants ─────────────────────────────────────────────────────────────
V_STAR    = 0.45631
DELTA_HYS = 0.10     # hysteresis threshold (10%)
WINDOW_S  = 300.0    # analysis window in seconds (5 min)
OVERLAP   = 0.50     # 50% overlap between windows

# ── LF / HF band definitions (Task Force 1996) ───────────────────────────────
LF_LOW  = 0.04   # Hz
LF_HIGH = 0.15   # Hz
HF_LOW  = 0.15   # Hz
HF_HIGH = 0.40   # Hz


# ── Core computations ─────────────────────────────────────────────────────────

def ze_velocity(binary_seq: np.ndarray) -> float:
    """v = N_S / (N-1) — fraction of switching events."""
    N = len(binary_seq)
    if N < 2:
        return float('nan')
    switches = int(np.sum(binary_seq[1:] != binary_seq[:-1]))
    return switches / (N - 1)


def ze_cheating_index(v: float) -> float:
    """χ_Ze = 1 − |v − v*| / max(v*, 1−v*)  ∈ [0,1]."""
    if np.isnan(v):
        return float('nan')
    return 1.0 - abs(v - V_STAR) / max(V_STAR, 1.0 - V_STAR)


def compute_lf_hf(rr_ms: np.ndarray, fs_interp: float = 4.0) -> Tuple[float, float]:
    """
    Compute LF and HF power from RR interval series.

    Steps:
      1. Interpolate unevenly-spaced RR to uniform grid (fs_interp Hz)
      2. Detrend (remove mean)
      3. Welch PSD
      4. Integrate power in LF and HF bands

    Parameters
    ----------
    rr_ms     : RR intervals in milliseconds
    fs_interp : target sampling rate for interpolation (Hz); 4 Hz is standard

    Returns
    -------
    (lf_power, hf_power) in ms²
    """
    from scipy.signal import welch
    from scipy.interpolate import interp1d

    if len(rr_ms) < 10:
        return float('nan'), float('nan')

    # Cumulative time axis (in seconds)
    t_rr = np.cumsum(rr_ms) / 1000.0
    t_rr -= t_rr[0]

    # Uniform grid
    t_uniform = np.arange(0, t_rr[-1], 1.0 / fs_interp)
    if len(t_uniform) < 8:
        return float('nan'), float('nan')

    # Cubic interpolation
    try:
        interp = interp1d(t_rr, rr_ms, kind='cubic', bounds_error=False,
                          fill_value=(rr_ms[0], rr_ms[-1]))
        rr_uniform = interp(t_uniform)
    except Exception:
        return float('nan'), float('nan')

    # Detrend
    rr_uniform -= rr_uniform.mean()

    # Welch PSD
    freqs, psd = welch(rr_uniform, fs=fs_interp, nperseg=min(256, len(rr_uniform)))

    # Band power (trapezoid integration)
    lf_mask = (freqs >= LF_LOW)  & (freqs <= LF_HIGH)
    hf_mask = (freqs >= HF_LOW)  & (freqs <= HF_HIGH)

    lf_power = float(np.trapz(psd[lf_mask], freqs[lf_mask])) if lf_mask.any() else 0.0
    hf_power = float(np.trapz(psd[hf_mask], freqs[hf_mask])) if hf_mask.any() else 0.0

    return lf_power, hf_power


def autonomic_state_with_hysteresis(
    rr_segments: List[np.ndarray],
    delta: float = DELTA_HYS,
    fs_interp: float = 4.0,
) -> np.ndarray:
    """
    Compute binary autonomic state sequence for a list of RR windows.

    State assignment per window:
      1 → sympathetic  (LF dominates)
      0 → parasympathetic (HF dominates)
      Previous state → hysteresis zone

    Parameters
    ----------
    rr_segments : list of RR arrays (one per window)
    delta       : hysteresis fraction [0.05–0.20]

    Returns
    -------
    state_seq : int8 array of 0/1 states, length = len(rr_segments)
    """
    states = np.zeros(len(rr_segments), dtype=np.int8)
    prev_state = -1  # undefined

    for i, seg in enumerate(rr_segments):
        lf, hf = compute_lf_hf(seg, fs_interp=fs_interp)
        if np.isnan(lf) or np.isnan(hf) or (lf + hf) < 1e-12:
            if prev_state >= 0:
                states[i] = prev_state
            continue

        balance = abs(lf - hf) / max(lf, hf)

        if balance <= delta:
            # Hysteresis zone: keep previous state
            states[i] = prev_state if prev_state >= 0 else 0
        elif lf > hf:
            states[i] = 1  # sympathetic
            prev_state = 1
        else:
            states[i] = 0  # parasympathetic
            prev_state = 0

    return states


# ── Windowed analysis ─────────────────────────────────────────────────────────

@dataclass
class HrvZeResult:
    """Result of Ze HRV analysis over a recording."""
    chi_ze_hrv: float             # primary Ze index
    v_hrv: float                  # switching velocity
    n_windows: int                # number of analysed windows
    lf_hf_ratios: List[float]     # LF/HF per window
    state_seq: np.ndarray         # binary state sequence
    window_chi: List[float]       # rolling χ_Ze (each window pair)
    lf_mean: float                # mean LF power (ms²)
    hf_mean: float                # mean HF power (ms²)
    rmssd: float                  # standard RMSSD (validation)
    sdnn: float                   # standard SDNN  (validation)
    quality_ok: bool              # False if too few windows or high NaN rate


def analyse_rr(
    rr_ms: np.ndarray,
    window_s: float = WINDOW_S,
    overlap: float  = OVERLAP,
    delta: float    = DELTA_HYS,
    fs_interp: float = 4.0,
    min_windows: int = 4,
) -> HrvZeResult:
    """
    Full Ze HRV analysis pipeline.

    Parameters
    ----------
    rr_ms      : 1-D array of RR intervals in milliseconds
    window_s   : window duration (seconds)
    overlap    : fractional overlap [0, 1)
    delta      : hysteresis parameter
    min_windows: minimum valid windows for reliable χ_Ze

    Returns
    -------
    HrvZeResult dataclass
    """
    if len(rr_ms) < 2:
        raise ValueError("Need at least 2 RR intervals")

    # Standard metrics (ms domain, no windowing)
    rmssd = float(np.sqrt(np.mean(np.diff(rr_ms) ** 2)))
    sdnn  = float(np.std(rr_ms, ddof=1))

    # Cumulative time
    t_cum = np.cumsum(rr_ms) / 1000.0
    total_s = t_cum[-1]

    # Window boundaries
    step_s   = window_s * (1.0 - overlap)
    starts   = np.arange(0, total_s - window_s + 1e-9, step_s)
    segments = []
    lf_vals, hf_vals, lf_hf_ratios = [], [], []

    for t0 in starts:
        t1    = t0 + window_s
        mask  = (t_cum >= t0) & (t_cum < t1)
        seg   = rr_ms[mask]
        if len(seg) < 20:
            continue
        lf, hf = compute_lf_hf(seg, fs_interp=fs_interp)
        if np.isnan(lf) or np.isnan(hf):
            continue
        segments.append(seg)
        lf_vals.append(lf)
        hf_vals.append(hf)
        lf_hf_ratios.append(lf / hf if hf > 1e-12 else float('nan'))

    quality_ok = len(segments) >= min_windows

    if not segments:
        return HrvZeResult(
            chi_ze_hrv=float('nan'), v_hrv=float('nan'),
            n_windows=0, lf_hf_ratios=[], state_seq=np.array([]),
            window_chi=[], lf_mean=float('nan'), hf_mean=float('nan'),
            rmssd=rmssd, sdnn=sdnn, quality_ok=False,
        )

    # Autonomic state sequence with hysteresis
    state_seq = autonomic_state_with_hysteresis(segments, delta=delta, fs_interp=fs_interp)

    # Ze velocity over state sequence
    v = ze_velocity(state_seq)
    chi = ze_cheating_index(v)

    # Rolling χ_Ze (every pair of consecutive states)
    window_chi = []
    for i in range(1, len(state_seq)):
        v_pair = ze_velocity(state_seq[max(0, i-4):i+1])
        window_chi.append(ze_cheating_index(v_pair))

    return HrvZeResult(
        chi_ze_hrv=chi,
        v_hrv=v,
        n_windows=len(segments),
        lf_hf_ratios=[r for r in lf_hf_ratios if not np.isnan(r)],
        state_seq=state_seq,
        window_chi=window_chi,
        lf_mean=float(np.mean(lf_vals)),
        hf_mean=float(np.mean(hf_vals)),
        rmssd=rmssd,
        sdnn=sdnn,
        quality_ok=quality_ok,
    )


# ── Demo with synthetic data ──────────────────────────────────────────────────

def demo_synthetic():
    """
    Demonstrate χ_Ze(HRV) on synthetic RR series representing 3 autonomic profiles:
      1. Healthy young:  balanced LF/HF switching → v ≈ v* → χ_Ze ≈ 0.85
      2. Sympathetic dominance (stress): rare switching → v → 0 → χ_Ze ↓
      3. Rigid (pre-disease): very rare switching → v → 0 → χ_Ze ↓↓
    """
    rng = np.random.default_rng(42)

    def make_rr(base_ms, noise_ms, n_beats, lf_amp, hf_amp, lf_hz=0.1, hf_hz=0.25, fs=1.0):
        """Synthetic RR series with LF + HF oscillations."""
        t = np.arange(n_beats) / fs
        lf = lf_amp * np.sin(2 * np.pi * lf_hz * t)
        hf = hf_amp * np.sin(2 * np.pi * hf_hz * t)
        noise = rng.normal(0, noise_ms, n_beats)
        return np.clip(base_ms + lf + hf + noise, 400, 1400)

    profiles = {
        "Healthy young (balanced)":     make_rr(800, 20, 1200, 40, 30),
        "Sympathetic dominant (stress)": make_rr(750, 10, 1200, 60,  5),
        "Autonomic rigidity (pre-disease)": make_rr(720,  5, 1200, 10,  3),
    }

    print("\n=== Ze HRV Demo (synthetic) ===\n")
    print(f"{'Profile':<40} {'χ_Ze(HRV)':>10} {'v_HRV':>8} {'RMSSD':>8} {'n_win':>6}")
    print("-" * 75)

    results = {}
    for name, rr in profiles.items():
        try:
            res = analyse_rr(rr, window_s=120, overlap=0.5, delta=DELTA_HYS, min_windows=3)
            print(f"{name:<40} {res.chi_ze_hrv:>10.4f} {res.v_hrv:>8.4f} "
                  f"{res.rmssd:>8.1f} {res.n_windows:>6}")
            results[name] = res
        except Exception as e:
            print(f"{name:<40} ERROR: {e}")

    return results


# ── Plotting ──────────────────────────────────────────────────────────────────

def plot_hrv_ze(result: HrvZeResult, label: str = "", out_dir: str = "results"):
    """Save HRV Ze analysis plot."""
    Path(out_dir).mkdir(exist_ok=True)
    fig, axes = plt.subplots(1, 3, figsize=(15, 4))
    fig.suptitle(f"Ze HRV Analysis — {label}", fontsize=12, fontweight='bold')

    # Panel 1: State sequence
    ax = axes[0]
    if len(result.state_seq) > 0:
        ax.step(range(len(result.state_seq)), result.state_seq, where='post',
                color='steelblue', linewidth=2)
        ax.set_yticks([0, 1]); ax.set_yticklabels(['Parasympath.\n(HF)', 'Sympath.\n(LF)'])
    ax.set_xlabel('Window #'); ax.set_title(f'Autonomic State Sequence\nv={result.v_hrv:.4f}')
    ax.grid(alpha=0.3)

    # Panel 2: Rolling χ_Ze
    ax = axes[1]
    if result.window_chi:
        ax.plot(result.window_chi, color='darkorange', linewidth=2)
        ax.axhline(result.chi_ze_hrv, color='red', linestyle='--',
                   label=f'mean χ_Ze={result.chi_ze_hrv:.3f}')
        ax.axhline(V_STAR, color='green', linestyle=':', alpha=0.7, label=f'v*={V_STAR}')
    ax.set_xlabel('Window #'); ax.set_ylabel('χ_Ze(HRV)')
    ax.set_title('Rolling Ze Index'); ax.legend(fontsize=8); ax.grid(alpha=0.3)
    ax.set_ylim(0, 1)

    # Panel 3: LF/HF ratios
    ax = axes[2]
    if result.lf_hf_ratios:
        ax.plot(result.lf_hf_ratios, color='mediumpurple', linewidth=1.5)
        ax.axhline(1.0, color='k', linestyle='--', alpha=0.5, label='LF/HF = 1 (balance)')
        ax.set_xlabel('Window #'); ax.set_ylabel('LF/HF ratio')
        ax.set_title('Sympathovagal Balance'); ax.legend(fontsize=8); ax.grid(alpha=0.3)

    plt.tight_layout()
    fname = label.replace(' ', '_').replace('/', '-')
    out_path = Path(out_dir) / f"ze_hrv_{fname}.png"
    plt.savefig(out_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"Saved: {out_path}")


# ── CLI ───────────────────────────────────────────────────────────────────────

def main():
    import argparse
    p = argparse.ArgumentParser(description="Ze HRV Analysis")
    p.add_argument('--demo',    action='store_true', help='Run synthetic demo')
    p.add_argument('--file',    type=str,            help='CSV file with RR intervals (ms, one per line)')
    p.add_argument('--label',   type=str, default='',help='Subject label')
    p.add_argument('--window',  type=float, default=WINDOW_S, help='Window size (s)')
    p.add_argument('--delta',   type=float, default=DELTA_HYS, help='Hysteresis δ')
    p.add_argument('--plot',    action='store_true', help='Save plot')
    p.add_argument('--out',     default='results',   help='Output directory')
    args = p.parse_args()

    if args.demo:
        results = demo_synthetic()
        if args.plot:
            for label, res in results.items():
                plot_hrv_ze(res, label=label, out_dir=args.out)

    elif args.file:
        rr = np.loadtxt(args.file)
        result = analyse_rr(rr, window_s=args.window, delta=args.delta)
        out = {
            "label":      args.label,
            "chi_ze_hrv": result.chi_ze_hrv,
            "v_hrv":      result.v_hrv,
            "n_windows":  result.n_windows,
            "rmssd_ms":   result.rmssd,
            "sdnn_ms":    result.sdnn,
            "quality_ok": result.quality_ok,
        }
        print(json.dumps(out, indent=2))
        if args.plot:
            plot_hrv_ze(result, label=args.label, out_dir=args.out)
    else:
        p.print_help()


if __name__ == '__main__':
    main()
