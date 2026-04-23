# Ontogenesis Rebuild Plan — 2026-04-21

**Status:** HALTED 2026-04-21. 6/6 PMIDs in KNOWLEDGE.md fabricated (quarantined). Critical omission: Bethlehem 2022 Nature (PMID 35388223). British Birth Cohorts n=36,603 (was stated as 27,432). "UK Biobank pediatric arm" does not exist. No submission until rebuild complete.

---

## §1 User Strategic Decisions Required (BEFORE rebuild starts)

### 1.1 Publication target
| Option | Fit | Pros | Cons |
|--------|-----|------|------|
| **Review paper — *Aging Cell*** (RECOMMENDED for now) | Strong | IF ~7.5; accepts 0–25 developmental-aging framings | Must have real cohort data references |
| *GeroScience* review | Strong | IF ~6; geroscience-friendly | Competitive; needs novelty angle |
| **NIH R21 proposal** | Requires US collaborator | ~$275K/2y; exploratory | Needs US PI; 8-page narrative; not solo-feasible |
| **EIC Pathfinder** (parallel to CommonHealth main) | Already in flight as umbrella | Fits ecosystem | Cannot double-submit; must be a distinct WP |
| *Lancet Child & Adolescent Health* (perspective) | Clinical | High reach | Tight scope; rejection common |

**Audit recommendation: *Aging Cell* review paper first, then use published paper to anchor a later R21 / EIC sub-proposal.**

**USER DECISION NEEDED:** pick one primary target; others become downstream.

### 1.2 Platform vs theoretical framework
- **Option A — Theoretical framework only.** A conceptual 0–25 ontogenesis framework citing existing cohorts (ABCD, ALSPAC, Generation R, PedBE, DunedinPACE, BBC, Bethlehem 2022). No software deliverable. Fastest; cleanest; publishable.
- **Option B — Platform + framework.** Include the Rust/backend/frontend scaffolding currently in `~/Desktop/CommonHealth/Ontogenesis/` as an implementation. Much bigger scope; higher risk; needs 6+ months.

**Audit recommendation: Option A for Phase 1 (publication); Option B deferred to Phase 2 (after paper accepted).** Decouple theory from code. The Cargo/Rust scaffolding can be demoted to `experimental/` until theory is established.

### 1.3 AIM integration
`treatment_recommender` module does NOT exist in AIM per today's audit (checked against `~/Desktop/AIM/` module list). Previously claimed integration was fictional.

**USER DECISION NEEDED:** either
- (a) remove all AIM integration claims from Ontogenesis, or
- (b) first build a real `treatment_recommender.py` in AIM (separate 2-week subproject), then cite it truthfully.

Audit recommendation: **(a) remove for now**, reconsider (b) after Aging Cell paper accepted.

---

## §2 Rebuild Phases (4–6 weeks)

### Phase A — Evidence inventory + critical additions (Week 1)
- Replace every quarantined PMID with a verified equivalent (§3 table)
- ADD Bethlehem 2022 Nature (PMID 35388223) — this is the foundational brain-chart paper; its absence was the single worst audit finding
- Fix BBC n to 36,603 (verified via Shireby 2025 IJE)
- Remove all mentions of "UK Biobank pediatric arm"
- Output: new `KNOWLEDGE.md` (full verified PMID set)

### Phase B — Ethics section draft (Week 2)
See §4. ~2 pages, mandatory for any pediatric cohort framework.

### Phase C — Competitor landscape + positioning (Week 3)
Systematic cite of competing/adjacent frameworks (§5). Ontogenesis's distinctive contribution must be stated crisply in one paragraph versus each competitor.

### Phase D — Manuscript rewrite for *Aging Cell* (Weeks 4-5)
- Review article structure: Introduction → Current landscape → Gaps → Proposed 0–25 ontogenesis framework → Predictions → Implications → Ethics
- Self-citation ≤ 15%
- APA 7; US Letter; Calibri

### Phase E — Internal + external review (Week 5-6)
- Internal Claude peer-review loop until ACCEPT
- External reviewer from `~/Desktop/Claude/REVIEWERS.md` — NOT user's circle
- Incorporate every comment

### Phase F — Submission (Week 6)
Cover letter via DeepSeek draft → user edit → submit.

---

## §3 Critical PMID replacements (from audit)

| Claim | Old (FABRICATED) | New (VERIFY THESE) |
|-------|------------------|---------------------|
| Knickmeyer 2008 infant brain growth | 18971494 (fake) | **19020011** |
| Jaiswal 2017 clonal hematopoiesis / epigenetic age | 28792876 (fake) | **28636844** |
| Lifespan brain chart | *missing entirely* | **35388223** (Bethlehem 2022 Nature — ADD) |
| British Birth Cohorts n=36,603 | stated as 27,432 | **Shireby 2025 IJE** — cite for correct n |
| "UK Biobank pediatric arm" | non-existent | **DELETE all mentions** |

Verification protocol: open `https://pubmed.ncbi.nlm.nih.gov/<PMID>/` for each, copy authors/year/title into `KNOWLEDGE.md`, confirm abstract supports the claim.

---

## §4 Minimum ethics section (~2 pages to draft)

