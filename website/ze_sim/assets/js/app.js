// If you want to use Phoenix channels, run `mix help phx.gen.channel`
// to get started and then uncomment the line below.
// import "./user_socket.js"

// You can include dependencies in two ways.
//
// The simplest option is to put them in assets/vendor and
// import them using relative paths:
//
//     import "../vendor/some-package.js"
//
// Alternatively, you can `npm install some-package --prefix assets` and import
// them using a path starting with the package name:
//
//     import "some-package"
//

// Include phoenix_html to handle method=PUT/DELETE in forms and buttons.
import "phoenix_html"
// Establish Phoenix Socket and LiveView configuration.
import {Socket} from "phoenix"
import {LiveSocket} from "phoenix_live_view"
import topbar from "../vendor/topbar"

// ── Ze Chart hooks (Chart.js loaded via CDN in root layout) ──────────────────

function makeLineChart(canvas, datasets, xlabel) {
  if (canvas._zeChart) { canvas._zeChart.destroy(); }
  const steps = datasets[0].data.length;
  const labels = Array.from({length: steps}, (_, i) => i + 1);
  canvas._zeChart = new Chart(canvas, {
    type: "line",
    data: { labels, datasets },
    options: {
      animation: false,
      responsive: false,
      plugins: { legend: { position: "top" } },
      scales: {
        x: { title: { display: true, text: xlabel } },
        y: { title: { display: true, text: "Value" } }
      }
    }
  });
}

const ThermoChart = {
  mounted() { this.draw(); },
  updated() { this.draw(); },
  draw() {
    const el = this.el;
    const sZe    = JSON.parse(el.dataset.sZe);
    const sBoltz = JSON.parse(el.dataset.sBoltz);
    const tau    = JSON.parse(el.dataset.tau);
    makeLineChart(el, [
      { label: "S_Ze",         data: sZe,    borderColor: "#2563eb", tension: 0.2, pointRadius: 0 },
      { label: "S_Boltzmann",  data: sBoltz, borderColor: "#16a34a", tension: 0.2, pointRadius: 0, borderDash: [6,3] },
      { label: "Mean τ_Z",     data: tau,    borderColor: "#dc2626", tension: 0.2, pointRadius: 0, yAxisID: "y2" }
    ], "Step");
  }
};

const ReproChart = {
  mounted() { this.draw(); },
  updated() { this.draw(); },
  draw() {
    const el      = this.el;
    const born    = JSON.parse(el.dataset.born);
    const uniform = JSON.parse(el.dataset.uniform);
    const ds      = JSON.parse(el.dataset.ds); // [[p_T, V_ze, V_qm], ...]

    if (el._zeChart)  { el._zeChart.destroy();  }
    if (el._dsChart)  { el._dsChart.destroy();  }

    // Canvas 1: τ_Z chain histories (Born vs Uniform)
    const steps  = Array.from({length: born.length}, (_, i) => i);
    el._zeChart = new Chart(el, {
      type: "line",
      data: {
        labels: steps,
        datasets: [
          { label: "Born τ_Z",    data: born,    borderColor: "#2563eb", tension: 0.1, pointRadius: 0 },
          { label: "Uniform τ_Z", data: uniform, borderColor: "#dc2626", tension: 0.1, pointRadius: 0 }
        ]
      },
      options: {
        animation: false, responsive: false,
        plugins: { legend: { position: "top" },
          title: { display: true, text: "τ_Z along first chain (Born vs Uniform)" } },
        scales: {
          x: { title: { display: true, text: "Step" } },
          y: { title: { display: true, text: "τ_Z" } }
        }
      }
    });

    // Canvas 2: double-slit visibility V_ze vs V_qm (if second canvas exists)
    const dsCanvas = document.getElementById("repro-ds-chart");
    if (dsCanvas) {
      const ptLabels = ds.map(([p]) => p.toFixed(2));
      const vZe  = ds.map(([_p, v])   => v);
      const vQM  = ds.map(([_p, _v, q]) => q);
      dsCanvas._dsChart = new Chart(dsCanvas, {
        type: "line",
        data: {
          labels: ptLabels,
          datasets: [
            { label: "V_Ze = 1−2p_T",      data: vZe, borderColor: "#2563eb", tension: 0.3, pointRadius: 3 },
            { label: "V_QM = √(1−p_T²)",   data: vQM, borderColor: "#16a34a", tension: 0.3, pointRadius: 3, borderDash: [6,3] }
          ]
        },
        options: {
          animation: false, responsive: false,
          plugins: { legend: { position: "top" },
            title: { display: true, text: "Ze vs QM Visibility (Prediction P4 vs Englert 1996)" } },
          scales: {
            x: { title: { display: true, text: "Detector efficiency p_T" } },
            y: { min: 0, max: 1, title: { display: true, text: "Fringe visibility V" } }
          }
        }
      });
    }
  }
};

