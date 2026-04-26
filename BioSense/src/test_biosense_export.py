#!/usr/bin/env python3
"""
test_biosense_export.py — BioSenseExport compatibility test.

Verifies that the Python BioSense pipeline produces valid BioSenseExport JSON
compatible with LongevityCommon's POST /api/biosense/compute endpoint.

Usage:
    python3 test_biosense_export.py                   # offline schema test
    python3 test_biosense_export.py --url http://localhost:3000  # live API test

Schema v1.0 — matches server/src/models/biosense.rs
"""

import json
import sys
import argparse
import numpy as np
from datetime import datetime, timezone

# ── Ze constants (must match server/src/services/ze_compute.rs) ──────────────
V_STAR         = 0.45631
K_DUAL         = 0.45
K_EEG_ONLY     = 0.42
K_HRV_ONLY     = 0.38
D_NORM_ALPHA   = 1.2
DEFAULT_SE_CHI = 0.05   # single-sample SE


def binarize(signal: np.ndarray) -> np.ndarray:
    return (signal > np.median(signal)).astype(np.int8)

def ze_velocity(binary_seq: np.ndarray) -> float:
    N = len(binary_seq)
    if N < 2:
        return 0.0
    return int(np.sum(binary_seq[1:] != binary_seq[:-1])) / (N - 1)

def chi_ze(v: float) -> float:
    return 1.0 - abs(v - V_STAR) / max(V_STAR, 1.0 - V_STAR)

def compute_eeg_metrics(signal: np.ndarray, sample_rate: int = 256) -> dict:
    binary = binarize(signal)
    v = ze_velocity(binary)
    return {
        "chi_ze_eeg": round(chi_ze(v), 6),
        "v_eeg": round(v, 6),
        "n_samples": len(signal),
        "method": "narrowband",
        "band": "alpha_8_12hz",
        "sample_rate_hz": sample_rate,
        "n_channels": 1,
    }

def compute_hrv_metrics(rr_intervals_ms: np.ndarray) -> dict:
    """RR intervals in milliseconds → Ze HRV metrics."""
    binary = binarize(rr_intervals_ms)
    v = ze_velocity(binary)
    rmssd = float(np.sqrt(np.mean(np.diff(rr_intervals_ms) ** 2)))
    return {
        "chi_ze_hrv": round(chi_ze(v), 6),
        "v_hrv": round(v, 6),
        "n_beats": len(rr_intervals_ms),
        "rmssd_ms": round(rmssd, 2),
        "lf_hf_ratio": None,  # Welch PSD not computed in this minimal test
    }

def build_biosense_export(
    eeg_signal: np.ndarray | None,
    rr_intervals: np.ndarray | None,
    chrono_age: float | None = None,
    subject_id: str | None = None,
    notes: str | None = None,
) -> dict:
    """Build a BioSenseExport dict from raw signals."""
    export = {
        "schema_version": "1.0",
        "recorded_at": datetime.now(timezone.utc).isoformat(),
    }
    if chrono_age is not None:
        export["chrono_age"] = chrono_age
    if subject_id is not None:
        export["subject_id"] = subject_id
    if eeg_signal is not None:
        export["eeg"] = compute_eeg_metrics(eeg_signal)
    if rr_intervals is not None:
        export["hrv"] = compute_hrv_metrics(rr_intervals)
    export["device"] = {
        "model": "BioSense-test",
        "pipeline_version": "1.0.0",
    }
    if notes is not None:
        export["notes"] = notes
    return export


# ── Expected server-side computation (mirrors Rust logic) ────────────────────

def expected_response(export: dict) -> dict:
    chi_eeg = export.get("eeg", {}).get("chi_ze_eeg") if "eeg" in export else None
    chi_hrv = export.get("hrv", {}).get("chi_ze_hrv") if "hrv" in export else None

    if chi_eeg is not None and chi_hrv is not None:
        chi_combined = (chi_eeg + chi_hrv) / 2.0
        k = K_DUAL
        cal = "dual"
    elif chi_eeg is not None:
        chi_combined = chi_eeg
        k = K_EEG_ONLY
        cal = "eeg_only"
    else:
        chi_combined = chi_hrv
        k = K_HRV_ONLY
        cal = "hrv_only"

    d_norm = min(max(D_NORM_ALPHA * (1.0 - chi_combined), 0.0), 1.0)

    bio_age = ci_low = ci_high = ci_stability = None
    if "chrono_age" in export:
        ca = export["chrono_age"]
        est = ca * (1.0 - d_norm * k)
        jacobian = ca * D_NORM_ALPHA * k
        ci_half = max(jacobian * DEFAULT_SE_CHI * 1.96, 0.5)
        stability = "high" if ci_half < 2.0 else ("medium" if ci_half < 5.0 else "low")
        bio_age = round(est, 2)
        ci_low = round(est - ci_half, 2)
        ci_high = round(est + ci_half, 2)
        ci_stability = stability

    return {
        "chi_ze_eeg": chi_eeg,
        "chi_ze_hrv": chi_hrv,
        "chi_ze_combined": round(chi_combined, 6),
        "d_norm": round(d_norm, 6),
        "bio_age": bio_age,
        "bio_age_ci_low": ci_low,
        "bio_age_ci_high": ci_high,
        "ci_stability": ci_stability,
        "calibration": cal,
        "schema_version": "1.0",
    }


