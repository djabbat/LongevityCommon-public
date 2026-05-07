#!/usr/bin/env python3
"""
tools/lab_evaluate.py — thin CLI shim around `lab_reference.py::evaluate`.

Used by `aim-patient-workspace` Labs tab (Phoenix LiveView).

Stage 1 in this overnight session — Rust port of LAB_RANGES is deferred
(P3 closure of `lab_reference.py` per STACK.md frozen-Python rule). Until
then this CLI is the single calling surface from Rust binaries / Phoenix.

USAGE:
    echo '[{"analyte_key":"hemoglobin","value":13.7}, ...]' \
        | python3 tools/lab_evaluate.py evaluate --sex F

OUTPUT:
    JSON array on stdout, one entry per input. Each entry adds:
        - status: normal / low / high / critical_low / critical_high / unknown
        - reference: "low–high" string (in default unit)
        - display: human label
        - notes: optional caveat
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

# Allow running from anywhere — add repo root to path.
REPO = Path(__file__).resolve().parent.parent
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from lab_reference import evaluate, LAB_RANGES  # noqa: E402


def resolve_sex_specific(analyte_key: str, sex: str | None) -> str:
    """`hemoglobin` → `hemoglobin_m` or `hemoglobin_f` based on sex.

    Falls back to the generic key (which won't exist in LAB_RANGES — that
    triggers `status: unknown`, which is honest behaviour).
    """
    if not sex:
        return analyte_key
    s = sex.strip().lower()
    suffix = "_m" if s in {"m", "male"} else ("_f" if s in {"f", "female"} else "")
    if not suffix:
        return analyte_key
    candidate = f"{analyte_key}{suffix}"
    if candidate in LAB_RANGES:
        return candidate
    return analyte_key


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest="cmd", required=True)
    e = sub.add_parser("evaluate", help="evaluate ParsedLab JSON list from stdin")
    e.add_argument("--sex", default=None, help="patient sex M/F (for sex-specific ranges)")
    args = p.parse_args()

    if args.cmd != "evaluate":
        print("unknown subcommand", file=sys.stderr)
        return 2

    raw = sys.stdin.read()
    if not raw.strip():
        print("[]")
        return 0

    try:
        items = json.loads(raw)
    except json.JSONDecodeError as ex:
        print(f"bad input json: {ex}", file=sys.stderr)
        return 2

    out = []
    for item in items:
        analyte_key = item.get("analyte_key", "")
        value = item.get("value", None)
        if not analyte_key or value is None:
            continue
        try:
            value = float(value)
        except (TypeError, ValueError):
            continue
        resolved_key = resolve_sex_specific(analyte_key, args.sex)
        result = evaluate(resolved_key, value)
        # Carry over abbreviation / line_no / unit_raw from input for UI traceability.
        for passthrough in ("abbreviation", "line_no", "unit_raw"):
            if passthrough in item:
                result[passthrough] = item[passthrough]
        out.append(result)

    print(json.dumps(out, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