const QuantumChart = {
  mounted() { this.draw(); },
  updated() { this.draw(); },
  draw() {
    const el   = this.el;
    const born    = JSON.parse(el.dataset.born);
    const uniform = JSON.parse(el.dataset.uniform);
    const anti    = JSON.parse(el.dataset.anti);
    makeLineChart(el, [
      { label: "Born (optimal)", data: born,    borderColor: "#2563eb", tension: 0.1, pointRadius: 0 },
      { label: "Uniform",        data: uniform, borderColor: "#f59e0b", tension: 0.1, pointRadius: 0 },
      { label: "Anti-Born",      data: anti,    borderColor: "#dc2626", tension: 0.1, pointRadius: 0 }
    ], "Step");
  }
};

// ── Ze Dynamics — Cosmic Ze-History Simulation ───────────────────────────────
//
// Simulates the evolution of Ze-systems from the Big Bang to today.
// 4 initial Ze-systems (Planck epoch). T-events spawn daughters; τ_Z → 0 terminates.
// Time is logarithmically compressed: events are extremely fast at the Big Bang,
// slowing to near-real-time at the present day.
//
// Cosmic epochs: Planck → GUT → Inflation → Electroweak → Quark → Hadron →
//               Lepton → BBN → CMB → First Stars → Galaxies → Solar System →
//               Life on Earth → Homo sapiens → Present.

