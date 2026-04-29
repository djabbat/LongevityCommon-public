# BioSense — Носимый браслет: EEG · HRV · Запах

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
