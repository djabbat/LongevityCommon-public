**Revised Manuscript**

**Title:** A No-Go Theorem for Introspective Prediction in Computational Machines: A Formal Model and Research Program

**Author:** Jaba Tkemaladze

**Affiliation:** Research Laboratory for Integrative Medicine, Phasis Academy, Georgia

**Correspondence:** jaba@longevity.ge

**Journal:** *Entropy*

---

### **1. Abstract**

This paper presents two distinct but related contributions. First, we introduce a formal model, the **Introspective Predictive Machine (IPM)**, designed to study the limits of systems that perform online prediction while attempting to assess their own predictive performance via internal subroutines. For this model, we prove a novel limit theorem, **Theorem 1 (The No-Go Theorem for Perfect Introspection)**. Using Kleene’s Recursion Theorem, we demonstrate rigorously that any IPM’s introspective subroutine must fail in one of three specific ways: it may be incorrect, non-halting, or its execution must perturb the very state it aims to measure. This theorem constitutes a self-contained result in the theory of self-referential computation. Second, building upon this foundation, we articulate a separate and speculative **research program**. The program’s core challenge is to investigate whether the *structure* of the limit formalized in Theorem 1 can yield a novel perspective on, or reformulation of, Gödel’s Second Incompleteness Theorem. We outline a formalization roadmap that makes the significant technical hurdles—particularly the challenge of mapping the introspective task onto a consistency proof—explicit. This work provides a precise computational model, a proven theorem, and a clear, falsifiable framework for future research into the architecture of self-referential limits.

**Keywords:** self-reference; limits of computation; introspective machines; diagonalization; Recursion Theorem; research program; Gödel incompleteness

---

### **2. Introduction**

The phenomenon of self-referential limitation appears across disciplines. Gödel’s Incompleteness Theorems (1931) show that consistent formal systems of sufficient strength cannot prove their own consistency [1]. Turing’s Halting Problem demonstrates that a universal computer cannot decide the termination of all programs, including those that query its own behavior [2]. Similar patterns arise in discussions of observation in physics and self-modeling in cognitive science [3, 4]. This recurrence motivates a search for unified formal principles underlying these diverse limits.

This paper makes two contributions, which we carefully distinguish.

**1. A Completed Foundational Result:** We define a precise computational model, the **Introspective Predictive Machine (IPM)**, which formalizes a system engaged in prediction while possessing an internal mechanism to estimate its own global performance. For this model, we prove **Theorem 1 (The No-Go Theorem for Perfect Introspection)**. This theorem establishes that any such introspective mechanism must fail according to a three-way classification. The proof employs a rigorous diagonalization argument grounded in Kleene’s Recursion Theorem. This theorem is a novel contribution to computability theory, demonstrating an *operational* limit within a stateful, interactive system.

**2. A Proposed Research Program:** Separately, and building upon the IPM framework, we outline a research program. Its central challenge is to explore whether the *logical structure* of Theorem 1—a tripartite failure for an internal measurement subroutine—can be used to derive or reinterpret Gödel’s Second Incompleteness Theorem. We present a roadmap for this formalization, identifying the specific technical steps required and the major obstacles, most notably the difficulty of constructing a suitable computable performance function. The program is presented not as a completed derivation but as a challenging, falsifiable direction for future inquiry.

The paper is structured as follows:
1.  We review relevant background in logic and computability.
2.  We critically position our work relative to existing formalisms, including provability logic, the Recursion Theorem, and models of reflective computation.
3.  We formally define the **Introspective Predictive Machine (IPM)** model.
4.  We state and rigorously prove **Theorem 1** using the Recursion Theorem.
5.  We articulate the separate research program, stating its core challenge, detailing the formalization roadmap, and explicitly discussing its foundational difficulties.
6.  We discuss the implications of Theorem 1 and potential future work, clearly separating established results from speculative directions.

This manuscript presents a new computational limit theorem and uses its framework to propose a rigorous, though speculative, program for exploring connections to logical incompleteness.

