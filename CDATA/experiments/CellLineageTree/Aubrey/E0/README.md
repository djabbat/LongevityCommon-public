# Experiment 0 — HW+SW Commissioning

**Цель:** валидировать hardware + software stack для AI-directed laser ablation rig на Zeiss IM 35 / ICM 405. **НЕ биологический пилот.**

**Что валидируется:**
1. **Claude Code `/overnight` agent** — автономное ведение time-lapse 24/7
2. **Laser TTL control** — Arduino PWM от Python команды → ablation target
3. **Motorized stage** — AI командует перемещение к target, центровка в FOV
4. **Feedback loop** — детекция → решение → действие → логирование
5. **Error handling** — interlock, temp overrun, UPS event, camera crash, network loss
6. **Data pipeline** — 6-мес TIFF stream без data loss
7. **Safety infrastructure** — light-tight box, двухконтурный interlock, UPS

**Что НЕ валидируется (явно):**
- Центриолярная биология (признано — см. `PEER_REVIEW_DRAFT.md`)
- Translational claims к mammalian CDATA
- Impetus pilot positioning

**Модель:** *Elodea canadensis* leaves, chloroplasts. Выбор обоснован:
- Бесплатно / non-precious / без ethics issues
- Chloroplasts видимы в brightfield без окраски
- Discrete targets для AI detection
- Ablation viz: выжигание видно на месте
- Позволяет отладить pipeline на реальных биологических данных без биологических claims

**Deliverable:**
- Open-source GitHub repo: Arduino sketch + Python tool functions + Claude agent PROMPT.md
- Technical paper на arXiv (HW/SW engineering): "AI-driven autonomous single-organelle laser ablation platform for sub-$1000"
- Proven workflow для переноса на Experiment A (правильная биология, Impetus Phase A)

---

## Архитектура системы

```
┌─────────────────────────────────────────────────────────────┐
│  EXTERNAL                                                   │
│  ┌─────────┐  ┌────────────┐  ┌──────────────────┐         │
│  │ Monitor │  │ UPS APC    │  │ External HDD 4TB │         │
│  │  24"    │  │ SMT1500    │  │ (weekly rsync)    │         │
│  └────▲────┘  └──────▲─────┘  └────▲──────────────┘        │
│       │ HDMI         │ 220V         │ USB 3.0                │
│  ┌────┴──────────────┴──────────────┴────────────────────┐ │
│  │ PC (Linux Ubuntu 22.04)                                │ │
│  │ - Micro-Manager 2.0 + PyMMCore-Plus                    │ │
│  │ - Claude Code CLI with /overnight agent                │ │
│  │ - Python tool functions (ablate_target, move_stage...)│ │
│  │ - Arduino serial over USB                              │ │
│  └───────┬────────────────────────────────────────────────┘ │
└──────────┼──────────────────────────────────────────────────┘
           │ USB2/3 + GPIO
           │
┌──────────┼──────────────────────────────────────────────────┐
│  LIGHT-TIGHT ENCLOSURE (ACP 3mm black, 600×500×700 мм)      │
│  ┌───────┴──────────┐                                        │
│  │ Zeiss IM 35      │                                        │
│  │  ┌────────────┐  │  ┌───────────────────────────────┐    │
│  │  │ LED xform  │◄─┼──┤ Arduino Nano (PWM LED, laser)  │    │
│  │  │ Cree XHP50 │  │  │ + ESP8266 (WiFi alerts, MQTT)  │    │
│  │  └────────────┘  │  │ + DS18B20 × 3 (temp)           │    │
│  │  ┌────────────┐  │  │ + BPW34 (brightness)           │    │
│  │  │ Motorized  │◄─┼──┤ + Interlock D2 (reed switch)   │    │
│  │  │ XY stage   │  │  │ + Laser GATE (SPDT relay HW)   │    │
│  │  │ NEMA-17×2  │  │  └───────────────────────────────┘    │
│  │  └────────────┘  │                                        │
│  │  ┌────────────┐  │  ┌───────────────────────────────┐    │
│  │  │ ToupCam    │──┼──┤ USB 3.0 → PC                   │    │
│  │  │ E3CMOS     │  │  └───────────────────────────────┘    │
│  │  └────────────┘  │                                        │
│  │  ┌────────────┐  │  ┌───────────────────────────────┐    │
│  │  │ Laser 450  │◄─┼──┤ 12V PSU + TTL from Arduino    │    │
│  │  │ nm 500mW   │  │  │ (via SPDT relay hardware kill) │    │
│  │  └────────────┘  │  └───────────────────────────────┘    │
│  └──────────────────┘                                        │
│                                                             │
│  OVERVIEW CAMERAS:                                          │
│  ┌─────────────────┐  ┌──────────────────┐                  │
│  │ RPi Cam 3 Wide  │  │ USB endoscope    │                  │
│  │ NoIR + IR LEDs  │  │ 1080p 8mm macro  │                  │
│  │ (Pi Zero 2W)    │  │ (close-up stage) │                  │
│  └─────────────────┘  └──────────────────┘                  │
│                                                             │
│  VENT: 2× Z-baffle light-trap + Noctua 120мм                │
│  INTERLOCK: magnetic door switch → reed → relay → Arduino   │
└─────────────────────────────────────────────────────────────┘
```

