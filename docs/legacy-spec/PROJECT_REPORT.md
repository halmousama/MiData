# MiData — Complete Project Report

> **Generated:** 2026-10-26
> **Method:** Every statement below was checked against the working tree on branch `adr-004-progressive-disclosure` (last commit `0c06a65`). Documentation was used only as context and is flagged wherever it disagrees with the code.
> **Verification status at time of writing:**
> - Backend: `485 passed, 2 skipped` in 11.4 s (`backend/.venv/bin/python -m pytest tests/ -q`)
> - Frontend: `76 passed` in 3 files (`pnpm run test`)
> - Frontend lint: `25 errors, 33 warnings` (`pnpm run lint`, exit 1)
> - Git: 80 commits, **0 tags**, no CI.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [The Idea: Problem, Audience, Positioning](#2-the-idea-problem-audience-positioning)
3. [Feature Inventory](#3-feature-inventory)
4. [Repository Map](#4-repository-map)
5. [System Architecture](#5-system-architecture)
6. [Data Architecture: The EAV Model](#6-data-architecture-the-eav-model)
7. [Backend Deep Dive](#7-backend-deep-dive)
8. [API Reference](#8-api-reference)
9. [Analytics Subsystem](#9-analytics-subsystem)
10. [AI Assistant Subsystem](#10-ai-assistant-subsystem)
11. [Domain Workflows](#11-domain-workflows)
12. [Frontend Architecture](#12-frontend-architecture)
13. [Design System](#13-design-system)
14. [Desktop Application & Packaging](#14-desktop-application--packaging)
15. [Configuration, Secrets, and Environment](#15-configuration-secrets-and-environment)
16. [Security Model](#16-security-model)
17. [Testing & Quality](#17-testing--quality)
18. [Build, Run, and Deploy](#18-build-run-and-deploy)
19. [Code-Level Findings: Docs Drift and Bugs](#19-code-level-findings-docs-drift-and-bugs)
20. [Technical Debt & Risks](#20-technical-debt--risks)
21. [Roadmap: ADR-004 and the Current Branch](#21-roadmap-adr-004-and-the-current-branch)
22. [Glossary](#22-glossary)
23. [Reading Guide: How to Onboard onto This Codebase](#23-reading-guide-how-to-onboard-onto-this-codebase)

---

## 1. Executive Summary

**MiData** is a local-first livestock management and analysis platform. It is a full-stack monorepo with:

| Layer | Technology | Role |
|---|---|---|
| Backend | Python 3.12.4, FastAPI 0.109, SQLModel 0.0.14, SQLite | REST API, schema owner, business logic |
| Analytics | DuckDB 0.10 (`:memory:`) + pandas/NumPy/SciPy | Fast EAV pivoting and statistics |
| Frontend | React 19.2.7, TypeScript 5.9.3, Vite 7.3.5, Tailwind CSS 4.3.1 | SPA served by the backend |
| AI | OpenAI-compatible local LLM (default LM Studio at `http://localhost:1234/v1`) | Conversational operator with tool-calling |
| Desktop | PyInstaller + CustomTkinter + pystray + pyngrok | Tray app that boots Uvicorn and exposes LAN/Internet access |

The central engineering bet is the **Entity–Attribute–Value (EAV) data model**: an animal is an identity row, every attribute is a row in a metadata table, and every value is a timestamped row. Adding a new measurable attribute — "hoof score", "withdrawal days", "EPD" — is *configuration*, not a schema migration. Everything else in the system (dynamic forms, modes, analytics, AI SQL, extensions, onboarding bundles) is built on top of that one decision.

The product is explicitly **offline-first, account-free, and Maghreb-oriented**: default currency TND, Arabic-speaking AI responses, and a hard cultural constraint that **Pig is not a supported species** (only Cow, Sheep, and Goat have field definitions).

At the time of writing the project is **pre-1.0**: 17 controller modules, 20 business services (+15 AI tool executors), 13 ORM tables + 1 raw-SQL table, 128 frontend TypeScript files, 485 backend tests and 76 frontend tests green, but lint red, no CI, no E2E coverage, and a committed frontend bundle (`backend/static/`) that is stale and still titled "Rumetrics".

---

## 2. The Idea: Problem, Audience, Positioning

### 2.1 The problem

Small and medium livestock operations — a few dozen to a few thousand head — usually manage records in paper notebooks, ad-hoc spreadsheets, or nothing at all. The failure mode is not a lack of data; it is that:

1. **Every farm measures different things.** A dairy cow operation tracks milk yield, somatic cell count, and calving interval. A meat sheep operation tracks average daily gain, litter size, and weaning weight. A goat operation tracks both. A generic fixed-schema database forces farms into someone else's columns.
2. **Records are locked in one place.** Data lives on one laptop, cannot be shared with a vet in the field, and cannot be analyzed without exporting to Excel.
3. **Analysis requires a data analyst.** Correlation, trends, outliers, and profitability are exactly what farmers need and exactly what they cannot compute.
4. **Cloud SaaS is a bad fit.** Connectivity is unreliable, data sovereignty matters, and recurring subscriptions are a barrier.

### 2.2 The answer

MiData combines four capabilities in one local application:

- **A flexible EAV core** so any farm can define its own attributes without code.
- **A workspace abstraction ("Modes")** so the same data can be viewed through task-specific lenses (Dairy, Vet, Reproduction, Finance) rather than as one giant spreadsheet.
- **A dual analytics engine** that pivots the EAV rows into a wide table for fast aggregation while keeping a reproducible ORM path for statistics.
- **A local AI operator** that can query the data, build dashboard widgets, and stage data changes behind explicit user confirmation — all without sending data to a cloud.

Optional LAN and ngrok sharing turn the same local server into something a vet or worker can open on a phone via QR code.

### 2.3 Audience

| Persona | Primary surfaces |
|---|---|
| Farmer / herd owner | Dashboard, Animals, Record, Analytics, AI Assistant |
| Veterinarian / technician | Health fields, validation alerts, pedigree, per-animal detail |
| Breeder | Pedigree graph, reproduction/breeding fields, birth trigger |
| Farm manager / analyst | Deep analysis, correlation matrix, profitability, Excel export |
| Developer / power user | Settings (fields, modes, formulas), Marketplace, Developer Mode (planned) |

### 2.4 Non-negotiable constraints encoded in the code

- **Species:** Cow, Sheep, Goat only. Pig references were deliberately removed. Poultry exists only in seed generation.
- **Currency:** TND is the default (`OnboardingRequest.currency = "TND"`; validator allows TND/USD/EUR).
- **Language:** AI system prompt ends with *"Format your responses in clear Arabic."* The UI and code are English.
- **Offline:** no cloud SDK, no auth server, no telemetry. The only network calls are to the user's local LLM, optional ngrok, and the browser's LAN connection.
- **Licence:** MIT.

---

## 3. Feature Inventory

### 3.1 Herd management

- Create/list/edit/delete entities with a user-defined `custom_id` (tag/serial).
- Dynamic attribute capture for every active field.
- Strict parent validation at creation: a `sire_id` must be Male, a `dam_id` must be Female.
- Soft-delete semantics for fields, modes, and extensions (`is_active`); hard delete for entities (cascades measurements).
- Per-animal detail page with measurements, charts, pedigree, alerts, and print cards.

### 3.2 Data entry

- **Batch entry:** many measurements for one animal (`POST /measurements/batch`).
- **Mass entry:** the same measurements for many animals (`POST /measurements/mass`).
- **Dynamic fields:** the form is generated from `field_definitions` (number, text, select, boolean, multi-select).
- **Excel/CSV import** with column mapping, conflict strategy (`skip` / `update`), and per-row error reporting.
- **Birth action trigger:** one atomic transaction creates offspring, records parturition, updates the dam, and writes pedigree links.

### 3.3 Organization & configuration

- **Field definitions:** key, label, category, data type, unit, JSON options, role, sort order, active flag.
- **Modes (workspaces):** named, icon-bearing, many-to-many with fields, ordered. A system mode ("General Husbandry") always shows all fields.
- **Computed fields:** formulas with `{{field_key}}` placeholders evaluated by `simpleeval`.
- **Validation rules:** declarative, date-aware rules with severity, messages, and per-entity evaluation.
- **Extensions/marketplace:** JSON manifests install fields, modes, and widgets atomically; uninstall deactivates.
- **Setup templates:** export/import workspace configuration (not data) as JSON.

### 3.4 Analytics

- Statistics (count, mean, median, std dev, min, max) per field with species/breed/sex/date filters.
- **Analysis Studio:** trend, distribution, correlation, and comparison studies returning chart-ready data, regression line, outliers, and generated insights.
- **Deep dive:** single-variable report with histogram, time trend, breed/sex breakdowns, top/bottom outliers, insights.
- **Correlation matrix** (Pearson) across numeric fields.
- **Dashboard:** server-driven widgets (KPI, bar, line, pie, table, progress list, alert board) whose SQL lives in the database.
- **Profitability:** dynamic production-field resolution, milk price, feed cost, currency, top/bottom performers.
- **Pedigree graph:** root-centric BFS, cycle-safe, phantom external parents, capped at 500 nodes.
- **Excel export:** flattened wide table, plus a summary workbook.

### 3.5 AI assistant

- Streaming SSE chat with 15 tool families.
- Read-only SQL/DSL querying with AST validation and column whitelisting.
- Preview → confirm workflow for writes, using one-time tokens.
- Conversation persistence, rename/delete, pinned 420 px sidecar.
- AI widget generation from natural language (SQL + chart blueprint, HTML sanitized).
- Context governance: row caps, size caps, token budget, smart summarization.
- Audit trail table for AI-driven changes.

### 3.6 Sharing & desktop

- LAN access via the launcher: binds `0.0.0.0`, prints a QR code with the LAN IP.
- Optional ngrok public tunnel, token stored in `~/.midata/midata_config.json`.
- System tray menu: Open, Network Settings, Public Link, Console Log, Exit.
- PyInstaller one-folder build (`MiData_dist/`).

---

## 4. Repository Map

```
MiData/
├── AGENTS.md                     Agent memory + engineering rules (Halmas ecosystem)
├── README.md                     Product overview (partly stale — see §19)
├── CONTRIBUTING.md               Contribution rules
├── LICENSE                       MIT
├── .python-version               3.12.4
├── .github/                      Issue/PR templates, CODEOWNERS, ISSUES/ backlog
├── .halmas/                      Live project memory (SOUL, STATE, TASKS, DECISIONS, …)
├── docs/
│   ├── DESIGN.md                 Bento Grid + Earthy Organic design system
│   ├── CHANGELOG.md              [Unreleased] + 1.0.0-beta (2026-06-01)
│   ├── dev/                      architecture, setup, testing, desktop, ADRs, ai/, backend/, frontend/
│   ├── user/                     end-user guides
│   └── PROJECT_REPORT.md         ← this document
├── backend/
│   ├── app/
│   │   ├── main.py               FastAPI app, router registration, SPA catch-all
│   │   ├── config.py             Pydantic Settings
│   │   ├── controllers/          17 HTTP modules
│   │   ├── services/             20 domain services
│   │   │   └── tools/            15 AI tool executors
│   │   ├── database/             tables.py (13 models) + db.py (engine, migrations)
│   │   ├── models/               Pydantic DTOs
│   │   └── data/                 bundle_catalogue.py (6 onboarding bundles)
│   ├── extensions/               3 JSON manifests (livestock, health, analytics)
│   ├── scripts/                  seed.py + seed_data/
│   ├── static/                   ⚠️ committed, stale SPA build
│   ├── tests/                    19 test modules + conftest
│   ├── launcher.py               tray app / ngrok / QR / server manager
│   ├── MiData.spec               PyInstaller spec
│   ├── requirements.txt          backend + test + desktop deps (mixed)
│   └── requirements-dev.txt      black, ruff
├── frontend/
│   ├── src/
│   │   ├── app/                  App, Layout, routes
│   │   ├── ui/                   9 UI slices
│   │   ├── api/                  7 typed API wrappers
│   │   ├── state/                queryClient, queryKeys, hooks, contexts
│   │   ├── transport/            single Axios instance
│   │   ├── shared/               types + pure utils
│   │   ├── tests/                3 Vitest files + setup
│   │   └── index.css             Tailwind v4 theme
│   ├── package.json              pnpm scripts + deps
│   ├── vite.config.ts            React + Tailwind plugins, Vitest jsdom
│   └── eslint.config.js          flat config, React hooks + refresh rules
└── midata_data/                  ⚠️ runtime dir (gitignored): livestock.db, exports/, backups/
```

**Size (measured):**

| Area | Files | Lines |
|---|---|---|
| `backend/app` | 75 `.py` | 17,051 |
| `backend/tests` | 20 `.py` | 10,289 |
| `frontend/src` | 128 `.ts/.tsx` | 23,573 (incl. CSS) |

---

## 5. System Architecture

### 5.1 Deployment topology (as implemented)

```
┌──────────────────────────────────────────────────────────────────┐
│ Desktop machine                                                  │
│                                                                  │
│  ┌─────────────────────────┐        ┌──────────────────────────┐ │
│  │ launcher.py             │        │ Browser (SPA)            │ │
│  │  • CustomTkinter root   │        │  React 19 + Vite build   │ │
│  │  • pystray tray icon    │        │  served by FastAPI       │ │
│  │  • UvicornManager       │        └────────────┬─────────────┘ │
│  │  • NgrokManager         │                     │ HTTP          │
│  │  • QR codes             │                     │               │
│  └────────────┬────────────┘                     │               │
│               │ starts                            │               │
│  ┌────────────▼──────────────────────────────────▼─────────────┐ │
│  │ FastAPI (Uvicorn, 0.0.0.0:8000–8009)                        │ │
│  │  controllers → services → SQLModel → SQLite                 │ │
│  │                        ↘ DuckDB :memory: (sqlite_attach)    │ │
│  └────────────┬───────────────────────────────┬────────────────┘ │
│               │                               │                  │
│     ┌─────────▼─────────┐          ┌──────────▼───────────────┐  │
│     │ midata_data/      │          │ ~/.midata/               │  │
│     │  livestock.db     │          │  midata_config.json      │  │
│     │  exports/         │          │  onboarding_config.json  │  │
│     │  backups/         │          └──────────────────────────┘  │
│     └───────────────────┘                                        │
└──────────────────────────────────────────────────────────────────┘
                              │ optional
                    ┌─────────▼─────────┐
                    │ ngrok public URL  │  ← QR code / clipboard
                    └───────────────────┘
```

There is **no separate frontend server in production**: `app/main.py` mounts `/assets` and serves `static/index.html` for every non-API path.

### 5.2 Request lifecycle

```
Browser
  → Axios (src/transport/client.ts; X-API-Key header; 10 s timeout)
  → FastAPI router (controllers/, thin)
  → Service (services/, business logic)
  → SQLModel Session → SQLite (WAL, FK ON)
        or
  → DuckDB cursor (analytics: sqlite_attach + PIVOT)
  → Pydantic response model
  → JSON / SSE / FileResponse
  → TanStack Query cache (30 s staleTime, 5 min gcTime)
  → React component
```

### 5.3 Backend layering (actual)

| Layer | Location | Responsibility | Rule observed |
|---|---|---|---|
| HTTP | `app/controllers/` | Parse/validate request, map errors to HTTP codes | No SQL beyond simple lookups in some controllers |
| Domain | `app/services/` | Transactions, algorithms, orchestration | Called by controllers and AI tools |
| Schemas | `app/models/` | Pydantic DTOs, request/response contracts | Separate from ORM models |
| Persistence | `app/database/` | Engine, PRAGMAs, tables, migrations | Single source of schema truth |
| Static | `backend/static/` | Built SPA | Build artifact, not hand-edited |

**Observed exceptions to the layering:** `controllers/entities.py` contains its own flattening and parent-gender validation; `controllers/ai_chat.py` composes a tool executor directly; `controllers/widgets.py` contains CRUD logic that could live in a service. The split is pragmatic, not dogmatic.

### 5.4 Frontend layering

```
main.tsx
└── QueryClientProvider (src/state/queryClient.ts)
    └── App (src/app/App.tsx)
        ├── ThemeProvider (storageKey="rumetrics-ui-theme")
        ├── BrowserRouter
        │   └── OnboardingGate (GET /system/onboard/status)
        │       └── AIAssistantProvider
        │           └── ModeProvider
        │               ├── AppRoutes (10 routes)
        │               └── Layout (nav, mode select, theme, AI button, sidecar)
        └── ErrorBoundary
```

---

## 6. Data Architecture: The EAV Model

### 6.1 Why EAV

A conventional schema would add a column per attribute. In livestock that schema is never stable: breeds, regulations, sensors, and management practices change. EAV trades join complexity for infinite extensibility:

```
entities (identity)
   │ 1
   │
   │ N
measurements (value + date) ── N │ 1 ── field_definitions (metadata)
```

### 6.2 Tables (from `backend/app/database/tables.py`)

| Table | Purpose | Key columns / constraints |
|---|---|---|
| `entities` | Animal/entity identity only | `id` PK, `custom_id` UNIQUE INDEX, `notes`, `created_at`, `updated_at` |
| `field_definitions` | Attribute metadata | `key` UNIQUE INDEX, `label`, `category` INDEX, `data_type`, `unit`, `options` JSON, `role`, `is_active`, `sort_order` |
| `measurements` | Values | `entity_id` FK CASCADE INDEX, `field_id` FK INDEX, `record_date` INDEX, `value_float`, `value_text`, `created_at`; **UNIQUE(entity_id, field_id, record_date)** |
| `computed_fields` | User formulas | `name` UNIQUE, `expression`, `category` |
| `validation_rules` | Declarative rules | `field_key`, `rule_type`, `rule_value`, `severity`, `custom_message` |
| `app_modes` | Workspaces | `name` UNIQUE, `icon`, `is_system`, `is_active` (soft delete) |
| `mode_field_links` | Mode ↔ Field M:N | Composite PK (`mode_id`, `field_id`), `sort_order` |
| `dashboard_widgets` | Server-driven widgets | `extension_id`, `widget_type`, `data_source`, `query_config` JSON, `visualization_config` JSON, `mode_id` SET NULL, `sort_order`, `is_active` |
| `installed_extensions` | Marketplace state | `extension_id` UNIQUE, `is_active`, `installed_at` |
| `app_settings` | Runtime config K/V | `key` PK, `value`, `value_type` (string/number/boolean/secret), `description`, `updated_at` |
| `change_audit_log` | AI change trail | `entity_id`, `field_key`, `old_value`, `new_value`, `changed_by`, `change_type`, `batch_token` |
| `ai_conversations` | Chat sessions | `title`, `message_count`, timestamps |
| `ai_messages` | Chat messages | `conversation_id` FK CASCADE INDEX, `role`, `content`, `tool_calls_json`, `tool_results_json` |

**Physical table #14:** `pending_changes` is created by raw SQL in `run_migrations()` (token, change_type, description, operations_json, mode, consumed, created_at, expires_at). It is **not** defined as a SQLModel table and is **not used anywhere** by the executors — see §19.

### 6.3 Value encoding

- `data_type == "number"` → `value_float` is set, `value_text` is NULL.
- Every other type (`text`, `select`, `boolean`, `multi-select`) → `value_text` is set.
- Exactly one row per `(entity, field, date)`; recording the same field twice on the same day is rejected by `uq_e_f_d`.
- Multi-select and select options are stored in `field_definitions.options` as JSON (`{"choices": [...]}`).

### 6.4 Flattening (EAV → wide)

The read path pivots rows into columns in two places:

**SQLModel path** — `controllers/entities.py::flatten_entity()`:

```python
select(FieldDefinition.key, Measurement.value_text, Measurement.value_float)
  .join(Measurement, Measurement.field_id == FieldDefinition.id)
  .where(Measurement.entity_id == entity.id)
```

**DuckDB path** — `services/analytics_engine.py`:

```sql
PIVOT (
  SELECT e.id, e.custom_id, fd.key,
         COALESCE(CAST(m.value_float AS VARCHAR), m.value_text) AS val
  FROM entities e
  JOIN measurements m ON e.id = m.entity_id
  JOIN field_definitions fd ON m.field_id = fd.id
)
ON key IN ('species','sex','weight', …)
USING first(val)
GROUP BY id, custom_id
```

This pivot becomes the view `flattened_view`, which is the only data source the AI is allowed to query.

> **Correctness caveat (verified in code):** neither flattening path filters to the latest `record_date` or orders by it. With time-series data (milk yield over months), the "flat" value is whichever row SQLite/DuckDB returns first — in practice often the first or last inserted, but not guaranteed. Any feature that treats the flattened value as "current" inherits this ambiguity. See §20.

### 6.5 Field roles

`role` is more than a label; it is the extension point that lets services find fields semantically:

- `primary` — always shown (e.g. species).
- `normal` — default.
- `system` — reserved.
- Domain roles consumed by name: `breeding_mating_date`, `breeding_pregnancy_status`, `breeding_gestation_period`, `breeding_expected_birth_date`, `breeding_birth_date`, `breeding_offspring_count`, `breeding_calving_ease`, plus `health_*` conventions.

`date_math_evaluator.resolve_fields_by_roles()` resolves a list of roles into `{role: FieldDefinition}` in one query, so services never hardcode field keys for extension-provided concepts.

### 6.6 Schema evolution

There is **no Alembic**, and `alembic` is imported nowhere (verified). `create_db_and_tables()` does `SQLModel.metadata.create_all(engine)` then `run_migrations()`, which performs nine inline, idempotent checks:

| # | Migration |
|---|---|
| 1 | Add `is_active` to `app_modes` |
| 2 | Add `is_active` to `installed_extensions` |
| 3 | Seed default AI settings if `app_settings` is empty |
| 4 | Add `sort_order` to `mode_field_links` |
| 5 | Create `change_audit_log` + 2 indexes |
| 6 | Create `pending_changes` + expiry index |
| 7 | Create `ai_conversations` / `ai_messages` as a safety net |
| 8 | Mark "General Husbandry" as a system mode |
| 9 | Ensure a system mode exists |

Each migration inspects `inspect(engine).get_columns(...)` before altering, so repeat boots are safe. The trade-off: migrations are hand-written SQL with no version table, no downgrade path, and no ordering guarantee beyond source order.

---

## 7. Backend Deep Dive

### 7.1 Application bootstrap (`app/main.py`)

- `FastAPI(title="MiData API", version="1.0.0", openapi_url="/api/openapi.json")`.
- Lifespan: logs a warning if `SECRET_KEY` is still the development fallback, then `create_db_and_tables()`.
- CORS: `allow_origins=["*"]`, `allow_credentials=True`, all methods/headers. **This combination is invalid per the CORS spec** (wildcard origin cannot be combined with credentials); browsers will reject credentialed requests. The app works because it does not actually use cookies.
- 18 `include_router()` calls (17 modules; `ai_chat.py` exports two routers).
- `GET /health` → `{"status": "healthy"}`.
- Static serving with PyInstaller awareness: `sys._MEIPASS/static` when frozen, otherwise `backend/static`. `/assets` is mounted; any other path returns `index.html` except prefixes `api`, `docs`, `openapi`, `redoc`.

### 7.2 Configuration (`app/config.py`)

```python
PROJECT_NAME = "MiData API"
VERSION = "1.0.0"
API_PREFIX = "/api"
DATABASE_URL = "sqlite:///./midata_data/livestock.db"   # informational; db.py is authoritative
SECRET_KEY = "your-secret-key-change-this-in-production"
ACCESS_TOKEN_EXPIRE_MINUTES = 60 * 24 * 8
API_KEY = "midata-dev-key"
```

`SettingsConfigDict(case_sensitive=True, env_file=".env", extra="ignore")`, with **no `env_prefix`**. Consequence: the comment claiming `MIDATA_API_KEY` overrides the key is misleading — pydantic-settings reads `API_KEY` unless an alias is declared.

### 7.3 Database engine (`app/database/db.py`)

- Path resolution: frozen → directory of the executable; dev → four parents up from `db.py` = repo root. Then `midata_data/livestock.db`.
- `create_engine(..., echo=MIDATA_SQL_ECHO, connect_args={"check_same_thread": False, "timeout": 30})`.
- Every connection runs `PRAGMA foreign_keys=ON` and `PRAGMA journal_mode=WAL`. The FK pragma is essential: without it the `ON DELETE CASCADE` on `measurements` is silently ignored.
- `get_session()` yields a `Session` per request.

### 7.4 Controllers (17 modules)

| Module | Prefix | Notes |
|---|---|---|
| `entities.py` | `/entities` | Flattened CRUD + birth trigger; parent gender checks |
| `fields.py` | `/fields` | CRUD, `grouped`, `key/{key}`, toggle active |
| `measurements.py` | `/measurements` | Batch, mass, per-animal, latest, list, delete |
| `analytics.py` | `/analytics` | Studio, deep dive, stats, correlation, exports, dashboard |
| `pedigree.py` | `/pedigree` | `GET /graph` delegating to service |
| `computed_fields.py` | `/computed-fields` | **Only router with API-key dependency** |
| `validation.py` | `/validation` | Rules list + entity evaluation |
| `modes.py` | `/modes` | CRUD, toggle, field links |
| `import_export.py` | `/data` | `POST /import` (note: **not** `/import`) |
| `extensions.py` | `/extensions` | List + install |
| `widgets.py` | `/widgets` | User widget CRUD (`extension_id="__user__"`) |
| `onboarding.py` | `/system/onboard` | status, bundles, execute |
| `system.py` | `/system` | Backup, verify, list, marketplace, templates |
| `ai_settings.py` | `/settings/ai` | GET/PUT config, connectivity test |
| `ai_widgets.py` | `/widgets/ai` | `POST /generate` |
| `ai_chat.py` | `/tools` and `/widgets/ai` | Query, commit tokens, SSE chat |
| `ai_conversations.py` | `/tools/conversations` | Conversation CRUD |

### 7.5 Services catalogue (20 + 15 tool executors)

| Service | Lines | Responsibility |
|---|---|---|
| `statistical_engine.py` | 1,720 | All pandas-based analytics: trend, distribution, correlation, comparison, deep dive, dashboard stats, profitability, dynamic dashboard |
| `onboarding_service.py` | 905 | Bundle merge, field/mode/widget creation, financial fields, mock data seeding |
| `ai_widget_service.py` | 763 | LLM prompt construction, SQL validation, widget blueprint, HTML sanitization |
| `sql_validator.py` | 590 | sqlparse AST SELECT-only validation + column whitelist |
| `ai_chat_service.py` | 462 | SSE orchestration, tool loop, confirmation events |
| `extension_service.py` | 381 | Manifest loading, atomic install/uninstall, gestation seeding |
| `pedigree_service.py` | 332 | Adjacency, bidirectional BFS, phantoms, truncation |
| `birth_service.py` | 265 | 7-step atomic parturition transaction |
| `context_governance.py` | 246 | Row/size caps, token budget, smart summarization |
| `template_service.py` | 241 | Export/import workspace configuration |
| `tool_executor.py` | 249 | Tool registry + result summary formatting |
| `import_service.py` | 197 | Row-by-row import with conflict strategy |
| `formula_service.py` | 183 | `{{key}}` formula parsing, simpleeval, per-animal computation |
| `mode_service.py` | 172 | Mode CRUD + field linking |
| `ai_settings.py` | 170 | Encrypted settings K/V (Fernet) |
| `analytics_engine.py` | 162 | DuckDB singleton, sqlite attach, PIVOT, sandbox |
| `date_math_evaluator.py` | 131 | `date_diff`/`date_add` sandbox + role resolution |
| `validation_service.py` | 103 | Date-rule evaluation + message templating |
| `stats_service.py` | ~60 | Field stats helpers |
| `export_service.py` | ~50 | DuckDB DataFrame → XLSX |

### 7.6 Error-handling conventions

- Domain errors are raised as `ValueError` subclasses (`BirthServiceError`) or `ValueError`, then mapped in controllers: `404` for "not found", `400/409` for conflicts and validation, `500` with `logger.exception` otherwise.
- Controllers never swallow exceptions silently; the Axios interceptor logs client-side.
- There is no global exception handler, so unexpected errors return FastAPI's default 500 body.
- `session.rollback()` is explicit only in the birth controller's catch-all; most services rely on request-scoped sessions being discarded.

---

## 8. API Reference

All routes below are registered in `app/main.py`. Base URL in development: `http://localhost:8000`.

### 8.1 Entities

| Method | Path | Purpose |
|---|---|---|
| GET | `/entities/` | List all entities, flattened |
| GET | `/entities/{entity_id}` | One flattened entity |
| POST | `/entities/` | Create entity + measurements from a flat payload |
| DELETE | `/entities/{entity_id}` | Delete entity and its measurements |
| POST | `/entities/action-trigger/birth` | Atomic birth event |

### 8.2 Fields

| Method | Path | Purpose |
|---|---|---|
| GET | `/fields/` | List field definitions (`include_inactive` query) |
| GET | `/fields/grouped` | Grouped by category |
| GET | `/fields/{field_id}` | One field |
| GET | `/fields/key/{field_key}` | Lookup by key |
| POST | `/fields/` | Create |
| PATCH | `/fields/{field_id}/toggle` | Activate/deactivate |
| DELETE | `/fields/{field_id}` | Delete |

### 8.3 Measurements

| Method | Path | Purpose |
|---|---|---|
| POST | `/measurements/batch` | Many measurements, one animal |
| POST | `/measurements/mass` | Same measurements, many animals |
| GET | `/measurements/{animal_id}` | History for one animal |
| GET | `/measurements/{animal_id}/latest` | Latest value per field |
| GET | `/measurements/` | List/filter |
| DELETE | `/measurements/{measurement_id}` | Delete one |

### 8.4 Analytics

| Method | Path | Purpose |
|---|---|---|
| POST | `/analytics/studio` | Custom study: trend/distribution/correlation/comparison |
| GET | `/analytics/deep-dive/{field_key}` | Single-variable report |
| POST | `/analytics/statistics` | Batch descriptive stats |
| GET | `/analytics/statistics/{field_key}` | One-field stats |
| GET | `/analytics/correlation?field_keys=a,b,c` | Pearson matrix |
| GET | `/analytics/export/excel` | Flattened XLSX |
| GET | `/analytics/export/summary` | Summary XLSX |
| GET | `/analytics/dashboard/stats?mode_id=` | KPI counts |
| GET | `/analytics/dashboard/profitability?mode_id=&milk_price=&feed_cost=&currency=` | Economics |
| GET | `/analytics/dashboard/dynamic?mode_id=` | Hydrated widgets |

### 8.5 Modes, validation, computed fields

| Method | Path | Purpose |
|---|---|---|
| GET/POST | `/modes/` | List/create modes |
| PATCH/DELETE | `/modes/{mode_id}` | Update/delete |
| PATCH | `/modes/{mode_id}/toggle` | Soft toggle |
| GET/PUT | `/modes/{mode_id}/fields` | Read/replace field links |
| GET | `/validation/rules` | List rules |
| GET | `/validation/evaluate/{entity_id}` | Alerts for one entity |
| GET/POST | `/computed-fields/` | List/create (API key required) |
| GET/PUT/DELETE | `/computed-fields/{id}` | Read/update/delete |
| GET | `/computed-fields/{id}/calculate` | Evaluate for all animals |

### 8.6 Data, extensions, widgets, system

| Method | Path | Purpose |
|---|---|---|
| POST | `/data/import` | Import rows (`skip`/`update`) |
| GET | `/extensions/` | List manifests + install state |
| POST | `/extensions/{extension_id}/install` | Atomic install |
| GET/POST | `/widgets/` | List/create user widgets |
| PUT/DELETE | `/widgets/{id}` | Update/delete |
| PATCH | `/widgets/{id}/toggle` | Activate/deactivate |
| GET | `/system/backup` | Download verified backup |
| GET | `/system/backup/verify` | Integrity check |
| GET | `/system/backup/list` | List backups |
| GET | `/system/marketplace` | Marketplace catalogue |
| GET | `/system/marketplace/template/{id}` | Template detail |
| GET | `/system/export-template` | Export setup JSON |
| POST | `/system/import-template` | Import setup JSON |
| DELETE | `/system/uninstall-template/{extension_id}` | Uninstall |

### 8.7 Onboarding & AI

| Method | Path | Purpose |
|---|---|---|
| GET | `/system/onboard/status` | `{onboarded, field_count, entity_count, mode_count}` |
| GET | `/system/onboard/bundles` | 6 bundle definitions |
| POST | `/system/onboard` | Idempotent merge setup |
| GET/PUT | `/settings/ai` | Read/update provider config |
| GET | `/settings/ai/test` | Connectivity check |
| POST | `/widgets/ai/generate` | Generate widget from prompt |
| POST | `/widgets/ai/chat` | SSE chat stream |
| POST | `/tools/query` | Direct read-only query |
| POST | `/tools/modify/commit/{token}` | Commit staged modification |
| POST | `/tools/batch/commit/{token}` | Commit staged batch |
| GET/POST | `/tools/conversations` | List/create conversations |
| PUT/DELETE | `/tools/conversations/{conv_id}` | Rename/delete |
| GET | `/tools/conversations/{conv_id}/messages` | History |
| GET | `/health` | Liveness |

---

## 9. Analytics Subsystem

### 9.1 Why two engines

| | DuckDB path | SQLModel/pandas path |
|---|---|---|
| Entry | `AnalyticsEngine` | `statistical_engine.py`, `stats_service.py` |
| Speed | Fast pivots and cross-filtered SQL | Adequate for test-sized data |
| Dependency | Needs the DuckDB `sqlite` extension | Pure ORM, works on in-memory SQLite |
| Used by | AI queries, widgets, pedigree, Excel export | `/analytics/studio`, `/statistics`, `/deep-dive`, tests |
| Persistence | `:memory:` only | N/A |

The split is deliberate: tests run against in-memory SQLite where the DuckDB extension may be unavailable, while production benefits from columnar pivoting.

### 9.2 DuckDB engine details

- One cached singleton (`_engine_cache`) guarded by a lock (double-checked locking).
- On construction: `INSTALL sqlite; LOAD sqlite; CALL sqlite_attach('<abs path>')`.
- Hardening: `enable_external_access=false`, `autoinstall_known_extensions=false`, `autoload_known_extensions=false`.
- Connection is **not** shared across threads for queries: every call creates `conn.cursor()`. Temporary views, however, are connection-scoped, so `create_flattened_view()` deliberately uses `self.conn` directly.
- `_validate_select_only()` is a fast pre-check (single statement, starts with SELECT/WITH). The authoritative check is the AST validator in `sql_validator.py`.
- Field keys are validated against `'` and `;` before being interpolated into the PIVOT.

### 9.3 Statistical engine

`perform_advanced_analysis(session, request)` dispatches on `analysis_type`:

- **trend** — group by `record_date`, mean y, linear regression (`slope`, `intercept`, `r²`), trend insights.
- **distribution** — histogram bins, single-value edge case, distribution insights.
- **correlation** — paired x/y data, Pearson `r` per group, correlation strength insights.
- **comparison** — group stats (count/mean/median/std/min/max) plus comparison insights.
- **categorical fields** — `select`/`text`/`boolean`/`multi-select` are routed to count-based distributions instead of numeric math.

`perform_deep_analysis(session, field_key)` returns: overview, histogram, time trend, by-breed, by-sex, top-5 low/high outliers, and textual insights. Empty datasets return a structured empty response rather than raising.

`compute_dashboard_stats` returns `totalEntities`, `recentActivities`, `alertsCount`, and per-category aggregations scoped to the mode's fields.

`compute_profitability` resolves a production field (explicit `target_field_key` or the first production-like numeric field), applies `milk_price` and `feed_cost`, and returns economic aggregates plus best/worst performers. Defaults are `1.5`, `0.5`, currency `USD` at the API layer — note the mismatch with the product-wide TND default.

`compute_dynamic_dashboard` is the server-driven UI core: reads `dashboard_widgets` for a mode, executes each widget's `query_config` through DuckDB, and returns hydrated data for the frontend renderer.

### 9.4 Insight generation

Insights are **deterministic text templates**, not LLM output. Functions such as `_generate_trend_insights`, `_generate_distribution_insights`, and `_generate_correlation_insights` inspect statistics (direction, R², spread, r strength) and emit sentences. This keeps analytics offline and reproducible; the LLM is not in the analytics path.

### 9.5 Pedigree algorithm

- Input: the flattened DataFrame (`custom_id`, `id`, `sire_id`, `dam_id`, `sex`, `species`, `breed`).
- Build `tag_to_id`, `id_to_row`, `parent_map`, `child_map`.
- With `root_id`: bidirectional BFS up (parents) and down (children), depth-bounded by `generations` (1–10, default 3), visited-set cycle protection.
- Without `root_id`: whole-herd graph, truncated to `MAX_GLOBAL_NODES = 500`.
- Phantom nodes represent parents referenced by `custom_id` but absent from the herd.
- Gender inconsistencies produce warnings, not hard failures.
- Output: `{nodes, edges, warnings, truncated, total_nodes, total_edges}`.

The frontend renders this with React Flow + dagre (`graphLayout.ts`), with phantom nodes styled distinctly.

---

## 10. AI Assistant Subsystem

### 10.1 Transport: SSE over POST

`POST /widgets/ai/chat` accepts:

```json
{
  "messages": [{"role": "user", "content": "..."}],
  "conversation_id": 12
}
```

The client keeps the last **20** user/assistant messages, sends an `AbortController` signal, and parses SSE frames split on `\n\n`. Event types implemented in `ai_chat_service.py` and `messageUtils.ts`:

| Event | Payload |
|---|---|
| `chunk` | `{"text": "..."}` |
| `tool_call` | `{"id", "name", "arguments"}` |
| `tool_status` | `{"id", "status": "executing\|completed\|error", "error?"}` |
| `tool_result` | `{"name", "summary", "data", "previewRows"}` |
| `confirmation` | `{"token", "description", "previewData"}` |
| `waiting_confirmation` | instruction to stop the tool loop |
| `done` | `{}` |
| `error` | `{"error": "..."}` |

The UI renders expandable per-tool blocks with status badges, a markdown chat stream (`react-markdown` + `remark-gfm`), and a confirmation card for staged writes.

### 10.2 Tool registry (15 families)

Every executor class may declare a `tool_definition` class attribute; `build_tool_definitions()` converts the registry into OpenAI function-calling schemas.

| Tool | Capabilities |
|---|---|
| `query` | `type: "sql"` (SELECT-only against `flattened_view`) or `type: "dsl"` (`select`, `from`, `where`, `order_by`, `aggregate`, limit) |
| `modify` | Preview + token for single-entity measurement updates |
| `batch` | Preview + token for `update`, `create_entity`, `delete_entity`; `transactional` or `per_item` mode; supports referencing a `custom_id` created earlier in the same batch |
| `ui` | Frontend actions: navigate, toast, open modal, filter, set theme, create widget (`__frontend__` marker passes through untouched) |
| `widget` | create/list/get/update/delete/duplicate/arrange |
| `analyze` | weight distribution, growth trends, herd composition, mortality, weight gain rate, production trend, health incidents, age distribution, fertility |
| `field` | list/create/update/delete/stats/bulk/suggest/export/import |
| `mode` | list/create/update/delete/activate/clone/set_fields/suggest_fields/reorder |
| `system` | get/update settings, DB info |
| `extension` | list/install/uninstall |
| `import_export` | trigger export, check import status |
| `computed_field` | list/create/update/delete/test |
| `validation` | list/create/update/delete/evaluate |
| `pedigree` | ancestry tree, descendants |
| `onboarding` | status, bundles |

`ToolExecutor.execute()` normalises every result to `{summary, data, previewRows, confirmation}`, with per-tool summary strings.

### 10.3 Security architecture (three layers)

1. **AST validation** (`sql_validator.py`, 590 lines): parses with `sqlparse`, allows exactly one statement, only `SELECT`/`WITH`, rejects mid-query semicolons, rejects DDL/DML keywords. This replaced a regex approach.
2. **Column whitelist:** `validate_query_columns(sql, valid_columns)` extracts identifier/CASE/function references and rejects unknown columns. `id` and `custom_id` are always allowed; the rest come from active `field_definitions` plus known analytics columns.
3. **DuckDB sandbox:** `enable_external_access=false`, extension auto-install disabled, sqlite attach limited to the app DB.

Additional controls:

- 60 s per-tool timeout.
- 5,000-row absolute cap, 50 KB result cap, 30 raw rows before summarization, 500 aggregated rows, 200-char strings.
- 32,000-token conversation budget with a warning at 24,000.
- HTML sanitization (`nh3`) with an explicit tag/attribute allowlist for AI-generated widgets.
- Read/write separation: writes are never executed during the chat turn; they are staged and require the user to click confirm, which calls a separate commit endpoint with a one-time token.

### 10.4 Confirmation tokens

Both `modify` and `batch` generate `secrets.token_urlsafe(32)` and store the pending plan in a **module-level in-memory dict** (`_preview_tokens`). Commit verifies existence, expiry (5 minutes), and single use, then deletes the token.

Consequences (verified):

- Tokens die on process restart.
- Tokens are not shared across multiple Uvicorn workers/processes.
- The `pending_changes` table created by migration #6 is never read or written; the intended persistence was not implemented.

### 10.5 Audit trail

`change_audit_log` records `entity_id`, `field_key`, `old_value`, `new_value`, `changed_by="ai_assistant"`, `change_type`, and `batch_token`. This is the accountability mechanism for AI writes; it is written by the modify/batch executors.

### 10.6 Provider configuration

Defaults seeded into `app_settings`:

| Key | Default | Type |
|---|---|---|
| `ai_base_url` | `http://localhost:1234/v1` | string |
| `ai_api_key` | `""` | secret |
| `ai_model_name` | `local-model` | string |
| `ai_temperature` | `0.3` | number |

Secrets are encrypted at rest with Fernet. The key is derived deterministically: `base64.urlsafe_b64encode(sha256(SECRET_KEY))`. Rotating `SECRET_KEY` makes existing secrets undecryptable; the code catches that and returns `""` rather than crashing (logged as an error). The API masks secrets as `****`.

### 10.7 AI widget generation

`POST /widgets/ai/generate` builds a system prompt from the live field definitions and modes, asks the LLM for a widget blueprint (type + SQL + presentation), validates the SQL and its columns, executes it through DuckDB (5,000-row cap, 360 s), computes aggregations if requested, sanitizes any HTML, and returns a schema the frontend can render. Supported template types: `kpi_card`, `bar_chart`, `line_chart`, `pie_chart`, `table`, `progress_list`, `alert_board`.

### 10.8 Prompt note

The system prompt is English but closes with *"Format your responses in clear Arabic."* This is a deliberate product decision for the target market, and it means the assistant is effectively Arabic-first even though the UI is English.

---

## 11. Domain Workflows

### 11.1 Onboarding

`POST /system/onboard` is **idempotent and merge-only**: existing fields, modes, entities, and widgets are never duplicated; new bundles are appended with get-or-create semantics.

Request contract (`models/onboarding.py`):

```
locale, currency (TND|USD|EUR, default TND),
financial_tracking, financial_purchase, financial_vet, financial_feed,
species (default [Cow, Sheep, Goat]), custom_species,
bundles: [{bundle_id, fields: [{key, included, label?, data_type?, unit?, options?}]}],
health_diagnostics, health_vaccinations, health_quarantine,
seed_mock_data
```

Six bundles (`app/data/bundle_catalogue.py`): `basic_identity`, `growth_body`, `dairy_production`, `health_vet`, `reproduction`, `nutrition_feed`.

Execution pipeline in `OnboardingService.execute()`:

1. Merge species.
2. Create/merge fields from bundles (respecting overrides).
3. Create modes and link fields.
4. Create dashboard widgets (global + per-mode).
5. Optionally create financial tracking fields.
6. Optionally seed mock entities/measurements with realistic distributions (weight, milk yield, temperature, BCS, lactation curves, ADG series, health chains).
7. Ensure the system mode exists.
8. Persist the configuration to `~/.midata/onboarding_config.json`.
9. Record installed bundles in `installed_extensions`.

The frontend wizard currently has **5 steps** (Welcome → Livestock → Fields → Health → Ready), although ADR-004 targets a 1-step flow.

### 11.2 Extensions

`ExtensionService` reads manifests from `backend/extensions/<id>/manifest.json`.

Bundled manifests:

| ID | Fields | Modes | Widgets | Description |
|---|---|---|---|---|
| `midata_livestock` | 9 | 2 | 0 | Core pedigree/milk/weight fields for Sheep, Goats, Cows |
| `midata_health_reproduction` | 14 | 2 | 5 | Treatments, withdrawal, pregnancy, EPD, birth events |
| `midata_analytics_basic` | 0 | 0 | 2 | Baseline analytics widgets |

Install is atomic (single commit or rollback):

1. **Fields:** if the key exists with a different `data_type` → raise (no silent corruption); if inactive → reactivate and refresh label/unit/options; if active → skip.
2. **Modes:** create or reactivate; link fields with sort order.
3. **Widgets:** create or reactivate.
4. **Gestation seeding:** writes species-specific gestation periods for breeding fields.
5. Record installation in `installed_extensions`.

Uninstall is a soft deactivation, not a destructive delete.

### 11.3 Birth trigger

`POST /entities/action-trigger/birth` delegates to `BirthService.trigger()`, a single transaction with seven documented steps:

1. Validate dam (and optional sire) exist.
2. Verify gender via the resolved sex field (`Male` / `Female`).
3. Record parturition measurements on the dam (role-resolved, falling back to keys).
4. Set pregnancy status to `Open`.
5. Create each offspring entity (duplicate `custom_id` aborts).
6. Write identity measurements and pedigree links (`sire_id`, `dam_id`).
7. One `commit()` — so DuckDB sees the new pedigree immediately.

`BirthServiceError` maps to 404 ("not found") or 400; any other exception rolls back and returns 500.

### 11.4 Computed fields

Formulas use `{{field_key}}` placeholders, e.g. `{{milk_yield_total}} / {{weight}}`. `FormulaService` extracts keys, fetches values in bulk, evaluates with `simpleeval` (no builtins), and returns `{animal_id: value}`. Division-by-zero, syntax, and unknown-name errors are mapped to farmer-friendly messages by the controller. Validation is exposed both at save time (`validate_formula`) and via `/computed-fields/{id}/calculate`.

### 11.5 Validation rules

`ValidationRule` supports `rule_type` values documented as `min`, `max`, `sex_required`, `species_required`. The **service only evaluates** `date_expiry_warning` and `date_math_formula`, using the `DateMathEvaluator` sandbox:

```
date_diff({{treatment_date}}, current_date) < {{withdrawal_days}}
```

Placeholders are substituted, `current_date` is injected, only `date_diff` and `date_add` are exposed, and any failure returns `False`. Triggered rules become alerts with severity and a templated message.

### 11.6 Import / export

- **Import** (`POST /data/import`): rows of `{data: {field_key: value}}`, conflict strategy `skip` or `update`. Entity-level keys (`custom_id`, `notes`, `record_date`, `id`, timestamps) are excluded from measurement creation. The whole batch is wrapped in try/commit/rollback with per-row errors collected.
- **Export** uses DuckDB's flattened DataFrame, optional species/breed filters, and writes `midata_data/exports/midata_export_<timestamp>.xlsx` with `pandas.to_excel` (openpyxl).
- **Backup** (`/system/backup`): WAL checkpoint, integrity verification, timestamped `.db` copy in `midata_data/backups/`, rotation keeping the newest 10.

### 11.7 Templates

`TemplateService` exports/imports **configuration only** — field definitions, modes, mode-field links, and widgets as JSON. It never exports entities or measurements. This is how a farm shares a workspace design without sharing data.

---

## 12. Frontend Architecture

### 12.1 Boot chain (code-verified)

1. `main.tsx` creates the React root, wraps in `StrictMode` and `QueryClientProvider`.
2. `App.tsx` wraps in `ThemeProvider` with `defaultTheme="light"` and `storageKey="rumetrics-ui-theme"`.
3. `BrowserRouter` → `OnboardingGate`.
4. `OnboardingGate` calls `GET /system/onboard/status` once, stores `midata_onboarded`, and shows a welcome dialog (dismissable via `midata_welcome_dismissed`) offering the wizard or a blank slate.
5. `MainApp` omits the Layout on `/onboarding`; otherwise `AIAssistantProvider` → `ModeProvider` → `Layout` + `AppRoutes`.
6. `Layout` renders a sticky 64 px header with logo, 7 nav items + Settings, a mode `<Select>`, theme toggle, AI button, and the AI sidecar (420 px).

### 12.2 Routes (10)

| Path | Component |
|---|---|
| `/onboarding` | `OnboardingWizard` (outside Layout) |
| `/` | `HomePage` (dashboard) |
| `/animals` | `AnimalsList` |
| `/animals/:id` | `AnimalDetailPage` |
| `/entry` | `BatchEntryForm` |
| `/analytics` | `AnalysisStudio` |
| `/deep-analysis` | `DeepAnalysisPage` |
| `/genetics` | `PedigreePage` |
| `/marketplace` | `MarketplacePage` |
| `/settings` | `SettingsPage` |
| `*` | redirect `/` |

### 12.3 State management

- **Server state:** TanStack Query. `staleTime` 30 s, `gcTime` 5 min, refetch on focus and mount.
- **Query keys:** centralised in `state/queryKeys.ts` (`animals`, `animalDetail`, `dashboardStats`, `dynamicWidgets`, `fields`, `modes`, `pedigree`, `stats`, `deepDive`, `computedFields`, `animalMeasurements`).
- **UI state:** React context + reducers.
  - `ThemeContext` → `light|dark` class on `<html>`, persisted.
  - `ModeContext` → modes, current mode, all/active fields, loading; persists `midata_active_mode_id`, migration flag `midata_mode_version`; system modes bypass field filtering.
  - `AIAssistantContext` → reducer over messages, tool calls/results, streaming, pinned sidecar, conversations, pending modifications, UI actions.
- **Hooks:** `useAnimals`, `useChatStream`, `useContextMenu`, `useConversations`, `usePersistedState`, `useToolExecution`.
- **Persisted keys (verified):** `rumetrics-ui-theme`, `midata_active_mode_id`, `midata_mode_version`, `midata-ai-sidecar-pinned`, `midata_welcome_dismissed`, `midata_onboarded`.

> **Inconsistency:** `HomePage` and `ModeContext` sometimes use literal keys (`["dynamicWidgets", id]`) instead of the `queryKeys` factory. Invalidation in `refetchModes` uses the factory, which still matches the prefix, so it currently works — but the single-source-of-truth rule is not enforced.

### 12.4 Transport and API layer

`transport/client.ts`:

```ts
baseURL = VITE_API_URL || (DEV ? "http://localhost:8000" : "")
timeout = 10_000
X-API-Key = VITE_API_KEY || "midata-dev-key"
```

Response interceptor logs 401/404/5xx and rethrows; there is no automatic retry, refresh, or toast.

`api/` wrappers: `analytics.ts`, `animals.ts`, `dashboard.ts`, `entities.ts`, `measurements.ts`, `modes.ts`, `onboarding.ts`, `settings.ts`. Components are expected to call these, never Axios directly — although `HomePage` does import `apiClient` directly.

### 12.5 UI slices

| Slice | Notable components (lines) |
|---|---|
| `animals/` | `AnimalsList` (636, AG Grid + mobile list), `AnimalDetailPage` (685), `CreateAnimalModal` (456), `AnimalContextMenu`, `AnimalsFilterBar` (230), `MobileAnimalsList`, `PrintCardsModal` (293), `animalsGridConfig` |
| `entry/` | `BatchEntryForm`, `MassEntryModal`, `QuickEntryModal`, `ImportWizard` (673), `DynamicField` (230), `EntryTabs` |
| `analytics/` | `AnalysisStudio` (683), `DeepAnalysisPage` (397), `PedigreePage` (711), `AnalyticsDashboard`, `DistributionChart`, `StatCard` |
| `dashboard/` | `HomePage` (362), `WidgetGrid`, `WidgetConstructor`, `AIWidgetConstructor` (829), `DynamicWidgetRenderer` (602), plus widgets: `AiKpiCard`, `AiBarChart`, `AiLineChart`, `AiPieChart`, `AiTable`, `AiProgressList`, `AiAlertBoard`, `AiLegacyFallback`, `CategorySection`, `TopPerformersList`, `CullingCandidatesList`, `ProfitabilityCalculator` |
| `assistant/` | `AIAssistantSidecar` (217), `ChatStream` (897), `ConversationList` (214), `InputBar`, `MessageBubble`, `ConfirmationCard`, `WidgetPreview`, `AssistantMarkdown`, `SidecarHeader`, `SidecarFooter`, `AIAssistantButton` |
| `settings/` | `SettingsPage` (5 tabs), `FieldManager`, `AddFieldModal`, `ComputedFieldsList`, `FormulaBuilderModal` (262), `ModeManager` (246), `SetupSharing` (490), `AIProviderSettings` (325) |
| `onboarding/` | `OnboardingWizard` (566), 5 step components, `BundleCustomizeModal`, `SpeciesIcon` (251), `StepCard`, `SummaryRow`, `Toggle` |
| `marketplace/` | `MarketplacePage` (626) |
| `shared/ui/` | `Button`, `Card`, `Input`, `Select`, `Textarea`, `Label`, `Modal`, `ConfirmDialog`, `ContextMenu`, `FocusTrap`, `DataGrid`, `AgGridWrapper`, `ErrorBoundary`, `Skeleton`, `tabs`, `SearchableSelect`, `QuickChoiceRow`, `cellRenderers` |

**Notable UX patterns in code:**

- AG Grid with a custom `themeQuartz`-based theme, 20 rows/page, 48 px rows, responsive mobile list fallback.
- Keyboard-accessible modals via `FocusTrap`, ESC handling, and ARIA labels.
- Optimistic loading skeletons and distinct empty states.
- Sidecar shifts the header and content width by `calc(100% - 420px)` rather than overlaying.

---

## 13. Design System

Defined in `docs/DESIGN.md` and implemented in `src/index.css` + shared components.

### 13.1 Palette (CSS variables, RGB triplets)

| Token | Light | Dark |
|---|---|---|
| `--background` | `245 240 234` (warm sand) | `38 29 20` |
| `--foreground` | `69 52 39` | `224 213 200` |
| `--card` | `255 255 255` | `26 20 16` |
| `--primary` | `117 144 72` (olive-500) | `138 172 94` |
| `--muted` | `232 223 211` | `42 33 26` |
| `--accent` | `244 247 237` | `58 71 38` |
| `--destructive` | `184 69 58` | `216 90 78` |
| `--border` | `232 223 211` | `42 33 26` |
| `--ring` | `174 196 134` | `138 172 94` |

Full earthy scale `earth-50..950` (`#f5f0ea` → `#261d14`) and olive scale `olive-50..950` (`#f4f7ed` → `#19210e`) are exposed as Tailwind utilities through `@theme`. Semantic tokens: success `#759048`, warning `#c48a3a`, destructive `#b8453a`.

### 13.2 Shape, type, motion

- Radii: card XL 1.75 rem (28 px), LG 1.5 rem (24 px), MD 1.25 rem, SM 1 rem, button 0.75 rem, input 0.875 rem.
- Typography: Inter; h1 `text-3xl` bold, h2 `text-xl` semibold, body `text-sm`; tabular numerals for data.
- Motion: 150–300 ms, `cubic-bezier(0.4, 0, 0.2, 1)`.
- Layout: 4-column bento grid with `col-span` 1–4; responsive 1/2/3/4 columns at 640/1024/1280 breakpoints; max width 1600 px on dashboard.
- Icons: Lucide, 18–20 px, stroke 1.5.
- Shadows: warm brown tint `rgba(101, 76, 54, 0.08)` instead of neutral grey.

### 13.3 Data visualisation

- AG Grid over HTML tables; charts via Recharts with the olive/earth palette.
- Pedigree via React Flow + dagre.
- Charts are memoised and sized to their bento card.

---

## 14. Desktop Application & Packaging

### 14.1 Launcher (`backend/launcher.py`, 523 lines)

Classes:

| Class | Responsibility |
|---|---|
| `ConfigManager` | Read/write `~/.midata/midata_config.json` (ngrok token) |
| `UvicornManager` | Probe ports 8000–8009 with `socket.bind`, run Uvicorn in a daemon thread bound to `0.0.0.0`, graceful `should_exit` shutdown |
| `NgrokManager` | Thread-safe tunnel start/stop with `pyngrok`, public URL property |
| `ConsoleLogWindow` | `CTkToplevel` log viewer |
| `NetworkSettingsWindow` | Token entry, Local Wi-Fi / Internet tabs, QR codes for both URLs |
| `MiDataTrayApp` | Tray icon + menu, hidden CTk root, lifecycle orchestration |

Threading model: the tray icon runs in a daemon thread; a **hidden** CustomTkinter root owns the main thread's event loop; all UI callbacks are marshalled with `root.after(0, ...)`. Shutdown order: ngrok → Uvicorn → tray → Tk.

Menu: Open MiData (default), Network Settings, Public Link (checked/enabled state), Show Console Log, Exit.

### 14.2 PyInstaller (`MiData.spec`)

- Entry: `launcher.py`.
- Datas: `app/` (source), `static/` (SPA build), `icon.png`, plus `customtkinter` data files.
- Hidden imports: Uvicorn loops/protocols/lifespan, FastAPI, SQLModel, `app.main`, `sqlite3`, PIL, qrcode, pystray.
- Excludes: pytest, tkinter tests, unittest, `test`.
- `console=False`, UPX enabled, **one-folder** output via `COLLECT(... name="MiData_dist")`.
- Frozen path handling: `sys._MEIPASS` for resources and static; `sys.executable` parent for the data directory.

### 14.3 Practical packaging gap

`backend/static/` is tracked and stale (its `index.html` still says `<title>Rumetrics</title>` and references `index-BWjhIxFK.js`). `pnpm run build` writes to `frontend/dist/`, which does **not exist** in the working tree and which nothing copies into `backend/static/`. Any desktop build from the current tree ships the old UI.

---

## 15. Configuration, Secrets, and Environment

| Setting | Where | Default | Notes |
|---|---|---|---|
| `SECRET_KEY` | `config.py` / `.env` | dev fallback | Drives Fernet key; warned at startup |
| `API_KEY` | `config.py` / `.env` | `midata-dev-key` | Only `/computed-fields` validates it |
| `DATABASE_URL` | `config.py` | `sqlite:///./midata_data/livestock.db` | Informational; `db.py` computes the real path |
| `MIDATA_SQL_ECHO` | environment | unset | Enables SQLAlchemy echo |
| `VITE_API_URL` | frontend build | dev `http://localhost:8000`; prod same-origin | |
| `VITE_API_KEY` | frontend build | `midata-dev-key` | Sent as `X-API-Key` |
| AI base URL / key / model / temperature | `app_settings` | LM Studio defaults | Key encrypted with Fernet |
| ngrok token | `~/.midata/midata_config.json` | empty | Required for public link |
| onboarding config | `~/.midata/onboarding_config.json` | created by wizard | Full request echoed as JSON |

**Secrets policy in practice:** `.env`, `*.db`, `midata_data/`, `midata_config.json`, `rumetrics_config.json`, and `api.secret.key` are gitignored. `backend/rumetrics_config.json` exists locally and must not be committed or printed.

---

## 16. Security Model

### 16.1 Trust boundaries

```
[User's browser] ──LAN/ngrok──▶ [FastAPI] ──SQL──▶ [SQLite/DuckDB]
                                      │
                                      └──HTTP──▶ [Local LLM]
```

The API is **unauthenticated by default** (except `/computed-fields`). Anyone who can reach the port can read and write data. This is acceptable for loopback use and is explicitly extended to LAN/Internet by the launcher, which is a deliberate (and risky) product decision.

### 16.2 Controls present

| Control | Implementation |
|---|---|
| SQL injection (AI) | `sqlparse` AST + single SELECT + column whitelist + DuckDB external access off |
| Formula injection | `simpleeval` with no builtins, only resolved fields |
| Date-rule injection | `simpleeval` with only `date_diff`/`date_add`, no builtins/filesystem/shell |
| HTML/XSS (AI widgets) | `nh3` sanitization with tag/attribute allowlist |
| Secret storage | Fernet (AES-128-CBC + HMAC) derived from `SECRET_KEY`; masked in API |
| File scope | Backups and exports confined to `midata_data/`; SQLite path derived from code |
| Destructive writes | Staged preview + one-time token + user confirmation |
| Audit | `change_audit_log` for AI-driven changes |
| CORS | Wide open (`*`) by design for dev/LAN |

### 16.3 Weaknesses / gaps

- **No authentication or authorization** on data routes; API key is only a shared constant for one router and is embedded in the frontend bundle.
- **CORS wildcard + credentials** is an invalid combination and should be either `*` without credentials or an explicit origin list.
- **In-memory confirmation tokens** are lost on restart and unsafe under multiple workers.
- **Default `SECRET_KEY`** is a publicly known string; production relies on the operator setting it.
- **No rate limiting**, no request size limits at the proxy layer, and no CSRF protection.
- **`X-API-Key` in the bundle** means it is not a secret at all.
- **Dependency risk:** `xlsx@0.18.5` (SheetJS) is known-problematic and flagged in `.github/ISSUES/003`; Axios advisories are tracked in `004`.

---

## 17. Testing & Quality

### 17.1 Measured status (this run)

| Gate | Command | Result |
|---|---|---|
| Backend tests | `.venv/bin/python -m pytest tests/ -q` | **485 passed, 2 skipped**, 14 warnings, 11.4 s |
| Frontend tests | `pnpm run test` | **76 passed** in 3 files, 2.96 s |
| Frontend lint | `pnpm run lint` | **FAIL** — 25 errors, 33 warnings |
| Frontend build | `pnpm run build` | Reported green historically; not re-run here |
| CI | — | none |

### 17.2 Backend test coverage map

| Module | Tests | Focus |
|---|---|---|
| `test_tool_query.py` | 71 | SQL/DSL validation, limits, security |
| `test_local_sharing.py` | 37 | LAN/ngrok/launcher behavior |
| `test_soft_delete.py` | 36 | `is_active` semantics |
| `test_health_reproduction.py` | 34 | Extension fields, withdrawal, pregnancy |
| `test_extensions.py` | 33 | Install atomicity, type conflicts, rollback |
| `test_analytics.py` | 30 | Studio, stats, dashboard |
| `test_pedigree.py` | 26 | BFS, depth, truncation, cycles |
| `test_onboarding.py` | 25 | Wizard flow, bundles |
| `test_ai_chat_service.py` | 24 | Streaming, events |
| `test_import.py` | 24 | Conflict strategies, validation |
| `test_tool_analyze.py` | 21 | Analysis tool functions |
| `test_ai_settings.py` | 20 | Encryption, CRUD, connectivity |
| `test_ai_conversations.py` | 19 | Conversation persistence |
| `test_formula_service.py` | 18 | Formula parsing/evaluation |
| `test_launcher.py` | 17 | Ports, config, lifecycle |
| `test_settings.py` | 16 | Modes/service settings |
| `test_tool_widget.py` | 12 | Widget tool |
| `test_database.py` | 8 | Engine, migrations |

Test infrastructure: `conftest.py` provides a session-scoped in-memory SQLite engine and per-test transactional rollback, so no external services or files are required. `pytest-cov` is installed but no threshold/report is configured.

### 17.3 Frontend test coverage

Only 3 files: `graphLayout.test.ts`, `ModeContext.test.tsx`, `cellRenderers.test.tsx`. That is **layout math, one context, and cell renderers** — the analytics pages, AI chat stream, import wizard, forms, and API layer have no unit tests, and there are **no E2E tests**.

### 17.4 Lint debt

25 errors, dominated by:

- `react-hooks/set-state-in-effect` (~13)
- `react-refresh/only-export-components` (6)
- `react-hooks/refs` (2)
- `react-hooks/preserve-manual-memoization` (2)
- `react-hooks/immutability` (1), `react-hooks/purity` (1)
- 33 `react-hooks/exhaustive-deps` warnings

### 17.5 Quality observations

- Comments and docstrings are unusually strong at the service layer (each module explains intent, invariants, and security rationale).
- Error messages are mostly actionable, though several controllers expose raw exception strings to the client.
- There is no structured logging configuration; Uvicorn/FastAPI defaults apply.
- No coverage threshold, no mutation testing, no contract tests between frontend and backend.

---

## 18. Build, Run, and Deploy

### 18.1 Backend (development)

```bash
cd backend
uv venv
uv pip install -r requirements.txt -r requirements-dev.txt
.venv/bin/python -m uvicorn app.main:app --reload --host 127.0.0.1 --port 8000
```

> `uv run python -m pytest` does **not** work as documented because there is no `pyproject.toml`/`uv.lock`; use the venv interpreter.

Optional seed data:

```bash
.venv/bin/python scripts/seed.py          # idempotent
.venv/bin/python scripts/seed.py --force  # rebuild
.venv/bin/python scripts/seed.py --verify # quality checks
```

Seeds: 32 field definitions, 5 modes, 50+ entities across Cow/Sheep/Goat, 2,000+ measurements (lactation curves, ADG, health chains), a 5-generation pedigree, 12 validation rules, 3 computed fields, 6 widgets.

### 18.2 Frontend (development)

```bash
cd frontend
pnpm install
pnpm run dev     # http://localhost:5173, calls API at :8000 directly (no proxy)
pnpm run build   # tsc -b && vite build → frontend/dist/
pnpm run test
pnpm run lint
```

### 18.3 Production bundle

```bash
cd frontend && pnpm run build
cp -r frontend/dist/* ../backend/static/   # manual step — nothing automates this
cd ../backend
pyinstaller MiData.spec                    # → MiData_dist/
```

`MiData_dist/MiData` starts the tray app, binds the first free port in 8000–8009, opens the browser, and writes data next to the executable in `midata_data/`.

### 18.4 Full validation (no CI)

```bash
cd backend && .venv/bin/python -m pytest tests/ -q \
  && cd ../frontend && pnpm run lint && pnpm run build && pnpm run test
```

Expect lint to fail today.

---

## 19. Code-Level Findings: Docs Drift and Bugs

These were discovered by reading the code, not the docs.

### 19.1 Confirmed bugs

1. **`generate_summary_report()` passes the wrong positional argument.**
   `export_service.py` defines `generate_excel_dump(session, species, breed, output_path)` and then calls `generate_excel_dump(session, output_path)` — so a custom `output_path` becomes the `species` filter. The `/analytics/export/summary` endpoint will silently filter by a path string (returning empty) instead of writing a summary file.

2. **Progressive disclosure is documented but absent.**
   `Layout.tsx` header says *"Core workflow nav items are always visible. Advanced configuration … only appears when Developer Mode is ON."* In reality: no `developer_mode` / `DeveloperMode` / `useDeveloperMode` identifier exists anywhere in `src/` (verified: 0 hits), the nav has 7 items + Settings, and the mode `<Select>` renders unconditionally. Git history shows `DeveloperModeContext.tsx` was added in `7f490e6` and **deleted** in `0c06a65` (the design-system commit), while the ADR and commit messages still claim it exists.

3. **`pending_changes` is dead schema.**
   Migration #6 creates it "for persistent preview tokens", but `modify_executor.py` and `batch_executor.py` use module-level `_preview_tokens` dicts. Tokens therefore vanish on restart and are per-process.

4. **`alembic==1.13.1` is a phantom dependency.**
   It is pinned in `requirements.txt` but imported nowhere; the project intentionally has no Alembic.

5. **Stale committed SPA.**
   `backend/static/index.html` says `<title>Rumetrics</title>` and points at `index-BWjhIxFK.js`; `frontend/dist/` does not exist. The desktop build would ship an old, differently branded UI.

6. **Theme storage key mismatch.**
   `App.tsx` passes `storageKey="rumetrics-ui-theme"`, while `ThemeContext` defaults to `"midata-ui-theme"` and the AGENTS/design docs reference the latter. Users' theme preference is stored under the legacy name.

7. **Validation rule types are only half-implemented.**
   The table documents `min`, `max`, `sex_required`, `species_required`, but `validation_service.py` evaluates only `date_expiry_warning` and `date_math_formula`. The other types are inert.

8. **CORS configuration is invalid with credentials.**
   `allow_origins=["*"]` + `allow_credentials=True` is rejected by browsers for credentialed requests.

9. **`API_KEY` override comment is wrong.**
   The comment says "override via env MIDATA_API_KEY", but `SettingsConfigDict` has no `env_prefix`/alias, so the variable read is `API_KEY`.

### 19.2 Documentation drift (code is authoritative)

| Doc claim | Reality |
|---|---|
| `routers/` directory | `controllers/` |
| Frontend `features/` | `ui/`, `state/`, `api/`, `transport/` |
| Import endpoint `/import` | `/data/import` |
| `uv run python -m pytest` | fails without a venv; use `.venv/bin/python` |
| Tests: 18 files / 252 passing | 19 test modules / 485 passing |
| Build output → `backend/static/` | `frontend/dist/`; copy is manual |
| Nav has 5 items | 7 items + Settings |
| Onboarding is 1 step (ADR target) | 5 steps |
| README architecture diagram | stale layer names and route count |
| `docs/dev/setup.md:152` claims build wiring | no `build.outDir`, no copy script |
| Theme key `midata-ui-theme` | `rumetrics-ui-theme` in `App.tsx` |
| Design doc "Rumetrics" naming | mixed: API says MiData, static/types/transport say Rumetrics |

### 19.3 Correctness / performance concerns

1. **N+1 on entity listing.** `GET /entities/` calls `flatten_entity()` per entity, each issuing a join query. For 10k animals this is 10k+ queries.
2. **Flattened values are not date-aware.** Neither `flatten_entity()` nor the DuckDB `PIVOT ... USING first(val)` filters to the latest `record_date`; time-series fields collapse arbitrarily.
3. **`GET /analytics/dashboard/stats` requires `mode_id`** (no default), while `dynamic` makes it optional — inconsistent contract.
4. **`compute_profitability` defaults to USD** at the API layer despite TND being the product default.
5. **`HomePage` bypasses the API layer** by importing `apiClient` directly, and bypasses `queryKeys`.
6. **`requirements.txt` mixes production, test, and desktop dependencies**, and includes `pyinstaller`/`pytest` for every install.
7. **Two lockfiles:** `pnpm-lock.yaml` (authoritative) and `package-lock.json` (stale, tracked).
8. **Static catch-all returns JSON** `{"error": "Not Found"}` with HTTP 200 for unknown `/api/*` paths, which can confuse clients.

---

## 20. Technical Debt & Risks

Prioritised by impact on users and maintainers.

### P0 — correctness / security

- Unauthenticated data API exposed on LAN and optionally the public internet via ngrok.
- Flattening ignores `record_date`, so "current weight/milk" can be an arbitrary historical value.
- `generate_summary_report` argument bug produces wrong/empty exports.
- Stale committed SPA ships an outdated UI in desktop builds.

### P1 — architecture

- In-memory confirmation tokens (no restart/multi-worker safety) and unused `pending_changes`.
- No migration versioning; inline migrations grow monotonically with no rollback.
- N+1 entity listing and per-entity flattening.
- Two analytics engines with duplicated semantics (risk of divergence between DuckDB and pandas results).
- Duplicate `/widgets/ai` routers from two modules make the API surface harder to reason about.

### P2 — quality

- Lint red (25 errors); no CI to enforce anything.
- No E2E tests; only 3 frontend test files.
- Docs drift across README, architecture, setup, and AGENTS.
- 3,203 kB single JS chunk (945 kB gzip) with no code splitting.
- Legacy "Rumetrics" naming in types, transport, static, and storage keys.
- Dead `alembic` dependency; stale `package-lock.json`.

### P3 — product

- Onboarding still asks farmers to choose bundles; ADR-004 wants one species step.
- 14+ configuration surfaces exposed at the same level as core tasks.
- No pre-built dashboard templates per species.
- Empty-state strategy not audited.

---

## 21. Roadmap: ADR-004 and the Current Branch

The active branch is `adr-004-progressive-disclosure`. ADR-004 ("Configuration Surface Reduction") diagnoses a configuration crisis and prescribes a frontend-only `midata_developer_mode` flag in `localStorage`. Its five phases:

| Phase | Target | Code state (verified) |
|---|---|---|
| 1 | Developer Mode flag + 5-item nav + gated mode/settings | ❌ No flag; 7 nav items; 5 ungated settings tabs |
| 2 | Pre-built dashboard templates + gated widget builders | ❌ Constructors ungated; no species templates |
| 3 | Collapse onboarding to 1 step | ❌ 5-step wizard |
| 4 | Derive mode from context, remove switcher | ❌ Switcher in nav; `ModeContext` has a migration flag only |
| 5 | Farmer terminology + empty-state overhaul | ◐ Terminology partially reworked; empty-state audit not done |

**Key insight for anyone continuing this work:** the mechanism existed briefly in commit `7f490e6` and was removed by `0c06a65`. The ADR remains Draft because the mechanism is not in the tree. The exact next step recorded in `.halmas/CURRENT-TASK.md` is to add `frontend/src/state/hooks/useDeveloperMode.ts` (mirroring `usePersistedState`), gate the mode `<Select>` at `Layout.tsx:140-155`, and split the nav array at `Layout.tsx:49-57`.

---

## 22. Glossary

| Term | Meaning in MiData |
|---|---|
| **Entity** | A tracked animal (or any object). Identity only; values live in measurements. |
| **Field definition** | Metadata for an attribute: key, label, category, type, unit, options, role. |
| **Measurement** | One value for one entity × field × date. |
| **Flattened view** | The EAV data pivoted into one column per field; `flattened_view` in DuckDB. |
| **Mode** | A workspace lens grouping a subset of fields (Dairy, Vet, Finance…). |
| **System mode** | A protected mode that always shows all fields ("General Husbandry"). |
| **Bundle** | A named set of field definitions used during onboarding. |
| **Extension** | A JSON manifest installing fields/modes/widgets atomically. |
| **Widget** | A dashboard card backed by `query_config` (SQL or stats config). |
| **Computed field** | A user formula over existing fields, evaluated by `simpleeval`. |
| **Validation rule** | A declarative condition that produces an alert for an entity. |
| **Phantom parent** | A pedigree node referenced by `custom_id` but absent from the herd. |
| **Preview token** | A one-time, 5-minute token authorising an AI-staged write. |
| **MCP / Halmas** | Agent-memory tooling around the repo (`.halmas/`), not part of the product runtime. |

---

## 23. Reading Guide: How to Onboard onto This Codebase

### 23.1 Read in this order

1. `backend/app/database/tables.py` — understand the EAV core before anything else.
2. `backend/app/services/analytics_engine.py` — see how EAV becomes a wide table.
3. `backend/app/controllers/entities.py` — the canonical read/write shape.
4. `backend/app/services/statistical_engine.py` (skim function map) — analytics behaviour.
5. `backend/app/services/tool_executor.py` + `services/tools/query_executor.py` — AI contract.
6. `backend/app/services/ai_chat_service.py` — SSE lifecycle and the tool loop.
7. `frontend/src/app/App.tsx` → `Layout.tsx` → `routes.tsx` — boot and navigation.
8. `frontend/src/state/context/ModeContext.tsx` — the workspace abstraction's frontend half.
9. `frontend/src/ui/dashboard/HomePage.tsx` + `DynamicWidgetRenderer.tsx` — server-driven UI.
10. `frontend/src/ui/assistant/ChatStream.tsx` — SSE rendering and tool blocks.
11. `backend/launcher.py` + `MiData.spec` — how it ships.

### 23.2 Mental model in five sentences

1. Everything is an entity plus dated key/value pairs.
2. Fields define what can be recorded; modes decide what is shown; bundles/extensions install fields in bulk.
3. Reads flatten EAV into columns — through SQLModel for correctness-oriented paths, through DuckDB for speed.
4. The AI never touches the database directly: it proposes SQL/operations, the validators gate them, and writes require a human confirmation token.
5. The same FastAPI process serves the API and the SPA, and the desktop launcher is only a process/tunnel/QR wrapper around it.

### 23.3 Where to change what

| Goal | Touch |
|---|---|
| Add an attribute | Create a `FieldDefinition` (API/UI/extension) — no migration |
| Add a species-specific field set | New JSON manifest in `backend/extensions/` |
| Add a table/column | `tables.py` + a new numbered block in `run_migrations()` |
| Add an API domain | New `controllers/<name>.py` + register in `main.py` |
| Add business logic | New `services/<name>.py`, keep controllers thin |
| Add an AI capability | New executor in `services/tools/` + register in `TOOL_EXECUTORS` + add a prompt line |
| Add a dashboard widget type | `DynamicWidgetRenderer` + `widgets/Ai*.tsx` + widget schema |
| Add a page | `ui/<slice>/` + route in `routes.tsx` + nav in `Layout.tsx` |
| Change theme | `src/index.css` variables + `docs/DESIGN.md` |

### 23.4 Traps to avoid

- Do not hand-edit `backend/static/`; rebuild and copy.
- Do not add Alembic or a second migration system.
- Do not reintroduce Pig anywhere (code, docs, seed, UI copy).
- Do not expose a new endpoint that trusts `X-API-Key` as real auth.
- Do not assume flattened values are the latest measurement.
- Do not trust `docs/` over the code; this report documents where they diverge.
- Do not use `npm`, `yarn`, `pip`, or `poetry`.
- Do not add a dependency without checking licence, maintenance, and the existing lockfile policy.

---

*End of report. Compiled from source on 2026-10-26; branch `adr-004-progressive-disclosure`, commit `0c06a65`.*
