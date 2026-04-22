# Automated Time-Lapse Microscopy — Inženernoe reshenie + budget

**Цель:** Round-the-clock наблюдение за BJ-hTERT fibroblast cultures в Phase A CDATA эксперименте. Автоматическое сканирование, auto-focus, периодический imaging (brightfield + fluorescence), remote monitoring.

**Базовая платформа:** Zeiss IM 35 inverted microscope (существующий)

---

## 🎯 Функциональные требования

1. **Automated XY movement** — 96-well plate или multi-dish scanning (5-10 positions per session)
2. **Automated Z-focus** — autofocus на каждой position для long-term drift compensation
3. **Time-lapse scheduling** — imaging every 30-60 min, 24/7, до 3 недель непрерывно
4. **Brightfield + fluorescence imaging** — GT335/polyE (FITC) + Ninein (TRITC), caught in same session
5. **Live cell environment** — 37°C, 5% CO₂, humidity control, sterile
6. **Remote monitoring** — Jaba может смотреть из дома без физического присутствия
7. **Data storage & auto-processing** — NAS или cloud backup, ImageJ/Fiji batch pipeline
8. **Power redundancy** — UPS (минимум 2 часа при blackout)
9. **Alert system** — SMS/email если что-то нарушилось (temperature drift, CO₂ loss, camera crash)

---

## 🔧 Инженерное решение — 3 варианта

### Вариант A — Entry-level retrofit ($3,500–5,000) 🎯 RECOMMENDED для Phase A

**Philosophy:** DIY + open-source. Максимально дёшево, достаточно для 6-месячного Phase A.

| Компонент | Модель / источник | Цена |
|-----------|-------------------|------|
| **Motorized XY stage adapter** | Arduino CNC-shield + stepper motors + linear rails (custom) or OpenFlexure-style DIY kit | $500 |
| **Motorized focus drive** | NEMA-17 stepper + belt drive on Zeiss coarse knob (3D-printed mount) | $200 |
| **Digital camera** | FLIR Blackfly S USB3 BFS-U3-63S4M-C (mono, 2448×2048, scientific) | $1,200 |
| **Live cell chamber** | OkoLab bold-line стандарт ($1,500) OR DIY acrylic chamber + Peltier heater + CO₂ solenoid ($400) | $400 (DIY) or $1,500 (pro) |
| **CO₂ + temperature controller** | OkoLab H301 or Tokai Hit INU compact ($800) OR DIY with CO₂ tank + Arduino + DHT22 sensor + solenoid ($250) | $250 (DIY) |
| **LED fluorescence illuminator** | SOLA SE II Lumencor ($1,500 used) or ThorLabs M470L4 + M565L3 + filter cube ($500) | $500 (LED stand-alone) |
| **Computer** | Refurbished Dell OptiPlex i5 16GB RAM 512GB SSD ($300) | $300 |
| **Acquisition software** | **Micro-Manager 2.0** (free, open source) + **PyMMCore-Plus** Python bindings | $0 |
| **Data processing** | **Fiji** (free) + custom Python pipeline | $0 |
| **UPS** | APC Back-UPS Pro 1500VA ($200) | $200 |
| **Remote monitoring** | VPN (WireGuard free) + VNC/RustDesk (free) | $0 |
| **NAS backup** | Synology DS220+ with 2×4TB HDD ($500) | $500 |
| **Misc** (cables, mounts, 3D prints) | | $150 |
| **Подитог Вариант A** | | **$4,200** |

**Плюсы:** Влезает в Phase A $80k с запасом. Полный control. Open-source.
**Минусы:** DIY = 2-3 недели setup time. Зависит от personal engineering skills. Calibration может дрейфовать.

---

### Вариант B — Mid-range retrofit ($8,000–12,000)

**Philosophy:** Quality off-the-shelf компоненты, меньше DIY.

