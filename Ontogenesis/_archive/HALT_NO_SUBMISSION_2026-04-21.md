# ⛔ Ontogenesis — SUBMISSION HALT (2026-04-21)

**Status:** 🔴 DO NOT SUBMIT to any journal, EIC Pathfinder, or external reviewer until fabricated PMIDs are fixed and Bethlehem 2022 is added.

## Why

Deep audit 2026-04-21 found:

1. **6/6 PMIDs in `KNOWLEDGE.md` are fabricated.** Specifically:
   - Knickmeyer 2008 cited as PMID 18971494 → actually a *C. difficile* paper (correct PMID: 19020011)
   - Jaiswal 2017 cited as PMID 28792876 → actually a NAD/niacin paper (correct PMID: 28636844)
   - Lebel 2012, Juul 1997, "NK cell at 70" — all resolve to unrelated PubMed records
2. **`CONCEPT.md §9.2` factual error:** British Birth Cohorts `n = 27,432` — actual figure is `n = 36,603` (Shireby et al. 2025 *IJE*).
3. **"UK Biobank pediatric arm" does not exist.** UKB is an adult cohort; no pediatric enrollment exists. All of Us began pediatric enrollment Aug 2024 (~1,600 kids 0-4 to date).
4. **Bethlehem 2022 *Nature* brain charts (PMID 35388223)** not cited — **disqualifying** omission for any neuroimaging-aware reviewer. Also missing: ABCD Study, ALSPAC, PedBE clock (McEwen 2020), DunedinPACE (Belsky 2020 / Elliott 2021), WHO MGRS methodology (de Onis 2004), CDC Extended BMI 2022.
5. **6-line ethics/regulatory section vastly insufficient** for a 0-25-year human cohort platform. Missing: 45 CFR 46 Subpart D (minors), GDPR Art. 8 (child consent 13-16), GDPR Art. 35 DPIA, RUO vs CDS boundary for AIM/treatment_recommender integration, FDA CDS 2022 + EU MDR 2017/745 Class IIa.
6. **EVIDENCE.md §1.1 confuses two different Frolkis papers** — cites wrong journal for PMID 10394081.
7. **Forced self-citations** (PMID 36583780, PMID 20480236) violate CLAUDE.md's "topically relevant" rule — neither concerns 0-25y developmental trajectories.

## Effect on EIC Pathfinder 2026-05-12

Ontogenesis is ❌ **NOT READY** for EIC Pathfinder 2026-05-12. Combined with the FCLC deferral (zero signed EU LoIs; budget inconsistent in 4 files), the umbrella CommonHealth EIC submission is formally deferred to 2027.

## Rebuild checklist

1. [ ] Fix 6 fabricated PMIDs in `KNOWLEDGE.md` using correct replacements:
   - Knickmeyer 2008 → PMID 19020011
   - Jaiswal 2017 → PMID 28636844
   - Lebel 2012, Juul 1997, "NK cell at 70" → quarantine, research, replace
2. [ ] Fix BBC cohort `n = 36,603` in `CONCEPT.md §9.2`
3. [ ] Delete "UK Biobank pediatric arm" claim — replace with "All of Us pediatric enrollment (Aug 2024, ~1,600 participants 0-4) and ABCD Study (adolescent, n=11,878)"
4. [ ] **Add Bethlehem 2022 *Nature* (PMID 35388223)** to KNOWLEDGE.md and CONCEPT brain-development section
5. [ ] Add ABCD Study, ALSPAC, PedBE clock, DunedinPACE, WHO MGRS, CDC Extended BMI
6. [ ] Fix EVIDENCE.md §1.1 Frolkis attribution
7. [ ] Remove forced self-citations (PMID 36583780, 20480236 — unrelated to 0-25y developmental topic)
8. [ ] Expand ethics section to ≥2 pages: 45 CFR 46 Subpart D, GDPR Art. 8 + 35, RUO/CDS classification, FDA CDS 2022, EU MDR 2017/745 Class IIa
9. [ ] External peer review (not internal) before any submission

## Governance

- **Audit file:** `~/Desktop/CommonHealth/Ontogenesis/DEEP_AUDIT_2026-04-21.md`
- **Halt effective:** 2026-04-21 06:45 Tbilisi
- **Halt lifted when:** KNOWLEDGE.md PMIDs all verified, Bethlehem 2022 added, ethics/regulatory section expanded, external peer review positive
- **Sister halt:** HAP (same DeepSeek hallucination pattern)
