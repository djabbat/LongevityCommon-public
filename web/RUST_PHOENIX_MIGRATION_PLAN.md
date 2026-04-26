# LongevityCommon/web (React/TypeScript) → Phoenix LiveView Migration Plan

**Дата:** 2026-04-25 (overnight)

---

## Текущее состояние web/

```
web/
├── src/
│   ├── components/   — React TS компоненты
│   ├── pages/        — page-level routes
│   ├── hooks/        — custom React hooks
│   └── api/          — fetch wrappers для Rust backend
├── public/
├── package.json      — Vite + React 18 + TS
├── tsconfig.json
└── vite.config.ts
```

## Целевая архитектура (Phoenix LiveView)

```
realtime/                  ← существует, базовый Phoenix Channels
└── lib/
    └── ze_web/
        ├── live/
        │   ├── dashboard_live.ex     ← главная (replaces web/src/pages/Dashboard)
        │   ├── ze_profile_live.ex    ← Ze·Profile (replaces ZeProfile.tsx)
        │   ├── ze_guide_live.ex      ← Ze·Guide AI чат (replaces ZeGuide.tsx)
        │   ├── feed_live.ex          ← лента постов (replaces Feed.tsx)
        │   └── data_export_live.ex   ← GDPR export (replaces DataExport.tsx)
        ├── components/
        │   ├── chart_component.ex    ← графики через Chart.js hook
        │   ├── disclaimer.ex         ← Ze·Guide disclaimer (обязателен)
        │   └── language_picker.ex    ← 9 языков
        └── controllers/
            └── api_proxy.ex          ← proxy к Rust REST на 4001
```

## Ключевые отличия LiveView vs React

| Аспект | React | Phoenix LiveView |
|---|---|---|
| State | useState/useReducer + context | server-side `assigns` |
| Routing | React Router | Phoenix Router + LiveView |
| API calls | fetch + useEffect | Rust REST через handle_event/handle_info |
| Realtime | WebSocket + custom | Phoenix Channels + PubSub встроены |
| Bundling | Vite | esbuild + tailwind через Phoenix |
| Type safety | TypeScript | Elixir + dialyzer |

## Преимущества миграции

1. **Меньше кода** — LiveView устраняет JS state management
2. **Realtime out of box** — Phoenix Channels уже работают
3. **Стек унифицирован** — экосистема LongevityCommon уже использует Elixir для realtime/
4. **Server-side rendering** — лучше SEO + быстрее первая загрузка
5. **Меньше CVE-surface** — нет npm dependency hell

## Недостатки

1. **Offline mode** — LiveView требует постоянного соединения
2. **PWA capabilities** — нужно отдельно реализовать (web push, service worker)
3. **Mobile UX** — анимации сложнее без client-side framework

## Стратегия миграции (поэтапная)

### Phase 1 — Hybrid (1-2 месяца)

Phoenix LiveView для нового UI; React сохраняется для:
- PWA / offline-first features
- Heavy animations (recharts, custom canvases)
- Третьесторонние integrations (Stripe Elements, Telegram Web App)

### Phase 2 — LiveView dominant (3-6 месяцев)

Большинство страниц переведены. React остаётся только для:
- Stripe payment forms (PCI scope)
- Telegram Web App embed

### Phase 3 — Phoenix only (6-12 месяцев)

Полная замена. Stripe — через Stripe.js client-side в LiveView hook.
Telegram — через iframe / Phoenix Channel proxy.

## Конкретные файлы для миграции

| React file | Phoenix LiveView equivalent | Сложность |
|---|---|---|
| `pages/Dashboard.tsx` | `live/dashboard_live.ex` | Low (basic layout) |
| `pages/ZeProfile.tsx` | `live/ze_profile_live.ex` | Medium (4 health factors) |
| `pages/ZeGuide.tsx` | `live/ze_guide_live.ex` | High (AI chat + disclaimer logging) |
| `pages/Feed.tsx` | `live/feed_live.ex` | Medium (DOI verification) |
| `components/Chart.tsx` | `components/chart_component.ex` | Medium (Chart.js hook) |
| `hooks/useAuth.ts` | session-based в `controllers/auth.ex` | Low |
| `api/client.ts` | `controllers/api_proxy.ex` | Low (просто HTTP proxy) |

## Скрипт автоматической миграции (план)

Не существует автоматического React→LiveView конвертера. Миграция вручную с поэтапным testing.

Рекомендация: использовать AIM (DeepSeek-Reasoner) для генерации LiveView эквивалентов из React TS — затем code review.

## Status

- [x] Migration plan создан
- [ ] Phase 1: новые страницы в LiveView
- [ ] Phase 2: миграция core страниц (Dashboard, Feed, ZeProfile)
- [ ] Phase 3: финальная замена React

## Альтернатива

Оставить React TS для web/ как есть — это **valid choice**:
- React TS = mature stack для PWA/mobile-first
- Phoenix используется только для realtime/ компонентов
- Нет жёсткого обязательства "всё на Phoenix"

В правиле "конвертировать в Rust или Phoenix" web frontend — серая зона. React TS для PWA лучше Phoenix LiveView для специфических use cases (offline-first, mobile install). Решение зависит от приоритета.

**Рекомендация (overnight):** оставить React TS для web/ до явного решения пользователя. Миграция — отложенный бэклог.
