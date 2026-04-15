# CommonHealth — News & Science Feed

**Дата обновления:** 2026-04-10  
**Статус:** ЯДРО · не публиковать в public git  
**Назначение:** агрегатор свежих научных новостей для обогащения всей экосистемы

---

## Как использовать этот файл

- Обновлять при каждой сессии (поиск свежих статей)
- Находки тематически дублируются в KNOWLEDGE.md соответствующего подпроекта
- Формат: дата · источник · тезис · связь с CommonHealth

---

## Апрель 2026

---

### [ОБЩЕЕ] Citizen Science и здоровье в 2026

**Источники:**
- [The Rise of Citizen Science in Health Research — Tandfonline](https://www.tandfonline.com/doi/full/10.1080/15265161.2019.1619859)
- [Citizen Health Science: Foundations of a New Data Science Arena — PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC7299478/)
- [Patient-Powered Digital Health 2026 — DCI Network](https://www.dcinetwork.org/patients2026)
- [Digital Citizen Science Observatory — Frontiers](https://www.frontiersin.org/journals/digital-health/articles/10.3389/fdgth.2024.1399992/full)

**Ключевые тезисы:**
- Citizen science получает признание в public health, urban health, policy-making как «bottom-up» подход
- Digital Citizen Science Observatory (DiScO): цель — трансформация систем здравоохранения через этически собранные данные граждан → поддержка принятия решений на уровне policy и пациента одновременно
- 08.04.2026: UNC Chapel Hill + UNC Health запустили **SHIRE** (Secure Health Informatics Research Environment) — облачная платформа для ответственного AI с real-world clinical data
- OpenAI и Anthropic объявили возможность синхронизации личных health data с ChatGPT/Claude — показатель растущего спроса на «персонального медицинского ассистента»

**Связь с CommonHealth:**
> CommonHealth строится на той же концепции — пациент как субъект исследования, а не объект. DiScO и SHIRE — институциональное подтверждение правильности вектора. Ze·Guide + Lab = реализация citizen science внутри экосистемы.

---

### [ОБЩЕЕ] Права пациентов на данные и Longevity Medicine 2026

**Источники:**
- [2026 — The Longevity Medicine Manifesto — David Luu](https://newsletter.longevitydocs.org/p/2026-the-longevity-medicine-manifesto)
- [Patient Data Ownership — Oxford JLB](https://academic.oup.com/jlb/article/8/2/lsab023/6380070)
- [Ownership of Health Data, Sharing, and Governance — BMC Medical Ethics](https://bmcmedethics.biomedcentral.com/articles/10.1186/s12910-022-00848-y)
- [Recent Digital Health Trends February 2026 — OpenLoop Health](https://openloophealth.com/blog/recent-digital-health-trends-and-news-february-2026)

**Ключевые тезисы:**
- В 2026 году сеть 500+ врачей в 50+ странах формирует «Longevity Medicine Manifesto» — 5 элементов: клинические стандарты, real-world outcomes data, управление без коммерческого влияния, инфраструктура для медицинских карьер, критическая масса обученных специалистов
- Дебаты об «ownership» здоровых данных: ни частная, ни публичная модель не решает задачу полностью — акцент смещается на **Data Access Committees** и процедурные механизмы
- Консенсус 2026: нужны прозрачные universal privacy policies, охватывающие сбор, хранение, передачу и ownership raw data

**Связь с CommonHealth:**
> FCLC + 5-уровневый privacy stack напрямую решает этот вопрос. Данные остаются у пользователя, агрегируются криптографически — не продаются. CommonHealth Pro = модель, где платформа зарабатывает на здоровье людей, а не на их данных.

---

### [FCLC] Федеративное обучение и приватность в здравоохранении 2026

**Источники:**
- [Securing FL with Blockchain in Medical Field — JMIR 2026](https://www.jmir.org/2026/1/e79052)
- [Federated Learning 2026: Privacy-First AI — Programming Helper](https://www.programming-helper.com/tech/federated-learning-2026-privacy-first-ai-training)
- [Federated Microservices + Blockchain — Nature Scientific Reports 2026](https://www.nature.com/articles/s41598-026-39837-1)
- [Health-FedNet Privacy Framework — ScienceDirect 2026](https://www.sciencedirect.com/science/article/pii/S2590123025025538)
- [FED-EHR Decentralized Analytics — MDPI 2026](https://www.mdpi.com/2079-9292/14/16/3261)
- [Federated Deep Learning for IoT Healthcare — Frontiers 2026](https://www.frontiersin.org/journals/computer-science/articles/10.3389/fcomp.2026.1725597/full)

**Ключевые тезисы:**
- Federated microservices: Kubernetes + TensorFlow Federated + Hyperledger Fabric → predictive accuracy 95.2%, API latency −42%, recovery time 10× faster vs monolithic
- Defense-in-depth: FL + Secure Enclaves + Differential Privacy — стандарт де-факто для sensitive medical data
- Secure Aggregation: сервер видит только агрегат, никогда индивидуальные апдейты
- DP + calibrated noise → математическая гарантия privacy при gradient leakage
- HIPAA + GDPR compliance теперь реализуется через federated архитектуры напрямую

**→ Дублировано в FCLC/KNOWLEDGE.md**

---

### [Ze] EEG-based Brain Age Clock — прорыв 2026

**Источники:**
- [BrainYears: EEG Brain Age Clock — bioRxiv 2026](https://www.biorxiv.org/content/10.64898/2026.03.26.714124v1.full)
- [Wearable Aging Clock — Nature Communications 2025](https://www.nature.com/articles/s41467-025-64275-4)
- [WHOOP 2026 Health Report: Rise of Healthspan — The Manual](https://www.themanual.com/fitness/whoop-2026-health-report/)
- [HRV-CV as Digital Biomarker 2026 — Science for ME](https://www.s4me.info/threads/heart-rate-variability-coefficient-of-variation-during-sleep-as-a-digital-biomarker-that-reflects-behavior-and-varies-by-age-and-sex-2026-grosicki.49521/)

**Ключевые тезисы:**
- **BrainYears (bioRxiv, март 2026):** EEG brain age clock на ML — Pearson r = **0.92**, MAE = **4.43 лет**. Нейромодуляционная интервенция снизила predicted brain age на **−5.18 лет** в группе. Нет MRI — только EEG. Portable, cost-effective, повторяемые измерения дома
- **Nature Communications (2025):** Wearable PPG aging clock сильно ассоциирует с болезнями, поведением, продольными изменениями. Подтверждает: wearable bio-age viable
- **HRV-CV (2026):** coefficient of variation HRV за ночь = scalable digital biomarker для поведенческого мониторинга, стратификации риска. Связан с алкоголем, физактивностью, качеством сна
- **WHOOP Age:** 9 параметров (sleep consistency, HRV, time in HR zones и др.) → biological age. Массовый продукт, подтверждающий рыночный спрос

**Связь с Ze:**
> BrainYears (r=0.92, MAE=4.43 лет) — прямой конкурент/валидатор χ_Ze. Chi_Ze даёт R²=0.84 на EEG+HRV. BrainYears — чисто EEG r=0.92. Нужно: 1) позиционировать χ_Ze как более интерпретируемый (физический смысл vs black-box ML); 2) WHOOP Age = доказательство рынка.

**→ Дублировано в Ze/KNOWLEDGE.md**

---

### [BioSense] Wearable EEG/HRV для longevity 2026

**Источники:**
- [Wearable EEG for MCI Detection — npj Digital Medicine 2026](https://www.nature.com/articles/s41746-026-02342-w)
- [Bioelectric Signal Healthcare Monitoring — npj Biomedical Innovations 2025](https://www.nature.com/articles/s44385-025-00061-7)
- [Hume Band: Biological Age Wearable 2026 — Newswire](https://www.newswire.com/news/hume-band-review-2026-biological-age-metabolic-health-wearable)
- [10 Top HRV Biofeedback Monitors 2026 — Outliyr](https://outliyr.com/best-hrv-biofeedback-monitors)

**Ключевые тезисы:**
- **npj Digital Medicine (2026):** Wearable EEG показывает высокий потенциал как скрининговый инструмент MCI (mild cognitive impairment) — прямое измерение нейронных осцилляций и функциональной связности
- **Hume Band 2026:** коммерческий wearable с HRV, SpO2, sleep, temperature → longevity feedback + biological age. Рынок wearable sleep-tracking к 2026 достиг **$7B**
- **Биоэлектрические сигналы:** multi-sensor EEG+PPG → real-time brain activity + blood flow + cognitive performance + emotional regulation — всё в одном устройстве
- Основная проблема consumer EEG: overestimation sleep, underestimation wakefulness vs polysomnography

**Связь с BioSense:**
> Hume Band — прямой аналог BioSense MVP. BioSense должен позиционироваться: 1) открытый стандарт (не закрытый Hume), 2) χ_Ze как теоретически обоснованный индекс (не proprietary black-box), 3) интеграция с CommonHealth/FCLC.

**→ Дублировано в BioSense/KNOWLEDGE.md**

---

### [CDATA] Центросомы, старение и сенесценция 2025–2026

**Источники:**
- [Drivers of Centrosome Abnormalities: Senescence Progression and Tumor Immune Escape — ScienceDirect 2025](https://www.sciencedirect.com/science/article/abs/pii/S1044579X25000173)
- [PLK4: Master Regulator of Centriole Duplication — Cytoskeleton 2025](https://onlinelibrary.wiley.com/doi/full/10.1002/cm.22031)
- [Senescence in Cancer — Cancer Cell 2025](https://www.cell.com/cancer-cell/fulltext/S1535-6108(25)00224-7)
- [Centrosome Dysfunction: Link Between Senescence and Tumor Immunity — Nature STTT](https://www.nature.com/articles/s41392-020-00214-7)

**Ключевые тезисы:**
- **ScienceDirect 2025:** Центросомные аберрации — hallmarks рака + сенесценции. ECASP (extra centrosome-associated secretory phenotype) через хроническую NF-κB активацию → IL-8, GDF-15, ANGPTL4. IL-8 = компонент SASP → иммуносупрессивное микроокружение
- **PLK4 клинические испытания (2025):** PLK4 — master regulator дупликации центриолей. Ингибитор **RP-1664** (orally bioavailable) вошёл в клинические испытания
- **Cancer Cell 2025:** Сенесценция играет двойную роль в предраке: сначала тумор-супрессорный барьер, потом про-туморальный PreTME через паракринный SASP
- Центросомная амплификация — наиболее частый дефект в опухолях; связана с геномной нестабильностью и ускоренным старением

**Связь с CDATA:**
> PLK4 ингибитор в клинике — прямое подтверждение терапевтического направления #2 (протеасомальная очистка / регуляция дупликации). ECASP + NF-κB путь хорошо согласуется с CDATA моделью: поврежденные центриоли → SASP → сенесценция → старение ткани. Обновить MCAI-модель с учётом ECASP компонента.

**→ Дублировано в CDATA/KNOWLEDGE.md**

---

## Ссылки для дальнейшего чтения

| Тема | Приоритет | Ссылка |
|------|-----------|--------|
| BrainYears preprint (полный текст) | ВЫСОКИЙ | https://www.biorxiv.org/content/10.64898/2026.03.26.714124v1.full |
| Federated Microservices + Blockchain (Nature) | ВЫСОКИЙ | https://www.nature.com/articles/s41598-026-39837-1 |
| PLK4 inhibitor RP-1664 clinical trial | ВЫСОКИЙ | https://onlinelibrary.wiley.com/doi/full/10.1002/cm.22031 |
| Wearable EEG for MCI (npj Digital Med) | СРЕДНИЙ | https://www.nature.com/articles/s41746-026-02342-w |
| DiScO — Digital Citizen Science Observatory | СРЕДНИЙ | https://www.frontiersin.org/journals/digital-health/articles/10.3389/fdgth.2024.1399992/full |
| SHIRE platform UNC (08.04.2026) | СРЕДНИЙ | https://www.unc.edu/posts/2026/04/08/university-unc-health-unveil-shire-health-care-innovation-platform/ |
| Senescence in Cancer (Cancer Cell 2025) | СРЕДНИЙ | https://www.cell.com/cancer-cell/fulltext/S1535-6108(25)00224-7 |
| HRV-CV biomarker 2026 | НИЗКИЙ | https://www.s4me.info/threads/heart-rate-variability-coefficient-of-variation-during-sleep-as-a-digital-biomarker-that-reflects-behavior-and-varies-by-age-and-sex-2026-grosicki.49521/ |

---

*NEWS.md — файл ядра CommonHealth | обновлять при каждой сессии*
