# Future Migration Plan — Core File Structure

**Дата создания:** 2026-04-22
**Статус:** черновик для исполнения после EIC Pathfinder submission (2026-05-12)

---

## Сейчас выполнено (2026-04-22)

✅ Добавлен `OPEN_PROBLEMS.md` во все 11 подпроектов
- Это критически необходимо для EIC submission (ABL-2 disclosure, χ_Ze pre-registered failures, MCOA dimensional gaps, scaffold-проекты без формул)
- Содержит falsification tests для каждой неразрешённой научной задачи
- Снимает риск "missed honest disclosure" при peer review

## Откладывается до после EIC (2026-05-12+)

### Фаза 1 — низкий риск (июнь 2026)

**EVIDENCE.md** как отдельный файл (1-2 дня работы):
- Переформатировать все verified PMIDs/DOIs в таблицы с date-of-verification
- Отделить от KNOWLEDGE.md (который станет "domain understanding")
- Автоматический audit скрипт: `tools/verify_evidence.sh` — проверяет все refs в EVIDENCE.md через PubMed + Crossref
- Результат: любой рецензент / аудитор может прочитать только один файл и проверить все claims

**Пример структуры EVIDENCE.md:**
```markdown
# Evidence — {project}

## Verified Literature (2026-04-22 через PubMed+Crossref)

### Supports Axiom 1
| Claim | PMID/DOI | Paper | Verified | Strength |
|-------|----------|-------|----------|----------|
| ... | ... | ... | ✅ 2026-04-22 | Strong/Moderate/Weak |

### Supports Axiom 2
...

## Internal Data
- `data/sobol_results_2026-04-15.csv` — Sobol sensitivity N=16384
- `data/LOO_CV_2026-04-17.json` — LOO-CV mean=-0.093

## Refuting Evidence (honest)
- Evidence that ABL-2 paradox may indicate CDATA is downstream of epigenetic drift (see OPEN_PROBLEMS.md)
```

### Фаза 2 — средний риск (август 2026)

**MEMORY + UPGRADE ✅-done → JOURNAL.md** (2-3 дня работы):
- Хронологический формат: дата → что изменилось → rationale
- Легко отвечать "что изменилось за неделю" одним `tail`
- UPGRADE планируемое остаётся отдельно как ROADMAP.md
- TODO объединяется с ROADMAP (активные задачи = ближайшие roadmap items)

**Пример JOURNAL.md:**
```markdown
# Journal — {project}

## 2026-04-22
- OPEN_PROBLEMS.md добавлен (ABL-2 disclosure)
- FCLC/MCOA core files перегенерированы после первой ошибки rate-limit
- EIC Part B v3 Variant B завершён, 10 файлов

## 2026-04-21
- MCOA reframe: CDATA as Counter #1 (Nature Aging Perspective)
- Sobol results pushed to `data/sobol/`
```

### Фаза 3 — rebrand, низкий риск (сентябрь 2026)

**CLAUDE.md → AGENTS.md** (просто rename):
- Индустрия уже стандартизирует `agents.md` convention
- Работает с Cursor, Codex, Continue.dev, Aider одинаково
- `AGENTS.md` лучше описывает, что это инструкции для LLM agents, не для Anthropic-специфичного ассистента

**CONCEPT.md → THEORY.md** (rename + trim):
- `CONCEPT` слишком аморфно (и концепция, и теория, и product vision смешались)
- `THEORY.md` = только формальная научная часть (axioms, formulas, predictions)
- Product vision переносится в README.md (стандарт индустрии)

### Фаза 4 — высокий риск, для оценки после факта (декабрь 2026)

**MAP.md → DESIGN.md с автогенерацией:**
- `tools/generate_design.sh` использует `tree -L 3` + структурированные секции
- Cron/git-hook обновляет автоматически при коммитах
- Избавляет от ручного поддержания устаревшей структуры

**Финальная идеальная структура (после всех фаз):**

| № | Файл | Содержит |
|---|------|----------|
| 1 | README.md | Lay summary, entry point для человека |
| 2 | THEORY.md | Формальная теория (axioms, formulas) |
| 3 | EVIDENCE.md | Верифицированные refs + internal data |
| 4 | OPEN_PROBLEMS.md | Честный список нерешённого |
| 5 | PARAMETERS.md | Quantitative params с provenance |
| 6 | DESIGN.md | Архитектура кода + auto-tree |
| 7 | AGENTS.md | LLM инструкции |
| 8 | JOURNAL.md | Chronological log |
| 9 | ROADMAP.md | Future plans |

## Не делать

**КЛЮЧЕВОЕ:** не запускать полную миграцию до EIC Pathfinder submission (2026-05-12). Любые структурные изменения внутри ~20 дней до дедлайна создают риск ошибок и потери синхронизации между 11 подпроектами. Миграция только после подачи.

## Триггеры для запуска

Запустить миграцию можно когда:

1. **EIC submission завершён** (2026-05-12)
2. **Нет активных LOI в работе** (следующие: Longevity Impetus 2026-04-25 уже подан)
3. **Минимум 1 спокойная неделя** без peer review / рецензентских ответов
4. **Пользователь явно запросил миграцию** (не автономно)

## Риски миграции

- **R1:** сломать ссылки между подпроектами, если одни мигрировали, другие нет → mitigation: миграция всех 11 одновременно за один день
- **R2:** потеря git history → mitigation: `git mv` вместо copy+delete
- **R3:** custom LLM workflows (Claude Code, Cursor) могут не найти AGENTS.md если ещё ждут CLAUDE.md → mitigation: symlink CLAUDE.md → AGENTS.md на переходный период 3 месяца
- **R4:** scaffold-проекты имеют минимальное содержимое — 9 файлов могут быть наполовину пустыми → mitigation: оставить как stub-ы, заполнить при активации

## Ответственные

- **Инициирует миграцию:** пользователь Jaba Tkemaladze
- **Выполнение:** Claude Code (этот ассистент) или ручное по CHECKLIST-у
- **Ревью после миграции:** внешний peer reviewer (как делали с EIC v2→v3)

---

**Итог:** структурное улучшение отложено до после EIC submission. До этого момента — минимальное изменение (только OPEN_PROBLEMS.md, критично нужный сейчас). Полный план миграции зафиксирован в этом документе для последующего исполнения.
