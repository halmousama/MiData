# Feature inventory — parity checklist vs the legacy behaviour spec

> **Version:** v1 · **Generated:** 2026-10-26 (BOOTSTRAP-1, task B0) · **Updated:** 2026-10-26 (P0-11, oracle
> extraction) · **Status of every row:** `planned` · **Rows:** 222 (195 v0 + 27 added when the legacy test
> extraction proved behaviours the v0 checklist had no row for — see `docs/testing.md` §8)
> **Source of truth for behaviour:** `docs/legacy-spec/PROJECT_REPORT.md` (reference **only** — never an
> implementation source) transcribed against `REBUILD_PLAN.md` §5–§9. Legacy endpoint paths, module
> names and code shapes are deliberately absent: rows describe **behaviour**, because this is a
> specification-driven rebuild (D1/D15), not a port.
>
> **Row contract** (`REBUILD_PLAN.md` §6.1): `id · behaviour · status · phase · evidence`.
> - `status` ∈ `planned` · `partial` · `done`. **A wired row does not count as done** — only behaviour
>   **with evidence** counts (test names, snapshot paths). Marking `done` requires both.
> - `phase` = the phase whose exit criteria close the row (`docs/roadmap.md`).
> - `evidence` = test name(s) and/or snapshot path(s), or `—` while planned. Never a promise.
>
> **Ratchets:** total coverage % and per-block coverage only ever increase. A row is never deleted to
> raise the percentage; a dropped capability is marked `waived` with a reason in `docs/decisions.md`
> (the legacy LAN/ngrok/tray/SPA surfaces are the only pre-approved waivers — see SY-10).
>
> **Counting rule:** `docs/scorecard.md` (once `cargo xtask stats` exists) is the only source of the
> coverage number; the table at the bottom of this file is a hand-maintained v0 cross-check until then.

## Legend

| Status | Meaning |
|---|---|
| `planned` | Behaviour described, not implemented. |
| `partial` | Some surface exists; incomplete against the row text. |
| `done` | Behaviour implemented, tested, and evidence recorded. |
| `waived` | Intentionally out of scope (owner decision + reason). |

---

## HM — Herd management

