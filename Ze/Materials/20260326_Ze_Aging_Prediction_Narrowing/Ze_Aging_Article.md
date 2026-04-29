**Title:** Aging as Informational Closure: A Predictive Processing Perspective on Progressive Adaptation Narrowing

**Author:** Jaba Tkemaladze
**Affiliation:** Research Laboratory for Integrative Medicine, Phasis Academy, Tbilisi, Georgia.
**Correspondence:** jaba@longevity.ge

**Keywords:** Aging, Predictive Coding, Active Inference, Allostasis, Systems Biology, Information Theory, Hormesis, Senescence, Conceptual Framework, Computational Modeling.

---

### 1. Abstract

Contemporary geroscience provides a detailed catalog of aging phenotypes but lacks an integrative, systems-level framework explaining their coordinated progression. This paper proposes a novel, hypothesis-generating perspective: biological aging can be usefully conceptualized as a process of **progressive adaptation narrowing** linked to dysregulated predictive homeostasis. We conceptualize an organism as a hierarchical predictive system managing a fundamental trade-off between the cost of updating its internal model and the penalty for bearing prediction errors. We introduce a core **postulate**: that under the constraints of accumulating stochastic damage and finite resources, aging systems exhibit a trend toward elevated thresholds for registering and responding to prediction errors. This reduces short-term updating costs but renders the system progressively less sensitive to novel signals, potentially leading to a drift away from optimal homeostasis. We present a **simplified conceptual model** to illustrate this trade-off. From this framework, we generate unique, testable predictions distinct from those of existing theories. We then explore how established hallmarks of aging could be *reinterpreted through this lens*, offering high-level, speculative analogies intended to generate novel research questions. The perspective is rigorously positioned against major evolutionary and computational theories of aging. We discuss hormesis as a potential countermeasure that may work by forcing model updates. Finally, we outline a critical research program for validation. This perspective aims to provide an integrative conceptual scaffold for investigating aging as a process of declining adaptive fidelity and informational openness.

---

### 2. Introduction

The hallmarks of aging describe the molecular and cellular deteriorations of senescence (López-Otín et al., 2013). While phenomenologically essential, they constitute a parts list, leaving open the question of an integrative, system-level principle for their coordination. Evolutionary theories like the disposable soma (Kirkwood, 1977; Kirkwood & Austad, 2000) and antagonistic pleiotropy (Williams, 1957) explain *why* aging exists, while mechanistic theories like hyperfunction (Gems & de la Guardia, 2013) propose *how* specific pathways drive decline. The allostatic load model describes the cumulative physiological cost of adaptation (McEwen, 1998; Sterling, 2012). Concurrently, systems-level approaches, such as network theory of aging (Kowald & Kirkwood, 1996; Xue et al., 2007; Barabási, 2016), information-theoretic perspectives (Shannon, 1948; Demetrius, 2001; Gavrilov & Gavrilova, 1991; Mitteldorf, 2004; Sokhansanj et al., 2022; Milholland et al., 2017; Zhavoronkov & Moskalev, 2020; Adami, 2002), and theories of resilience and complexity loss (Kauffman, 1993; Yashin et al., 2012; Cohen et al., 2022; Franceschi et al., 2000), frame aging as a loss of regulatory capacity and information. However, a formal, integrative principle linking information processing, adaptive decision-making, and systemic decline across scales from molecules to behavior is still nascent.

We propose a perspective grounded in **predictive processing/active inference** (Friston, 2010; Clark, 2013; Pezzulo et al., 2015), a framework where organisms act as active inferential systems minimizing long-term prediction error (variational free energy). We suggest a potential failure mode in this process underlies senescence. Specifically, we posit that a useful way to view aging is through the lens of a system's changing management of the trade-off between the cost of updating its internal model and the cost of bearing prediction errors. We **postulate** that a myopically optimal, but ultimately pathological, solution to accumulating stress and finite resources is a progressive elevation of the detection threshold for prediction errors. This trend could lead to **informational closure**, defined here as a measurable decline in the mutual information between the system's predictive model and its environment.

