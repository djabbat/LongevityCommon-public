# BioSense — Носимый браслет: EEG · HRV · Запах

## Backend port (decided 2026-05-07)

**`:4502`** (loopback). Front-end Phoenix LiveView (`biosense-web.service`) живёт на `:4501`. nginx `biosense.longevity.ge` уже маршрутизирует:
- `/api/` → `127.0.0.1:4502` (Rust backend, считает χ_Ze из counter feeds)
- `/live/` и `/` → `127.0.0.1:4501` (biosense-web Phoenix dashboard)

Это переопределяет более ранний CONCEPT.md (упоминал `:4101`) — выбор `:4502` cohort root `PARAMETERS.md § 8` и устраняет необходимость править nginx.

---


## 📌 Правило: DeepSeek для нетехнических задач

**Код (Python/Rust) — Claude. Всё остальное — DeepSeek API.**
Примеры: статьи о χ_Ze, peer review, введение/обсуждение, переводы.
**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Вход:** `~/Desktop/AIM/llm.py`
**Модели:** `deepseek-chat` (быстро) · `deepseek-reasoner` (научные рассуждения)

---

## Проект

Верификация Ze-теории на EEG-данных. Гипотеза: χ_Ze(молодые) > χ_Ze(пожилые).
Статья `ze_eeg_paper.docx` v8 (410KB) — почти готова к отправке.

## Связь с AIM

`ze_ecg.py` → AIM HRV анализ пациентов (χ_Ze, v*, RMSSD).
