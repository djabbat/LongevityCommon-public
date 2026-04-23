# HAP Rebuild Plan — 2026-04-21

**Status:** HALTED 2026-04-21. 10/10 PMIDs in EVIDENCE.md fabricated (quarantined as `EVIDENCE.md.QUARANTINED_2026-04-21`). Actual BHCA 9.5/27 (Class III), not claimed 20/27. No submission until rebuild complete.

---

## §1 User Strategic Decisions Required (BEFORE rebuild starts)

The user must decide these three before Phase A begins. Each decision constrains the entire rebuild.

### 1.1 Target venue
| Option | Word count | Fit | Pros | Cons |
|--------|-----------|-----|------|------|
| **BioEssays** (RECOMMENDED) | ~4,000 | Hypothesis/speculative | IF 3.2; welcomes bold theory; fast decisions | Requires crisp single hypothesis |
| *Medical Hypotheses* | ~2,000 | Short hypothesis | Low bar; fast | Very low IF (~1.3); weak for PhD/CV |
| *Biological Reviews* | 15,000–25,000 | Systematic review | IF 11.0; high prestige | Demands exhaustive lit coverage; 6+ months; BHCA must be Class I/II |

**Audit recommendation: BioEssays.** Matches current evidence depth realistically after rebuild; accepts provocative framings; single verified PMID set of ~40 refs is achievable in 8 weeks.

**USER DECISION NEEDED:** confirm BioEssays, or choose alternative.

### 1.2 Scope
- **Option A** — Keep "Hepato-Affective Primacy Theory" framing (liver → affect causal arrow). Narrow, distinctive, but literature thinner.
- **Option B** — Reframe as **gut–liver–brain axis** contribution with HAP as specific subclaim. Much wider verified literature (TGR5, bile-acid signalling, microbiome–liver–CNS). Safer for BHCA recovery; less distinctive.

**Audit recommendation: Option B framing, Option A headline.** Title invokes HAP; body embeds HAP within gut-liver-brain axis. Best of both.

### 1.3 Claims to save vs discard
From `DEEP_AUDIT_2026-04-21.md`:

**LIKELY SURVIVES AUDIT (keep):**
- Bile acid → TGR5 → CNS signalling exists (PMID 38518778 verified route)
- Liver disease ↔ depression comorbidity (PMID 36634820)
- Cross-species evidence for visceral → affective state (PMID 28554773, 21636277)

**DISCARD / DOWNGRADE:**
- "Liver is primary driver of affect" as strong causal claim — downgrade to "liver-origin signals are under-recognised contributors"
- Any numerical effect sizes drawn from quarantined PMIDs
- Claims about hepatic receptor density in limbic structures without a verified anatomy source

**USER DECISION NEEDED:** sign off on save/discard list above, or modify.

---

## §2 Rebuild Phases

### Phase A — Evidence inventory (Weeks 1-2)
- Enumerate every surviving claim in `THEORY.md` and `CONCEPT.md`
- For each claim → 2–3 verified PMIDs via PubMed (manual search; NEVER DeepSeek)
- Output: new `EVIDENCE.md` (replace quarantined) with claim → PMID → 1-sentence relevance → verified quote
- Gate: ≥ 80% of core claims have ≥ 2 verified PMIDs, else return to §1.3 and prune