// ── Cosmic epoch table ────────────────────────────────────────────────────────
const ZE_COSMIC_EPOCHS = [
  { t: 0,         name: "Planck Epoch",           tLabel: "t = 10⁻⁴³ s",   color: "#ffffff", ms: 15,   pT: 0.95, tau0: 100 },
  { t: 1e-43,     name: "Grand Unification",       tLabel: "t = 10⁻⁴³ s",   color: "#e8d4ff", ms: 20,   pT: 0.88, tau0: 90  },
  { t: 1e-36,     name: "Inflation",               tLabel: "t = 10⁻³⁶ s",   color: "#cc88ff", ms: 28,   pT: 0.80, tau0: 82  },
  { t: 1e-32,     name: "Reheating",               tLabel: "t = 10⁻³² s",   color: "#bb55ff", ms: 40,   pT: 0.72, tau0: 74  },
  { t: 1e-12,     name: "Electroweak Symmetry",    tLabel: "t = 10⁻¹² s",   color: "#ff88cc", ms: 60,   pT: 0.62, tau0: 64  },
  { t: 1e-6,      name: "Quark Epoch",             tLabel: "t = 10⁻⁶ s",    color: "#ff4466", ms: 90,   pT: 0.52, tau0: 55  },
  { t: 1e-4,      name: "Hadron Epoch",            tLabel: "t = 10⁻⁴ s",    color: "#ff6633", ms: 130,  pT: 0.44, tau0: 48  },
  { t: 1,         name: "Lepton Epoch",            tLabel: "t = 1 s",        color: "#ff9944", ms: 190,  pT: 0.38, tau0: 42  },
  { t: 60,        name: "Big Bang Nucleosynthesis", tLabel: "t = 3 min",      color: "#ffcc44", ms: 280,  pT: 0.32, tau0: 36  },
  { t: 1e13,      name: "Matter Domination",       tLabel: "t = 50 kyr",     color: "#88ccff", ms: 420,  pT: 0.26, tau0: 30  },
  { t: 1.2e13,    name: "Recombination / CMB",     tLabel: "t = 380 kyr",    color: "#44aaff", ms: 600,  pT: 0.20, tau0: 26  },
  { t: 4.7e15,    name: "First Stars",             tLabel: "t = 150 Myr",    color: "#3388ff", ms: 800,  pT: 0.16, tau0: 22  },
  { t: 3e16,      name: "Galaxy Formation",        tLabel: "t = 1 Gyr",      color: "#2255ee", ms: 1000, pT: 0.13, tau0: 19  },
  { t: 2e17,      name: "Milky Way Forms",         tLabel: "t = 6 Gyr",      color: "#1144cc", ms: 1200, pT: 0.10, tau0: 16  },
  { t: 2.9e17,    name: "Solar System",            tLabel: "t = 9.2 Gyr",    color: "#0088aa", ms: 1500, pT: 0.08, tau0: 14  },
  { t: 4.1e17,    name: "Life on Earth",           tLabel: "t = 13.0 Gyr",   color: "#00aa77", ms: 1800, pT: 0.06, tau0: 12  },
  { t: 4.34e17,   name: "Homo sapiens",            tLabel: "t = 13.75 Gyr",  color: "#00cc88", ms: 2100, pT: 0.05, tau0: 10  },
  { t: 4.355e17,  name: "Present",                 tLabel: "t = 13.8 Gyr",   color: "#00ff99", ms: 2500, pT: 0.04, tau0: 8   },
];

// ── Utilities ─────────────────────────────────────────────────────────────────
let _zeIdCounter = 0;
function hexRgb(hex) {
  const r = parseInt(hex.slice(1,3),16), g = parseInt(hex.slice(3,5),16), b = parseInt(hex.slice(5,7),16);
  return `${r},${g},${b}`;
}
function clamp(v,lo,hi) { return Math.max(lo, Math.min(hi, v)); }

