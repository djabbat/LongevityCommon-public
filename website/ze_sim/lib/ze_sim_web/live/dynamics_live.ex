defmodule ZeSimWeb.DynamicsLive do
  @moduledoc """
  Ze Vectors Theory — Main interactive page.

  Three components:
  1. Falsifiable Predictions dropdown (FP-1…FP-8, with status and experiment design)
  2. Expected Experiments dropdown (concrete measurable tests, incl. third-party)
  3. Ze-Dynamics canvas: Game-of-Life style animation of Ze-system relationships,
     starting from 4 Ze-systems. T-events spawn daughters; τ_Z → 0 terminates.
  """
  use ZeSimWeb, :live_view

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="ze-main">

      <!-- ── Header ──────────────────────────────────────────────────────── -->
      <div class="ze-main-header">
        <div class="ze-main-title">
          <h1>Ze Vectors Theory</h1>
          <p class="ze-main-subtitle">
            A unified observer-based framework for spacetime and quantum mechanics.
            Physical reality emerges from statistics of causal event updates.
          </p>
          <p class="ze-main-ref">
            Tkemaladze J. (2026) · <em>Longevity Horizon</em> ·
            <a href="https://doi.org/10.65649/3hm9b025" target="_blank">DOI 10.65649/3hm9b025</a>
          </p>
        </div>

        <!-- ── Two dropdown buttons ──────────────────────────────────── -->
        <div class="ze-action-bar">

          <!-- Falsifiable Predictions -->
          <div class="ze-dropdown" id="drop-falsifiable">
            <button class="ze-drop-btn ze-drop-falsifiable"
                    onclick="zeToggleDropdown('drop-falsifiable')">
              ⚗ Falsifiable Predictions
              <span class="ze-drop-arrow">▾</span>
            </button>
            <div class="ze-drop-panel" id="drop-falsifiable-panel">
              <div class="ze-drop-title">8 Falsifiable Predictions (FP-1…FP-8)</div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp1')">
                <span class="ze-fp-label">FP-1</span>
                <span class="ze-fp-name">Lorentz time dilation τ(v)/τ(0) = √(1−v²)</span>
                <span class="ze-badge ze-verified">✓ Verified</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp2')">
                <span class="ze-fp-label">FP-2</span>
                <span class="ze-fp-name">Implementation equivalence: τ depends only on (N_T, N_S)</span>
                <span class="ze-badge ze-verified">✓ Verified</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp3')">
                <span class="ze-fp-label">FP-3</span>
                <span class="ze-fp-name">Acceleration independence of proper time</span>
                <span class="ze-badge ze-verified">✓ Verified</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp4')">
                <span class="ze-fp-label">FP-4</span>
                <span class="ze-fp-name">Emergent speed limit c_eff = λ·Δx/Δt from event rate</span>
                <span class="ze-badge ze-proposed">◎ Proposed</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp5')">
                <span class="ze-fp-label">FP-5</span>
                <span class="ze-fp-name">Causal chain length τ = √(L_c² + 2L_c·N_S)</span>
                <span class="ze-badge ze-verified">✓ Verified</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp6')">
                <span class="ze-fp-label">FP-6</span>
                <span class="ze-fp-name">Twin paradox: τ_A > τ_B without frame reference</span>
                <span class="ze-badge ze-verified">✓ Verified</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp7')">
                <span class="ze-fp-label">FP-7</span>
                <span class="ze-fp-name">Quantum coherence tuning: V = V₀·√(1−P²)</span>
                <span class="ze-badge ze-experiment">⚗ Experiment needed</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('fp8')">
                <span class="ze-fp-label">FP-8</span>
                <span class="ze-fp-name">SR / causal-set / twistor convergence</span>
                <span class="ze-badge ze-partial">~ SR verified</span>
              </div>
            </div>
          </div>

          <!-- Expected Experiments -->
          <div class="ze-dropdown" id="drop-experiments">
            <button class="ze-drop-btn ze-drop-experiments"
                    onclick="zeToggleDropdown('drop-experiments')">
              🔬 Expected Experiments
              <span class="ze-drop-arrow">▾</span>
            </button>
            <div class="ze-drop-panel" id="drop-experiments-panel">
              <div class="ze-drop-title">Concrete experimental tests</div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex1')">
                <span class="ze-fp-label">EX-1</span>
                <span class="ze-fp-name">Ze Interferometer: double-slit + polarization which-path tagging</span>
                <span class="ze-badge ze-experiment">⚗ Near-term</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex2')">
                <span class="ze-fp-label">EX-2</span>
                <span class="ze-fp-name">Optical lattice atomic clock τ(v) comparison</span>
                <span class="ze-badge ze-experiment">⚗ Near-term</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex3')">
                <span class="ze-fp-label">EX-3</span>
                <span class="ze-fp-name">Photonic quantum walk: Born vs uniform Ze-strategy</span>
                <span class="ze-badge ze-experiment">⚗ Near-term</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex4')">
                <span class="ze-fp-label">EX-4</span>
                <span class="ze-fp-name">NMR T₁/T₂ relaxation ↔ Ze T-event rate mapping</span>
                <span class="ze-badge ze-third">~ Third-party data</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex5')">
                <span class="ze-fp-label">EX-5</span>
                <span class="ze-fp-name">Quantum eraser: continuous P-tuning to verify V(P)</span>
                <span class="ze-badge ze-experiment">⚗ Near-term</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex6')">
                <span class="ze-fp-label">EX-6</span>
                <span class="ze-fp-name">Hafele-Keating style Ze experiment (airborne clocks)</span>
                <span class="ze-badge ze-third">~ Existing data</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex7')">
                <span class="ze-fp-label">EX-7</span>
                <span class="ze-fp-name">Molecular dynamics: OU thermostat Ze-entropy vs Boltzmann</span>
                <span class="ze-badge ze-sim">✓ Simulated here</span>
              </div>
              <div class="ze-drop-item" onclick="zeShowDetail('ex8')">
                <span class="ze-fp-label">EX-8</span>
                <span class="ze-fp-name">Biological HRV τ_Z: heart rate variability Ze-observer model</span>
                <span class="ze-badge ze-proposed">◎ Proposed</span>
              </div>
            </div>
          </div>

        </div><!-- /ze-action-bar -->
      </div><!-- /ze-main-header -->

      <!-- ── Central Ze-Dynamics Window ────────────────────────────────── -->
      <div class="ze-dynamics-section">
        <div class="ze-dynamics-header">
          <h2>Ze-System Dynamics</h2>
          <p>
            Starting from <strong>4 Ze-systems</strong>.
            Each step: T-event → τ_Z decrements + daughter spawned.
            S-event → observer unchanged.
            τ_Z = 0 → system terminates.
          </p>
          <div class="ze-dynamics-legend">
            <span class="ze-legend-dot ze-legend-born"></span> High τ_Z (young)
            <span class="ze-legend-dot ze-legend-dying"></span> Low τ_Z (aging)
            <span class="ze-legend-dot ze-legend-dead"></span> Terminated
            <span class="ze-legend-line"></span> Parent → daughter
          </div>
        </div>

        <div class="ze-canvas-area" id="ze-canvas-area" phx-hook="ZeDynamics">
          <canvas id="ze-canvas" width="860" height="460"></canvas>
        </div>

        <div class="ze-dynamics-controls">
          <button id="ze-btn-toggle" class="ze-ctrl-btn ze-ctrl-primary">▶ Start</button>
          <button id="ze-btn-reset"  class="ze-ctrl-btn">↺ Reset</button>
          <label class="ze-ctrl-label">
            Speed
            <input id="ze-speed" type="range" min="80" max="900" value="350" />
          </label>
          <label class="ze-ctrl-label">
            τ₀
            <input id="ze-tau0" type="range" min="5" max="60" value="20" />
            <span id="ze-tau0-val">20</span>
          </label>
          <label class="ze-ctrl-label">
            p_T
            <input id="ze-pt" type="range" min="0.05" max="0.70" step="0.05" value="0.25" />
            <span id="ze-pt-val">0.25</span>
          </label>
        </div>

        <div class="ze-dynamics-stats" id="ze-stats">
          <div class="ze-stat"><span>Active systems</span><strong id="ze-stat-active">4</strong></div>
          <div class="ze-stat"><span>Cosmic era</span><strong id="ze-stat-era">Planck</strong></div>
          <div class="ze-stat"><span>Total spawned</span><strong id="ze-stat-spawned">4</strong></div>
          <div class="ze-stat"><span>T-events</span><strong id="ze-stat-tevents">0</strong></div>
          <div class="ze-stat"><span>Max τ_Z alive</span><strong id="ze-stat-maxtau">20</strong></div>
        </div>
      </div>

      <!-- ── Detail panel (modal) ──────────────────────────────────────── -->
      <div id="ze-detail-overlay" class="ze-detail-overlay" onclick="zeCloseDetail()" style="display:none;"></div>
      <div id="ze-detail-panel" class="ze-detail-panel" style="display:none;">
        <button class="ze-detail-close" onclick="zeCloseDetail()">✕</button>
        <div id="ze-detail-content"></div>
      </div>

      <!-- ── Simulators navigation ─────────────────────────────────────── -->
      <div class="ze-sims-nav">
        <div class="ze-sims-title">Digital Twin Simulators</div>
        <div class="ze-sims-grid">
          <a href="/thermo" class="ze-sim-card ze-sim-thermo">
            <div class="ze-sim-level">Level 2</div>
            <div class="ze-sim-name">Thermodynamics</div>
            <div class="ze-sim-desc">Second Law · S_Ze · Maxwell's Demon</div>
          </a>
          <a href="/quantum" class="ze-sim-card ze-sim-quantum">
            <div class="ze-sim-level">Level 3</div>
            <div class="ze-sim-name">Quantum</div>
            <div class="ze-sim-desc">Born optimality · Theorem 5.1 · τ_Z depletion</div>
          </a>
          <a href="/repro" class="ze-sim-card ze-sim-repro">
            <div class="ze-sim-level">Level 4</div>
            <div class="ze-sim-name">Reproduction</div>
            <div class="ze-sim-desc">Axiom Z4 · Double-slit V_Ze vs V_QM</div>
          </a>
          <a href="/slit" class="ze-sim-card ze-sim-slit">
            <div class="ze-sim-level">P1</div>
            <div class="ze-sim-name">Double-Slit</div>
            <div class="ze-sim-desc">V = 1−2p_T · Englert bound</div>
          </a>
        </div>
      </div>

    </div><!-- /ze-main -->

    <!-- ── Inline JS: dropdowns, detail panel, Ze-dynamics ────────────── -->
    <script>
      // ── Dropdown toggle ────────────────────────────────────────────────
      function zeToggleDropdown(id) {
        const panel = document.getElementById(id + '-panel');
        const allPanels = document.querySelectorAll('.ze-drop-panel');
        allPanels.forEach(p => { if (p.id !== id + '-panel') p.classList.remove('ze-drop-open'); });
        panel.classList.toggle('ze-drop-open');
      }
      document.addEventListener('click', function(e) {
        if (!e.target.closest('.ze-dropdown')) {
          document.querySelectorAll('.ze-drop-panel').forEach(p => p.classList.remove('ze-drop-open'));
        }
      });

      // ── Detail panel data ──────────────────────────────────────────────
      const ZE_DETAILS = {
        fp1: {
          title: 'FP-1 — Universal Lorentz Time-Dilation Function',
          status: '✅ Computationally verified',
          body: `
            <p><strong>Formal statement:</strong> For any Ze system with velocity parameter v = N_S/N ∈ [0,1):</p>
            <pre class="ze-code">τ(v) / τ(0) = √(1 − v²)</pre>
            <p>This holds in the limit N→∞, independent of stream structure.</p>
            <p><strong>Proof:</strong> τ(0)=N; τ(v)=√(N²−N_S²)=N√(1−v²). Therefore τ(v)/τ(0)=√(1−v²). □</p>
            <p><strong>Falsification condition:</strong> |τ(v)/τ(0) − √(1−v²)| > ε for any well-defined (N_T,N_S) in large-N limit.</p>
            <p><strong>Experimental result:</strong> 21 velocity values, N=10⁷, residuals < 10⁻⁵. Status: VERIFIED ✓</p>
            <p><strong>Reference:</strong> Tkemaladze (2026) DOI 10.65649/1p3e3b94</p>
          `
        },
        fp2: {
          title: 'FP-2 — Implementation Equivalence',
          status: '✅ Computationally verified',
          body: `
            <p><strong>Formal statement:</strong> Two Ze systems with different stream-generation mechanisms
            but identical (N_T, N_S) yield exactly the same proper time τ.</p>
            <p><strong>Non-trivial test:</strong> i.i.d. Bernoulli, Markov chain, and deterministic periodic streams
            with same effective velocity v produce identical τ/τ₀ to machine precision.</p>
            <p><strong>Result:</strong> N=10⁶. Three generators agree on τ at every tested v. Status: VERIFIED ✓</p>
            <p><strong>Implication:</strong> Ze proper time is a function of (N_T, N_S) only — the microscopic
            ordering of events does not matter. This is a stronger structural claim than mere Lorentz invariance.</p>
          `
        },
        fp3: {
          title: 'FP-3 — Acceleration Independence of Proper Time',
          status: '✅ Computationally verified',
          body: `
            <p><strong>Formal statement:</strong> Proper time τ depends only on the total counter pair (N_T, N_S),
            not on the temporal profile p(k) of the flip probability.</p>
            <p><strong>Test:</strong> Four p-profiles (constant, linear ramp, step function, sinusoidal) producing
            the same effective v=N_S/N give identical τ.</p>
            <p><strong>Result:</strong> N=2×10⁶. All profiles agree when v is matched. Status: VERIFIED ✓</p>
            <p><strong>Note:</strong> This does NOT contradict GR gravitational time dilation — that involves
            different v in curved spacetime, which Ze handles via Ze-potential field coupling (future work).</p>
          `
        },
        fp4: {
          title: 'FP-4 — Emergent Speed Limit',
          status: '◎ Proposed — experimental design needed',
          body: `
            <p><strong>Formal statement:</strong> The dimensionless Ze velocity v = N_S/N ∈ [0,1) corresponds to
            a physical speed via v_physical = v_Ze · c_eff, where c_eff = λ·Δx/Δt is determined by the
            event rate λ and physical scale of the system.</p>
            <p><strong>Falsifiable claim:</strong> c_eff should be measurable in physical Ze implementations
            (e.g., photon emission chains in optical fibres) and match the speed of light when λ·Δx/Δt = c.</p>
            <p><strong>Status:</strong> No new postulate of c is needed — it emerges. Currently only theoretical;
            requires a physical Ze implementation with calibrated event rate.</p>
          `
        },
        fp5: {
          title: 'FP-5 — Causal Chain Length Formula',
          status: '✅ Computationally verified',
          body: `
            <p><strong>Formal statement:</strong> Define causal chain length L_c = N_T. Then:</p>
            <pre class="ze-code">τ = √(L_c² + 2·L_c·N_S)</pre>
            <p>This is algebraically equivalent to the Minkowski interval τ = √(T²−X²).</p>
            <p><strong>Result:</strong> τ_Ze and τ_L agree to machine precision for N=10⁶. Status: VERIFIED ✓</p>
            <p><strong>Interpretation:</strong> Proper time = causal chain length, a purely graph-theoretic quantity.
            No background manifold needed.</p>
          `
        },
        fp6: {
          title: 'FP-6 — Twin Paradox Without Frame Dependence',
          status: '✅ Computationally verified',
          body: `
            <p><strong>Formal statement:</strong> System A stays at rest (τ_A = 2N). System B travels at v₁ then v₂:</p>
            <pre class="ze-code">τ_B = N·√(1−v₁²) + N·√(1−v₂²)</pre>
            <p>Result determined by causal event graph of B, without reference to coordinate frames.</p>
            <p><strong>Result:</strong> N_seg=10⁶ per leg. τ_A/τ_B matches SR prediction to O(10⁻³). Status: VERIFIED ✓</p>
            <p><strong>Physical test:</strong> Hafele-Keating type experiment. Existing data (Hafele & Keating 1972)
            is consistent with Ze but was not designed to test Ze specifically (see EX-6).</p>
          `
        },
        fp7: {
          title: 'FP-7 — Quantum Coherence Continuous Tuning',
          status: '⚗ Experiment required — not yet tested',
          body: `
            <p><strong>Formal statement:</strong> Interference visibility V is continuously tunable via
            predictability parameter P of which-path information:</p>
            <pre class="ze-code">V = V₀ · √(1 − P²)</pre>
            <p>Ze is falsified if V cannot be continuously tuned by adjusting future which-path information
            access, or if the functional form deviates from √(1−P²).</p>
            <p><strong>Experimental apparatus (proposed):</strong> Double-slit with polarization-based which-path
            tagging, microcontroller feedback to tune P continuously from 0 to 1. Measure V(P) at 20 points.</p>
            <p><strong>Existing setup:</strong> Described in Tkemaladze (2026) DOI 10.65649/pt1hx971.</p>
            <p><strong>Status:</strong> PROPOSED — near-term experimental work required (see EX-1, EX-5).</p>
          `
        },
        fp8: {
          title: 'FP-8 — Structural Convergence with SR / Causal Sets / Twistor Theory',
          status: '~ SR verified; causal sets theoretical; twistor long-term',
          body: `
            <p><strong>SR convergence:</strong> In the large-N limit with v=N_S/N, Ze reproduces the Lorentz factor
            exactly. Proven analytically and verified numerically (FP-1). ✓</p>
            <p><strong>Causal sets:</strong> Ze event streams are a specific subclass of causal sets (Bombelli et al., 1987)
            with binary state space and Bernoulli-generated links. Ze predicts proper time equals causal set distance.
            Requires formal embedding proof — theoretically proposed.</p>
            <p><strong>Twistor theory:</strong> Ze spinor-twistor correspondence described in Ze Vector Theory §9.
            Long-term research goal — not yet a falsifiable prediction in the current programme.</p>
            <p><strong>Spin networks:</strong> Ze → twistor → spin network connection exists at the level of
            mathematical structure. Experimental test not yet specified.</p>
          `
        },
        ex1: {
          title: 'EX-1 — Ze Interferometer (Double-Slit + Polarization Tagging)',
          status: '⚗ Near-term experimental test',
          body: `
            <p><strong>Objective:</strong> Verify FP-7: V = V₀·√(1−P²) where P is continuously tunable.</p>
            <p><strong>Setup:</strong></p>
            <ul>
              <li>Single-photon source (SPDC BBO crystal)</li>
              <li>Double-slit with polarization-based which-path tagger</li>
              <li>Microcontroller feedback to tune tagging probability P ∈ [0, 1] in 20 steps</li>
              <li>Coincidence counting to measure fringe visibility V at each P</li>
              <li>Expected result: V(P) = √(1−P²), contrasted with standard QM V² + D² ≤ 1</li>
            </ul>
            <p><strong>Falsification:</strong> If V(P) deviates from √(1−P²) while satisfying V²+D² < 1,
            Ze Prediction P4 (FP-7) is falsified.</p>
            <p><strong>Reference:</strong> Apparatus description in DOI 10.65649/pt1hx971</p>
          `
        },
        ex2: {
          title: 'EX-2 — Optical Lattice Atomic Clock τ(v) Comparison',
          status: '⚗ Near-term (requires clock facility)',
          body: `
            <p><strong>Objective:</strong> Test FP-1 with sub-10⁻¹⁸ precision optical lattice clocks.</p>
            <p><strong>Setup:</strong></p>
            <ul>
              <li>Two optical lattice clocks (Sr or Yb) at different velocities (v ≈ 0.001–0.01 c via moving platform)</li>
              <li>Compare accumulated proper time after N = 10⁹ events</li>
              <li>Ze prediction: Δτ/τ = 1 − √(1−v²) ≈ v²/2 for small v</li>
              <li>Current state of art: NIST/PTB clocks achieve 10⁻¹⁸ precision — sufficient for Ze test at v ≈ 10⁻³</li>
            </ul>
            <p><strong>Distinction from existing tests:</strong> Standard Michelson-Morley and Hafele-Keating test SR.
            Ze test verifies additionally that τ depends only on the counter pair (N_T, N_S), not on implementation —
            a new structural claim (FP-2).</p>
          `
        },
        ex3: {
          title: 'EX-3 — Photonic Quantum Walk: Born vs Uniform Ze-Strategy',
          status: '⚗ Near-term (photonic chip)',
          body: `
            <p><strong>Objective:</strong> Verify Theorem 5.1 (Born rule minimises τ_Z depletion) in a physical system.</p>
            <p><strong>Setup:</strong></p>
            <ul>
              <li>Photonic waveguide array implementing a discrete-time quantum walk</li>
              <li>Programmable beam splitters to implement Born, Uniform, and Anti-Born measurement strategies</li>
              <li>Measure click statistics and compute empirical T-event rate for each strategy</li>
              <li>Ze prediction: R(Born) < R(Uniform) < R(Anti-Born) for all Hilbert space dimensions d > 2^θ_Q</li>
            </ul>
            <p><strong>Required:</strong> Programmable photonic chip (e.g., Xanadu, QuiX Quantum) with
            state preparation and measurement in arbitrary bases. ~2 weeks of experiment time.</p>
          `
        },
        ex4: {
          title: 'EX-4 — NMR T₁/T₂ Relaxation ↔ Ze T-Event Rate',
          status: '~ Third-party data exists',
          body: `
            <p><strong>Objective:</strong> Map NMR spin relaxation rates to Ze T-event rates.</p>
            <p><strong>Ze prediction:</strong> Spin relaxation time T₁ ∝ 1/R_T where R_T is the Ze T-event rate
            of the spin system modelled as a Ze-observer of its own thermal environment.</p>
            <p><strong>Existing data:</strong> Thousands of published T₁/T₂ measurements exist for organic molecules,
            proteins, and solids. Ze prediction can be tested against this data without new experiments.</p>
            <p><strong>Method:</strong></p>
            <ul>
              <li>Model spin as Ze-observer with θ_Z calibrated to thermal fluctuation energy</li>
              <li>Compute predicted T₁ from Ze OU simulation (Ze Level 2)</li>
              <li>Compare to published NIST/BMRB NMR data</li>
            </ul>
            <p><strong>Status:</strong> Analysis planned for Ze-NMR paper (Q3 2026).</p>
          `
        },
        ex5: {
          title: 'EX-5 — Quantum Eraser with Continuous Erasure',
          status: '⚗ Near-term quantum optics',
          body: `
            <p><strong>Objective:</strong> Test FP-7 via a quantum eraser with continuously variable erasure strength.</p>
            <p><strong>Setup:</strong></p>
            <ul>
              <li>Entangled photon pair (signal + idler)</li>
              <li>Signal photon through double-slit, idler photon through variable polarization rotator</li>
              <li>Vary rotation angle θ ∈ [0°, 90°]: at θ=0° full which-path info (D=1, V=0); at θ=90° full erasure (D=0, V=1)</li>
              <li>Ze maps this to P = cos(θ), predicts V(P) = √(1−P²) = sin(θ)</li>
              <li>Standard QM allows V² + D² ≤ 1 — Ze claims tight equality</li>
            </ul>
            <p><strong>Critical test:</strong> Standard QM allows V < sin(θ) for same D. Ze claims V = sin(θ) exactly.
            Deviations > 1% would falsify Ze Prediction P4.</p>
          `
        },
        ex6: {
          title: 'EX-6 — Hafele-Keating Style Ze Experiment',
          status: '~ Existing data (1972) is consistent but not designed for Ze',
          body: `
            <p><strong>Reference experiment:</strong> Hafele & Keating (1972) flew atomic clocks around the world
            and compared with ground-based clocks. Result: τ_ground > τ_air (eastbound) by ~59 ns.</p>
            <p><strong>Ze interpretation:</strong> The flying clock accumulates fewer T-events (N_T) due to
            higher v = N_S/N. FP-6 predicts the exact ratio without reference to coordinate frames.</p>
            <p><strong>New Ze-specific test needed:</strong></p>
            <ul>
              <li>Fly two clocks on the same aircraft but with different internal event rates λ (different isotopes)</li>
              <li>Ze FP-2 predicts: if both clocks have same effective v but different λ, they give same τ</li>
              <li>This tests the implementation equivalence claim that Hafele-Keating did not test</li>
            </ul>
            <p><strong>Estimated cost:</strong> Commercial airline flight + two portable optical clocks (~$200k).</p>
          `
        },
        ex7: {
          title: 'EX-7 — Molecular Dynamics: OU Thermostat Ze-Entropy Correspondence',
          status: '✓ Simulated in Ze Level 2 (this website)',
          body: `
            <p><strong>Objective:</strong> Verify that Ze-entropy S_Ze = k_B·ln(1+Ω) co-increases with
            S_Boltzmann during non-equilibrium thermalization.</p>
            <p><strong>Current simulation (Level 2):</strong> OU thermostat, cold start v=0,
            Spearman ρ ≈ 0.66 during first 50 steps (thermalization phase). ✓</p>
            <p><strong>Physical realisation:</strong></p>
            <ul>
              <li>Colloidal particle in a harmonic trap (optical tweezer)</li>
              <li>Trap suddenly displaced → particle thermalizes from new starting point</li>
              <li>Track position via high-speed camera (10 kHz), compute surprises vs EMA prediction</li>
              <li>Ze prediction: Spearman ρ(S_Ze, S_Boltzmann) > 0.6 during thermalization transient</li>
            </ul>
            <p><strong>Reference:</strong> Mazonka & Jarzynski (1999) optical tweezer thermalisation.
            Reanalysis with Ze framework planned.</p>
          `
        },
        ex8: {
          title: 'EX-8 — Biological τ_Z: HRV as Ze-Observer Readout',
          status: '◎ Proposed — data collection phase',
          body: `
            <p><strong>Hypothesis:</strong> The heart is a Ze-observer of its own vascular environment.
            Heart rate variability (HRV) reflects the T-event rate of the cardiac Ze-system.</p>
            <p><strong>Ze prediction:</strong></p>
            <ul>
              <li>Healthy young subject (high τ_Z): low T-event rate, high HRV (SDNN > 50 ms)</li>
              <li>Stressed / aging subject (low τ_Z): high T-event rate, low HRV</li>
              <li>Quantitative mapping: τ_Z_depletion_rate = α · (1/HRV) with α fitted to age+stress data</li>
            </ul>
            <p><strong>Connection to CDATA:</strong> Aging Ze-systems (centrioles) have reduced τ_Z;
            this should manifest as reduced HRV in CDATA-predicted aging trajectory.</p>
            <p><strong>Data needed:</strong> Longitudinal HRV from wearables (Garmin/Apple Watch) in
            100 subjects aged 20–80, aligned with AIM health indicators.</p>
          `
        }
      };

      function zeShowDetail(key) {
        const d = ZE_DETAILS[key];
        if (!d) return;
        document.getElementById('ze-detail-content').innerHTML =
          `<h3>${d.title}</h3><div class="ze-detail-status">${d.status}</div><div class="ze-detail-body">${d.body}</div>`;
        document.getElementById('ze-detail-panel').style.display = 'block';
        document.getElementById('ze-detail-overlay').style.display = 'block';
        document.querySelectorAll('.ze-drop-panel').forEach(p => p.classList.remove('ze-drop-open'));
      }

      function zeCloseDetail() {
        document.getElementById('ze-detail-panel').style.display = 'none';
        document.getElementById('ze-detail-overlay').style.display = 'none';
      }
    </script>
    """
  end
end
