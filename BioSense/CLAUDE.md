# BioSense — Носимый браслет: EEG · HRV · Запах

## Backend port — `:4502` (native systemd, production since 2026-05-08)

**Production:** `biosense-backend.service` (native Rust, this crate
`BioSense/backend/`) on `127.0.0.1:4502`. nginx `biosense.longevity.ge`
proxies `/api/` → `:4502/api/*`, `/live/` and `/` → `:4501`
(`biosense-web` Phoenix LiveView dashboard).

**Wire format:** ChiZeRequest accepts BOTH conventions via `serde(alias)`:
- `{"v_eeg": x, "v_hrv": y, "v_resp": z, "v_sleep": w}` — idiomatic Rust shape.
- `{"eeg": x, "hrv": y, "resp": z, "sleep": w}` — legacy shape used by
  Phoenix `biosense-web` client and the retired Docker container.

**Routes** (all mounted at both `/<name>` and `/api/<name>`):
- `GET  /healthz` — liveness
- `POST /chi_ze` — composed χ_Ze biomarker computation
- `POST /bridge` — CDATA D → χ_Ze stub
- `POST /exacerbation` — risk score
- `GET  /v_star` — canonical v* (Article + Python forms)

**History:** Until 2026-05-08, `:4502` was held by the Docker container
`deploy-biosense-backend-1` (image `deploy-biosense-backend`, running
since Apr 30). After Phase 4.4 field-name reconciliation
(`#[serde(alias)]`), the container was stopped and native systemd
took over. Per memory `feedback_no_docker` rule.

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
