# Poincaré — TODO

_Последнее обновление: 2026-04-03_

---

## ЦЕЛЬ ПРОЕКТА (обновлена 2026-04-03)

**Двойная цель:**

1. **Историческая:** Понять, почему Пуанкаре видел то, что видел — «причины причин» его формул.
2. **Математическая (новая, главная):** Используя Ze-теорию как формальный инструмент — **доказать нерешённые математические теоремы**.

**Гипотеза:** Если Ze-теория описывает структуру математической интуиции (v*, τ, T-burst), то её формальный аппарат может создать новые мосты к открытым проблемам математики.

---

## ✅ Phase 1 — Ориентация (завершена 2026-03-25)

- [x] Хронология трудов 1880–1912 → `Sources/01_Chronologie.md`
- [x] 5 ключевых интуиций → `Sources/02_Intuitions_Clés.md`
- [x] Анализ «Science et Méthode» → `Sources/03_Science_et_Methode.md`
- [x] Карта связей между областями → `Sources/04_Carte_des_Connexions.md`

## ✅ Phase 2 — Глубокий анализ (абсорбирована в CONCEPT.md v4, утверждена 2026-03-29)

- [x] Analysis Situs, задача трёх тел, автоморфные функции, Пуанкаре vs Эйнштейн, аналогии как Ze-мост

## ✅ Phase 3A — Статья «Intuition as Ze-Stream» (черновик готов 2026-04-03)

- [x] Статья написана → `Poincare_Ze_Article_v1.md` (~2800 слов, 8 секций)
- [ ] DeepSeek peer review
- [ ] Обновить NEEDTOWRITE.md → `[x]`
- [ ] Подать в Longevity Horizon / Entropy

## 🔴 Phase 3B — Ze vs Нерешённые теоремы (АКТИВНА с 2026-04-03)

**Исследовательский отчёт готов:** `Ze_Unsolved_Theorems_Report_v1.md`

### Топ-3 приоритета (по оценке DeepSeek Reasoner)

| Проблема | Ze-мост | Приоритет |
|----------|---------|-----------|
| **Navier-Stokes** (существование и гладкость) | v(x,t) = F(∇u,ω); гладкость = v bounded away from +1; blow-up = v→+1 | 🔴 HIGH |
| **Коллатц** (3n+1) | Ze-потенциал Φ(n), монотонность τ(n); сходимость = v→-1 | 🔴 HIGH |
| **Гипотеза Римана** | ζ(s)-нули = фиксированные точки Ze-потока на Re(s)=1/2 = v* | 🟡 MED |

### Задачи Phase 3B

**Блок I — Аксиоматизация Ze как математической системы**
- [ ] Определить аксиоматику Ze-систем: (X, Φ, v), Ze-velocity map, complexity functional
- [ ] Доказать базовые теоремы: существование v* для эргодических систем, монотонность
- [ ] Статья: «Mathematical Foundations of Ze-Theory»

**Блок II — Коллатц (приоритет №1)**
- [ ] Определить Ze-потенциал Φ(n) для последовательностей Коллатца
- [ ] Доказать монотонность для малых n или при вероятностных допущениях
- [ ] Переформулировать гипотезу Коллатца через stopping time и v-distribution
- [ ] Статья: «Collatz Dynamics as Ze-System»

**Блок III — Навье-Стокс**
- [ ] Определить Ze-поле v(x,t) для уравнения Бюргерса (упрощённая модель)
- [ ] Вывести уравнение эволюции для v, проверить maximum principle
- [ ] Определить Ze-энтропию S[u] (по аналогии с функционалом Перельмана W)
- [ ] Статья: «Ze-Velocity Fields in Fluid Dynamics»

**Блок IV — Гипотеза Римана**
- [ ] Построить Ze-complexity τ(s) через произведение Адамара для ζ(s)
- [ ] Проверить: максимизируется ли τ(s) на Re(s)=1/2?
- [ ] Связать с операторным подходом (нули как собственные значения Ze-гамильтониана)