*Source: PROJECT_REPORT §3.1 (herd management), §11.3 (birth), §17.2 (`test_soft_delete`), plus the
owner decisions that promote identity to columns (D7) and define sire/dam rules (§5.5). Oracle scenarios:
`docs/oracles/retention-and-soft-delete.md`, `docs/oracles/pedigree-and-reproduction.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| HM-1 | Create an animal with a farm-supplied identifier (tag/serial) | planned | P1 | — |
| HM-2 | Reject a duplicate identifier with an actionable, i18n'd error | planned | P1 | — |
| HM-3 | List animals: identity columns plus the active mode's field columns | planned | P1 | — |
| HM-4 | Sort, filter, paginate and multi-select the animal list | planned | P1 | — |
| HM-5 | Update an animal's identity attributes | planned | P1 | — |
| HM-6 | Delete an animal (hard delete; its observations go with it) | planned | P1 | — |
| HM-7 | Capture dynamic attributes for every active field on creation | planned | P1 | — |
| HM-8 | Reject a sire that is not Male | planned | P1 | — |
| HM-9 | Reject a dam that is not Female | planned | P1 | — |
| HM-10 | Record a phantom parent by external tag when the parent is not in the herd | planned | P1 | — |
| HM-11 | Render phantom parents distinctly from herd parents | planned | P4 | — |
| HM-12 | Set status Alive/Sold/Dead with exit date and reason | planned | P1 | — |
| HM-13 | Compute age from birth date; never store it | planned | P1 | — |
| HM-14 | Accept an unknown/null birth date without breaking list, age or analytics | planned | P1 | — |
| HM-15 | Keep breed as free text with per-species suggestions | planned | P1 | — |
| HM-16 | Animal detail: identity card with latest values | planned | P1 | — |
| HM-17 | Animal detail: full observation history timeline | planned | P1 | — |
| HM-18 | Animal detail: per-field history chart | planned | P1 | — |
| HM-19 | Animal detail: alerts panel | planned | P2 | — |
| HM-20 | Animal detail: pedigree panel | planned | P4 | — |
| HM-21 | Export an animal card as PDF | planned | P3 | — |
| HM-22 | A deactivated field's stored values are retained but hidden from current-value surfaces | planned | P2 | — |

## DE — Data entry

*Source: PROJECT_REPORT §3.2 (data entry), §5.3/§5.4 (value encoding, flattening), §11.6 (import).
Oracle scenarios: `docs/oracles/exchange-and-onboarding.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| DE-1 | Record one observation (animal, field, date, value) | planned | P1 | — |
| DE-2 | Batch entry: many fields for one animal at one date | planned | P1 | — |
| DE-3 | Mass entry: the same fields for many animals at one date | planned | P1 | — |
| DE-4 | Dynamic form built from field definitions (number, text, select, boolean, multi-select, date) | planned | P1 | — |
| DE-5 | Form honours unit and inline min/max bounds | planned | P1 | — |
| DE-6 | Form honours cardinality: `single` upserts, `series` appends | planned | P1 | — |
| DE-7 | "Current value" = latest by record date, tie-break newest row (date-aware, never insertion order) | planned | P1 | — |
| DE-8 | Recording the same animal/field/date twice replaces instead of duplicating | planned | P1 | — |
| DE-9 | Delete a single observation | planned | P1 | — |
| DE-10 | Import rows mapping spreadsheet columns to identity columns + field keys | planned | P3 | — |
| DE-11 | Import conflict strategy `skip` leaves existing values untouched | planned | P3 | — |
| DE-12 | Import conflict strategy `update` overwrites and audits | planned | P3 | — |
| DE-13 | Import reports per-row errors without discarding valid rows' outcome | planned | P3 | — |
| DE-14 | Import shows progress and can be cancelled | planned | P3 | — |
| DE-15 | Birth action records parturition, offspring and pedigree in one atomic transaction | planned | P4 | — |
| DE-16 | An import batch commits as one transaction: a concurrent reader sees none of it or all of it | planned | P3 | — |
| DE-17 | An empty import batch is a successful no-op with zero counts and no errors | planned | P3 | — |
| DE-18 | An empty spreadsheet cell means "no value" — never a zero and never an empty-string observation | planned | P3 | — |
| DE-19 | Duplicate identifiers within one import file are refused with an actionable error naming the rows (never silently merged) | planned | P3 | — |

## OR — Organization & configuration