This **Adaptive Threshold Elevation Framework** makes distinct, testable claims: (1) Aging is characterized by a quantifiable increase in the stimulus intensity required to trigger adaptive updates across multiple physiological and cognitive systems. (2) This elevation may contribute to a systemic property of declining informational openness. (3) It offers a novel interpretation for the loss of resilience and increased rigidity observed in aging, framing it not merely as damage accumulation, but as a strategic degradation of signal detection fidelity.

This paper is structured as follows: First, we present a simplified conceptual model to illustrate the update-versus-error trade-off and introduce the central **postulate** of threshold elevation as a testable pattern. Second, we describe a resulting positive feedback loop—the "narrowing cycle"—and explicitly discuss the causal ambiguity between active strategy and passive degradation. Third, we outline novel, testable predictions generated by the framework. Fourth, we explore speculative analogies, synthesizing the perspective with aging hallmarks as high-level interpretations intended to generate new questions. Fifth, we rigorously position the framework against established evolutionary and computational theories. Sixth, we reinterpret hormesis through this lens and discuss complexities like desensitization. We conclude with a discussion of limitations and a focused research agenda.

---

### 3. A Conceptual Model: Illustrating a Fundamental Trade-off

To clarify the proposed trade-off, we present a simplified, illustrative model. This model serves *only* to intuitively frame the postulated trade-off; it is not a realistic model of biological inference, a derivation from first principles, nor a source of proof. The core postulate is introduced as the foundational, testable assumption of the framework.