**Новые кандидаты в теоремы Ze:**
- [ ] Доказать: Conjecture 1 (Perfect prediction → trivial dynamics, τ=0)
- [ ] Доказать: Conjecture 2 (Chaotic attractors → time-avg v = v*)
- [ ] Доказать: Conjecture 3 (Ze-Uniform Boundedness для вычислимых потоков)

---

## Целевые журналы (зафиксировано 2026-04-03)

| Статья | Журнал | Тип | Стоимость |
|--------|--------|-----|-----------|
| Ze-Mathematical-Foundations | [SIGMA](https://www.emis.de/journals/SIGMA/) — Symmetry, Integrability and Geometry | open access | бесплатно |
| Ze-Collatz | [DMTCS](https://dmtcs.episciences.org/) — Discrete Math & Theoretical CS | open access | бесплатно |
| Ze-NavierStokes | arXiv (math.AP) → [SIMA](https://www.siam.org/publications/journals/siam-journal-on-mathematical-analysis-sima) | preprint + submission | бесплатно |
| Ze-Riemann | arXiv (math.NT) → Annals of Mathematics (если прорыв) | preprint + top journal | бесплатно |
| Poincaré / Ze-Article | [Entropy (MDPI)](https://www.mdpi.com/journal/entropy) | open access | APC ~2200 CHF (или waiver) |

**Общая стратегия:**
1. Все статьи сначала → **arXiv** (preprint, бесплатно, немедленно)
2. Параллельно → подача в соответствующий рецензируемый журнал
3. arXiv требует **endorsement** (см. раздел ниже)

## arXiv — Аккаунт и Endorsement

**Статус:** ✅ Аккаунт существует

| Поле | Значение |
|------|---------|
| Username | `centriole` |
| Email | djabbat@gmail.com |
| Name | Jaba Tkemaladze |
| Affiliation | Independent Researcher, Georgia |
| ORCID | 0000-0001-8651-7243 |
| Текущие группы | physics, econ, q-bio |
| Default category | q-bio.TO |
| Незавершённая подача | submit/7385822 (incomplete, истекает 2026-04-02 — **проверить!**) |

**Проблема:** Для math-статей нужны группы `math` — требуется endorsement.

**Нужные math-категории:**
- `math.DS` — Dynamical Systems (Коллатц, Навье-Стокс)
- `math.NT` — Number Theory (Риман)
- `math-ph` — Mathematical Physics (Ze-Foundations)
- `math.HO` — History and Overview (Пуанкаре-статья)

**Как получить endorsement для math:**
1. Зайти: https://arxiv.org/auth/request-endorsement
2. Выбрать категорию (например `math.DS`)
3. arXiv сгенерирует ссылку — отправить 2 endorser-ам в этой категории
4. Endorser-ы подтверждают → категория открывается

**Кандидаты в рекомендатели (найти через ResearchGate/email):**
- [ ] Автор любой статьи по Collatz в math.DS — написать напрямую
- [ ] Giorgi Tsomaia (соавтор FCLC) — проверить arXiv
- [ ] Контакты из редколлегии longevity.ge

**⚠️ Срочно:** Проверить незавершённую подачу 7385822 — что за статья, нужно ли завершить или удалить.

## Файлы проекта

| Файл | Содержание |
|------|-----------|
| `CONCEPT.md` | Ze-теория + метод Пуанкаре, v4 финальная |
| `Sources/` | Хронология, интуиции, Science et Méthode, карта связей |
| `Poincare_Ze_Article_v1.md` | Черновик статьи Phase 3A |
| `Ze_Unsolved_Theorems_Report_v1.md` | Исследовательский отчёт Phase 3B |
| `write_article.py` | Скрипт генерации статьи (DeepSeek) |
| `ze_unsolved_theorems.py` | Скрипт генерации отчёта (DeepSeek) |