### Phase B — BHCA recomputation (Weeks 3-4)
- Apply 9 Bradford-Hill criteria (0-3 each) against verified evidence only
- **Target: ≥ 18/27 (Class II).** If < 18, trigger §5 fallback
- Document score per criterion in new `BHCA_2026-XX-XX.md`
- Peer cross-check score with one external reviewer (not user's circle)

### Phase C — Manuscript rewrite (Weeks 5-6)
- Full rewrite targeting BioEssays structure: Abstract → Problem → Hypothesis → Evidence → Predictions → Falsification → Implications
- Self-citation ≤ 15% per CLAUDE.md rule
- APA 7; US Letter; Calibri; formulas as .png only

### Phase D — Internal + external review (Week 7)
- Internal peer-review loop (Claude) until ACCEPT verdict
- External reviewer from `~/Desktop/Claude/REVIEWERS.md` — NOT from user's close circle; ≥ 1 week turnaround
- Incorporate every reviewer comment (feedback_peer_review_until_accept rule)

### Phase E — Submission (Week 8)
- Cover letter via DeepSeek draft → user edit
- Submit via journal portal
- Record in `NEEDTOWRITE.md` as submitted; update MEMORY entry to remove "NOT YET PUBLISHED" flag only on acceptance

---

## §3 Priority PMIDs to verify first

These 5 external supporting papers are the audit's suggested starting anchors. Verify on PubMed that (a) PMID resolves, (b) title/authors/year match, (c) abstract supports the claim we intend to cite for.

| PMID | Reference | Supports claim |
|------|-----------|----------------|
| 38518778 | Li et al. 2024 — TGR5 signalling | Bile-acid receptor CNS route |
| 36634820 | Ntona et al. 2023 | NAFLD ↔ bile acids ↔ depression comorbidity |
| 28554773 | DopEcR Drosophila | Cross-species visceral→affect receptor |
| 24679535 | Anderson & Adolphs 2014 | Affective-state framework |
| 21636277 | Bateson et al. 2011 — bees | Invertebrate affect/judgement bias |

Verification protocol: open `https://pubmed.ncbi.nlm.nih.gov/<PMID>/`, copy abstract, paste into EVIDENCE.md with the quoted sentence supporting the HAP claim.

---

## §4 Concrete first-session tasks (3–5 h)

1. **User reads §1 and writes decisions into `REBUILD_DECISIONS.md`** (15 min)
2. **Verify all 5 PMIDs in §3** on PubMed; record verified titles/abstracts (45 min)
3. **Extract every factual claim from `THEORY.md` and `CONCEPT.md`** into a flat checklist (60 min)
4. **Tag each claim** SAVE / DOWNGRADE / DISCARD per §1.3 (45 min)
5. **Draft new `EVIDENCE.md` skeleton** with claim → expected PMID slots (60 min)
6. **Delete `.QUARANTINED` file's `.docx` mirrors** if any leaked into artefacts (15 min)

Session success criterion: new `EVIDENCE.md` skeleton committed; 5 anchor PMIDs verified and pasted; claim checklist ready for Phase A.

---

## §5 Risks and mitigations

| Risk | Probability | Mitigation |
|------|-------------|------------|
| BHCA still < 18 after verified evidence | Medium | Fallback to *Medical Hypotheses* (short format, accepts Class III); keep Biological Reviews off-table |
| Key claim (liver primacy) has no verified support at all | Medium-High | Reframe per §1.2 Option B as gut-liver-brain axis paper; HAP becomes internal subclaim |
| External reviewer rejects | Medium | Use feedback loop until ACCEPT (feedback_peer_review_until_accept); ≥ 2 review rounds budgeted |
| User discovers more hallucinated PMIDs mid-rebuild | Low-Medium | Enforce verify_references rule: EVERY PMID checked on PubMed before it enters EVIDENCE.md |
| Timeline slip past 8 weeks | Medium | Monthly MEMORY checkpoint; if > 10 weeks, freeze and reassess scope |
| Competing liver-brain axis paper published first | Low | Acceptable — our distinctive contribution is HAP framing, not priority on axis |

**Do not submit until Phase D ACCEPT is recorded.**

---

## Phase A Status (2026-04-21)

- **Anchor PMIDs verified: 5/5** via NCBI E-utilities esummary + efetch (JSON + text). All titles, authors, years, journals match the citations used in the plan. All abstracts genuinely support the HAP-relevant claim.
  - 38518778 — Li 2024 *Neuron* — TGR5 LHA-GABA→dCA3→DLS circuit regulates depressive-like behavior. **VERIFIED.**
  - 36634820 — Ntona 2023 *Neurochem Int* — NAFLD-metabolic-state → depression via bile acids / SCFA / LPS / monoamines. **VERIFIED.**
  - 28554773 — Lark/Kitamoto/Martin 2017 *BBA Mol Cell Res* — DopEcR dual ecdysone+dopamine receptor modulates Drosophila MB activity. **VERIFIED.**
  - 24679535 — Anderson & Adolphs 2014 *Cell* — cross-species emotion primitives framework. **VERIFIED.**
  - 21636277 — Bateson 2011 *Curr Biol* — agitated honeybees show pessimistic cognitive bias + hemolymph DA/5-HT/OA drop. **VERIFIED.**
- **Claims audited: 25 total.** SAVE = 8, DOWNGRADE = 7, DISCARD = 10. Details in `EVIDENCE_REBUILD_2026-04-21.md` §2.
- **Critical findings:**
  - CONCEPT v4.0 line "принята к публикации (Biological Reviews)" is FALSE — must be purged before any external use.
  - 56-taxa table and "100% correlation p<0.0001" have no verifiable sources — rebuild from citation-level taxon reports in Phase B.
  - BHCA subscores in CONCEPT v4.0 §5 are pre-audit and must be recomputed in Phase B against verified evidence only.
  - 4 inherited counterexamples (decapod crustaceans, C. elegans state-dependent behavior, planarian aversive learning, earthworm chloragogen + nociception) remain active liabilities; any one reconfirmed with verified PMIDs falsifies the biconditional form D1.
- **New artifact:** `~/Desktop/CommonHealth/HAP/EVIDENCE_REBUILD_2026-04-21.md` (Phase A draft; does NOT replace the quarantined file, which remains untouched).
- **Next action (Phase B kickoff):** execute the 10-item prioritized PubMed sweep in `EVIDENCE_REBUILD_2026-04-21.md` §4 (quantitative bile-acid BBB permeability; C. elegans Cermak 2020 verification; decapod crustacean nociception; liver-transplant affect literature; germ-free mouse behavior; FXR/TGR5 CNS atlases; ecdysone/insect affect beyond single paper; NAFLD↔depression meta-analyses; insect fat body as hepatic analog; cephalopod hepatopancreas neuroendocrinology). Gate for Phase B → C: ≥80% of SAVE/DOWNGRADE claims back-stopped by ≥2 verified PMIDs each, AND BHCA recomputation ≥18/27.
