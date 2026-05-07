#!/bin/bash
# seed.sh — minimal seed data for dev/staging:
#   1 system user + 10 fake studies. Idempotent (ON CONFLICT DO NOTHING).
#
# Per Phase 4.2 of REMEDIATION_ROADMAP_2026-05-07.md.

set -euo pipefail

DB_URL="${DATABASE_URL:-}"
if [ -z "$DB_URL" ] && [ -f /etc/aim/lc_social.env ]; then
    # shellcheck disable=SC1091
    . /etc/aim/lc_social.env
    DB_URL="${DATABASE_URL:-}"
fi
if [ -z "$DB_URL" ]; then
    echo "FAIL: DATABASE_URL not set." >&2
    exit 1
fi

echo "Seeding into ${DB_URL//:*@/:****@}..."

# Idempotency: skip studies-insert when seed-marked rows already exist.
# We mark seeded studies with creator_id = system user UUID so we can
# detect prior runs. If we already have ≥10 system-authored studies, skip.
existing=$(psql "$DB_URL" -tAc \
    "SELECT COUNT(*) FROM studies \
     WHERE creator_id = '00000000-0000-0000-0000-000000000001'::uuid" \
    | tr -d ' ')
if [ "$existing" -ge 10 ]; then
    echo "  ✓ already seeded ($existing system-authored studies); skipping"
    exit 0
fi

psql "$DB_URL" -v ON_ERROR_STOP=1 <<'SQL'
-- System user (fixed UUID so seeds are reproducible)
INSERT INTO users (id, username, email, created_at)
VALUES ('00000000-0000-0000-0000-000000000001',
        'longevitycommon-system',
        'system@longevitycommon.test',
        NOW())
ON CONFLICT (id) DO NOTHING;

-- 10 studies authored by the system user.
INSERT INTO studies (id, creator_id, title, hypothesis, protocol, target_n,
                      duration_days, status)
SELECT gen_random_uuid(),
       '00000000-0000-0000-0000-000000000001'::uuid,
       v.title, v.hyp,
       jsonb_build_object('description', v.descr,
                          'tracks',      v.tracks),
       v.target_n,
       v.duration,
       'draft'
FROM (VALUES
  ('CDATA Phase 0 (centriolar dynamics in HSC)',
   'Centriolar polyGlu accumulates monotonically with division count, predicting Hayflick limit.',
   'Single-cell live imaging of centriolar PTM during HSC division.',
   'CDATA',  60,  180),
  ('Telomere x EpigeneticDrift coupling (Cuban EEG cohort)',
   'Γ_{2,4} > 0.1 in HSC-like progenitors of Cuban N=88.',
   'Cross-counter coupling Γ_{2,4} measurement.',
   'Telomere,EpigeneticDrift', 88, 120),
  ('MitoROS heteroplasmy clonal expansion timeline',
   'Clonal expansion rate β_3 > 0.05/yr in skeletal muscle post-50.',
   'Single-cell mtDNA seq across age strata.',
   'MitoROS', 100, 365),
  ('Proteostasis collapse in cortex aging',
   'D_5 trajectory inflects sharply post-65 in cortex but not striatum.',
   'Aggregate-protein FRET in cortical neurons.',
   'Proteostasis', 80, 270),
  ('χ_Ze biomarker calibration N=2000',
   'Pre-registered weights generalise (R² > 0.4) to held-out N≥500.',
   'Pre-registered cohort to validate post-hoc χ_Ze weights.',
   'BioSense', 2000, 365),
  ('Ze·Profile vs DunedinPACE comparison',
   'Ze·Profile correlates ρ > 0.6 with DunedinPACE in N=200 longitudinal.',
   'Cross-clock validation of Ze velocity-based aging biomarker.',
   'BioSense,EpigeneticDrift', 200, 730),
  ('FCLC federated calibration of MCOA tissue weights',
   'Federated weights converge within ε=0.05 of pooled estimate at q=0.013, T=5.',
   'Distributed calibration of w_i(tissue) without raw-data egress.',
   'FCLC,MCOA', 5, 90),
  ('AutomatedMicroscopy night-shift accuracy validation',
   'Cohen κ > 0.8 between AI and human focus/ROI decisions.',
   'Concordance of Claude-Code night-shift decisions vs human technician.',
   'AutomatedMicroscopy', 30, 60),
  ('Aqtivirebuli (T. aestivum heritage) longevity correlate',
   'Daily Korkoti consumption associates with +0.05 χ_Ze (FDR q<0.10).',
   'Whole-grain Korkoti consumption vs χ_Ze in Kvemo Kartli cohort.',
   'Aqtivirebuli,BioSense', 300, 365),
  ('Patient-Activation (PAM-13) vs χ_Ze trajectory pilot',
   'PAM-13 trajectory ≥ MCID 5.4 correlates with χ_Ze improvement.',
   'AIM-driven L3 (Patient-Project) test in n=30 over 3 months.',
   'AIM,BioSense', 30, 90)
) AS v(title, hyp, descr, tracks, target_n, duration)
ON CONFLICT DO NOTHING;
SQL

count=$(psql "$DB_URL" -tAc 'SELECT COUNT(*) FROM studies' | tr -d ' ')
echo "  ✓ studies in DB now: $count"