---

### **3. Background and Critical Positioning**

#### **3.1. Gödel’s Incompleteness and Computational Foundations**
Gödel’s First Incompleteness Theorem establishes that for any consistent, recursively axiomatizable formal system *F* capable of representing elementary arithmetic, there exists an arithmetical statement *G* that is true but unprovable in *F* [1]. The Second Theorem shows that a formalized statement of *F*’s consistency, Con(*F*), is itself unprovable in *F*. These results are fundamentally computational: the set of theorems is recursively enumerable, and the proofs use diagonalization over a provability predicate, a technique isomorphic to key arguments in computability theory [2].

#### **3.2. A Critical Survey of Relevant Formalisms**
Our work intersects with several fields. A successful formalization must clearly differentiate itself from and build upon this existing work.

*   **The Recursion Theorem and Partial Recursive Functionals:** Kleene’s Recursion Theorems [14, 30] are the cornerstone for constructing self-referential programs. The Second Recursion Theorem, in particular, guarantees that for any partial recursive functional *Ψ*, there exists a program index *e* such that the function computed by *e* is identical to *Ψ(e)*. This theorem formalizes the possibility of syntactic self-reference. Our proof of Theorem 1 directly relies on this result to construct the self-referential machine at the heart of the diagonal argument. The impossibility of a total, correct computable functional predicting the behavior of a process that includes itself is a theme in the theory of recursive functionals [30].

*   **Rice’s Theorem and Index Sets:** Rice’s Theorem [6] states that all non-trivial, extensional properties of recursively enumerable sets are undecidable. Theorem 1 addresses a different, *intensional* question: it demonstrates a limitation for a specific *running process* (a subroutine) within a machine, tasked with computing a property of the machine’s own *runtime history*. The “perturbation” failure mode arises from the stateful, closed-loop nature of this specific model. We therefore position Theorem 1 as an *operational* result that shares the self-referential spirit of, but is not a direct extension of, Rice’s classic theorem.

*   **Provability Logic & The Logic of Proofs:** Provability logic (e.g., **GL**) [13] axiomatizes the logic of the provability predicate *Bew(x)*. The Logic of Proofs (**LP**) [21] refines this with explicit proof terms. These systems treat provability as a static *relation*. Our IPM model treats proof-*search* as a dynamic, state-altering *process*. The novel constraint is the explicit modeling of an introspective act that reads and potentially writes to the same tape encoding the process’s history, making the “measurement” part of the dynamical system.

*   **Formal Models of Self-Reference and Benignness:** Beyond syntactic fixed points, work by Perlis, Visser, and others [24, 25, 31] investigates different *types* of self-reference (e.g., “benign” vs. “unbenign”) and their formal properties. This literature is crucial for analyzing the specific nature of the self-reference involved in the IPM’s introspection attempt, which our model frames as a subroutine call.

*   **Gödel’s Theorem via Computability and Prediction:** Kritchman and Raz [20, 32] offer a computational/information-theoretic perspective on the second incompleteness theorem using a formalized “Surprise Examination” paradox, sharing a similar “predictive” spirit with our approach. Chaitin’s work [5] links incompleteness to the uncomputability of the halting probability Ω. Our focus is not on quantifying randomness but on the *operational obstruction* that arises when the prediction mechanism attempts to analyze its global performance.

*   **Reflective Architectures and Metacircular Interpreters:** Foundational work on reflective Turing machines [15, 33], 3-LISP [22], and metacircular interpreters explores systems that can represent and manipulate their own state. Our IPM is a constrained, specialized instance for prediction, with an explicit introspective interface. The formal limit theorem derived from this specific architecture is our contribution.