*Source: PROJECT_REPORT §3.3 (fields, modes, computed, rules, packs, templates), §6.5 (field roles →
`purpose` tags, D7), §11.4–§11.5 (formulas, rules), §17.2 (`test_formula_service`, `test_settings`).
Oracle scenarios: `docs/oracles/storage-and-config.md`, `docs/oracles/retention-and-soft-delete.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| OR-1 | Create/edit a field definition: key, EN/FR labels, category, data type, unit, options, purpose tag, sort order | planned | P1 | — |
| OR-2 | Toggle a field active/inactive (soft delete) | planned | P1 | — |
| OR-3 | Group fields by category for managers and forms | planned | P1 | — |
| OR-4 | Reject an invalid data type or an unknown option selection | planned | P1 | — |
| OR-5 | Species registry lists the presets plus user-added species | planned | P0 | — |
| OR-6 | Add a custom species with no code change | planned | P1 | — |
| OR-7 | Edit a species' gestation default (data, not code) | planned | P1 | — |
| OR-8 | Toggle a species inactive | planned | P1 | — |
| OR-9 | Create/edit a mode (workspace) with icon and ordering | planned | P2 | — |
| OR-10 | Link fields to a mode with explicit ordering | planned | P2 | — |
| OR-11 | A protected system mode always shows all fields | planned | P2 | — |
| OR-12 | Activating a mode re-scopes every data surface (list, detail, entry) | planned | P2 | — |
| OR-13 | Computed field: create, update, delete | planned | P2 | — |
| OR-14 | Formula with `{{field_key}}` placeholders evaluated in a sandbox | planned | P2 | — |
| OR-15 | Reject a formula with a syntax error or an unknown field name at save time | planned | P2 | — |
| OR-16 | Division by zero yields a friendly message, never a crash | planned | P2 | — |
| OR-17 | Calculate computed fields herd-wide | planned | P2 | — |
| OR-18 | Rule type `min` evaluates and can raise an alert | planned | P2 | — |
| OR-19 | Rule type `max` evaluates and can raise an alert | planned | P2 | — |
| OR-20 | Rule type `range` evaluates and can raise an alert | planned | P2 | — |
| OR-21 | Rule type `required` evaluates and can raise an alert | planned | P2 | — |
| OR-22 | Rule type `date_expiry` (withdrawal windows) evaluates | planned | P2 | — |
| OR-23 | Rule type `date_expression` evaluates with only `date_diff`, `date_add`, `current_date` | planned | P2 | — |
| OR-24 | A malformed date expression evaluates false and never crashes | planned | P2 | — |
| OR-25 | Rule severity info/warning/critical drives alert styling | planned | P2 | — |
| OR-26 | Rule message rendered from an i18n template | planned | P2 | — |
| OR-27 | Rules evaluate per animal and herd-wide | planned | P2 | — |
| OR-28 | Alerts centre: nav badge plus alerts list | planned | P2 | — |
| OR-29 | Export a setup template: configuration only (fields, modes, links, widgets) | planned | P6 | — |
| OR-30 | Import a setup template without touching animals or observations | planned | P6 | — |
| OR-31 | Protected configuration fields refuse deactivation with an actionable i18n'd error and change nothing | planned | P1 | — |
| OR-32 | Identity-carrying attributes cannot be deactivated — identity is structural, not a deactivatable field | planned | P1 | — |
| OR-33 | Mode listing supports an active-only filter; the unfiltered listing exposes inactive configuration to management surfaces | planned | P2 | — |
| OR-34 | Toggle a mode active/inactive, with reactivation restoring it to listings and navigation | planned | P2 | — |
| OR-35 | One toggle command drives both directions for fields, modes and widgets — no separate activate/deactivate verbs | planned | P2 | — |

## AN — Analytics

*Source: PROJECT_REPORT §3.4 (statistics, studio, deep dive, correlation, profitability), §9.3–§9.4
(engine behaviours, deterministic insights), §17.2 (`test_analytics`, `test_tool_analyze`).
Oracle scenarios: `docs/oracles/analytics-and-dashboard.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| AN-1 | Descriptive stats (count, mean, median, std dev, min, max) per numeric field | planned | P3 | — |
| AN-2 | Same stats scoped by species/breed/sex/date filters | planned | P3 | — |
| AN-3 | Percentiles | planned | P3 | — |
| AN-4 | Trend: mean per date plus linear regression slope/intercept/r² | planned | P3 | — |
| AN-5 | Trend insight text derived deterministically from the statistics | planned | P3 | — |
| AN-6 | Distribution: histogram with bins | planned | P3 | — |
| AN-7 | Distribution: single-value edge case handled | planned | P3 | — |
| AN-8 | Distribution insight text | planned | P3 | — |
| AN-9 | Correlation: paired x/y data with Pearson r per group | planned | P3 | — |
| AN-10 | Correlation strength insight text | planned | P3 | — |
| AN-11 | Correlation matrix across numeric fields | planned | P3 | — |
| AN-12 | Comparison: group stats (count, mean, median, std, min, max) | planned | P3 | — |
| AN-13 | Comparison insight text | planned | P3 | — |
| AN-14 | Categorical fields route to count-based distributions, never numeric math | planned | P3 | — |
| AN-15 | Study request dispatches trend/distribution/correlation/comparison and returns chart-ready data | planned | P3 | — |
| AN-16 | Deep dive: overview block | planned | P3 | — |
| AN-17 | Deep dive: histogram block | planned | P3 | — |
| AN-18 | Deep dive: time-trend block | planned | P3 | — |
| AN-19 | Deep dive: by-breed breakdown | planned | P3 | — |
| AN-20 | Deep dive: by-sex breakdown | planned | P3 | — |
| AN-21 | Deep dive: top-5 low and top-5 high outliers | planned | P3 | — |
| AN-22 | Empty dataset returns a structured empty result, not an error | planned | P3 | — |
| AN-23 | Profitability: resolve the production field by purpose tag or explicit choice | planned | P3 | — |
| AN-24 | Profitability: prices and feed cost in integer minor units, currency from settings (TND default) | planned | P3 | — |
| AN-25 | Profitability: best and worst performers | planned | P3 | — |
| AN-26 | Every insight is a deterministic template; the LLM is never in the analytics path | planned | P3 | — |
| AN-27 | Growth trend: observations grouped by day/week/month with per-period average, min, max and count, plus direction and percentage change | planned | P3 | — |
| AN-28 | Herd composition: counts and percentages by species × sex × age group, with a herd total | planned | P3 | — |
| AN-29 | Mortality: deaths and death rate per year plus an overall rate, with the denominator stated in the result | planned | P3 | — |

