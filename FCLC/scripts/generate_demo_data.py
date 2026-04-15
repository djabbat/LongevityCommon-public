#!/usr/bin/env python3
"""
FCLC Demo Data Generator

Generates synthetic OMOP-compatible CSV records for testing FCLC without
real patient data. Uses seeded random to produce reproducible datasets.

Usage:
    python3 generate_demo_data.py --nodes 3 --records 500 --seed 42
    python3 generate_demo_data.py --out data/

Output: data/clinic_node{N}_demo.csv for each node

CSV format (FCLC OmopRecord):
    age_years, sex, diabetes_year, hba1c, bmi, has_nephropathy,
    has_retinopathy, hospitalized_last_12m, hospitalized_next_12m
"""

import argparse
import csv
import math
import os
import random
import sys


CURRENT_YEAR = 2026

# Realistic distribution parameters (from CDC/NHANES 2017-2020)
AGE_DISTRIBUTION = [
    (30, 0.10), (35, 0.10), (40, 0.12), (45, 0.13),
    (50, 0.14), (55, 0.13), (60, 0.12), (65, 0.10),
    (70, 0.07), (75, 0.05), (80, 0.04),
]
SEX_DISTRIBUTION = [("M", 0.48), ("F", 0.52)]

# HbA1c distribution (diabetic cohort): mean~8.0, SD~1.5
HBA1C_MEAN = 8.0
HBA1C_SD = 1.5
HBA1C_MIN = 5.5   # threshold for pre-diabetes
HBA1C_MAX = 14.0  # clinical maximum

# BMI distribution (diabetic cohort): mean~32, SD~6
BMI_MEAN = 32.0
BMI_SD = 6.0
BMI_MIN = 18.0
BMI_MAX = 55.0

# Complication rates (rough estimates for T2DM cohort)
# Vary per node to simulate different clinic populations
NEPHROPATHY_BASE_RATE = 0.22   # 22% of diabetics (KDIGO 2020)
RETINOPATHY_BASE_RATE = 0.28   # 28% of diabetics (AAO 2020)
HOSP_LAST_RATE = 0.18          # 18% hospitalized in last 12m
HOSP_NEXT_RATE = 0.15          # 15% outcome (prediction target)

# Correlation structure: complications increase hospitalization risk
NEPHROPATHY_HOSP_OR = 2.5   # odds ratio for hospitalization
RETINOPATHY_HOSP_OR = 1.8
AGE_HOSP_SLOPE = 0.012       # per year of age


def weighted_choice(rng, choices):
    """Choose from (value, weight) list."""
    values, weights = zip(*choices)
    total = sum(weights)
    r = rng.random() * total
    cumulative = 0.0
    for v, w in zip(values, weights):
        cumulative += w
        if r <= cumulative:
            return v
    return values[-1]


def clipped_normal(rng, mean, sd, min_val, max_val):
    """Sample from Normal, clip to [min_val, max_val]."""
    v = rng.normalvariate(mean, sd)
    return max(min_val, min(max_val, v))