Mandatory frames for any pediatric ontogenesis platform / framework. Draft in this order:

1. **45 CFR 46 Subpart D — Additional Protections for Children (US)**
   - §46.404 minimal risk
   - §46.405 greater-than-minimal risk but prospect of direct benefit
   - §46.406 minimal increase over minimal risk
   - §46.408 assent of children + parental permission
   - State which subpart(s) our framework targets

2. **GDPR Art. 8 — Conditions applicable to child's consent** (EU)
   - Age threshold (16, or member-state 13–16)
   - Parental authorization
   - Verifiable consent mechanisms

3. **GDPR Art. 35 — Data Protection Impact Assessment (DPIA)**
   - Required whenever processing children's data at scale
   - Outline DPIA scope, lawful basis, risk mitigation
   - Appoint DPO if framework becomes implemented

4. **MDR Class IIa framework** (EU Medical Device Regulation)
   - If the framework outputs clinical-influence risk scores, Class IIa likely
   - Conformity assessment route
   - CE marking implications
   - Note: affects only implemented platform (Option B §1.2), not pure theory

Draft ~500 words per subsection, target ~2 pages total in final manuscript appendix.

---

## §5 Competitors to cite (for positioning paragraph)

| Competitor | Domain | Why cite |
|-----------|--------|----------|
| **ABCD Study** | US, 9–10y → 20y longitudinal; n≈11,880 | Largest active pediatric neuro-cohort; our framework must acknowledge and differentiate |
| **ALSPAC** (Children of the 90s) | UK Bristol; birth cohort | Deep phenotyping 0–25+; gold standard for long-arc ontogenesis |
| **Generation R** | Rotterdam; fetal-onset cohort | Prenatal + pediatric imaging; European counterpart to ABCD |
| **PedBE clock** | Pediatric buccal epigenetic age (Horvath-lineage) | Direct methodological competitor for pediatric biological age |
| **DunedinPACE** | Pace-of-aging epigenetic metric | Sets the bar for biological-age rate measures; our framework must cite and differentiate |

Positioning paragraph structure (one per competitor): "Framework X covers [scope]. Ontogenesis differs by [distinctive claim]. Ontogenesis complements X by [integration point]."

---

## §6 Risks and mitigations

| Risk | Probability | Mitigation |
|------|-------------|------------|
| Verified PMIDs do not support original claims | Medium-High | If >30% of claims lose support, downgrade from review paper to perspective/opinion (*Aging Cell* accepts both) |
| Ethics section reviewers find gaps | Medium | Have one bioethics external reviewer specifically; budget extra week |
| Competitor paper scoops 0–25 framework | Low-Medium | Accelerate — target submission Week 6; monitor bioRxiv weekly |
| Platform code (Option B) resurfaces and inflates scope | Medium | Keep Rust scaffolding in `experimental/` until theory paper accepted |
| User discovers more hallucinated facts | Low-Medium | Enforce verify_references rule for every PMID AND every numeric claim (n=, effect sizes, cohort sizes) |

**Do not submit until Phase E ACCEPT is recorded and every PMID + every cohort n has been verified against its primary source.**

---

## Phase 1 Status (2026-04-21)

- **6 fake PMIDs:** 6 replaced with verified equivalents (0 needing further search).
  - Knickmeyer 2008 → PMID 19020011 (J Neurosci, infant brain MRI 0–2yr) ✓
  - Jaiswal 2017 → PMID 28636844 (NEJM, clonal hematopoiesis) ✓
  - Lebel 2011 (childhood→adulthood white matter) → PMID 21795544 (J Neurosci) ✓
  - Juul 1997 IGF-1 pediatric reference (n=1430) → PMID 9253324 (JCEM) ✓
  - NK cell aging (was "NK cell at 70") → PMID 24998470 (Campos 2014, Immunol Lett) ✓
  - Frolkis 1999 → PMID 10394081 VERIFIED (was real, but needed claim/journal realignment to Gerontology 45:227–232) ✓
- **Bethlehem 2022 added:** ✓ PMID 35388223 (*Nature* 604:525–533, "Brain charts for the human lifespan")
- **Shireby 2025 BBC resource added:** ✓ PMID 40825593 (*Int J Epidemiol* 54(5):dyaf141)
- **CONCEPT §9.2:** BBC n fixed 27 432 → 36 603 (line 33 + §9.2 block); UK Biobank pediatric claim deleted (UK Biobank is adult-only) and replaced with All of Us pediatric (Aug 2024, ~1 600 kids 0–4) + ABCD Study (n=11 878) + ALSPAC + Generation R.
- **New file:** `KNOWLEDGE_REBUILD_2026-04-21.md` (does NOT restore quarantined v0).
- **Audit trail:** 8 PMIDs verified via PubMed esummary API; 0 DeepSeek calls (per `feedback_deepseek_no_citations`).

**Next action (Phase 2):** scope narrowing — move Rust/Cargo scaffolding (`backend/`, `frontend/`, `src/`, `target/`, `Cargo.toml`, `Cargo.lock`) into `experimental/` subfolder so the theory paper stays decoupled from code (per §1.2 Option A). Done next session.