## DB — Dashboard

*Source: PROJECT_REPORT §3.4 (server-driven widgets whose configuration lives in the database),
§9.3 (`compute_dashboard_stats`, `compute_dynamic_dashboard`), §13.3 (visualisation).
Oracle scenarios: `docs/oracles/analytics-and-dashboard.md`, `docs/oracles/assistant.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| DB-1 | KPI widget | planned | P3 | — |
| DB-2 | Line-chart widget | planned | P3 | — |
| DB-3 | Bar-chart widget | planned | P3 | — |
| DB-4 | Donut/pie widget via custom painter (or logged gap with bar fallback — §16 O3) | planned | P3 | — |
| DB-5 | Table widget | planned | P3 | — |
| DB-6 | Progress-list widget | planned | P3 | — |
| DB-7 | Alert-board widget | planned | P3 | — |
| DB-8 | Profitability widget | planned | P3 | — |
| DB-9 | Widget data source stored as configuration in the database and hydrated at load | planned | P3 | — |
| DB-10 | Dashboard summary: total animals, recent activity count, alert count | planned | P3 | — |
| DB-11 | Widgets scoped per mode; global widgets always visible | planned | P3 | — |
| DB-12 | Enable/disable and reorder widgets | planned | P3 | — |
| DB-13 | Hydration budget: 10 widgets over 1k animals < 300 ms | planned | P3 | — |

## PD — Pedigree & reproduction

*Source: PROJECT_REPORT §3.4 (pedigree graph), §9.5 (BFS/bounds/phantoms/cap/warnings), §11.3
(birth trigger), §17.2 (`test_pedigree`, `test_health_reproduction`).
Oracle scenarios: `docs/oracles/pedigree-and-reproduction.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| PD-1 | Root-centric bidirectional BFS (ancestors up, descendants down) | planned | P4 | — |
| PD-2 | Generation bound 1–10, default 3 | planned | P4 | — |
| PD-3 | Visited-set cycle protection (no infinite traversal) | planned | P4 | — |
| PD-4 | Phantom nodes for parents referenced by tag but absent from the herd | planned | P4 | — |
| PD-5 | Global cap of 500 nodes with a `truncated` flag | planned | P4 | — |
| PD-6 | Sex-consistency warnings are reported, not fatal | planned | P4 | — |
| PD-7 | Ancestors query returns a tree for one animal | planned | P4 | — |
| PD-8 | Descendants query | planned | P4 | — |
| PD-9 | Pedigree layout maths as a pure, unit-tested function | planned | P4 | — |
| PD-10 | Pedigree canvas: pan/zoom, node cards, click-through to animal detail | planned | P4 | — |
| PD-11 | Birth: validate that the dam (and optional sire) exist | planned | P4 | — |
| PD-12 | Birth: verify the sex of dam and sire | planned | P4 | — |
| PD-13 | Birth: record parturition observations on the dam via purpose tags | planned | P4 | — |
| PD-14 | Birth: set the dam's pregnancy status to Open | planned | P4 | — |
| PD-15 | Birth: create each offspring; a duplicate identifier aborts the whole action | planned | P4 | — |
| PD-16 | Birth: write identity observations and pedigree links | planned | P4 | — |
| PD-17 | Birth: all-or-nothing — an injected mid-way failure leaves zero partial state | planned | P4 | — |
| PD-18 | Pregnant-animals view | planned | P4 | — |
| PD-19 | Upcoming-expected-births view | planned | P4 | — |
| PD-20 | Reproduction alerts appear in the alerts centre | planned | P4 | — |
| PD-21 | A pedigree node renders an animal whose sex or species is unrecorded as "unknown" rather than omitting the node | planned | P4 | — |

