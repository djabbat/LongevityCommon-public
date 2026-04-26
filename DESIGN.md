# DESIGN — LongevityCommon (Ecosystem)

**Назначение:** архитектура umbrella + связь подпроектов.

## 1. Stack

| Слой | Технология |
|---|---|
| Backend API | Rust (Axum, sqlx) |
| Frontend | React + TypeScript (Vite, PWA) |
| Realtime | Elixir/Phoenix (Channels, LiveView) |
| Auth | OIDC через Keycloak (планируется) |
| DB | PostgreSQL 15+ (sqlx compile-time queries) |
| Federated layer | FCLC (Rust SecAgg+, Python OMOP) |

## 2. Repository structure (monorepo)

```
LongevityCommon/
├── server/                — Rust/Axum REST API
├── web/                   — React TS PWA
├── realtime/              — Elixir/Phoenix Channels
├── MCOA/                  — мета-теория
├── CDATA/                 — Counter #1
├── HAP/                   — гепато-аффективная теория
├── Ze/                    — Entropic-Geometric TOE (книги + код)
├── BioSense/              — EEG/HRV/olfactory
├── FCLC/                  — federated learning
├── Ontogenesis/           — онтогенез 0-25
└── _archive/              — старые версии
```

## 3. Subproject coupling rules

| Тип coupling | Allowed | Notes |
|---|---|---|
| Подпроект ↔ MCOA | да | через CONCEPT.md cross-reference |
| Подпроект ↔ FCLC | да | через REST API |
| Подпроект ↔ другой подпроект | да | через documented interfaces |
| Code dependencies между подпроектами | нет | каждый — independent crate/repo |

## 4. Database schema

См. `server/migrations/001_initial.sql`.

Ключевые таблицы:
- `users`, `consents`, `gdpr_export_requests`
- `ze_samples` — биосенсорные данные (организм)
- `health_factors` — психика/сознание/социум
- `posts`, `comments`, `ranks`
- `ze_guide_logs` — все ответы Ze·Guide AI с disclaimer
- `dois` — Crossref-validated references

## 5. Critical architecture rules

1. **No SQL injection:** только sqlx parametrized queries
2. **GDPR:** soft delete через `deleted_at`; export через `GET /api/data/export`
3. **API errors:** `Json(value)` или `(StatusCode, String)`; никогда `unwrap()` в handlers
4. **Ze·Guide disclaimer:** перед каждым ответом, логировать всё
5. **Биологический возраст:** point estimate + 95% CI + stability label всегда

## 6. Anti-fraud

- DOI verification через Crossref API при создании поста
- Неверный DOI → `rank_penalty += 2.0`

## 7. Performance targets

- API p95 < 100 ms
- Frontend FCP < 1.5 s
- Realtime channel latency < 50 ms
- DB indexes на `ze_samples`, `posts` (timestamp + user_id)
