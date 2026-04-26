#!/usr/bin/env python3
"""compare_all.py — run every counter simulator × every tissue, emit
a pairwise Δ matrix across all combinations.

Per updated rule (feedback_mcoa_cdata_comparison v2, 2026-04-21):
5 counter simulators (CDATA, Telomere, MitoROS, EpigeneticDrift,
Proteostasis) + MCOA coordinator. Every full run produces all 15
pairwise counter-vs-counter residuals across 6 tissues = 90 cells.

Usage
-----
    cd ~/Desktop/LongevityCommon/MCOA/scripts
    python3 compare_all.py --days 3650 --rate 0.01 \
        --out ../docs/comparisons/2026-04-21_full/

Requires: pandas, numpy. Rust counter binaries built via cargo.
"""
from __future__ import annotations
import argparse
import datetime as dt
import subprocess
import sys
from pathlib import Path

ROOT = Path.home() / 'Desktop' / 'LongevityCommon'

COUNTERS = [
    # (subproject,       crate_dir_name,           cli_bin,       number)
    ('CDATA',           'cell_dt_cli',            'cell-dt-sim',  1),
    ('Telomere',        'telomere_counter',       'telomere-sim', 2),
    ('MitoROS',         'mito_ros_counter',       'mito_ros-sim', 3),
    ('EpigeneticDrift', 'epigenetic_counter',     'epigenetic-sim',4),
    ('Proteostasis',    'proteostasis_counter',   'proteostasis-sim',5),
]
TISSUES = ['HSC','Fibroblast','Neuron','Cardiomyocyte','Hepatocyte','IntestinalCrypt']


def run_sim(sub: str, crate: str, bin_name: str, tissue: str, days: float, rate: float, out_csv: Path) -> bool:
    crate_dir = ROOT / sub / 'crates' / crate
    if not crate_dir.exists():
        sys.stderr.write(f'[warn] missing crate dir: {crate_dir}\n')
        return False
    try:
        r = subprocess.run(
            ['cargo','run','--release','--quiet','--bin', bin_name,'--',
             '--tissue', tissue, '--days', str(days), '--rate', str(rate)],
            cwd=crate_dir, capture_output=True, text=True, timeout=180,
        )
    except Exception as e:
        sys.stderr.write(f'[err] {sub}/{tissue}: {e}\n')
        return False
    if r.returncode != 0:
        sys.stderr.write(f'[err] {sub} cargo rc={r.returncode}: {r.stderr[:300]}\n')
        return False
    out_csv.write_text(r.stdout)
    return True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--days', type=float, default=3650.0)
    ap.add_argument('--rate', type=float, default=0.01)
    ap.add_argument('--out', default=None)
    args = ap.parse_args()

    out_dir = Path(args.out).expanduser().resolve() if args.out else \
        ROOT / 'MCOA' / 'docs' / 'comparisons' / f'{dt.date.today().isoformat()}_full'
    out_dir.mkdir(parents=True, exist_ok=True)
    print(f'[compare_all] output dir: {out_dir}')
    print(f'[compare_all] horizon: {args.days} days, rate: {args.rate}/day')

    runs = {}
    for sub, crate, bin_name, num in COUNTERS:
        for tissue in TISSUES:
            out_csv = out_dir / f'{sub.lower()}_{tissue}.csv'
            ok = run_sim(sub, crate, bin_name, tissue, args.days, args.rate, out_csv)
            if ok:
                runs[(sub, tissue)] = out_csv
                print(f'  ok  {sub}/{tissue}')
            else:
                print(f'  --  {sub}/{tissue}')

    try:
        import pandas as pd
        import numpy as np
    except ImportError:
        sys.stderr.write('pandas + numpy required\n')
        return 1

    subs = [s[0] for s in COUNTERS]
    rows = []
    for tissue in TISSUES:
        for i, s_i in enumerate(subs):
            for s_j in subs[i:]:
                p_i = runs.get((s_i, tissue))
                p_j = runs.get((s_j, tissue))
                if not p_i or not p_j:
                    rows.append({'tissue':tissue,'i':s_i,'j':s_j,'rms_delta':None,'note':'missing'})
                    continue
                di = pd.read_csv(p_i)
                dj = pd.read_csv(p_j)
                m = min(len(di), len(dj))
                if m == 0:
                    rows.append({'tissue':tissue,'i':s_i,'j':s_j,'rms_delta':None,'note':'empty'})
                    continue
                delta = (di['d'].values[:m] - dj['d'].values[:m])
                rms = float(np.sqrt(np.mean(delta**2)))
                rows.append({'tissue':tissue,'i':s_i,'j':s_j,'rms_delta':rms})
    df = pd.DataFrame(rows)
    csv_out = out_dir / 'matrix_rms_delta.csv'
    df.to_csv(csv_out, index=False)
    print(f'[compare_all] matrix → {csv_out}')

    total = len(TISSUES) * len(subs)
    success = sum(1 for v in runs.values() if v is not None)
    md = [
        f'# MCOA full comparison — {dt.date.today().isoformat()}',
        '',
        f'Horizon: {args.days} days · division rate: {args.rate} / day',
        '',
        f'Simulators run: {success}/{total} cells successful',
        '',
        '## Counters compared',
        '',
        '| # | Subproject | Binary |',
        '|---|-----------|--------|',
    ] + [f'| {n} | {s} | {b} |' for s, _, b, n in COUNTERS] + [
        '',
        '## Tissues',
        '',
        '- ' + '\n- '.join(TISSUES),
        '',
        '## Pairwise Δ (RMS over trajectory)',
        '',
        f'Full numerical matrix → `matrix_rms_delta.csv` ({len(rows)} rows: 6 tissues × 15 pairs including self).',
        '',
        '## Interpretation stub',
        '',
        '- [ ] Plot heatmap per tissue (matrix_rms_delta.csv → heatmap.png)',
        '- [ ] Per non-trivial Δ: assign label {numerical artefact | missing counter | real biological signal | bug}',
        '- [ ] Cross-validate against empirical datasets (Phase 6)',
        '',
    ]
    (out_dir / 'INTERPRETATION.md').write_text('\n'.join(md))
    print(f'[compare_all] stub → {out_dir}/INTERPRETATION.md')
    return 0


if __name__ == '__main__':
    sys.exit(main())