| Компонент | Модель | Цена |
|-----------|--------|------|
| **Prior 3rd-party motorized XY stage** | Prior ProScan III H117 с controller | $3,500 (used eBay) |
| **Prior motorized focus (Z)** | Prior Z-Drive motor + adapter | $1,500 (used) |
| **Digital camera** | Hamamatsu ORCA-Fusion BT ($6k new, $2k used) | $2,000 (used) |
| **Live cell chamber + CO₂** | OkoLab H301-K-Frame bold-line complete | $2,500 (used) |
| **LED fluorescence** | Lumencor SOLA SE II | $1,500 (used) |
| **Computer** | Dell Precision 3650 Xeon, 32GB RAM, 1TB NVMe | $800 |
| **Software** | Micro-Manager (free) or μManager Studio | $0 |
| **UPS + NAS + miscellaneous** | | $900 |
| **Подитог Вариант B** | | **$12,700** |

**Плюсы:** Industrial-grade reliability. 2-3 days setup. No calibration issues.
**Минусы:** Дороже. Больше overhead для Phase A bootstrap.

---

### Вариант C — Complete turnkey ($25,000–50,000)

**Philosophy:** Buy ready-to-use automated microscopy system, ignore Zeiss IM 35.

- **Nikon Eclipse Ti2-E** motorized inverted + Perfect Focus System (PFS) + DS-Qi2 camera + NIS-Elements HCA-module + OkoLab environmental
- Used market price: $25,000–50,000 depending on age and configuration

**Плюсы:** State-of-the-art, plug-and-play.
**Минусы:** Выходит за budget Phase A; может быть revisited для Phase B или post-grant.

---

## 📊 Recommended для LOI v24: **Вариант A ($4,200)**

Укладывается в существующую строку бюджета "Microscope Upgrade ($2,000)" — нужно **increase to $4,500** (с reserve $300) и перераспределить:

### Новая budget line в LOI Phase A

**Раздел "Microscope Upgrade & Automation":** $4,500 (было $2,000)

| Sub-item | Cost |
|----------|------|
| Motorized XY stage (DIY kit / Arduino CNC) | $500 |
| Motorized Z focus drive | $200 |
| FLIR Blackfly S digital camera | $1,200 |
| LED fluorescence illuminator (ThorLabs M470L4+M565L3) | $500 |
| CO₂ + environmental DIY controller | $250 |
| DIY live cell chamber (acrylic + heater) | $400 |
| Computer (refurbished) | $300 |
| UPS 1500VA | $200 |
| NAS backup system | $500 |
| 3D-printed mounts + cables + misc | $150 |
| Setup contingency | $300 |
| **Total** | **$4,500** |

**Где взять +$2,500** (relative to existing LOI):
- Снизить "General Consumables" с $12k на $10k (тщательный бюджет)
- Либо снизить "Technician Salary" с $18k на $17k (1 мес меньше FTE)
- Либо добавить строку "Automation Equipment" = $4,500 отдельно и увеличить total request на $2,500 → $82,500 (legal в Impetus, limit $150k per phase)

---

## ⏱️ Timeline for setup

| Week | Task |
|------|------|
| 1 | Ordering components (Arduino, FLIR, chamber, LED, NAS) |
| 2 | Delivery + initial assembly |
| 3 | Motorized stage integration + Z-focus motor mounting |
| 4 | Camera + Micro-Manager installation + calibration |
| 5 | Environmental chamber + CO₂ flow testing |
| 6 | LED fluorescence setup + filter alignment |
| 7 | Full system integration test (dry run without cells) |
| 8 | First cell experiment (BJ-hTERT validation) |
| 9-24 | Phase A data collection (experiments Aim A.1, A.2, A.3) |

**Setup occupies Months 1-2 of Phase A budget**. Experiments begin Month 3. Data analysis through Month 6.

---

## 🔬 Technical specifications

### Imaging parameters