# ── Tests ─────────────────────────────────────────────────────────────────────

def test_schema_valid(export: dict) -> bool:
    """Check required fields are present and valid."""
    assert "schema_version" in export, "Missing schema_version"
    assert export["schema_version"] == "1.0", f"Bad schema_version: {export['schema_version']}"
    assert "recorded_at" in export, "Missing recorded_at"
    assert "eeg" in export or "hrv" in export, "Must have eeg or hrv"
    if "eeg" in export:
        e = export["eeg"]
        assert 0.0 <= e["chi_ze_eeg"] <= 1.0, f"chi_ze_eeg out of range: {e['chi_ze_eeg']}"
        assert 0.0 <= e["v_eeg"] <= 1.0, f"v_eeg out of range: {e['v_eeg']}"
        assert e["n_samples"] > 0, "n_samples must be > 0"
    if "hrv" in export:
        h = export["hrv"]
        assert 0.0 <= h["chi_ze_hrv"] <= 1.0, f"chi_ze_hrv out of range: {h['chi_ze_hrv']}"
        assert h["n_beats"] > 0, "n_beats must be > 0"
    return True


def test_computation_accuracy(export: dict, server_response: dict, tol: float = 1e-4) -> bool:
    """Verify Python-computed expected values match server response."""
    expected = expected_response(export)
    for key in ["chi_ze_combined", "d_norm"]:
        exp_v = expected[key]
        got_v = server_response[key]
        assert abs(exp_v - got_v) < tol, f"{key}: expected {exp_v}, got {got_v}"
    if expected["bio_age"] is not None and server_response["bio_age"] is not None:
        assert abs(expected["bio_age"] - server_response["bio_age"]) < tol, \
            f"bio_age: expected {expected['bio_age']}, got {server_response['bio_age']}"
    return True


def run_offline_tests():
    """Run schema validation tests without a live server."""
    print("Running offline BioSenseExport compatibility tests...\n")
    np.random.seed(42)

    test_cases = [
        {
            "name": "EEG only — young subject (25y)",
            "eeg": np.random.randn(2560),   # 10s at 256Hz
            "rr": None,
            "age": 25.0,
        },
        {
            "name": "HRV only — middle-aged (50y)",
            "eeg": None,
            "rr": 800 + 60 * np.random.randn(300),   # 5min HRV
            "age": 50.0,
        },
        {
            "name": "Dual sensor — elderly (72y)",
            "eeg": np.random.randn(2560),
            "rr": 900 + 80 * np.random.randn(250),
            "age": 72.0,
        },
        {
            "name": "No age — schema only",
            "eeg": np.random.randn(1280),
            "rr": None,
            "age": None,
        },
    ]

    passed = 0
    for tc in test_cases:
        try:
            export = build_biosense_export(tc["eeg"], tc["rr"], tc["age"])
            test_schema_valid(export)
            # Compute expected and verify self-consistency
            exp = expected_response(export)
            assert 0.0 <= exp["chi_ze_combined"] <= 1.0
            assert 0.0 <= exp["d_norm"] <= D_NORM_ALPHA
            if tc["age"] is not None:
                assert exp["bio_age"] is not None
                assert exp["ci_stability"] in ("high", "medium", "low")
            print(f"  ✓ {tc['name']}")
            print(f"    χ_Ze_combined={exp['chi_ze_combined']:.4f}  "
                  f"D_norm={exp['d_norm']:.4f}  "
                  f"bio_age={exp.get('bio_age')}  "
                  f"CI=[{exp.get('bio_age_ci_low')}, {exp.get('bio_age_ci_high')}]  "
                  f"cal={exp['calibration']}")
            passed += 1
        except AssertionError as e:
            print(f"  ✗ {tc['name']}: {e}")

    print(f"\n{passed}/{len(test_cases)} tests passed.")
    return passed == len(test_cases)


def run_live_tests(base_url: str):
    """Run tests against a live LongevityCommon server."""
    import urllib.request
    print(f"Running live tests against {base_url} ...\n")
    np.random.seed(42)

    url = f"{base_url}/api/biosense/compute"
    export = build_biosense_export(
        eeg_signal=np.random.randn(2560),
        rr_intervals=800 + 60 * np.random.randn(300),
        chrono_age=45.0,
        subject_id="test_001",
    )
    body = json.dumps(export).encode()
    req = urllib.request.Request(url, data=body,
                                  headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            response = json.loads(resp.read())
        test_schema_valid(export)
        test_computation_accuracy(export, response)
        print("  ✓ Live API test passed")
        print(f"    Response: {json.dumps(response, indent=2)}")
    except Exception as e:
        print(f"  ✗ Live test failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", help="Base URL of LongevityCommon server for live test")
    args = parser.parse_args()

    if args.url:
        run_live_tests(args.url)
    else:
        ok = run_offline_tests()
        sys.exit(0 if ok else 1)