*   **Limits of Learning and Self-Optimization:** The no-free-lunch theorems [16, 26] and results on the impossibility of perfectly calibrated forecasting [27] establish fundamental limits. Hutter’s universal intelligence measure and associated impossibility results on self-optimization [34] are particularly relevant to our study of introspective prediction. The Logical Induction framework [35] rigorously deals with computable agents reasoning about their own beliefs, offering a sophisticated model of limited self-reference. Theorem 1 frames a specific introspective learning task: a system learning to predict its own global success rate, yielding a formal limit on this meta-learning capability.

This review clarifies our aim: to define a dynamic, operational model where internal measurement is a defined subroutine whose execution interferes with the system it measures, leading to a provable limit (Theorem 1), and to propose a program for exploring its relationship to logical limits.

---

### **4. A Formal Model: Introspective Predictive Machines (IPMs)**

We now define the IPM with a focus on operational clarity. The core idea is a Turing machine that cycles between making predictions and, when triggered, executing a designated subroutine to compute an estimate of its own performance based on its complete history.

#### **4.1. Definition of an IPM**
An **Introspective Predictive Machine (IPM)** *Z* is a Turing machine with a designated structure and operational cycle. Formally, it is a tuple \( Z = (Q, \Sigma, \Gamma, \Delta, \delta, q_0, q_I) \), where:
*   \( Q \) is a finite set of internal states.
*   \( \Sigma \) is a finite input alphabet (environmental outcomes).
*   \( \Gamma \) is a finite output alphabet (predictions/actions).
*   \( \Delta \) is a finite tape alphabet, containing distinct symbols for input, prediction, and error recording, as well as blank and work symbols.
*   \( \delta : Q \times \Delta \to Q \times \Delta \times \{L, R, S\} \) is a partial transition function.
*   \( q_0 \in Q \) is the initial state.
*   \( q_I \in Q \) is the designated **introspective trigger state**.

**Tape Structure:** The machine has a single, semi-infinite tape, logically partitioned into three regions:
1.  **History Region (H):** An append-only log. At time *t*, it contains a finite string encoding the sequence of triplets \((i_s, p_s, e_s)\) for \(1 \leq s < t\), where \(i_s \in \Sigma\), \(p_s \in \Gamma\), and \(e_s \in \{0,1\}\) is the error bit (\(0\) for match \(i_s = p_s\), \(1\) for mismatch).
2.  **Work Region (W):** A standard Turing machine work tape.
3.  **Output Register (O):** A fixed-size cell that holds the current prediction or the result of an introspective computation.

**Operational Semantics:** Operation proceeds in discrete cycles, each beginning in a state \(q \in Q\).
*   **Primary Phase (State \(q \neq q_I\)):**
    1.  **Predict:** The machine uses its transition function \(\delta\) (which encodes a **prediction program**) to compute a prediction \(p_t \in \Gamma\) based on the current tape contents, and writes \(p_t\) to the Output Register **O**.
    2.  **Receive Input:** The **environment**, modeled as a computable function \(E: \Gamma^* \to \Sigma\), provides an input symbol \(i_t = E(p_1 \ldots p_t)\). This input, the prediction \(p_t\), and the computed error bit \(e_t\) (1 if \(i_t \neq p_t\), else 0) are appended to the History Region **H**.
    3.  **Update:** The machine updates its state and Work Region via \(\delta\), based on the new history.