| Parameter | Value |
|-----------|-------|
| **Resolution** | 2448×2048 px (FLIR BFS-U3-63S4M) |
| **Bit depth** | 12-bit → 16-bit stored |
| **Frame rate** | 74 FPS max (not limiting for time-lapse) |
| **Exposure** | 50-500 ms (brightfield), 500-3000 ms (fluorescence) |
| **Z-stack** | 5-11 slices × 2 μm spacing (optional for 3D imaging) |
| **Channels** | Brightfield + FITC (GT335/polyE) + TRITC (Ninein) |
| **Time-lapse interval** | 30 min (day/night) |
| **Data per session** | ~5 GB/day (if 10 positions × 3 channels × 48 timepoints) |
| **Total data 6 mo** | ~900 GB (easily fits NAS 4TB) |

### Stage automation

- **XY range:** Ensure coverage of 35mm dish (~40mm × 40mm) or 6-well plate
- **XY accuracy:** ±5 μm positioning (achievable with Arduino CNC-style steppers + linear rails; Prior stages are ±0.1 μm but more expensive)
- **Z range:** 10mm travel, ±0.5 μm accuracy (adequate for autofocus)
- **Autofocus:** Software-based, contrast detection on DAPI/Hoechst channel OR hardware laser-based (more expensive, Перспектива в upgrade)

### Environmental control

- **Temperature:** 37°C ±0.3°C (Peltier + PID controller)
- **CO₂:** 5% ±0.2% (mass flow controller + CO₂ tank; one tank lasts ~2 weeks)
- **Humidity:** 95% (passive evaporation from humidifier insert)

---

## 🌐 Remote monitoring setup

1. **VPN** (WireGuard) into lab network from Jaba's home
2. **Micro-Manager live display** via VNC/RustDesk
3. **Camera preview stream** via MJPEG over HTTP (for quick peek)
4. **SMS alerts** via Twilio API ($0.01/msg, ~$1/month) when:
   - Temperature deviates >0.5°C
   - CO₂ deviates >0.5%
   - Camera disconnects
   - UPS engaged (power failure)
5. **Daily email summary** — automated Python script sends overview image + environmental log at 8am

---

## 🧰 Список для закупки (ordering list)

### Arduino / electronics (AliExpress / RobotDyn / Amazon — $500 total)
- Arduino Mega 2560 R3 ($25)
- CNC Shield V3 ($10)
- 3× NEMA-17 stepper motors (X, Y, Z) ($40)
- 3× A4988 stepper drivers (already on shield)
- Linear rails 400mm × 2 + 200mm × 2 (X, Y) with bearings + belt drive ($120)
- 12V 10A power supply ($30)
- Endstops × 6 ($10)
- Wires + breadboard + misc ($30)
- DHT22 temperature/humidity sensor ($10)
- CO₂ solenoid valve 12V ($40)
- MH-Z19B CO₂ sensor ($25)
- DS18B20 waterproof temp probe ($10)
- MOSFET modules × 4 ($20)
- Peltier heater module + heatsink ($40)
- 3D-printed parts (stage adapter, focus adapter, chamber mount) — print yourself or Shapeways ($90)

### Camera + optics ($1,700 total)
- FLIR Blackfly S USB3 BFS-U3-63S4M-C monochrome scientific camera ($1,200, new)
- C-mount to Zeiss IM 35 adapter ($80)
- ThorLabs M470L4 blue LED (470 nm, for FITC excitation) ($180)
- ThorLabs M565L3 green LED (565 nm, for TRITC excitation) ($180)
- ThorLabs LEDD1B LED driver ($60)

### Live cell environmental ($1,050 total)
- Acrylic chamber (custom-cut 4mm thick, ~200mm × 200mm × 60mm internal, $120)
- Glass cover slip for bottom ($20)
- CO₂ tank 9kg (refillable, $60 Tbilisi medical supply)
- CO₂ regulator + flow meter ($80)
- Peltier element TEC1-12706 + heatsink + fan ($25)
- PID controller (Inkbird ITC-100 or equivalent) ($35)
- Humidifier insert (plastic tray with foam) ($10)
- OKolab compact CO₂ incubator mini chamber (used market, optional upgrade, $700)