## IX — Import / export / backup

*Source: PROJECT_REPORT §3.2 (Excel/CSV import), §3.4 (Excel export), §11.6 (import, export, backup),
§17.2 (`test_import`). Oracle scenarios: `docs/oracles/exchange-and-onboarding.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| IX-1 | Read XLSX workbooks | planned | P3 | — |
| IX-2 | Read CSV files | planned | P3 | — |
| IX-3 | Column mapping preview before commit | planned | P3 | — |
| IX-4 | Entity-level keys never become observations | planned | P3 | — |
| IX-5 | Export the flattened wide table to XLSX | planned | P3 | — |
| IX-6 | Export filtered by species/breed | planned | P3 | — |
| IX-7 | Export a summary workbook | planned | P3 | — |
| IX-8 | Export animal cards / report sheets as PDF | planned | P3 | — |
| IX-9 | Backup: WAL checkpoint, integrity check, timestamped copy | planned | P3 | — |
| IX-10 | Backup rotation keeps the newest 10 | planned | P3 | — |
| IX-11 | Backup verify command | planned | P3 | — |
| IX-12 | Restore round-trip yields an identical database | planned | P3 | — |
| IX-13 | Exports and backups land in the platform exports/backups directories | planned | P3 | — |

## SY — System, settings, i18n, theme

*Source: PROJECT_REPORT §3.6 (sharing/desktop — mostly dropped, see SY-10), §15 (configuration),
REBUILD_PLAN §4.5 (storage), §7.4 (i18n), RG golden rules 12/14 (i18n, audit).
Oracle scenarios: `docs/oracles/storage-and-config.md`, `docs/oracles/launcher-waivers.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| SY-1 | Settings key/value store read and written through commands | planned | P0 | — |
| SY-2 | Language `en`/`fr`, live-switchable, persisted | planned | P0 | — |
| SY-3 | Light/dark theme persisted, all keys from one constants module | planned | P0 | — |
| SY-4 | Currency TND/USD/EUR with TND default | planned | P0 | — |
| SY-5 | Every user-facing string resolves through i18n (EN source, FR first-class) | planned | P0 | — |
| SY-6 | One audit row per mutating command: command id, actor, entity/field, old/new, timestamp | planned | P1 | — |
| SY-7 | Actors user/cli/seed/ai are distinguishable in the audit log | planned | P1 | — |
| SY-8 | Developer-mode flag persisted (gating itself arrives in P6) | planned | P1 | — |
| SY-9 | Offline: no telemetry, no accounts, no network peer except the configured LLM endpoint | planned | P0 | — |
| SY-10 | Legacy sharing surfaces absent by design: no LAN server, no tunnel, no QR code, no tray, no SPA bundle | planned | P0 | — |
| SY-11 | Portable mode: data directory next to the executable via env flag | planned | P1 | — |
| SY-12 | Database lives in the platform data directory (Linux + Windows paths) | planned | P0 | — |
| SY-13 | Opening an empty data file creates the complete schema at the current version, with no manual step | planned | P0 | — |
| SY-14 | One embedded file-based database is the only storage backend — no second engine, no server | planned | P0 | — |
| SY-15 | The data file runs in WAL journal mode; memory-only databases are never used for real data | planned | P0 | — |
| SY-16 | An absent, corrupt or schema-short settings store degrades to defaults instead of crashing; a second write replaces the first | planned | P0 | — |

## AI — AI assistant

