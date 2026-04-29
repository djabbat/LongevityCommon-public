#!/usr/bin/env python3
"""
write_article.py — Poincaré Phase 3
Пишет статью «Poincaré and Ze-theory: Intuition as Ze-Stream» через DeepSeek.
"""
import sys, os
sys.path.insert(0, os.path.expanduser("~/Desktop/Claude/scripts"))
from llm import ask_llm, MODEL_SMART

BASE = os.path.dirname(os.path.abspath(__file__))

def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()

# ── Собираем корпус ───────────────────────────────────────────────────────────
concept   = read(f"{BASE}/CONCEPT.md")[:8000]
sources_2 = read(f"{BASE}/Sources/02_Intuitions_Clés.md")[:7000]
sources_3 = read(f"{BASE}/Sources/03_Science_et_Methode.md")[:5000]
sources_4 = read(f"{BASE}/Sources/04_Carte_des_Connexions.md")[:4000]

corpus = f"""
=== CONCEPT.md (v4, финальный — авторитетный документ) ===
{concept}

=== Sources/02 — 5 ключевых интуиций ===
{sources_2}

=== Sources/03 — Анализ «Science et Méthode» ===
{sources_3}

=== Sources/04 — Карта связей ===
{sources_4}
"""

# ── Промпт ────────────────────────────────────────────────────────────────────
PROMPT = f"""
You are writing a complete academic article in English.

Title: "Poincaré and Ze-theory: Intuition as Ze-Stream"
Target journal: Entropy / Longevity Horizon
Author: Jaba Tkemaladze

Write ALL 8 sections in full. Each section must be substantive (not a placeholder).
Use the source material below as the authoritative basis — do not invent facts.

STRUCTURE (write every section completely):

§1 INTRODUCTION
- Open with Poincaré's famous omnibus moment (Coutances, 1881) as a hook
- State the central question: not "what did Poincaré prove?" but "why did he see it?"
- Introduce Ze-theory hypothesis: intuition = optimal Ze-flow at v*
- State the paper's contribution: first formal application of Ze-theory to historical cognitive data

§2 METHODOLOGY
- Ze-theory parameters: v (Ze-velocity), v* (optimal state), τ (complexity/synthesis), T-burst (illumination event)
- Operationalization table: Ze-parameter → historical proxy → inference method
- Four validation criteria: successes / STR "failure" / analogy as bridge / negative cases
- Source strategy: primary texts over secondary historiography

§3 CASE STUDY 1 — AUTOMORPHIC FUNCTIONS (1881): THE PROTOTYPE ZE-FLOW
- Conscious preparation phase: months of failed attempts, correspondence with Klein
- Incubation: geological excursion to Coutances, conscious disengagement
- T-burst: stepping onto the omnibus — "I felt immediate certainty"
- Ze-interpretation: high v (forced analysis) → v* (incubation) → τ-max → T-burst
- Direct quotes from Science et Méthode

§4 CASE STUDY 2 — TOPOLOGY AND CHAOS (1890–1900): τ-TRANSFER ACROSS DOMAINS
- Celestial mechanics forced qualitative analysis → phase space as geometric universe
- Phase space concept = τ-transfer from mechanics to topology
- Analysis Situs (1895): algebraic topology born from need to classify phase space
- Iterative corrections (complements 1–4, 1899–1902) = τ accumulation toward the Conjecture
- Ze-interpretation: τ-transfer between domains via geometric structuralism

§5 CASE STUDY 3 — RELATIVITY (1905): WHERE ZE-FLOW STOPPED SHORT
- Poincaré had the complete mathematics: Lorentz group as rotations in 4D (June 1905)
- Einstein started from physical postulates; Poincaré from mathematical symmetry
- Ze-interpretation: conventionalist philosophy blocked the T-burst
- "Simultaneity is a convention" — Poincaré could not physically revise this
- Negative validation: Ze-flow reached v* for mathematical structure, not for physical reinterpretation

§6 ANALOGY AS Ze-BRIDGE: THE MECHANISM OF τ-TRANSFER
- Geological stratification → Fuchsian functions (τ from visual-spatial to analytic)
- Kaleidoscope → groups of transformations / Lorentz group (τ from physical optics to abstract algebra)
- Weaving / braiding → homoclinic tangle (τ from craft manipulation to phase space topology)
- Central claim: analogies are not illustrations — they are mechanisms of τ-transfer between domains

§7 NEGATIVE CASES: WHERE Ze-FLOW DID NOT COMPLETE
- Quantum theory: discontinuity was alien to Poincaré's geometric intuition (died 1912)
- Poincaré Conjecture (1904): intuition said "true"; Ze-flow accumulated τ through 4 error-filled complements; proof required Ricci flow (beyond his toolset)
- Significance: negative cases validate the Ze-model by showing where v* was not reached or T-burst was incomplete

§8 CONCLUSION
- Ze-theory as a formal model of mathematical intuition
- Three modes confirmed: τ-transfer via analogy, incubation→T-burst, conventionalism as Ze-blocker
- Broader implication: Ze-flow at v* = state of creative insight in any domain
- Limitations: retrospective inference, no direct neurophysiological measurement
- Future work: Ze-model applied to other historical cases (Ramanujan, Boltzmann); connection to predictive coding (Friston 2010)

SOURCE MATERIAL (use this as ground truth):
{corpus}

REQUIREMENTS:
- Full academic prose, not bullet points (except in methodology table)
- 4000–6000 words total
- Include direct quotes from Poincaré where available in the sources
- Maintain Ze-theory terminology consistently (v, v*, τ, T-burst, S-dominance)
- Do not add unsourced claims
"""

print("📤 Отправляю в DeepSeek Reasoner...")
print("   (deepseek-reasoner, max_tokens=8000 — займёт 1–3 минуты)\n")

article = ask_llm(
    PROMPT,
    system="You are an expert academic writer specializing in philosophy of mathematics and cognitive science.",
    model=MODEL_SMART,
    max_tokens=8000,
    temperature=0.4,
)

# ── Сохраняем ─────────────────────────────────────────────────────────────────
out_path = f"{BASE}/Poincare_Ze_Article_v1.md"
with open(out_path, "w", encoding="utf-8") as f:
    f.write("# Poincaré and Ze-theory: Intuition as Ze-Stream\n\n")
    f.write("**Author:** Jaba Tkemaladze  \n")
    f.write("**Version:** v1 (DeepSeek draft, 2026-04-03)  \n")
    f.write("**Status:** Draft — needs peer review  \n\n---\n\n")
    f.write(article)

print(f"✅ Статья сохранена: {out_path}")
print(f"   Размер: {len(article)} символов / ~{len(article.split())} слов")