### Computer + storage ($1,000 total)
- Refurbished Dell OptiPlex (i5-8500, 16GB, 512GB SSD, Win10 Pro) ($300)
- Synology DS220+ NAS ($230) + 2× 4TB Seagate IronWolf ($180) = $590
- APC Back-UPS Pro 1500VA ($200)
- External monitor 24" (existing or $150)

**Total Variant A:** $4,250 + shipping/VAT (~$200) = **$4,450**

---

## 📝 LOI v24 budget amendment

**Текущая Phase A budget:** $80,000

**Предлагаемое изменение:**
- Уменьшить "General Consumables" с $12,000 на $9,500 (-$2,500)
- Увеличить "Microscope Upgrade" с $2,000 на $4,500 (+$2,500)
- Переименовать line в "**Microscope Automation & Upgrade**"
- Добавить footnote: "Includes motorized XY + Z stage for round-the-clock imaging, FLIR digital camera, LED fluorescence illumination, environmental chamber (37°C + 5% CO₂), NAS backup, and UPS. Full specification in supplementary materials."

**Net effect:** $80,000 total unchanged. Более точная allocation для automation.

---

## ⚠️ Key risks & mitigations

1. **DIY stage calibration drift** → use linear rails с encoders (add ~$100); alignment every 2 weeks
2. **CO₂ tank run-out mid-experiment** → install low-pressure sensor + SMS alert; keep 2 tanks rotating
3. **Power outage > UPS runtime** → UPS gives 2h; manually restart if needed; alert sent immediately
4. **Contamination** (long-term cultures) → weekly media change, sterile technique, monthly bleach decontamination of chamber
5. **Camera failure** → budget $500 contingency for replacement (not in main budget; covered by $300 setup contingency + technician hours)
6. **Software crash** → Micro-Manager + watchdog script: if no new frame in 2h, restart acquisition automatically
7. **Remote access compromised** → WireGuard with ed25519 keys; no public SSH; lab IP whitelist

---

## 🤖 Claude Code `/overnight` — AI-operated microscopy

### Концепция