// ── Ze Dynamics hook ──────────────────────────────────────────────────────────
const ZeDynamics = {
  mounted() {
    this.canvas = document.getElementById('ze-canvas');
    this.ctx    = this.canvas.getContext('2d');
    this.W      = this.canvas.width;
    this.H      = this.canvas.height;
    this.ANIM_H = this.H - 90; // reserve bottom for timeline

    // Simulation state
    this.systems     = [];
    this.connections = [];
    this.epochIdx    = 0;
    this.stepCount   = 0;
    this.totalSpawn  = 0;
    this.totalTEvt   = 0;
    this.running     = false;
    this.timer       = null;

    // Generate static star field
    this.stars = Array.from({length: 200}, () => ({
      x: Math.random() * this.W,
      y: Math.random() * this.ANIM_H,
      r: Math.random() * 1.4 + 0.3,
      a: Math.random() * 0.6 + 0.2,
      twinkle: Math.random() * Math.PI * 2
    }));
    this.starTick = 0;

    this._initSystems();
    this._drawFrame();

    // Controls
    const btn  = document.getElementById('ze-btn-toggle');
    const rst  = document.getElementById('ze-btn-reset');
    if (btn) btn.addEventListener('click', () => this.running ? this._pause() : this._start());
    if (rst) rst.addEventListener('click', () => this._reset());
  },

  destroyed() {
    clearTimeout(this.timer);
  },

  // ── Init 4 Ze-systems at Big Bang point (center) ────────────────────────────
  _initSystems() {
    _zeIdCounter = 0;
    this.systems = [];
    this.connections = [];
    this.epochIdx = 0;
    this.stepCount = 0;
    this.totalSpawn = 4;
    this.totalTEvt = 0;
    const cx = this.W / 2, cy = this.ANIM_H / 2;
    const ep = ZE_COSMIC_EPOCHS[0];
    for (let i = 0; i < 4; i++) {
      const angle = (i / 4) * Math.PI * 2;
      const dist  = 18;
      this.systems.push({
        id: _zeIdCounter++, x: cx + Math.cos(angle)*dist, y: cy + Math.sin(angle)*dist,
        vx: Math.cos(angle)*0.3, vy: Math.sin(angle)*0.3,
        tau: ep.tau0, tau0: ep.tau0, era: 0, color: ep.color,
        parentId: null, age: 0, alive: true, birthStep: 0, flash: 0
      });
    }
    this._updateStats();
  },

  _start() {
    this.running = true;
    const btn = document.getElementById('ze-btn-toggle');
    if (btn) btn.textContent = '⏸ Pause';
    this._scheduleNext();
  },

  _pause() {
    this.running = false;
    clearTimeout(this.timer);
    const btn = document.getElementById('ze-btn-toggle');
    if (btn) btn.textContent = '▶ Continue';
  },

  _reset() {
    this._pause();
    this._initSystems();
    this._drawFrame();
    const btn = document.getElementById('ze-btn-toggle');
    if (btn) btn.textContent = '▶ Start';
  },

  _scheduleNext() {
    const ep = ZE_COSMIC_EPOCHS[Math.min(this.epochIdx, ZE_COSMIC_EPOCHS.length - 1)];
    this.timer = setTimeout(() => {
      this._step();
      if (this.running) this._scheduleNext();
    }, ep.ms);
  },

  // ── Simulation step ──────────────────────────────────────────────────────────
  _step() {
    const N_EPOCHS  = ZE_COSMIC_EPOCHS.length;
    const STEPS_PER_EPOCH = 14;
    const MAX_LIVE = 72;
    const SPAWN_RADIUS = 50;

    this.epochIdx = Math.min(Math.floor(this.stepCount / STEPS_PER_EPOCH), N_EPOCHS - 1);
    const ep = ZE_COSMIC_EPOCHS[this.epochIdx];
    const cx = this.W / 2, cy = this.ANIM_H / 2;
    const alive = () => this.systems.filter(s => s.alive);

    const newSys = [];
    for (const s of this.systems) {
      if (!s.alive) { s.age++; continue; }
      s.age++;
      s.flash = Math.max(0, s.flash - 0.15);

      // Slight Brownian motion + cosmological expansion drift
      s.vx += (Math.random()-0.5)*0.4 + (s.x - cx)*0.0004;
      s.vy += (Math.random()-0.5)*0.4 + (s.y - cy)*0.0004;
      // Soft repulsion from neighbours
      for (const o of this.systems) {
        if (o.id === s.id || !o.alive) continue;
        const dx = s.x - o.x, dy = s.y - o.y, d2 = dx*dx + dy*dy;
        if (d2 < 1600 && d2 > 0.01) {
          const d = Math.sqrt(d2);
          s.vx += dx/d * 1.8/d;
          s.vy += dy/d * 1.8/d;
        }
      }
      s.vx = clamp(s.vx, -2.2, 2.2) * 0.88;
      s.vy = clamp(s.vy, -2.2, 2.2) * 0.88;
      s.x  = clamp(s.x + s.vx, 12, this.W - 12);
      s.y  = clamp(s.y + s.vy, 12, this.ANIM_H - 12);

      // T-event or S-event
      if (Math.random() < ep.pT) {
        s.tau--;
        s.flash = 1.0;
        this.totalTEvt++;
        if (s.tau <= 0) {
          s.alive = false;
        } else if (alive().length + newSys.length < MAX_LIVE) {
          // Axiom Z4: T-event spawns a daughter Ze-system
          const ang  = Math.random() * Math.PI * 2;
          const dist = 15 + Math.random() * SPAWN_RADIUS;
          const d = {
            id: _zeIdCounter++,
            x: clamp(s.x + Math.cos(ang)*dist, 12, this.W - 12),
            y: clamp(s.y + Math.sin(ang)*dist, 12, this.ANIM_H - 12),
            vx: Math.cos(ang)*0.6, vy: Math.sin(ang)*0.6,
            tau: ep.tau0, tau0: ep.tau0, era: this.epochIdx, color: ep.color,
            parentId: s.id, age: 0, alive: true, birthStep: this.stepCount, flash: 0.8
          };
          newSys.push(d);
          this.connections.push({ from: s.id, to: d.id, born: this.stepCount, era: this.epochIdx });
          this.totalSpawn++;
        }
      }
    }
    this.systems.push(...newSys);

    // Prune old dead systems (keep visual history bounded)
    const deadOld = this.systems.filter(s => !s.alive && s.age > 60);
    if (deadOld.length > 8) {
      const remove = new Set(deadOld.slice(0, deadOld.length - 4).map(s => s.id));
      this.systems     = this.systems.filter(s => !remove.has(s.id));
      this.connections = this.connections.filter(c => !remove.has(c.from) && !remove.has(c.to));
    }
    // Prune old connections
    this.connections = this.connections.filter(c => this.stepCount - c.born < 60);

    this.stepCount++;
    this.starTick++;
    this._drawFrame();
    this._updateStats();
  },

  // ── Draw ──────────────────────────────────────────────────────────────────────
  _drawFrame() {
    const ctx = this.ctx, W = this.W, H = this.H, AH = this.ANIM_H;
    const ep  = ZE_COSMIC_EPOCHS[Math.min(this.epochIdx, ZE_COSMIC_EPOCHS.length - 1)];

    // ── Background ──
    ctx.fillStyle = '#05050f';
    ctx.fillRect(0, 0, W, H);

    // Subtle vignette
    const vig = ctx.createRadialGradient(W/2, AH/2, AH*0.2, W/2, AH/2, AH*0.9);
    vig.addColorStop(0, 'rgba(0,0,0,0)');
    vig.addColorStop(1, 'rgba(0,0,20,0.6)');
    ctx.fillStyle = vig;
    ctx.fillRect(0, 0, W, AH);

    // Twinkling stars
    for (const s of this.stars) {
      const a = s.a * (0.6 + 0.4 * Math.sin(this.starTick * 0.04 + s.twinkle));
      ctx.beginPath();
      ctx.arc(s.x, s.y, s.r, 0, Math.PI*2);
      ctx.fillStyle = `rgba(255,255,255,${a.toFixed(2)})`;
      ctx.fill();
    }

    // Era radial glow from center
    const cx = W/2, cy = AH/2;
    const eGrd = ctx.createRadialGradient(cx, cy, 0, cx, cy, 180);
    eGrd.addColorStop(0, `rgba(${hexRgb(ep.color)},0.06)`);
    eGrd.addColorStop(1, 'rgba(0,0,0,0)');
    ctx.fillStyle = eGrd;
    ctx.fillRect(0, 0, W, AH);

    // ── Connection lines ──
    for (const c of this.connections) {
      const p = this.systems.find(s => s.id === c.from);
      const d = this.systems.find(s => s.id === c.to);
      if (!p || !d) continue;
      const age = this.stepCount - c.born;
      const alpha = Math.max(0, 0.55 - age * 0.012);
      if (alpha < 0.01) continue;
      ctx.beginPath();
      ctx.moveTo(p.x, p.y);
      ctx.lineTo(d.x, d.y);
      ctx.strokeStyle = `rgba(${hexRgb(ZE_COSMIC_EPOCHS[c.era].color)},${alpha.toFixed(2)})`;
      ctx.lineWidth = 0.9;
      ctx.stroke();
    }

    // ── Ze-system circles ──
    for (const s of this.systems) {
      const frac = s.alive ? s.tau / s.tau0 : 0;
      const r    = s.alive ? 4 + 9 * frac : 2.5;
      const rgb  = hexRgb(s.color);

      if (s.alive && s.flash > 0.05) {
        // Flash burst on T-event
        const grd = ctx.createRadialGradient(s.x, s.y, 0, s.x, s.y, r * 3.5);
        grd.addColorStop(0, `rgba(${rgb},${(s.flash*0.7).toFixed(2)})`);
        grd.addColorStop(1, 'rgba(0,0,0,0)');
        ctx.beginPath(); ctx.arc(s.x, s.y, r*3.5, 0, Math.PI*2);
        ctx.fillStyle = grd; ctx.fill();
      }
      if (s.alive && frac > 0.2) {
        // Soft glow
        const gl = ctx.createRadialGradient(s.x, s.y, r*0.4, s.x, s.y, r*2.2);
        gl.addColorStop(0, `rgba(${rgb},0.28)`);
        gl.addColorStop(1, 'rgba(0,0,0,0)');
        ctx.beginPath(); ctx.arc(s.x, s.y, r*2.2, 0, Math.PI*2);
        ctx.fillStyle = gl; ctx.fill();
      }

      // Circle body
      ctx.beginPath(); ctx.arc(s.x, s.y, r, 0, Math.PI*2);
      ctx.fillStyle = s.alive ? `rgba(${rgb},${(0.7 + 0.3*frac).toFixed(2)})` : `rgba(60,60,60,0.3)`;
      ctx.fill();

      // τ_Z label inside circle
      if (s.alive && r >= 7) {
        ctx.fillStyle = frac > 0.5 ? '#000' : '#fff';
        ctx.font = `bold ${Math.round(r * 0.85)}px monospace`;
        ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
        ctx.fillText(s.tau, s.x, s.y);
      }
    }

    // ── Era header strip ──
    ctx.fillStyle = `rgba(${hexRgb(ep.color)},0.12)`;
    ctx.fillRect(0, 0, W, 38);
    // Era name
    ctx.fillStyle = ep.color;
    ctx.font = 'bold 15px system-ui';
    ctx.textAlign = 'left'; ctx.textBaseline = 'middle';
    ctx.fillText('Ze · ' + ep.name, 14, 19);
    // Cosmic time
    ctx.fillStyle = `rgba(${hexRgb(ep.color)},0.8)`;
    ctx.font = '12px monospace';
    ctx.textAlign = 'center';
    ctx.fillText(ep.tLabel, W/2, 19);
    // Event speed (top right)
    const speedPct = (ep.pT * 100).toFixed(0);
    ctx.textAlign = 'right'; ctx.fillStyle = 'rgba(255,255,255,0.55)';
    ctx.font = '11px monospace';
    ctx.fillText(`T-rate: ${speedPct}% of Planck`, W - 14, 19);

    // ── Cosmic timeline bar ──
    this._drawTimeline(ctx, W, AH);
  },

  _drawTimeline(ctx, W, AH) {
    const N    = ZE_COSMIC_EPOCHS.length;
    const bx   = 10, by = AH + 8, bw = W - 20, bh = 22;
    const prog = Math.min(this.epochIdx, N - 1) / (N - 1);

    // BG
    ctx.fillStyle = 'rgba(255,255,255,0.04)';
    ctx.beginPath(); ctx.roundRect(bx, by, bw, bh, 4); ctx.fill();

    // Colored epoch segments
    for (let i = 0; i < N - 1; i++) {
      const x1 = bx + (i / (N-1)) * bw;
      const x2 = bx + ((i+1)/(N-1)) * bw;
      ctx.fillStyle = ZE_COSMIC_EPOCHS[i].color + '40';
      ctx.fillRect(x1, by, x2 - x1, bh);
    }

    // Progress fill
    ctx.fillStyle = 'rgba(255,255,255,0.15)';
    ctx.fillRect(bx, by, prog * bw, bh);

    // Current epoch cursor
    const cx = bx + prog * bw;
    ctx.fillStyle = 'white';
    ctx.fillRect(cx - 1.5, by - 5, 3, bh + 10);

    // Milestone labels
    const milestones = [
      {i:0, label:"Big Bang", align:"left"},
      {i:4, label:"Electroweak",align:"center"},
      {i:8, label:"BBN",align:"center"},
      {i:10,label:"CMB",align:"center"},
      {i:12,label:"Galaxies",align:"center"},
      {i:14,label:"Solar System",align:"center"},
      {i:15,label:"Life",align:"center"},
      {i:17,label:"Now",align:"right"}
    ];
    ctx.font = '9px system-ui'; ctx.textBaseline = 'bottom';
    for (const m of milestones) {
      const lx = bx + (m.i/(N-1)) * bw;
      ctx.textAlign = m.align;
      ctx.fillStyle = ZE_COSMIC_EPOCHS[m.i].color;
      ctx.fillText(m.label, lx, by - 3);
    }

    // Speed bar (below timeline)
    const ep = ZE_COSMIC_EPOCHS[Math.min(this.epochIdx, N-1)];
    const sbx = bx, sby = by + bh + 6, sbw = bw, sbh = 8;
    ctx.fillStyle = 'rgba(255,255,255,0.05)'; ctx.fillRect(sbx, sby, sbw, sbh);
    const speedFrac = ep.pT; // 0.04..0.95
    const sg = ctx.createLinearGradient(sbx, 0, sbx+sbw, 0);
    sg.addColorStop(0, '#ff4466'); sg.addColorStop(0.5, '#ffcc44'); sg.addColorStop(1, '#00ff99');
    ctx.fillStyle = sg;
    ctx.fillRect(sbx, sby, sbw * speedFrac, sbh);

    // Speed label
    ctx.fillStyle = 'rgba(255,255,255,0.45)'; ctx.font = '10px monospace';
    ctx.textAlign = 'left'; ctx.textBaseline = 'top';
    const evtPerSec = Math.round(1000 / ep.ms);
    ctx.fillText(`Event speed: ${evtPerSec} steps/s  (${speedPct(ep)} % of Planck)`, sbx, sby + sbh + 3);
  },

  _updateStats() {
    const alive = this.systems.filter(s => s.alive);
    const maxTau = alive.reduce((m, s) => Math.max(m, s.tau), 0);
    const ep = ZE_COSMIC_EPOCHS[Math.min(this.epochIdx, ZE_COSMIC_EPOCHS.length-1)];
    const set = (id, v) => { const el = document.getElementById(id); if (el) el.textContent = v; };
    set('ze-stat-active',  alive.length);
    set('ze-stat-era',     ep.name);
    set('ze-stat-spawned', this.totalSpawn);
    set('ze-stat-tevents', this.totalTEvt);
    set('ze-stat-maxtau',  maxTau);
  }
};

function speedPct(ep) { return (ep.pT * 100).toFixed(0); }

let csrfToken = document.querySelector("meta[name='csrf-token']").getAttribute("content")
let liveSocket = new LiveSocket("/live", Socket, {
  longPollFallbackMs: 2500,
  params: {_csrf_token: csrfToken},
  hooks: { ThermoChart, QuantumChart, ReproChart, ZeDynamics }
})

// Show progress bar on live navigation and form submits
topbar.config({barColors: {0: "#29d"}, shadowColor: "rgba(0, 0, 0, .3)"})
window.addEventListener("phx:page-loading-start", _info => topbar.show(300))
window.addEventListener("phx:page-loading-stop", _info => topbar.hide())

// connect if there are any LiveViews on the page
liveSocket.connect()

// expose liveSocket on window for web console debug logs and latency simulation:
// >> liveSocket.enableDebug()
// >> liveSocket.enableLatencySim(1000)  // enabled for duration of browser session
// >> liveSocket.disableLatencySim()
window.liveSocket = liveSocket

