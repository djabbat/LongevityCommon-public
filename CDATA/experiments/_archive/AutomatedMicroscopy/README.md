# AutomatedMicroscopy — LongevityCommon subproject

**Purpose:** Low-cost ($4,500) AI-operated time-lapse microscopy platform для round-the-clock live-cell imaging, enabling single-PI labs to conduct industrial-grade imaging experiments without human shift overhead.

**Parent ecosystem:** LongevityCommon (longevity research ecosystem)
**Flagship role:** Experimental infrastructure for CDATA Phase A (Impetus Grant 2026-04-25) + future MCOA Counter validation experiments

**Status:** Engineering design complete (2026-04-21). Bill-of-materials ready. Assembly expected Months 1-2 of Phase A Impetus grant (if funded).

**Core innovation:** Claude Code `/overnight` режим управляет микроскопом, интерпретируя естественно-языковой PROMPT (описание целей и задач эксперимента), принимая routine decisions автономно и сигнализируя человека только при стратегически важных событиях.

**Budget target:** $4,500 retrofit (Вариант A DIY) vs $12,700 mid-tier (Вариант B) vs $25-50k turnkey (Вариант C).

## Quick links

- **Theory:** see `THEORY.md`
- **Evidence / references:** see `EVIDENCE.md`
- **Open problems / research questions:** see `OPEN_PROBLEMS.md`
- **Bill of materials / quantitative params:** see `PARAMETERS.md`
- **System architecture / code structure:** see `DESIGN.md`
- **AI agent instructions:** see `AGENTS.md`
- **Changelog / decisions:** see `JOURNAL.md`
- **Future roadmap:** see `ROADMAP.md`

## Контекст в экосистеме LongevityCommon

AutomatedMicroscopy — **инфраструктурный слой** для experimental подпроектов (CDATA, Telomere, MitoROS, EpigeneticDrift, Proteostasis), которые требуют длительного live-cell imaging.

Сравнение с другими подпроектами:
- **CDATA, Telomere, etc.** — scientific hypotheses / damage counters
- **FCLC** — federated data sharing infrastructure
- **MCOA** — theoretical framework
- **AutomatedMicroscopy (this)** — experimental infrastructure for data collection

## Ссылки

- Parent: `~/Desktop/LongevityCommon/CONCEPT.md`
- Related grant: `~/Documents/Grants/LongevityCommon/CDATA/docs/IMPETUS_2026-04-25/`
- External source: `~/Documents/Engineering/AutomatedMicroscopy_2026-04-21/`

## License

MIT (all code + BOM + PROMPT templates released post-Phase A).