*Source: PROJECT_REPORT §3.5 (assistant capabilities), §10.3–§10.5 (validation, caps, tokens, audit),
§17.2 (`test_tool_query`, `test_ai_chat_service`, `test_ai_settings`, `test_ai_conversations`).
Oracle scenarios: `docs/oracles/assistant.md`, `docs/oracles/query-validation.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| AI-1 | Streaming chat with incremental chunks | planned | P5 | — |
| AI-2 | Event set: chunk, tool_call, tool_status, tool_result, confirmation, done, error | planned | P5 | — |
| AI-3 | Conversations persisted, listed, renamed, deleted | planned | P5 | — |
| AI-4 | Docked assistant panel, pinnable | planned | P5 | — |
| AI-5 | Tool list generated from the command registry (new commands are automatically offerable) | planned | P5 | — |
| AI-6 | Read-only SQL: exactly one statement, SELECT/WITH only | planned | P5 | — |
| AI-7 | Columns whitelisted against identity columns + active fields | planned | P5 | — |
| AI-8 | Caps: 5,000 rows / 50 KB result / 30 raw rows before summarization / 500 aggregated / 200-char strings | planned | P5 | — |
| AI-9 | Per-tool timeout | planned | P5 | — |
| AI-10 | Write tools stage a plan — nothing executes during the chat turn | planned | P5 | — |
| AI-11 | Staged plan persisted with a single-use token and 5-minute expiry | planned | P5 | — |
| AI-12 | Staged plans survive an app restart | planned | P5 | — |
| AI-13 | Confirmation card commits (transactional or per-item) or cancels | planned | P5 | — |
| AI-14 | Audit rows carry the batch token | planned | P5 | — |
| AI-15 | Provider settings: base URL, model, temperature | planned | P5 | — |
| AI-16 | API key in the OS keyring; connectivity test command | planned | P5 | — |
| AI-17 | Context governance: last 20 messages + summarization, 32k budget with a 24k warning | planned | P5 | — |
| AI-18 | AI widget proposal: natural language → validated query + chart config → preview → saved as AI-sourced | planned | P5 | — |
| AI-19 | Injection attempts rejected by the validation lab | planned | P5 | — |
| AI-20 | Assistant failures surface as i18n messages, never raw exception text | planned | P5 | — |
| AI-21 | Query result envelope: columns, rows, row count and a truncated flag form the tool contract | planned | P5 | — |
| AI-22 | An empty result set is a successful response, never an error | planned | P5 | — |
| AI-23 | Assistant drives view navigation through a closed action set dispatched by command id, carrying no domain logic | planned | P5 | — |

## EX — Extensions / packs

*Source: PROJECT_REPORT §3.3 (extensions/marketplace), §11.2 (install atomicity, type conflicts,
reactivation, gestation seeding, soft uninstall), §17.2 (`test_extensions`).
Oracle scenarios: `docs/oracles/exchange-and-onboarding.md`, `docs/oracles/retention-and-soft-delete.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| EX-1 | Discover pack manifests (bundled + user-added) | planned | P6 | — |
| EX-2 | Install a pack's fields atomically | planned | P6 | — |
| EX-3 | Reject a pack field whose key exists with a different data type | planned | P6 | — |
| EX-4 | Reactivate an inactive field and refresh its label/unit/options | planned | P6 | — |
| EX-5 | Create or reactivate modes and link fields with ordering | planned | P6 | — |
| EX-6 | Create or reactivate widgets | planned | P6 | — |
| EX-7 | Seed per-species gestation defaults from the pack | planned | P6 | — |
| EX-8 | Install is all-or-nothing: any failure rolls back every write | planned | P6 | — |
| EX-9 | Uninstall deactivates; it never deletes data | planned | P6 | — |
| EX-10 | Packs screen lists packs with their install state | planned | P6 | — |
| EX-11 | Bundled packs ship with the application | planned | P6 | — |
| EX-12 | Install state survives a restart | planned | P6 | — |
| EX-13 | Re-installing an installed pack refreshes or skips existing fields, modes and widgets in place — never duplicates them | planned | P6 | — |
| EX-14 | Pack-owned widgets cannot be deactivated individually; their state follows the pack's install state | planned | P6 | — |
| EX-15 | Uninstalling a pack that was never installed returns an actionable error and changes nothing | planned | P6 | — |
| EX-16 | Pack install state is reported as active / deactivated / never-installed by a single read command | planned | P6 | — |

## OB — Onboarding

*Source: PROJECT_REPORT §3.3/§11.1 (onboarding pipeline, merge-only idempotency, bundles, demo seed),
§5.7 (seed determinism), D9 (seed species policy), D21 (one-step flow), §17.2 (`test_onboarding`).
Oracle scenarios: `docs/oracles/exchange-and-onboarding.md`.*

| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| OB-1 | One-step onboarding: species presets + optional demo data | planned | P0 | — |
| OB-2 | Onboarding is merge-only and idempotent — running twice duplicates nothing | planned | P0 | — |
| OB-3 | Selected species presets are merged into the registry | planned | P0 | — |
| OB-4 | Field bundles are created with user overrides honoured | planned | P0 | — |
| OB-5 | Modes are created and their fields linked | planned | P0 | — |
| OB-6 | Default dashboard widgets are created (global and per mode) | planned | P0 | — |
| OB-7 | The system mode exists after onboarding | planned | P0 | — |
| OB-8 | Optional demo seed is deterministic, idempotent, and rebuildable | planned | P0 | — |
| OB-9 | Demo seed content: ~50 animals, ~2,000 observations, 5-generation pedigree, 2 phantom parents, sample modes/widgets/rules/computed fields | planned | P0 | — |
| OB-10 | Seed contains only registry preset species (policy test) | planned | P0 | — |
| OB-11 | Onboarding status reports whether setup is complete | planned | P0 | — |
| OB-12 | Language and currency are chosen at onboarding and persisted | planned | P0 | — |
| OB-13 | Onboarding with nothing selected is a clean no-op or an actionable refusal, never a half-configured app | planned | P0 | — |
| OB-14 | An unknown bundle or pack identifier is refused with an actionable error, never silently ignored | planned | P0 | — |
| OB-15 | Optional financial field groups (pricing, treatment cost, feed cost) are created only on explicit opt-in | planned | P0 | — |

---

## Coverage (v0, hand-maintained cross-check)

Every row above is `planned`, so every number below is zero. Once `cargo xtask stats` exists
(P0-1/P0-9) this table is generated into `docs/scorecard.md` and this section becomes a pointer.

| Block | Rows | Done | Coverage |
|---|---|---|---|
| HM | 22 | 0 | 0% |
| DE | 19 | 0 | 0% |
| OR | 35 | 0 | 0% |
| AN | 29 | 0 | 0% |
| DB | 13 | 0 | 0% |
| PD | 21 | 0 | 0% |
| IX | 13 | 0 | 0% |
| SY | 16 | 0 | 0% |
| AI | 23 | 0 | 0% |
| EX | 16 | 0 | 0% |
| OB | 15 | 0 | 0% |
| **Total** | **222** | **0** | **0%** |

## Waivers

| id / surface | Reason | Recorded in |
|---|---|---|
| LAN sharing server, ngrok tunnel, QR codes | D2: native desktop only; remote access is a documented future feature | `docs/decisions.md` D2, `REBUILD_PLAN.md` §6.2 DROP |
| System tray, PyInstaller packaging, committed SPA bundle | Not applicable to a native Rust binary; no build artifacts are ever committed | `REBUILD_PLAN.md` §6.2 DROP, §13.4 |
| CORS / API-key machinery, DuckDB second engine | No server exists; single analytics engine (D16) | `REBUILD_PLAN.md` §6.2 DROP |
| Five-step onboarding wizard, Arabic UI, forced-Arabic AI responses | Replaced by 1-step onboarding (D21), EN+FR only (D3), mirror-language AI (D10) | `docs/decisions.md` D3/D10/D21 |
| Legacy structured query-plan (DSL) surface | One validated SQL path; a second query language doubles validation for no gain | `docs/oracles/query-validation.md` |

**Correction (2026-10-26, P0-11).** The legacy test module whose name suggested local sharing
(`test_local_sharing`, 37 tests) was previously assumed to be covered by the D2 waiver. It is not: it
contains no sharing, tunnel, QR-code or browser-UI assertion at all — it tests read-only query validation
(16), setup-template export (7), import (11) and round-trip (3), **all of which this inventory keeps**. The
waiver applies to the 12 launcher tests, not to 49. See `docs/oracles/launcher-waivers.md` and
`docs/testing.md` §7.

Oracle extraction for every legacy test module is **complete** (2026-10-26, P0-11) — scenarios live in
`docs/oracles/`, indexed by `docs/testing.md` §2. A row without an oracle scenario is written fresh from the
plan prose (D15); the `## Fresh` section of each oracle file lists exactly which behaviours those are.