Microscope управляется **Claude Code** (Anthropic's official CLI для AI-assisted workflows) в специальном **`/overnight` режиме** — автономный цикл наблюдения, принятия решений и корректировки эксперимента ночью, когда человек не в лаборатории (20:00–08:00).

В `/overnight` режиме agent получает авторизацию на **routine decisions** (выбор поле зрения, autofocus, channel switching, ROI refinement) без ежеминутного одобрения человека. **Strategic decisions** (изменение протокола, остановка эксперимента) остаются у человека.

### Архитектура

```
┌──────────────────────────────────────────────────────────────────┐
│              HUMAN (Jaba, 20:00–08:00 off-duty)                   │
│                      ↕ SMS/email alerts only                       │
└──────────────────────────────────────────────────────────────────┘
                                  ↕
┌──────────────────────────────────────────────────────────────────┐
│              CLAUDE CODE — /overnight agent                        │
│  • Reads experiment protocol (PROMPT.md, см. ниже)                 │
│  • Monitors image stream в real-time                               │
│  • Calls tool functions: move_stage(), autofocus(),                │
│     capture_multichannel(), switch_led(), etc.                     │
│  • Decision loop: каждые 30 мин inspect last batch,                │
│     выбрать next action (stay/move/rotate/alarm)                   │
│  • Journaled decisions с rationale                                 │
│  • On blocker: pauses + SMS to experimenter                        │
└──────────────────────────────────────────────────────────────────┘
                                  ↕ Python tool functions (PyMMCore-Plus)
┌──────────────────────────────────────────────────────────────────┐
│     MICRO-MANAGER 2.0 core + pymmcore-plus + Arduino firmware      │
└──────────────────────────────────────────────────────────────────┘
                                  ↕
┌──────────────────────────────────────────────────────────────────┐
│      Zeiss IM 35 + motorized XY/Z + camera + chamber + LEDs        │
└──────────────────────────────────────────────────────────────────┘
```

### Tool functions для Claude Code

```python
# микроскоп-управление
move_stage_xy(x_mm, y_mm); move_stage_relative(dx, dy)
autofocus(mode='contrast'|'laplacian')
capture_single(channel, exposure_ms); capture_zstack(ch, z_range, step)
capture_multichannel(channels)
switch_led(channel, intensity_pct)

# image analysis
detect_cells(image); detect_centrioles(image)
measure_polyglu_intensity(cell_roi, channel)
segment_nuclei(image); count_dead_cells(image)

# environmental
get_temperature_c(); get_co2_percent(); get_humidity_percent()
set_target_temp(c); open_co2_valve(duration_s)

# alerting / journaling
send_sms_to_human(message, priority='info'|'warn'|'crit')
log_decision(action, rationale, observed)
save_checkpoint(state); halt_experiment(reason)  # requires human unlock
```

### Cost

Claude Code subscription: **$20/мес Pro tier** × 6 мес = **$120 total** — negligible vs $4,500 hardware.

---

## 📝 PROMPT-based experimentor signaling

### Концепция

Experimentator (Jaba) пишет **PROMPT** на естественном языке описывающий **цели и задачи эксперимента**. Claude Code интерпретирует этот prompt и автоматически:

1. **Мониторит данные** в реальном времени (image stream, environmental log)
2. **Проверяет цели** каждые 30 минут
3. **Отправляет сигналы** человеку когда происходит что-то важное: достижение цели, anomaly, риск для эксперимента, необходимость intervention

**Преимущество:** Jaba не нужно программировать условия срабатывания. Он описывает цели словами — Claude сам формулирует критерии и мониторит.

### Формат PROMPT файла

`~/Documents/Experiments/CDATA_PhaseA_Aim1/PROMPT.md`:

```markdown
# Aim A.1 — α vs β discrimination
## Цель
Disentangle division-dependent (α) from time-dependent (β) polyGlu accumulation
in BJ-hTERT fibroblasts by comparing proliferating vs contact-inhibited conditions.

## Условия
- **Proliferating wells** (A1-A6): sub-confluent, passaged every 3 days
- **Contact-inhibited wells** (B1-B6): seeded confluent, media changed 2× per week
- **Time points for imaging**: days 0, 7, 14, 21
- **Channels**: Brightfield, FITC (GT335 polyGlu), TRITC (Ninein mother centriole)

## Что измеряем
- polyGlu intensity at mother centriole (FITC co-localized with TRITC)
- cell density (brightfield segmentation)
- morphology changes (senescence markers: flattened cells, vacuoles)

## Сигналы, которые я хочу получить

### Позитивные (progress toward goal)
- **"Signal detected"**: если polyGlu intensity в proliferating wells вырос на >30% vs day 0 baseline
- **"α > 0 confirmed"**: если при day 14 in proliferating wells средний intensity significantly higher (p<0.05, t-test) vs contact-inhibited wells
- **"Parrinello signal"**: если 20% O₂ wells показывают >40% difference vs 3% O₂ wells (for O₂ modulation Aim A.2)

### Предупреждения (possible trouble)
- **"Contamination risk"**: если cell density drops >20% unexpectedly в 2+ wells
- **"Focus drift"**: если autofocus fails в 3+ positions consecutively
- **"Channel saturation"**: если FITC mean intensity > 60,000 counts (max 65,535)
- **"Morphology anomaly"**: если >15% cells show senescent markers earlier than expected

### Критические (требуют немедленного вмешательства)
- **"Experiment compromised"**: если cell death >40% in any well → stop imaging, SMS Jaba, save last data
- **"Environment failure"**: если temp <35°C, CO₂ <4% или >6%, humidity <80% — pause, SMS Jaba
- **"Hardware fault"**: если camera disconnects, stage jams, UPS engaged — halt, SMS Jaba

## Что Claude может делать автономно
- Выбирать best focal plane для imaging
- Adjust LED intensity within ±20% if signal too dim/bright
- Re-image if blur detected
- Skip dead cells в analysis
- Increase imaging frequency на interesting positions (if Claude detects cell division event)
- Request Z-stack если 2D image недостаточно

## Что Claude НЕ делает без моего разрешения
- Менять protocol (пример: не добавляет лишние channels)
- Остановить experiment раньше scheduled end
- Открыть chamber (sterility)
- Изменить cells физически (no chemical addition)

## Формат отчётов
- **Daily 8 AM email**: summary of last 24 hrs + 4 sample images
- **Mid-day SMS (14:00)**: short status (OK / warning / critical)
- **Cycle log (every 30 min)**: в JOURNAL.md, no notification to human unless signaled above
- **End-of-experiment report**: day 21, full analysis, recommended next steps

## Контекст для Claude (для понимания важности сигналов)
Данный эксперимент — Aim A.1 of Impetus Phase A ($80k grant). Результат определяет
go/no-go decision для Phase B ($180k total, €100k to Ulm lab — Geiger confirmed 2026-04-22). CDATA hypothesis предсказывает α > 0 значимо.
Null result (α ≈ 0 и β ≈ 0) = хonest falsification, publish negative result.
```

### Как Claude использует PROMPT

1. **Парсинг целей:** извлекает из текста конкретные метрики (polyGlu intensity, cell density, effect sizes) и thresholds (>30%, >40%, p<0.05)
2. **Постоянная проверка:** каждые 30 мин на основе последних imaging data проверяет — сработало ли какое-то условие из "Сигналы"
3. **Генерация сигнала:**
   - "Signal detected" → send_sms с кратким summary и attached preview image
   - "Contamination risk" → send_sms + предлагает actions ("recommend: check wells A3, A5")
   - "Experiment compromised" → halt + SMS + save checkpoint
4. **Log of reasoning:** каждый decision журналируется в JOURNAL.md с quoting the relevant prompt line и показателями

### Пример SMS сигналов

**Scenario 1 — Positive progress:**
```
[CDATA-A1] 14:30 | INFO
α > 0 confirmed: proliferating wells show 2.3x higher polyGlu vs contact-inhibited
at day 14 (p=0.003, n=6 wells each). Keeping protocol. Full data: dashboard/cycle_156
```

**Scenario 2 — Warning:**
```
[CDATA-A1] 03:15 | WARN
Cell density dropped 28% in wells A4, A5 between 02:00 and 03:00 imaging.
Possible contamination. Recommend visual check at 8 AM.
Wells A1-A3, A6 normal. Continuing schedule, flagged for review.
```

**Scenario 3 — Critical:**
```
[CDATA-A1] 05:42 | CRIT
CO₂ dropped to 3.1% (target 5.0%). Opened valve 45s × 3, no recovery.
Tank may be empty. HALT imaging. Cells survive ~4 hrs at low CO₂.
Action needed before 10 AM. Checkpoint saved.
```

### Как adaptировать PROMPT для разных aim'ов

**Aim A.2 (20% vs 3% O₂):** добавить signals для differential wells; modify critical thresholds для O₂-sensitive markers.

**Aim A.3 (CCP1/TTLL6-OE causal):** добавить signals for population doubling rates, transduction efficiency (GFP% cells), senescence onset timing.

**Phase B (HSC mouse):** полностью другой PROMPT.md — focus на chimerism, flow cytometry results, не microscopy.

### Template для experimentor

```markdown
# [Aim Name]

## Цель
[один параграф о scientific goal]

## Условия
[список conditions: wells, channels, time points]

## Что измеряем
[metrics to quantify]

## Сигналы

### Позитивные (progress)
- [condition] → signal "[name]": [action]

### Предупреждения
- [condition] → signal "[name]": [action]

### Критические
- [condition] → signal "[name]": HALT + SMS Jaba

## Автономные действия Claude
[что можно без authorization]

## НЕ делать без разрешения
[что требует human approval]

## Отчёты
- [частота: daily/mid-day/end-of-experiment]
- [формат: email/SMS/dashboard]

## Контекст
[почему это важно, stakes, acceptable null result]
```

### Преимущества prompt-driven approach

1. **Zero programming:** Jaba описывает словами, не пишет Python
2. **Domain-expert friendly:** biologist формулирует условия в биологических терминах, Claude переводит в thresholds
3. **Easy iteration:** менять цели эксперимента = редактировать PROMPT.md, без code changes
4. **Explanatory:** Claude's decisions всегда ссылаются на конкретные lines PROMPT.md — transparency
5. **Reusable:** PROMPT.md template для каждого Aim, можно делиться с коллегами
6. **Fail-safe:** если PROMPT не покрывает сценарий — Claude не действует autонomously, pause и запрашивает human

### Validation перед submission

Перед тем как Claude запускает `/overnight`, Jaba делает:
1. **Dry run** на синтетических данных — проверить что сигналы срабатывают корректно
2. **Day-1 validation**: первые 24 часа Claude в `/overnight` режиме, но Jaba проверяет logs каждый час вручную
3. **Progressive автономность:** после 48 часов без issues — full autonomy; если issue — уменьшить autonomy

### Для Impetus LOI v24 — Methodological Innovation

Этот prompt-driven signaling — дополнительный **scientific selling point**:

> "Our automated microscopy platform pairs low-cost retrofit ($4,500) with **prompt-driven AI supervision** (Claude Code `/overnight`). The experimenter articulates goals и thresholds в natural-language PROMPT.md; Claude continuously monitors data and signals the human only when strategically important events occur (positive progress, warnings, critical failures). This eliminates the 'experimenter wake-up cost' traditional time-lapse requires and democratizes AI-assisted experimental science for resource-limited labs. All PROMPT templates, tool functions, and decision logs released under MIT license concurrent with Phase A preprint."

---

## 🚀 Follow-up upgrades (post-Phase A, if Go decision)

If Phase A passes и funding extends (Phase B or follow-on grant):

- **Hamamatsu ORCA-Fusion BT** upgrade (+$4k used) → higher sensitivity for low-expression fluorescence
- **Perfect Focus System** (Nikon PFS) retrofit (+$3k used) → hardware-based autofocus, no software drift
- **Motorized filter turret** (+$1k) → automated channel switching, faster imaging
- **Liquid handling robot** (OpenTrons OT-2 $2.5k new) → automated media change, ideal for 3-4 week contact-inhibition protocols

**Budget for upgrades:** place into Phase B or post-grant $10-15k capital line.

---

## ✅ Decision for LOI v24 submission (2026-04-24)

**Recommendation:** Вариант A ($4,500 total equipment line) — максимально cost-effective, polностью покрывает Phase A experimental requirements (automated day-night imaging, environmental control, remote monitoring, data backup). Upgradeable to Phase B levels later.

**Action:**
1. Amend Phase A budget: "Microscope Upgrade" $2k → "Microscope Automation & Upgrade" $4.5k (reallocate $2.5k from Consumables)
2. Add this document as supplementary material
3. Include in Cover Letter: "A custom-engineered automated time-lapse imaging platform (see Supplementary: Automated Microscopy Setup) will enable round-the-clock tracking of centriolar polyGlu dynamics in live fibroblasts, maximizing temporal resolution within budget constraints."

---

*Engineered 2026-04-21 for Impetus LOI v24 submission 2026-04-25.*
