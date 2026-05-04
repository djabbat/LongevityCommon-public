# AIM Phase 3 — журнал этапов (autonomously)

Каждый этап = один git commit на `main`. Регенерируется
`_journal/regenerate.sh` из `git log`.

Последнее обновление: **2026-05-04 20:05**.

Workspace: **90 крейтов**.

## Phase 3 commits (newest first)

| sha | message |
|-----|---------|
| 5785695 | agents Phase 3 — aim-memory-deduplicate (find + merge near-dups) |
| 1526162 | agents Phase 3 — aim-profile (multi-tenant profile isolation) |
| 40f2ebc | agents Phase 3 — aim-health-extended (G9 full system snapshot) |
| 1490b8e | agents Phase 3 — aim-memory-health (healthcheck framework) |
| 5846bf8 | agents Phase 3 — aim-recall-perf (SL1 cache + slow-query detector) |
| 1b29171 | agents Phase 3 — aim-own-pubs-tracker (PV1 Crossref author watcher) |
| fae742b | agents Phase 3 — aim-regimen-validator (D1 hard-refusal layer) |
| c6832ad | agents Phase 3 — aim-voice (voice in/out shim) |
| bac4d70 | agents Phase 3 — aim-follow-up-generator (E1 polite drafts) |
| b0d67c7 | agents Phase 3 — aim-unicode-guard (UN1 lookalike-attack guard) |
| be3c389 | agents Phase 3 — aim-speculative + aim-impact-analyser |
| 4a83ee3 | agents Phase 3 — aim-quick-action (Q1 deterministic intent classifier) |
| 39f08c1 | agents Phase 3 — aim-smart-context (importance-aware truncation) |
| 32ed5ca | agents Phase 3 — aim-smart-fallback (multi-tier provider chain) |
| a4d899b | agents Phase 3 — aim-project-state-machine (P5 phase transitions) |
| 47899c1 | agents Phase 3 — aim-project-owner (P1 long-running project agent) |
| b651a23 | agents Phase 3 — aim-stakeholder-tracker (P3 contacts DB) |
| 39aecb5 | agents Phase 3 — aim-smart-routing (cheapest-adequate-model picker) |
| b007df1 | agents Phase 3 — aim-brief-preamble (B1 morning header) |
| 6491f49 | agents Phase 3 — aim-kpi-tracker (K1 per-project KPIs) |
| 180f254 | agents Phase 3 — aim-brief-preferences (B2 user-tunable digest) |
| 78130df | agents Phase 3 — aim-debate (multi-persona high-stakes debate) |
| 370d451 | agents Phase 3 — aim-reflexion + aim-ensemble |
| b2a3d94 | agents Phase 3 — aim-skill-synthesis (S7 named-macro skills) |
| 5a93eb9 | agents Phase 3 — aim-escalation (P6 project rule DSL) |
| 8f164f5 | agents Phase 3 — aim-tool-synthesis (S2 pair → new tool) |
| e08ece8 | agents Phase 3 — aim-cost-monitor (token-cost caps + alerts) |
| f541324 | agents Phase 3 — aim-ab-router (S5 strategy tournament) |
| afb8fe9 | agents Phase 3 — aim-permission (G3 interactive consent broker) |
| 0d097ec | AIM/STACK.md — record "no Docker" hard rule |
| 34d5b62 | agents Phase 3 — aim-llm-cache (semantic LLM response cache) |
| ff0abde | agents Phase 3 — aim-feature-flags (FX1 cleanup-tag tracker) |
| 3fe6ddd | agents Phase 3 — aim-request-deduplicator (sliding-TTL dup drop) |
| e228446 | agents Phase 3 — aim-adaptive-limiter (token bucket + backpressure) |
| eff2136 | agents Phase 3 — aim-ast-verify (R-3 Python AST fact verifier) |
| ae13509 | agents Phase 3 — aim-prompt-optimizer (evolutionary prompt search) |
| 4d3fda8 | agents Phase 3 — aim-complexity (heuristic task classifier) |
| cc60628 | agents Phase 3 — aim-citation-linter (PR2 repo-wide citation lint) |
| 0b4b379 | agents Phase 3 — aim-literature-watch (L2 PubMed dedup digest) |
| 5e61fd3 | agents Phase 3 — aim-evals (S1 closed-loop self-improvement keystone) |
| 9c6b812 | agents Phase 3 — pattern-miner (S4 session-log analyser) |
| c851892 | agents Phase 3 — hub-client + hub-auth (multi-user infrastructure) |
| 8f53711 | agents Phase 3 — aim-notify (multi-channel notification mux) |
| fc47e86 | agents Phase 3 — citation-guard + deadline-scanner + morning-brief wired |
| a294de6 | agents Phase 3 — cost-ledger + worktree (Rust ports) |
| 0cde4cb | ai/aim Phase 2 COMPLETE — eval-synthesiser + self-modify |
| 0193a54 | ai/aim Phase 2 cont. — self-diagnostic + runner |
| 147fe41 | ai/aim-dashboard: wire explainer + doctor sections |
| 49d8062 | ai/aim Phase 2 cont. — aim-ai-doctor (DR2 wiring smoke) |
| e6271f2 | ai/aim Phase 2 cont. — explainer (EX1) |
| c7fa92c | ai/aim Phase 2 cont. — finding-validator + auto-sweep |
| fb5e1f1 | ai/aim Phase 2 cont. — gap-detector + dashboard fully wired |
| df4e0f4 | ai/aim Phase 2 cont. — distillation + reflexion clusters |
| d45ff46 | ai/aim Phase 2 cont. — compliance-promoter + skill-standard |
| d8da7e8 | ai/aim Phase 2 cont. — case-archiver + morning-brief + findings-to-evals |
| 824553b | ai/aim Phase 2 cont. — prompt-impact + regression-alert + backup |
| f088ce7 | ai/aim Phase 2 cont. — safety-gate + suppressions + i18n |
| e216182 | ai/aim Phase 2 cont. — meta-evaluator + stable-run + dashboard |
| 6bcd8a7 | ai/aim Phase 2 cont. + donate everywhere + UPGRADE planning |
| 9f03ea0 | ai/aim: 3 Rust crates + Phoenix DiagLive (closed-loop in Rust) |
| 74d6fad | aim-web: Phoenix LiveView app, first screen HiveLive |
| 30d36fa | hive: aim-hive-queen + aim-hive-consumer Rust crates (Phase 1 done) |
| 07a3917 | aim-hive-worker: Rust port of AI/ai/hive_telemetry.py + STACK rule |
| 513cbc8 | aim-dp: Rust crate for DP-budget accountant + Gaussian noise |
| 08ae865 | audit fixes: Phoenix loopback bind + Erlang dist port pinning |
| 8008450 | deploy: comment out ze-web/biosense-web from docker-compose-all.yml |
| fcf3799 | phoenix: native essence panels in Ze/BioSense templates + dark mode |
| 8f1f173 | deploy: native systemd units for Ze/BioSense/FCLC Phoenix services |
| 94ffd77 | eco-inject: collapsible project-essence panel on Phoenix subdomains |
| c7b6536 | project landings: detailed essence on subdomains, brief on longevity.ge root |
| 2acb516 | release: AIM v0.1.0 — packaged Linux/macOS/Windows releases |
| 4dca60b | longevity.ge home: detailed sub-project cards + Hive added |
| 2ac0342 | hive: load shared eco-inject.js → cross-subdomain header above queen header |
| c411181 | eco-inject: per-subdomain SVG favicon (MCOA/CDATA/Ze/BioSense/FCLC/Hive) |
| 202c994 | hive: shared header + Hive nav link + OS-detected install + FCLC borrow plan |
| 5d345f2 | AIM/AI queen_deploy: web landing page + Hive button integration guide |
| 72d62eb | AIM: per-user LLM provider keys + setup-key CLI command |
| a968e80 | AIM/AI: queen_deploy/ — production deployment package for hive.longevity.ge |
| 93f4362 | AIM/AI Hive: federated collective intelligence (P1-P4 implemented) |
| 04a7cf4 | AIM/AI: hive architecture proposal — queen + worker bees federation |
| d161dd4 | AIM/AI dashboard: --compact for Telegram-friendly 1-line-per-section render |
| f1d1044 | AIM/AI: EX1 explainer — actionable score breakdown |
| 6a678cd | AIM/AI auto_sweep: prune phantoms BEFORE score snapshot |
| 38dde9e | AIM/AI: FV1 finding_validator + smoke extensions |
| f970275 | AIM overnight wave (2026-05-03): closed-loop diagnostic infrastructure |
| fbcd5d3 | AIM: cleanup time-stamped backup snapshots (agents.bak.*) |
| c560df6 | AIM: gitignore time-stamped agent snapshots (agents.bak.*) |
| afdd522 | AIM: harden bash + web_fetch gates, raise memory_recall timeout |
| addd0d0 | AIM Ze-AST: third sceptic layer (semantic checks beyond regex) |
| 9bcb8cf | AIM kernel: activate dead laws, central orchestrator, Ze-everywhere, auto-verify |
| cac75cc | AIM SSA/DiffDiagnosis: fix stale ~/Desktop/AIM paths |
| 162960c | AIM CLI: shell-prompt sanitizer, paste mode, /maxiters command |
| 1ebddc3 | chore: AIM merged into LongevityCommon as subproject (per subproject rule) |
| 163116d | Squashed 'Ze/' content from commit b0f8d5f |
| 8dee658 | Squashed 'CDATA/' content from commit c2b0438 |
| bbfe13f | Squashed 'BioSense/' content from commit 0eea0fc |
| b2413bb | remove Ze for clean subtree integration |
| eb5ee19 | remove CDATA for clean subtree integration |
| 851f096 | remove BioSense for clean subtree integration |
| c01c048 | remove Ontogenesis for clean subtree integration |

## Текущая ветка

```
## main...origin/main [ahead 42]
 M ROADMAP_SURPASS_ClaudeCode_2026-05-02.md
?? _journal/
?? ../erl_crash.dump
```