---

## Файлы проекта

| Файл | Что |
|---|---|
| `README.md` | Этот файл — обзор + архитектура |
| `Полное_Описание.md` | ⭐ **МАСТЕР-ДОКУМЕНТ** — все детали (inventory, BOM, сборка, протокол, параметры) |
| `BOM.md` | Детальный Bill of Materials с ссылками |
| `ENCLOSURE.md` | CAD-схема светонепроницаемого бокса |
| `Покупки_Китай.md` | Приоритетный чеклист по неделям |
| `CLAUDE_AGENT.md` | ⭐ Tool functions + PROMPT.md template для Claude Code agent |
| `PEER_REVIEW_DRAFT.md` | Критический peer review от DeepSeek reviewer |

---

## Фазы (6 месяцев)

| Месяц | Задача |
|---|---|
| **1** | Закупка (3-4 нед shipping) + локально Тбилиси |
| **2** | Сборка бокса + LED retrofit + motorized stage DIY + Arduino sketch |
| **3** | Калибровка Köhler, stage precision, laser focus, interlock test |
| **4** | Claude Code agent integration test (dry-run на синтетических данных) |
| **5** | 6-нед continuous run с Claude /overnight контролем |
| **6** | Анализ, GitHub release, arXiv preprint, transition plan в Experiment A |

---

## Бюджет

**Sync'd с `PARAMETERS.md` (authoritative source) 2026-04-24:**

- **Минимум:** $881 (с LGY40-C + NEMA-8 мotorization + ToupCam + UPS Back-UPS 1500)
- **Оптимум:** $1687 (с premium LGY40-C linear stepper actuator + Smart-UPS SMT1500 + reserve камеры)

Полная смета — в `docs/BOM.md` (§1b updated для LGY40-C 2026-04-23, §1.5/1.6 halogen = OSRAM 64607 8V 50W).

> **Примечание:** предыдущие оценки $530-770-1400 считались на старый план (NEMA-17 + knob coupling) с частично отсутствующими позициями; не использовать.

---

## Транзит к Experiment A (Impetus Phase A, 2026 Q3+)

**Что переиспользуется из Experiment 0:**
- ✅ Бокс + interlock + вентиляция
- ✅ Overview cameras (RPi + endoscope)
- ✅ Arduino sketch (расширить для fluorescence PWM каналов)
- ✅ Claude Code agent + tool functions framework
- ✅ Motorized stage (precision ±20-50 μm достаточно для cell-level targeting)
- ✅ Zeiss microscope + objectives
- ✅ UPS + storage
- ✅ LED transmitted light (возможен upgrade на high-CRI)

**Что меняется:**
- ❌ Laser: 450 nm CW → pulsed ns UV 355 nm (Cobolt Tor или Rapp UGA-42, $15-25K)
- ❌ Biological model: Elodea → iPSC-organoids с Centriolin-RITE или Drosophila GSC
- ❌ HBO replacement: LED epi-source для fluorescence
- ❌ Objective: upgrade to Plan-Apo 100×/1.4 oil
- ❌ Environmental: 37°C + CO₂ chamber для mammalian
- ❌ Compliance: BSL-2 collaboration с Georgian biomedical institute

**Realistic reuse: ~50-60%** hardware, ~80% software stack.

---

*README создан 2026-04-23. Reframe после peer review.*