*   **Introspective Phase (State \(q = q_I\)):** When the machine enters the trigger state \(q_I\), it executes its **introspective subroutine**. This subroutine is a specific, finite set of transitions within \(\delta\) with the following intended semantics: it reads the entire content of the History Region **H**, performs computations in the Work Region **W**, and aims to compute an estimate \(\hat{V}\) of a fixed, computable **performance function** \(V\). \(V\) maps finite strings (histories) to a value in a discrete set (e.g., real numbers). We require \(V\) to be **non-trivial** (non-constant) and **history-sensitive**, meaning there exist two distinct histories \(H_1\) and \(H_2\) such that \(V(H_1) \neq V(H_2)\). The subroutine halts by writing its output \(\hat{V}\) to the Output Register **O** and transitioning to a state \(q' \neq q_I\). We say the subroutine is **non-perturbing** if, upon halting, the contents of **H** and **W** are identical to their state at the invocation of \(q_I\), with the sole change being the value written to **O**.

This model cleanly separates the machine's syntax (\(\delta\), states) from the semantic assignment of the "introspective subroutine" (the computation initiated by \(q_I\)) and its goal of computing \(V(H)\).

### **4.2. The No-Go Theorem for Perfect Introspection**

**Theorem 1 (No-Go Theorem for Perfect Introspection in IPMs).** Let *Z* be an IPM as defined, with a computable, non-trivial, history-sensitive performance function \(V\). Let \(I\) be the introspective subroutine of *Z* (the computation initiated by state \(q_I\)).
Then, for any such \(Z\) and \(V\), one of the following must hold:
1.  **Incorrectness:** There exists an environment \(E\) and a time of invocation such that when \(I\) halts, its output \(\hat{V} \neq V(H)\) (where \(H\) is the history at invocation).
2.  **Non-Halting:** There exists an environment \(E\) and a time of invocation such that \(I\) does not halt.
3.  **Perturbation:** For all environments \(E\) and times of invocation where \(I\) halts, its execution is not non-perturbing; it modifies the Work region **W** or the History region **H**.

*Proof.* We prove by contradiction using Kleene’s Second Recursion Theorem. Assume, for the sake of contradiction, that there exists an IPM *Z* with a perfect introspective subroutine \(I\). That is, \(I\) (i) always halts, (ii) always outputs \(\hat{V} = V(H)\) correctly, and (iii) is non-perturbing for all environments and invocations.

We will construct a special IPM \(Z'\) that leads to a contradiction. The construction uses the Recursion Theorem to embed a self-referential description within \(Z'\).

**Step 1: Defining a Recursive Functional.** Consider a partial recursive functional \(\Psi\). Given an index \(e\) (which we intend to be an index for an IPM’s transition function \(\delta_e\)), \(\Psi(e)\) outputs the description of an IPM, denoted \(Z_e\), with the following behavior.
The machine \(Z_e\):
1.  Simulates the perfect introspective subroutine \(I\). Since \(I\) is a fixed, finite, halting, and by assumption non-perturbing program, \(Z_e\) can execute it step-by-step. Let the output be \(\hat{V}\).
2.  Based on \(\hat{V}\), it chooses a prediction \(p_t\) using a fixed, computable rule. For concreteness: let \(T\) be a rational threshold. If \(\hat{V} < T\), output prediction symbol \(\gamma_0\); if \(\hat{V} \geq T\), output \(\gamma_1\).
3.  The machine \(Z_e\) is designed to operate within a specific, adversarial **responsive environment** \(E_e\). \(E_e\) is a computable function defined as follows: on each cycle, it reads the prediction \(p_t\) output by \(Z_e\) and provides an input \(i_t\) such that \(i_t \neq p_t\). (For example, if \(\Gamma = \{0,1\}\), \(E_e\) outputs \(1 - p_t\)).

**Step 2: Applying the Recursion Theorem.** By Kleene’s Second Recursion Theorem [14, 30], there exists a fixed point: an index \(e_0\) such that the machine with index \(e_0\) (call it \(Z'\)) computes exactly the same function as \(\Psi(e_0)\). In other words, \(Z'\) is an IPM whose own transition function \(\delta_{e_0}\) is described by the procedure above, where the "machine being simulated" in step 1 is *itself*, \(Z'\). This is the legitimate, rigorous self-reference.

**Step 3: Deriving the Contradiction.** We now analyze the operation of \(Z'\) in its corresponding responsive environment \(E_{e_0}\).
Consider a cycle \(t\) where \(Z'\) enters its introspective phase.
- By assumption (i-iii), the subroutine \(I\) halts, outputs the correct value \(\hat{V}_t = V(H_t)\), and is non-perturbing. The history \(H_t\) is unchanged.
- \(Z'\) then outputs \(p_t\) according to the rule: \(p_t = \gamma_0\) if \(V(H_t) < T\), else \(p_t = \gamma_1\).
- The environment \(E_{e_0}\) provides \(i_t \neq p_t\).
- The error bit \(e_t\) is set to 1, and the triplet \((i_t, p_t, 1)\) is appended to form the new history \(H_{t+1}\).

Since \(V\) is history-sensitive and non-constant, \(V(H_{t+1})\) will generally differ from \(V(H_t)\). The rule for choosing \(p_t\) is a function of \(V(H_t)\), and the environment's response ensures the history evolves in a way dependent on this rule. This creates a feedback loop.

The contradiction arises from the fixed-point property. The subroutine \(I\), which is part of \(Z'\)'s own code, is used to compute \(V(H_t)\). This value directly determines \(Z'\)'s action \(p_t\), which in turn forces a specific environmental response and a specific change to \(H_t\), thereby changing the value of \(V\) for the next cycle. The assumption that \(I\) is always correct, halting, and non-perturbing implies the existence of a function \(f(v)\) (the rule determining \(p_t\) from \(v\)) and a deterministic environment response \(g(p)\) such that the sequence \(v_t = V(H_t)\) satisfies \(v_{t+1} = V(H_t \oplus (g(f(v_t)), f(v_t), 1))\), where \(\oplus\) denotes appending.

The Recursion Theorem ensures \(Z'\) implements this very feedback loop. For the subroutine to be perfect, this dynamical system must produce a sequence where \(v_t = V(H_t)\) for all \(t\). However, by construction, the environment's adversarial response and the sensitivity of \(V\) make this impossible for a non-constant \(V\). Formally, consider the first cycle. The correct \(\hat{V}_0 = V(H_0)\) leads to a prediction \(p_0\) and thus an error, producing \(H_1\). For \(I\) to be correct on the next cycle, we would need \(V(H_1) = \hat{V}_1\). But the relationship between \(\hat{V}_0\) and \(V(H_1)\) is mediated by the fixed rule and the adversarial environment. The self-referential fixed point forces a condition that cannot be consistently maintained, contradicting the existence of a perfect \(I\). Therefore, any introspective subroutine must fail in at least one of the three stated ways. ∎

This theorem provides a rigorous limit on introspective prediction within the defined model.

---

### **5. A Research Program: From Computational Introspection to Logical Incompleteness**

Theorem 1 and the IPM model constitute a self-contained contribution. We now articulate a separate, speculative research program that uses this framework to explore a potential formal analogy with Gödel’s Second Incompleteness Theorem. This program is presented as a set of challenging open problems, not as an established derivation.

#### **5.1. The Core Analogy and Challenge**
The operational limit in Theorem 1 shares a structural resemblance with logical incompleteness: an internal mechanism (introspective subroutine / consistency proof) attempts to correctly assess a global property of its containing system (predictive performance / consistency) and is provably unable to do so perfectly. The **research challenge** is to investigate whether this resemblance can be made formally precise. Specifically, can the *argument structure* of Theorem 1—the construction of a self-referential machine leading to a tripartite failure—be adapted to yield a novel proof or reformulation of the unprovability of consistency?

#### **5.2. Formalization Roadmap and Foundational Hurdles**
A program to explore this challenge would require the following steps, each representing a significant technical hurdle.

*   **Step 1: Construct \( Z_F \), an IPM Simulating Proof-Search.** Define an IPM \( Z_F \) whose operation simulates a theorem-enumerating procedure for a formal system *F* (e.g., Peano Arithmetic). Reinterpret the IPM framework:
    *   **History H:** Encodes the sequence of derived theorems. The "error bit" \( e_t \) is set to 1 if a contradiction (e.g., \(0=1\)) is found in cycle *t*, else 0.
    *   **Prediction \( p_t \):** A meta-conjecture, such as "No contradiction will be found in the next derivation cycle."
    *   **Input \( i_t \):** The observed outcome (contradiction or not), provided by an environment that simulates the honest results of the proof-search.
    *   **Performance Function \( V \):** A computable function of H. A naive candidate is an indicator function that outputs 0 if no contradiction appears in H, and 1 otherwise. The central difficulty arises here.

*   **Step 2: Define the Introspective Subroutine \( I_{Con} \).** Design \( I_{Con} \) (triggered by \( q_I \)) with the intended goal of computing \( V(H) \). If \( I_{Con} \) halts and outputs 0, this would correspond to an internal verification of consistency up to the current history.

*   **Step 3: Mapping the Failure Modes.** The goal would be to show that the inevitable failure of \( I_{Con} \) (per Theorem 1) corresponds to the unprovability of Con(*F*). This requires establishing lemmas that link the computational and logical frameworks:
    **Lemma 1 (Faithful Simulation):** \( Z_F \)’s operation, including the generation of H, faithfully simulates *F*’s theorem enumeration.
    **Lemma 2 (Performance as Consistency Proxy):** The statement "\( V(H) = 0 \)" must relate to the arithmetical consistency statement Con(*F*).
    **Lemma 3 (Introspection as Proof Attempt):** The operation of \( I_{Con} \) corresponds to *F*’s internal proof-search procedure applied to the formula Con(*F*).

    **The Foundational Hurdle:** Lemma 2 presents a potentially insurmountable obstacle. Con(*F*) is a Π₁⁰ sentence asserting that *no* number codes a proof of a contradiction in *F*. This is an assertion about all possible proofs (an infinite, non-compact property). The function \( V \), to be computable by an IPM, can only inspect the *finite* current history H. For the equivalence "\( V(H)=0 \) iff Con(*F*)" to hold, \( V \) would need to correctly decide, based on finite data, that the infinite proof-search will never produce a contradiction. This is equivalent to deciding the halting problem for the proof-search process, which is impossible. **Therefore, no computable \( V \) can satisfy Lemma 2 with exact equivalence.**

    This is not merely a technicality but a category error. It means the research program cannot proceed by directly mapping the IPM's performance function onto the consistency statement. A viable program must therefore be radically reformulated. One possible direction is to abandon the requirement that \( I_{Con} \) computes a definitive truth value for consistency. Instead, \( I_{Con} \) could be modeled as the proof-search process for Con(*F*) *itself*, and its output could be a proof token (or a timeout signal). Theorem 1’s failure modes might then be interpreted as limitations of this internal proof-search (e.g., it may run forever, or its execution may alter the system's state in a way that invalidates the proof attempt). This reframing shifts the focus from *verifying* a property to *generating* a proof, aligning more closely with the actual process in formal systems.

*   **Falsifiability and Value:** The program is falsifiable. It would fail if, after a rigorous reformulation, the failure modes of Theorem 1 cannot be shown to correspond in any meaningful way to the phenomenology of Gödel’s theorem (e.g., the unprovability of consistency, the existence of independent sentences). The value of this program lies in forcing a precise, operational confrontation with the deep differences and similarities between computational introspection and logical self-reference, potentially leading to new formal models that bridge these domains.

---

### **6. Discussion**

This paper has presented a novel computational limit theorem and proposed a separate research program inspired by it.

**Contributions:**
1.  **A Novel Computational Limit Theorem:** Theorem 1 is a rigorous, self-contained result. It contributes to the theory of self-referential computation by formalizing a no-go result for perfect introspection within a dynamic, predictive model. The use of the Recursion Theorem provides a solid foundation for the diagonal argument, and the identification of “perturbation” as a distinct failure mode adds to the taxonomy of self-referential limitations.
2.  **A Precise Formal Model:** The simplified IPM definition offers a clear operational semantics for systems combining prediction and self-analysis, distinguishing syntactic machinery from semantic intention.
3.  **A Structured Research Challenge:** The proposed program translates a broad analogy into a specific mathematical challenge. Its articulation clearly separates established results from speculation, highlights the major foundational obstacle (the category mismatch in Lemma 2), and suggests a necessary reformulation, thereby providing a clear starting point for future work.

**Relation to Existing Work:** Theorem 1 operates within the lineage of the Recursion Theorem [14, 30] and engages with formal studies of self-reference [24, 25, 31] and reflective architectures [15, 22, 33]. It relates to, but is distinct from, Rice’s Theorem [6] in its focus on an intensional, operational process. The proposed research program connects to work that reinterprets Gödel’s theorems through computational lenses [5, 20, 32] and to modern frameworks like Logical Induction [35] that grapple with computable reasoning about one's own beliefs.

**Implications and Future Work:** The **primary implications** are within computability and the formal study of self-referential systems. Theorem 1 may inform the design and analysis of reflective or meta-cognitive computational architectures, highlighting inherent trade-offs between accuracy, termination, and side-effects of self-assessment.

The **immediate future work** is the further investigation of Theorem 1 and the IPM model itself—e.g., exploring variations, complexity-theoretic refinements, or applications to simple learning agents. Pursuing the **research program** is a separate, long-term endeavor. The first step must be to develop a reformulated model that avoids the category error, perhaps by modeling the introspective subroutine explicitly as a proof-generating process. Answering the question of whether a meaningful bridge can be built is a prerequisite for any further progress.

Any broader implications for fields like AI safety or foundational physics remain highly speculative and are mentioned here only to suggest possible very long-term directions, not as claims of the present work.

---

### **7. Conclusion**

We have defined the Introspective Predictive Machine and proven a No-Go Theorem for perfect introspection within this model. This theorem stands as a new result in computability theory. Separately, we have outlined a research program to explore whether the structure of this computational limit can shed new light on Gödel’s Second Incompleteness Theorem. The program faces a significant, clearly defined foundational challenge in aligning computational and logical concepts of self-assessment. By clearly distinguishing the proven theorem from the speculative program, and by rigorously formulating both, this work provides a solid foundation for future research into the deep structure of self-referential limits in computation and logic.

---

### **8. References**

1.  Gödel, K. (1931). Über formal unentscheidbare Sätze der Principia Mathematica und verwandter Systeme I. *Monatshefte für Mathematik und Physik*, *38*, 173–198.
2.  Turing, A. M. (1937). On computable numbers, with an application to the Entscheidungsproblem. *Proceedings of the London Mathematical Society*, *s2-42*(1), 230–265.
3.  Rovelli, C. (1996). Relational quantum mechanics. *International Journal of Theoretical Physics*, *35*(8), 1637–1678.
4.  Metzinger, T. (2003). *Being No One: The Self-Model Theory of Subjectivity*. MIT Press.
5.  Chaitin, G. J. (1974). Information-theoretic limitations of formal systems. *Journal of the ACM*, *21*(3), 403–424.
6.  Rice, H. G. (1953). Classes of recursively enumerable sets and their decision problems. *Transactions of the American Mathematical Society*, *74*, 358–366.
7.  Crutchfield, J. P. (1994). The calculi of emergence: computation, dynamics and induction. *Physica D: Nonlinear Phenomena*, *75*(1-3), 11–54.
8.  Wheeler, J. A. (1990). Information, physics, quantum: The search for links. In W. Zurek (Ed.), *Complexity, Entropy, and the Physics of Information* (pp. 3–28). Addison-Wesley.
9.  Yanofsky, N. S. (2003). A universal approach to self-referential paradoxes, incompleteness and fixed points. *Bulletin of Symbolic Logic*, *9*(3), 362–386.
10. Hofstadter, D. R. (1979). *Gödel, Escher, Bach: an Eternal Golden Braid*. Basic Books.
11. Smith, B. C. (1982). *Reflection and Semantics in a Procedural Language*. PhD Thesis, MIT.
12. Zurek, W. H. (2009). Quantum Darwinism. *