def generate_record(rng, node_idx, record_idx):
    """Generate one synthetic OmopRecord."""
    # Demographics
    age = weighted_choice(rng, AGE_DISTRIBUTION) + rng.randint(-2, 2)
    age = max(18, min(90, age))
    sex = weighted_choice(rng, SEX_DISTRIBUTION)

    # Diabetes diagnosis year (1-40 years ago, weighted toward more recent)
    years_since_dx = int(rng.expovariate(0.12)) + 1
    years_since_dx = min(years_since_dx, 40)
    diabetes_year = CURRENT_YEAR - years_since_dx

    # HbA1c — node-specific population variation
    node_hba1c_shift = (node_idx - 1) * 0.4  # node 1: -0.4, node 2: 0, node 3: +0.4
    hba1c = clipped_normal(rng, HBA1C_MEAN + node_hba1c_shift, HBA1C_SD, HBA1C_MIN, HBA1C_MAX)
    hba1c = round(hba1c, 1)

    # BMI
    bmi = clipped_normal(rng, BMI_MEAN, BMI_SD, BMI_MIN, BMI_MAX)
    bmi = round(bmi, 1)

    # Complications (node-specific rates simulate different clinic populations)
    node_risk_multiplier = [0.8, 1.0, 1.3][node_idx % 3]  # node 3 = sicker population
    nephropathy = int(rng.random() < NEPHROPATHY_BASE_RATE * node_risk_multiplier)
    retinopathy = int(rng.random() < RETINOPATHY_BASE_RATE * node_risk_multiplier)

    # Hospitalization in last 12 months
    hosp_last = int(rng.random() < HOSP_LAST_RATE * node_risk_multiplier)

    # Outcome: hospitalization in next 12 months (prediction target)
    # Logistic model: age, nephropathy, retinopathy, HbA1c, hosp_last all increase risk
    log_odds = (
        -3.5
        + AGE_HOSP_SLOPE * age
        + math.log(NEPHROPATHY_HOSP_OR) * nephropathy
        + math.log(RETINOPATHY_HOSP_OR) * retinopathy
        + 0.15 * (hba1c - 7.0)      # higher HbA1c → more risk
        + 0.8 * hosp_last            # prior hospitalization → strong predictor
        + 0.1 * (node_idx - 1)       # node-specific baseline
    )
    prob_hosp_next = 1.0 / (1.0 + math.exp(-log_odds))
    hosp_next = int(rng.random() < prob_hosp_next)

    return {
        "age_years": age,
        "sex": sex,
        "diabetes_year": diabetes_year,
        "hba1c": hba1c,
        "bmi": bmi,
        "has_nephropathy": nephropathy,
        "has_retinopathy": retinopathy,
        "hospitalized_last_12m": hosp_last,
        "hospitalized_next_12m": hosp_next,
    }


FIELDS = [
    "age_years", "sex", "diabetes_year", "hba1c", "bmi",
    "has_nephropathy", "has_retinopathy",
    "hospitalized_last_12m", "hospitalized_next_12m",
]


def generate_node_data(node_idx, n_records, seed):
    """Generate n_records for a single clinic node."""
    node_seed = seed + node_idx * 31337
    rng = random.Random(node_seed)
    records = [generate_record(rng, node_idx, i) for i in range(n_records)]
    return records


def write_csv(path, records):
    """Write records to CSV."""
    os.makedirs(os.path.dirname(path) if os.path.dirname(path) else ".", exist_ok=True)
    with open(path, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(records)
    print(f"  ✓ {path} ({len(records)} records)")


def print_stats(node_idx, records):
    """Print summary statistics for a generated dataset."""
    n = len(records)
    hosp_rate = sum(r["hospitalized_next_12m"] for r in records) / n
    neph_rate = sum(r["has_nephropathy"] for r in records) / n
    ret_rate = sum(r["has_retinopathy"] for r in records) / n
    mean_hba1c = sum(r["hba1c"] for r in records) / n
    mean_age = sum(r["age_years"] for r in records) / n
    sex_m = sum(1 for r in records if r["sex"] == "M") / n
    print(f"  Node {node_idx}: n={n}, "
          f"age={mean_age:.1f}, "
          f"M={sex_m:.0%}, "
          f"HbA1c={mean_hba1c:.1f}, "
          f"Neph={neph_rate:.0%}, "
          f"Ret={ret_rate:.0%}, "
          f"Hosp(outcome)={hosp_rate:.0%}")


def main():
    parser = argparse.ArgumentParser(description="FCLC Demo Data Generator")
    parser.add_argument("--nodes", type=int, default=3, help="Number of clinic nodes (default: 3)")
    parser.add_argument("--records", type=int, default=500, help="Records per node (default: 500)")
    parser.add_argument("--seed", type=int, default=42, help="Random seed (default: 42)")
    parser.add_argument("--out", type=str, default="data", help="Output directory (default: data/)")
    args = parser.parse_args()

    print(f"\nFCLC Demo Data Generator")
    print(f"  Nodes: {args.nodes} | Records/node: {args.records} | Seed: {args.seed}")
    print(f"  Output: {args.out}/\n")

    for node_idx in range(1, args.nodes + 1):
        records = generate_node_data(node_idx, args.records, args.seed)
        path = os.path.join(args.out, f"clinic_node{node_idx}_demo.csv")
        write_csv(path, records)
        print_stats(node_idx, records)

    print(f"\n✓ Done. Import these CSV files into FCLC Node (Data tab → CSV path).")
    print(f"  Use 'bash run.sh node' to open the FCLC Node GUI.\n")


if __name__ == "__main__":
    main()
