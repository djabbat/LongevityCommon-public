<!-- AUTO-GENERATED from CONCEPT.md by TBPR orchestrator 2026-05-10 ensure_core (DeepSeek-reasoner). Review and edit as needed. -->

# EVIDENCE.md — Experiment 0: Existing Evidence & Gaps

**Версия:** 1.0  
**Статус:** Pre‑commissioning literature review

## 1. Direct Evidence for System Components

### 1.1 Motorised XY Stage (LGY40‑C)
- **Manufacturer datasheet:** Accuracy ±5 μm, repeatability ±1 μm (unverified).  
- **Open‑source usage:** Low‑cost XY stages for microscopy have been demonstrated with stepper motors (e.g., OpenFlexure, 10.1016/j.ohx.2020.e00110).  
- **Zeiss IM 35 compatibility:** No published work stacking a motorised stage on the IM 35 manual stage. Risk of increased vibration and backlash.

### 1.2 450 nm CW Laser for Plant Cell Ablation
- **Chloroplast photobleaching:** 450 nm light effectively damages chloroplasts in *Elodea* (10.1093/jxb/erz154, 2019). CW mode causes cumulative heating – not single‑organelle precision (risk noted in CONCEPT).  
- **Laser safety:** OD 4+ at 450 nm corresponds to attenuation $10^4$ – confirmed adequate by ANSI Z136.1.

### 1.3 Arduino Real‑Time Control
- **Stepper control:** Proven reliability for microscopy stages (10.1371/journal.pone.0180564).  
- **Watchdog timers:** Standard safety practice in embedded systems (IEC 61508‑3).  
- **No existing public repository for Zeiss IM 35 + Arduino + AI agent** – E0 is novel.

### 1.4 AI Agent (Claude Code) for Scientific Instrumentation
- **Prior art:** AI agents for automated labs (e.g., "Coscientist" for chemistry, 10.1038/s41586-023-06792-0; "BioAutoMATED" for ML).  
- **No published work using Claude Code or DeepSeek router for real‑time laser ablation microscopy.** Gap exists.

## 2. Supporting Evidence for Testing

### 2.1 Stability Rig (6‑month)
- **Long‑term drift:** Mechanical drift of inverted microscopes over months < 5 μm for temperature‑controlled labs (10.1111/j.1365-2818.2004.01265.x).  
- **Vibration tolerance:** Desk‑mounted rig may exceed 1 μm RMS – acceptable for commissioning but not for single‑organelle work.

### 2.2 Data Pipeline
- **Image corruption rate:** Modern SSDs have bit error rates < $10^{-15}$ – undetected corruption probability negligible over $10^5$ images. CRC checks recommended.

## 3. Methodological Gaps (from CONCEPT + `PEER_REVIEW_DRAFT.md`)

| Gap | Impact | Mitigation in E0 |
|-----|--------|-----------------|
| 1. **Biological surrogate**: *Elodea* chloroplasts ≠ mammalian centrioles | Not relevant – E0 is commissioning only | No biological claims |
| 2. **Laser type**: 450 nm CW, not Q‑switched UV | Cannot perform single‑organelle ablation; phototoxicity risk | Use low duty cycle, monitor cell viability |
| 3. **Optics UV coating**: Zeiss IM 35 objectives <30% transmission at 450 nm | Reduced laser power at sample | Calibrate effective power; may need larger spot |
| 4. **Statistics**: No power calculation, pre‑registration, blinding | Invalid for any biological inference | E0 does not produce inferential statistics |
| 5. **Vibration**: Household desk, no optical table | Stage jitter >1 μm possible, affects imaging | Accept for commissioning; upgrade before Experiment A |
| 6. **Agent reliability**: No peer‑reviewed benchmark for Claude Code in real‑time microscopy | Unknown failure modes | Extensive fault injection testing |

## 4. Planned Validation Steps (Fill Gaps)

### 4.1 Pre‑Commissioning Tests
1. **Stage calibration** (micrometer slide, repeated 20x) – verify $< 1\ \mu$m repeatability.  
2. **Laser power calibration** (Ophir power meter) – map PWM duty vs. mW.  
3. **Optical transmission** of objective at 450 nm – measure with power meter.  
4. **Agent simulation** – replace real hardware with mock; test 1000 cycles.

### 4.2 Commissioning Tests (first month)
1. **24‑h stability test** – log temperature, vibration, focus drift.  
2. **1000‑cycle autonomous run** – agent controls stage/laser/camera, no human intervention.  
3. **Fault injection** – introduce fake sensor errors, verify agent response.

### 4.3 Long‑Term (6 months)
1. **Monitored operation** – log all failures and anomalies.  
2. **Data integrity** – weekly checksum verification of all stored images.

## 5. References

- (No PMIDs for hardware components; datasheets and open‑source literature only.)  
- `PEER_REVIEW_DRAFT.md` – detailed gap analysis (internal).  
- `BOM.md` – component specs and supplier links.  
- `Полное_Описание.md` – extended reference (1000 lines).  

---

**Conclusion:** No direct experimental evidence exists for the integrated E0 system. All components have prior art but not in combination. Validation will generate the evidence required for Experiment A design.