**Model Setup: Illustrating a Decision Problem**
Consider a biological subsystem at a discrete time *t*. It observes a prediction error δ_t (the mismatch between its internal model's prediction and sensory input). The error is drawn from a distribution with variance σ_δ², which may increase over time due to accumulating stochastic damage. The system must choose an action *a_t*: either **ignore** the error or **update** its internal model.

**A Simple Cost Function for Illustration:**
To illustrate a trade-off, we define an *immediate expected cost* of taking action *a* in response to an observed error δ:
*C(δ, a) = c_e ⋅ δ ⋅ I(a = ignore) + [c_u + c_e ⋅ δ_{res}] ⋅ I(a = update)*
where:
- *c_e* is a constant scaling the penalty per unit of sustained error.
- *c_u* is the **cost of updating** the model. *c_u* may increase with age due to factors like molecular damage, cellular senescence, or declining resource availability.
- *δ_{res} ≥ 0* is a small, fixed residual error that remains even after an update (acknowledging imperfect corrections).
- *I(⋅)* is the indicator function.

**A Threshold Policy:**
For illustration, the system employs a simple threshold policy parameterized by θ. It updates if the observed error magnitude |δ_t| ≥ θ; otherwise, it ignores.

**Core Postulate of Threshold Elevation (The Testable Pattern):**
We introduce the following as the central, testable **postulate** of the framework:
*We hypothesize that aging systems exhibit a measurable increase in detection thresholds (θ) for triggering adaptive updates across multiple scales. This pattern may arise from, and contribute to, increasing update costs (c_u) and/or increasing variance of prediction errors (σ_δ²).*

**Conceptual Rationale:**
The rationale is intuitive: A higher *c_u* makes the "update" action more expensive. A system minimizing immediate costs could, in principle, raise θ to avoid this expense, accepting more frequent, smaller sustained errors. This is a short-sighted strategy; it minimizes cost now while allowing the internal model to drift into inaccuracy, increasing future errors. This model illustrates that trade-off, but the postulate itself—the observable pattern of rising thresholds—is what requires empirical validation.

**Simulation Illustration (Purely Conceptual):**
A simple computational simulation (see **Supplementary Appendix A**) where *c_u* and σ_δ² increase linearly over time can illustrate this dynamic. An agent that adjusts θ to minimize the one-step expected cost will show a monotonic increase in θ(t), as depicted in Figure 1. **This model is a toy illustration of a cost-benefit trade-off, not a simulation of biological aging.**

**Figure 1 (Conceptual Description):** Schematic output of an illustrative simulation. Over simulated time steps, as *c_u* (blue line) and error variance σ_δ² increase, a myopically optimized threshold θ* (red line) rises. The average magnitude of unregistered errors (δ < θ*, green dashed line) initially increases due to the raised threshold, but later, the absolute model error grows as the outdated model fails.

**Conclusion from Conceptual Framework:**
This illustrative model defines a trade-off and provides a conceptual scaffold for the **core postulate**. The subsequent framework explores the systemic implications *if* this postulated pattern of threshold elevation is observed in biological systems.

**Proposed System-Level Implications:**
*   **Reduced Model Updating:** Fewer events exceed θ, leading to less frequent updates.
*   **Predictive Drift & Chronic Stress:** The internal model becomes obsolete, increasing the *objective* prediction error δ. This unresolved mismatch could manifest as chronic, low-grade stress.
*   **Loss of Granularity:** The system may become less able to distinguish significant signals from noise, reducing adaptive precision.

---

### 4. The Narrowing Cycle and the Causality Challenge

Integrating the core postulate, we describe a dynamic, positive feedback cycle that could, in principle, drive systemic decline—the "Narrowing Cycle" (Figure 2). A critical ambiguity must be addressed head-on: is threshold elevation an active, regulated strategic failure, or a passive epiphenomenon of a degraded system?

**Figure 2: Causal Loop Diagram of the Hypothesized Narrowing Cycle.**
[Diagram would show: (R1) θ ↑ → Model Updates ↓ → Model Error (δ) ↑ → Physiological Stress/Damage ↑ → Update Cost (c_u) ↑ → θ ↑. (R2) θ ↑ → Model Updates ↓ → Model Error (δ) ↑ → Error Variance (σ_δ²) ↑ → θ ↑. Reinforcing loops R1 and R2 create a potential cycle of progressive narrowing.]

1.  **Initiation:** Lifelong exposure generates a stream of prediction errors (δ): molecular damage, pathogens, and environmental shifts.
2.  **Threshold Elevation (Postulate):** Due to rising update costs (*c_u ↑*) and/or a noisier error distribution (σ_δ² ↑), the system exhibits elevated detection thresholds θ.
3.  **Increase in Unregistered Error & Predictive Drift:** The elevated θ causes fewer errors to trigger model updates. The internal model may become increasingly inaccurate.
4.  **Manifestation as Chronic Stress & Hallmark Accumulation:** This unresolved mismatch could manifest as low-grade, sub-threshold stress. The hallmarks of aging may accumulate as *consequences* of the model's growing inaccuracy.
5.  **Further Cost Increase & Potential Cycle Closure:** This chronic stress and accumulated damage could further increase the baseline cost of cellular operations and model updating (*c_u ↑↑*) and likely increase error variance (σ_δ² ↑), creating conditions for further threshold elevation.

**The Central Causality Challenge:**
This cycle describes a potential reinforcement loop, but it does not resolve primacy. It is equally plausible that primary molecular damage (e.g., DNA mutations, ROS) directly increases *c_u* and σ_δ², and the observed "threshold elevation" is a passive *epiphenomenon* of a degraded system (e.g., fewer receptors, impaired signaling cascades) rather than an active, strategic policy shift. **The framework does not yet resolve this.** Therefore, a major challenge is to design experiments that distinguish between:
*   **Active Policy Shift:** A regulated change in sensitivity mediated by conserved signaling nodes (e.g., specific kinases, nutrient sensors) that modulate θ-like parameters in a coordinated fashion.
*   **Passive Degradation:** A loss of sensitivity arising directly from the failure of molecular components, with no overarching regulatory intent.

The predictive power of the framework hinges on validating the former—that there exists a coherent, system-wide sensitivity "setting" that can be strategically, and pathologically, altered. This will be a central focus of the proposed research agenda (Section 9).

---

### 5. Novel, Testable Predictions

For the framework to be useful, it must generate unique predictions that differentiate it from existing theories. The following are novel and falsifiable:

1.  **Cross-System Correlation of Sensitivity Decline:** The framework predicts that diverse measures of reduced physiological and cognitive sensitivity (e.g., elevated sensory thresholds, blunted heart rate variability reactivity to stress, dampened immune cell cytokine production to standard stimuli, increased cognitive rigidity in reversal learning) will be positively correlated within individuals, more so than expected by chance or damage alone. This pattern could be modeled as a latent variable (e.g., using structural equation modeling on normalized Z-scores of reactivity measures). We predict this latent "threshold elevation factor" will be a stronger predictor of future healthspan decline and mortality than chronological age or single-system biomarkers. **Falsification:** If no such latent factor exists, or if it is not predictive of decline.

2.  **Manipulation of Sensitivity Nodes Alters Aging Trajectories Distinctly:** Interventions that increase the sensitivity (effectively lower θ) of key stress-response pathways in mid-life model systems should produce coordinated, youth-like effects *distinct from simply reducing damage*.
    *   *Operationalized Test:* Genetic or pharmacological upregulation of a sensitivity node like NRF2 in mid-life *Drosophila* should: (a) lower the biochemical threshold (e.g., EC50) for activating antioxidant target genes in response to oxidative stress, and (b) lead to downstream effects consistent with slowed narrowing, such as improved proteostasis and extended healthspan. A key **unique** behavioral correlate would be **preserved behavioral flexibility** in associative learning or reversal learning tasks, which directly tests the system's capacity for model updating. **Falsification:** If enhancing sensitivity improves damage markers but does not preserve behavioral/cognitive flexibility or alter other system-wide sensitivity measures.

3.  **Update Cost Biomarkers Predict Sensitivity State:** Biomarkers theorized to reflect high cellular update cost (e.g., low NAD+/NADH ratio, high senescent cell burden, low free chaperone availability) will be inversely correlated with *in vivo* measures of physiological reactivity (from prediction #1) in cross-sectional and longitudinal studies. The framework predicts that high "cost" physiologically precedes or accompanies low "sensitivity." **Falsification:** If high update cost markers are associated with *increased* or unchanged physiological reactivity.

4.  **Hormesis Modulates Sensitivity Setpoints:** The efficacy of a hormetic intervention will correlate with its ability to acutely *increase* physiological sensitivity (lower a measured threshold), and chronic application will lead to a sustained downward shift in the threshold's setpoint.
    *   *Conceptual Test:* A longitudinal exercise intervention study could test if the dose of training required to elicit a standard heart rate variability (HRV) reactivity response decreases over time (suggesting a lowered θ_autonomic). This change should correlate with improvements in cellular update cost markers (e.g., mitochondrial function). **Falsification:** If hormetic benefits occur without any measurable change in physiological sensitivity thresholds.

---

### 6. Speculative Implications: Reinterpreting Hallmarks through the Framework

For the framework to be generative, it should offer novel perspectives on known phenomena. The following are presented as **speculative analogies and high-level interpretations** intended to inspire new research questions. **They are not proposed as specific mechanistic truths.** The biological reality is vastly more complex, involving multiple, nonlinear feedback loops. These analogies are dramatic simplifications for conceptual clarity.

**Table 1: Hallmarks as Potential Manifestations of Dysregulated Sensitivity: Interpretive Analogies and Generative Questions**

| **Hallmark / Process** | **Interpretive Analogy within the Predictive Narrowing Framework** | **Generative Research Question** |
| :--- | :--- | :--- |
| **Genomic Instability & Cellular Replicative Limits** | Accumulating DNA damage and the approach to the Hayflick limit (Hayflick & Moorhead, 1961) could be *consistent with* an elevated threshold (θ_DDR/θ_Repair) for triggering full repair or senescence pathways, allowing low-level damage to persist. | Does the dose-response relationship for DDR activation (e.g., γH2AX foci formation) or the telomere length threshold for senescence entry shift with replicative age? |
| **Epigenetic Alterations** | Epigenetic drift could *functionally resemble* an elevated transcriptional threshold (θ_Transcriptional) at stress-response gene promoters, potentially reducing sensitivity to activation signals. **Caveat:** This is a high-level analogy; the molecular implementation is complex and indirect. | Do aged cells require a stronger oxidative stress signal to induce a standard NRF2 transcriptional response, and is this correlated with specific promoter chromatin modifications? |
| **Loss of Proteostasis** | Proteostasis collapse could be *interpreted as* resulting from a raised effective threshold (θ_UPR) for activating the unfolded protein response (UPR), allowing misfolded proteins to accumulate. | Is the misfolded protein load required to trigger a robust UPR (e.g., XBP1 splicing, ATF4 translation) higher in aged cells or tissues? |
| **Mitochondrial Dysfunction** | Inefficient mitophagy could be *viewed as* reflecting an elevated threshold (θ_Mitophagy) for recognizing and removing dysfunctional mitochondria, permitting their persistence. | Does the degree of mitochondrial membrane depolarization required to initiate mitophagy increase with age in primary cells? |
| **Cellular Senescence** | Senescence entry can be analogized as an *alternative, terminal action* when error (e.g., damage) is high but the cost of a faithful model update (repair/proliferation) is deemed prohibitive. The SASP may itself act as a source of noisy, dysregulated signaling for neighboring cells. | Does the probability of entering senescence in response to a standardized sub-lethal stressor change with replicative age, and does it correlate with proxies for cellular "update cost" (e.g., energy charge, NAD+ levels)? |
| **Altered Intercellular Communication (e.g., Inflammaging)** | Inflammaging may reflect a dysregulated immune sensitivity landscape, consistent with allostatic load theory (McEwen, 1998; Barrett, 2017): a **lowered threshold** (θ_Innate) for innate/inflammatory priming coupled with an **elevated threshold** (θ_Adaptive/Resolution) for triggering robust adaptive or resolution responses. | Do immune cells from older adults show heightened cytokine production to low-dose PAMPs but blunted responses to resolution signals (e.g., specialized pro-resolving mediators)? |

**Narrative Synthesis and Critical Caveat:**
This framework suggests a shift in focus from the mere accumulation of damaged components to the **fidelity of the detection and response systems** that should manage them. **It is crucial to reiterate that equating complex, nonlinear biological pathways to a scalar threshold θ is a dramatic oversimplification for illustrative purposes.** The value lies not in mechanistic explanation, but in generating novel questions about sensitivity shifts and their system-wide coordination. If validated, a direct implication is that interventions restoring youthful sensitivity profiles might have coordinated benefits.

---

### 7. Engagement with Evolutionary and Computational Theories of Aging

A robust framework must be positioned within the existing theoretical landscape.

*   **Disposable Soma Theory (DST):** DST posits aging results from optimal allocation of finite resources between somatic maintenance and reproduction. Our framework is compatible but adds a layer of specificity: it proposes that a key "resource" being allocated is **information-processing capacity for model updating**. The rising cost *c_u* in our illustrative model can be linked to the diminishing somatic maintenance budget. We suggest a novel manifestation: this budget constraint may drive a system-wide *reconfiguration of sensitivity settings* (θ) to conserve updating energy, reframing maintenance as an active inferential process.

*   **Antagonistic Pleiotropy (AP):** AP states genes beneficial early in life can be harmful later. Our framework offers a potential interpretation and directly incorporates its logic. The **myopic optimization** in our illustrative model is conceptually analogous to AP: it maximizes short-term fitness (minimizes immediate cost) at the expense of long-term function. Genes promoting robust early-life responses might do so by setting **low adaptive thresholds (θ)**. These same low thresholds, maintained amid rising costs and noise, could force excessive updating, exhausting resources and triggering a compensatory, pathological threshold elevation consistent with the narrowing cycle.

*   **Hyperfunction Theory:** Hyperfunction posits aging is caused by the continued, damaging activity of developmental pathways (e.g., mTOR). Our framework provides a complementary, systems-level interpretation: hyperfunction can be seen as a **pathologically stable prediction (a fixed prior)**. The system's model becomes "stuck," leading to constitutive pathway activity. This could be both a *cause* and a *consequence* of threshold elevation. Noisy signaling from hyperfunctional pathways could widen the error distribution (σ_δ² ↑), driving θ up. Conversely, an elevated threshold for negative feedback on these pathways (e.g., for autophagy induction) would prevent their downregulation, sustaining hyperfunction. The relationship is likely bidirectional.

*   **Network, Information, & Homeodynamic Theories:** Theories describing aging as a loss of network connectivity, system complexity and robustness (Kauffman, 1993; Kowald & Kirkwood, 1996; Xue et al., 2007; Barabási, 2016), increasing entropy or programmed information loss (Demetrius, 2001; Mitteldorf, 2004; Sokhansanj et al., 2022), declining homeodynamic capacity (Yates, 1994; Cohen, 2016), or loss of dynamical compensation (Cohen et al., 2022; Yashin et al., 2012) are highly congruent with our framework. Threshold elevation directly reduces effective signal propagation (connectivity) and mutual information with the environment. **Informational closure** is synonymous with a loss of complexity and adaptive capacity. Our framework provides a proposed *potential driver* for these changes: a myopic cost-minimization policy. It also engages with the concept of allostatic load as the wear-and-tear from failed prediction (McEwen, 1998; Sterling, 2012).

**Novel Emphasis and Acknowledged Complexities:** A primary distinction of our framework lies in its proposed *pattern*: the myopic optimization of a prediction-error cost function leading to strategic informational closure. It uniquely predicts that integrative decline may reflect an underlying shift in **adaptive signal detection thresholds**. It must also explicitly acknowledge and engage with phenomena that challenge a simple monotonic threshold elevation, such as **cell-type-specific hyper-sensitivity** (e.g., to apoptotic signals in aged neurons) and the complex dynamics of hormesis, including **adaptive desensitization (tachyphylaxis)** where repeated stress can *raise* thresholds, a point explored in the next section.

---

### 8. Reinterpretation of Hormesis and Sensitivity Dynamics

Hormesis, the beneficial effect of mild stress (Mattson, 2008), is a key geroprotective concept. Our framework offers a specific reinterpretation: hormetic interventions may be effective because they act as **controlled, supra-threshold prediction errors** that force model updating in a system that has pathologically raised its threshold.

*   **Cold Exposure:** Presents an unambiguous thermal prediction error (δ ≥ θ_thermoregulation), compelling a systemic model update involving brown adipose tissue activation and autonomic recalibration.
*   **Exercise:** Challenges predictions of musculoskeletal and cardiovascular capacity, triggering adaptive updates in protein synthesis and mitochondrial biogenesis.
*   **Fasting/Caloric Restriction:** Violates the predicted constant nutrient supply, forcing a shift in metabolic model from anabolism to catabolism and repair (autophagy).
*   **Cognitive Novelty:** Presents clear prediction errors to neural circuits, forcing synaptic updates, potentially counteracting cognitive rigidity.

The dose-response curve maps directly to our illustrative model. A dose below θ may be ignored. An optimal dose is just above θ, forcing an adaptive update without overwhelming the system.

**Engaging Complexity: Desensitization and Allostatic Load**
This view must be reconciled with the well-documented phenomenon of **tachyphylaxis** or **desensitization**, where repeated exposure to the same stressor leads to a *raised* threshold (e.g., receptor downregulation). This appears contradictory to a framework proposing hormesis lowers thresholds. A more nuanced interpretation is required: hormesis may work by **recalibrating the dynamic range of sensitivity**, not simply lowering a static threshold. An effective hormetic regimen might initially penetrate an elevated threshold, force an update that resets the system's priors, and ultimately lead to a more *precise and appropriate* sensitivity profile—one that responds robustly to novel threats but does not overreact to benign noise. This aligns with the concept of **allostatic load versus allostatic overload** (McEwen & Karatsoreos, 2015), where healthy adaptation requires appropriate sensitivity and shutdown of stress responses. Aging, in this view, could involve a failure to habituate to non-threatening signals (low threshold for noise) while simultaneously failing to mount robust responses to true threats (high threshold for signals), a dysregulation captured in the inflammaging analogy (Table 1).

---

### 9. Future Directions, Limitations, and Conclusions

**Critical Research Program:**
1.  **Develop Threshold/Sensitivity Biomarkers:** Create and validate standardized assays for sensitivity across systems: sensory psychophysics, autonomic and metabolic challenge tests, ex vivo immune cell stimulation assays, and computational models of learning to infer perceptual priors.
2.  **Longitudinal Data Analysis:** Test prediction #1 by analyzing existing cohort data (e.g., Baltimore Longitudinal Study of Aging, UK Biobank) for correlations between diverse reactivity measures and their joint predictive power for morbidity/mortality.
3.  **Mechanistic Exploration:** Test the generative links suggested in Section 6. Use genetic, pharmacological, or optogenetic tools to modulate proposed sensitivity pathways and measure effects on hallmarks and system-wide reactivity.
4.  **Intervention Studies:** Design proof-of-concept trials to test if the benefits of interventions (e.g., exercise, fasting-mimicking diets) correlate with changes in measured physiological sensitivity (prediction #4).
5.  **Disentangle Causality:** Design experiments to distinguish active policy shifts from passive degradation. For example, search for conserved molecular nodes whose manipulation broadly resets sensitivity across disparate systems in aged animals, or use longitudinal single-cell omics to track if changes in sensitivity-associated transcripts precede hallmark accumulation.

**Explicit Limitations:**
1.  **Framework, Not Derived Theory:** This is a hypothesis-generating perspective, not a first-principles derivation. The core postulate of threshold elevation is presented as a testable pattern, not a mathematical certainty. The illustrative model is a conceptual tool, not a biological simulation.
2.  **Causality and Reciprocity:** We propose threshold elevation as a potential driver within a reinforcing loop, but primacy is not resolved. It likely exists in a reciprocal, reinforcing relationship with hallmark accumulation. Disentangling this is a major empirical challenge.
3.  **Specificity of Interpretations:** The proposed interpretations of hallmarks are high-level analogies. Equating complex, nonlinear biological pathways to a scalar threshold θ is a major simplifying assumption. The interpretations in Section 6 are speculative and intended to generate novel questions, not to describe established mechanisms.
4.  **Incomplete Engagement with Active Inference:** The illustrative model focuses on perceptual inference (to update or not). A more complete framework would need to incorporate **active inference**—how organisms act to sample data and change their sensory input to fit predictions—which is a core tenet of predictive processing.

**Conclusion:**
We have presented a perspective framing biological aging as a systems-level process of **adaptive narrowing**. We illustrated a core trade-off and introduced the **postulate** that aging involves a measurable elevation of thresholds for adaptive updates. By outlining unique predictions and exploring speculative analogies to hallmarks, we aim to move the idea from metaphor to a generative, falsifiable framework. It generates distinct predictions, synthesizes with existing theories, and offers a novel lens on hormesis and sensitivity dynamics. The essence of senescence may lie not just in the accumulation of damage, but in the systemic decline in the capacity to *register* and *respond* to it with fidelity. The ultimate value of this framework will be determined by its ability to motivate and withstand targeted experimental investigation.

---

### 12. References

Adami, C. (2002). What is complexity? *BioEssays*, 24(12), 1085-1094.

Barabási, A. L. (2016). Network science. *Cambridge university press*.

Barrett, L. F. (2017). The theory of constructed emotion: an active inference account of interoception and categorization. *Social cognitive and affective neuroscience*, 12(1), 1-23.

Clark, A. (2013). Whatever next? Predictive brains, situated agents, and the future of cognitive science. *Behavioral and Brain Sciences*, 36(3), 181–204.

Cohen, A. A., Ferrucci, L., Fulop, T., Gravel, D., Hao, N., Kriete, A., ... & Senior, A. M. (2022). A complex systems approach to aging biology. *Nature Aging*, 2(7), 580-591.

Cohen, A. A. (2016). Complex systems dynamics in aging: new evidence, continuing questions. *Biogerontology*, 17(1), 205-220.

Demetrius, L. (2001). Mortality plateaus and directionality theory. *Proceedings of the Royal Society of London. Series B: Biological Sciences*, 268(1476), 2029-2037.

Franceschi, C., Valensin, S., Bonafè, M., Paolisso, G., Yashin, A. I., Monti, D., & De Benedictis, G. (2000). The network and the remodeling theories of aging: historical background and new perspectives. *Experimental gerontology*, 35(6-7), 879-896.

Friston, K. (2010). The free-energy principle: a unified brain theory? *Nature Reviews Neuroscience, 11*(2), 127–138.

Gavrilov, L. A., & Gavrilova, N. S. (1991). The biology of life span: A quantitative approach. *Harwood Academic Publishers*.

Gems, D., & de la Guardia, Y. (2013). Alternative perspectives on aging in *Caenorhabditis elegans*: reactive oxygen species or hyperfunction? *Antioxidants & Redox Signaling*, 19(3), 321-329.

Hayflick, L., & Moorhead, P. S. (1961). The serial cultivation of human diploid cell strains. *Experimental cell research*, 25(3), 585-621.

Kauffman, S. A. (1993). The origins of order: Self-organization and selection in evolution. *Oxford University Press*.

Kirkwood, T. B. L. (1977). Evolution of ageing. *Nature*, 270(5635), 301-304.

Kirkwood, T. B., & Austad, S. N. (2000). Why do we age? *Nature*, 408(6809), 233-238.

Kowald, A., & Kirkwood, T. B. (1996). A network theory of ageing: the interactions of defective mitochondria, aberrant proteins, free radicals and scavengers in the ageing process. *Mutation Research/DNAging*, 316(5-6), 209-236.

López-Otín, C., Blasco, M. A., Partridge, L., Serrano, M., & Kroemer, G. (2013). The hallmarks of aging. *Cell, 153*(6), 1194–1217.

Mattson, M. P. (2008). Hormesis defined. *Ageing Research Reviews, 7*(1), 1–7.

McEwen, B. S. (1998). Stress, adaptation, and disease: Allostasis and allostatic load. *Annals of the New York Academy of Sciences, 840*(1), 33–44.

McEwen, B. S., & Karatsoreos, I. N. (2015). Sleep deprivation and circadian disruption: stress, allostasis, and allostatic load. *Sleep medicine clinics*, 10(1), 1-10.

Milholland, B., Suh, Y., & Vijg, J. (2017). Mutation and catastrophe in the aging genome. *Experimental gerontology*, 94, 34-40.

Mitteldorf, J. (2004). Ageing selected for its own sake. *Evolutionary Ecology Research*, 6(7), 937-953.

Pezzulo, G., Rigoli, F., & Friston, K. (2015). Active Inference, homeostatic regulation and adaptive behavioural control. *Progress in neurobiology*, 134, 17-35.

Shannon, C. E. (1948). A mathematical theory of communication. *The Bell system technical journal*, 27(3), 379-423.

Sokhansanj, B. A., Fitch, J. P., Quong, J. N., & Quong, A. A. (2022). A unified model of aging and longevity. *Journal of Theoretical Biology*, 548, 111199.

Sterling, P. (2012). Allostasis: a model of predictive regulation. *Physiology & behavior*, 106(1), 5-15.

Williams, G. C. (1957). Pleiotropy, natural selection, and the evolution of senescence. *Evolution, 11*(4), 398-411.

Xue, H., Xian, B., Dong, D., Xia, K., Zhu, S., Zhang, Z., ... & Tian, X. (2007). A modular network model of aging. *Molecular systems biology*, 3(1), 147.

Yashin, A. I., Arbeev, K. G., Akushevich, I., Kulminski, A., Ukraintseva, S. V., Stallard, E., & Land, K. C. (2012). The quadratic hazard model for analyzing longitudinal data on aging, health, and the life span. *Physics of life reviews*, 9(2), 177-188.

Yates, F. E. (1994). Order and complexity in dynamical systems: Homeodynamics as a generalized mechanics for biology. *Mathematical and Computer Modelling*, 19(6-8), 49-74.

Zhavoronkov, A., & Moskalev, A. (2020). The intersection of deep learning and neuroscience for aging research. *Aging Research Reviews*, 59, 101036.

---
**Supplementary Appendix A: Illustrative Simulation of the Conceptual Trade-off Model**

*(This appendix contains the simplified code and parameters used to generate the conceptual plot in Figure 1, explicitly noting its illustrative, non-mechanistic purpose.)*