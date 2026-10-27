# Architecture

> **Status:** P0-1 landed — the workspace and `xtask` gates exist; **only `xtask` is a crate**. Product
> crates are birth-gated from P0-2 (D26), so nothing below is implemented yet. This file records what
> `REBUILD_PLAN.md` §4–§9 and the §15 decision log bind us to, so a task can be judged against it.
> It is updated when a crate is born **and** when a binding decision is fixed ahead of the crate that
> implements it (the §9 injection seams bind from P0-2, before those crates exist).
> **Source:** `REBUILD_PLAN.md` §4 (target architecture), §5 (data model), §8 (analytics), §9 (AI),
> §10 (agent surfaces); owner decisions D1–D25.

## 1. Shape of the system

MiData is a **native, offline, single-SQLite-file desktop application** in Rust: `eframe`/`egui` on
wgpu for the shell, `rusqlite` for storage, no server, no webview, no accounts, no telemetry (C1/C2/D2).
The only permitted network peer is the user-configured OpenAI-compatible LLM endpoint (Phase 5).

The single idea that organises everything else: **every user-visible behaviour is a command**. The UI,
the CLI, the AI assistant and (Phase 6) the control channel / MCP server are thin dispatchers over one
registry. A behaviour is written once and becomes available on every surface — that is what makes the
product drivable by agents as well as by people (D13).

## 2. Layers and crates

Layers are a **registered DAG**, not a strict chain — every crate and every allowed edge is declared in
`xtask/src/layers.rs` and checked by `cargo xtask check-layers` (D24). An undeclared edge fails CI, so a
layer violation is a build fact, not a review opinion. Application crates are roots, never libraries:
the headless `midata-cli` may depend only on `midata-domain`, `midata-i18n`, `midata-store` and
`midata-engine` (never the egui shell), while `apps/midata` — the UI composition root — is the one crate
allowed to link `midata-app`.

```text
L0  midata-domain      pure types & value semantics (species/sex/status/data types/cardinality/value
                       rules/CommandError) — zero I/O; serde + chrono + thiserror only
L0  midata-i18n        TSV catalogues (en.tsv source, fr.tsv), t!() lookup, coverage data — pure,
                       shared by app AND cli
L1  midata-store       rusqlite engine (WAL, FK ON), migrations (PRAGMA user_version), repositories,
                       canonical latest-value/flatten queries, validated-query execution
L2  midata-analytics   statistics, studies, dashboard hydration, pedigree graph — the single engine
L2  midata-queryguard  SQL validation lab (sqlparser AST): SELECT-only, column whitelist, caps — pure
L2  midata-exchange    xlsx/csv read, xlsx write, PDF render — the only home of file-format crates
L2  midata-ops         domain workflows (birth, onboarding merge, pack install, import pipeline,
                       backup ops, seed generation) — keeps engine commands thin
L3  midata-engine      Session + Command Registry + audit hooks + undo seam
L4  midata-ai          LLM client (SSE) + tool loop over the registry — the ONLY crate allowed
                       tokio/reqwest
L5  midata-app         egui shell: screens (thin), widget kit, serde app state, theme tokens
```

Allowed edges (summary): `domain, i18n → store → {analytics, queryguard, exchange} → ops → engine → ai
→ app`, plus intra-layer `ops → analytics/exchange/store`. `apps/*` may depend on anything; nothing may
depend on `apps/*` or on `midata-app`; `midata-testkit` is a dev-dependency only.

Hard consequences:

- **`midata-app` depends only on `midata-domain`, `midata-engine`, `midata-i18n`** (+ the egui stack).
  All data reaches the UI as command output — the thin UI is a Cargo fact.
- **Nothing below `midata-app` may import `egui`, `eframe`, `egui_plot`, `egui_extras`, `winit` or
  `rfd`.**
- **No crate before the phase that first uses it.** An empty crate is dead schema by another name. P0
  set: domain, i18n, store, ops, engine, app, testkit + `apps/midata`, `apps/midata-cli`, `xtask`.
- Inside `midata-app`: `widgets/` never imports from `screens/` (keeps a future `midata-widgets`
  extraction mechanical, §4.8).
- Each crate carries a `README.md` documenting its **public API** — the interface an agent reads instead
  of the implementation. Keep it current with every public-API change.
- Split triggers are measurable, not dates: `midata-widgets` extraction when ≥2 agents work on UI in
  parallel **or** `widgets/` exceeds ~5k lines **or** the kit API is stable while screens churn; any
  `<area>_cmds.rs` that outgrows the budget splits into modules first, a crate last.

## 3. The command seam

```text
Command {
  id:          "animal.create"          // lowercase dot-notation, stable forever (a published surface)
  title_key:   "cmd.animal.create.title"// i18n key, EN + FR
  params_doc:  { ... }                  // what agents read via `midata commands`
  enabled:     |ctx| -> bool            // greys UI entry points when false
  run:         |ctx, params| -> Result<CommandOutput, CommandError>
                                        // exactly one audit_log row; invertible → one undo step
}
```

- Lives in `midata-engine`; implementations live in **per-area modules** (`animal_cmds.rs`,
  `observation_cmds.rs`, `field_cmds.rs`, `mode_cmds.rs`, `analytics_cmds.rs`, `data_cmds.rs`,
  `system_cmds.rs`, `ai_cmds.rs`…), each exposing `specs()`, merged in one `commands.rs` list. Shared
  files never grow.
- `run` never panics on any params or state; it returns a typed `CommandError` carrying an i18n key.
- Only pure view/window state (navigation, theme, layout, language) belongs to the shell (`app.*`).
- Ids are never renamed or reused. A behaviour that disappears leaves a documented gap, not a recycled
  id.
- Domain logic lives in the **lowest** crate that fits: pure rules → `midata-domain`; persistence →
  `midata-store`; computation → `midata-analytics`; multi-step workflow → `midata-ops`. The command
  itself is ~30–60 lines of dispatch.

## 4. Data model (v1)

Two kinds of facts, never mixed:

- **Identity = columns** on `entities`: `custom_id`, `species_id`, `sex`, `breed`, `birth_date`,
  `sire_id`/`dam_id` (+ phantom `sire_tag`/`dam_tag`), `status`/`exit_date`/`exit_reason`, `notes`,
  timestamps (D7). Age is **computed** from `birth_date`, never stored.
- **Everything else = EAV**: `field_definitions` (key, EN/FR labels, category, `data_type`,
  `cardinality`, unit, options JSON, `purpose` tag, inline validation, `is_active`) and `observations`
  (entity, field, `recorded_on`, exactly one of `value_num`/`value_text`, source, created_at) (D6).

Supporting tables in the same schema family: `species` (open registry — data, never code, D8), `modes` +
`mode_fields` (workspaces, one protected system mode), `computed_fields`, `validation_rules`, `widgets`,
`settings`, `audit_log`; later phases add `conversations`/`messages`/`pending_changes` (P5). Full DDL:
`REBUILD_PLAN.md` §5.2.

**Value encoding** (§5.3): number/boolean → `value_num`; text/select/multi-select-as-JSON-array/
date-as-ISO → `value_text`; exactly one non-NULL, matching the field's `data_type`, and values are only
constructed through domain constructors. `cardinality='single'` keeps at most one row per
(entity, field) via upsert; `series` enforces `UNIQUE(entity_id, field_id, recorded_on)`.

**Business vocabulary is configuration data; structural identity is typed code**
(`config.business-vocabularies`, D7). Values a farm may reasonably reword or extend — purpose tags on a
field (a field may serve several), pregnancy-like statuses, health-issue and suggestion lists — are
**multi-valued configuration data read from field/species config** (options JSON, purpose join table),
never Rust enums. Values that define what a record *is* — sex, entity lifecycle status (Alive/Sold/Dead),
`data_type`, `cardinality` — are **typed code constraints** enforced by enums, CHECK constraints and
exhaustive matches. Adding a purpose or a suggestion list entry is data; adding a sex is a §15 decision.
A business value must never require a code change, and a structural value must never be editable from
settings.

**Latest-value semantics** — the legacy P0 fix — is one statement, never a loop:

```sql
SELECT entity_id, field_id, value_num, value_text FROM (
  SELECT o.*, ROW_NUMBER() OVER (
    PARTITION BY o.entity_id, o.field_id
    ORDER BY o.recorded_on DESC, o.id DESC) AS rn
  FROM observations o
) WHERE rn = 1;
```

This logical flattened view is the **single source** for the animal list, exports, the AI query
whitelist surface, dashboard hydration and analytics filtering. No second flattening implementation
exists anywhere; date-blind flattening and N+1 listing are anti-patterns we must never reintroduce.

Invariants (§5.5): sire must be Male, dam must be Female (enforced at command level with graceful
errors); duplicate `custom_id` rejected with a friendly key; entity delete cascades observations; field
delete is soft (`is_active = 0`); modes and packs soft-toggle. Every invariant has an integration test.

## 5. Concurrency

Everything in domain/store/analytics/engine/app is **synchronous** (D22). Long work (imports, heavy
analytics, AI streaming) runs on std background threads and reports back over `crossbeam-channel`; the
UI polls each frame and requests a repaint. `tokio` exists only inside `midata-ai` (isolated runtime on
its worker thread) for `reqwest`/SSE. DB access is one `Store` handle wrapping
`Mutex<rusqlite::Connection>` on WAL — single user, negligible contention, shared safely by the UI and
workers. Lock poisoning is handled (`into_inner`), never propagated as a panic.

## 6. Storage, configuration, secrets

| What | Where |
|---|---|
| Database | platform data dir via `directories`: Linux `~/.local/share/midata/midata.db`, Windows `%APPDATA%\midata\midata.db`, macOS `~/Library/Application Support/midata/midata.db` |
| Portable mode | `MIDATA_PORTABLE=1` → `midata_data/` next to the executable (farm-USB friendly) |
| Exports / backups | `…/midata/exports/`, `…/midata/backups/` (rotation keeps the newest 10) |
| Logs | `…/midata/logs/` (`tracing` + rotating file appender) |
| App settings | `settings` table, key/value JSON (language, theme, currency, mode prefs, AI provider config) |
| Secrets | **OS keyring only** (`keyring`). Never SQLite, settings files, logs, or values derived from a constant (D17) |

## 7. Analytics (single engine)

`midata-analytics` is the only analytics code path, serving the UI, CLI, widgets and AI tools (D16). It
computes: descriptive stats, trend (+ linear regression slope/intercept/r²), distribution (histogram,
single-value edge case), correlation (Pearson per group and across numeric fields), comparison group
stats, deep-dive blocks (overview, histogram, time trend, by-breed, by-sex, top/bottom-5 outliers),
dashboard hydration, profitability (production field resolved by `purpose` tag or explicit choice,
integer minor units, TND default) and the pedigree graph (bidirectional BFS, generations 1–10,
visited-set cycle protection, phantom nodes, 500-node cap, sex warnings).

Categorical fields route to count-based distributions, never numeric math. Empty datasets return a
structured empty result, never an error. Insights are deterministic i18n templates — **the LLM is never
in the analytics path**. Randomness is seeded; DuckDB and polars are excluded.

## 8. Agent-facing surfaces

| Surface | What | Phase |
|---|---|---|
| egui UI | human surface: dispatches commands, renders state | 0–7 |
| `midata-cli` | `commands`, `run <id> --params <json>`, `seed`, later `import`/`export`/`backup`/`restore`/`stats` | skeleton P0, grows P3+ |
| Audit log | one row per mutation: command, actor (user/ai/cli/seed), entity/field, old/new, batch token | P1 |
| Control channel | loopback TCP JSON-lines, token-file auth: `engine.execute`, `engine.commands`, `app.inspect`, `ui.screenshot` | P6 |
| MCP server | `midata-cli mcp --bridge <port>` over the same protocol | P6 |
| Mobile/remote | documented strategy only, not code | P7 doc |

`midata-app` and app state are serde-serializable so an automation surface can read and drive them; the
registry is self-documenting via `params_doc`, so an agent discovers capabilities without reading source.

## 9. Dependency-injection seams (binding from P0-2)

The rebuild must be testable without hidden globals. The legacy defect
`test-integrity.injected-dependencies` (“module-level cache patched by tests, fixtures inferred from
another file”) is structurally excluded by one rule: **everything that touches the outside world is
injected through the `Session`/`Context`, never reached through a global.**

| Seam | Shape (lands in) | Why |
|---|---|---|
| Clock | `trait Clock { fn now_utc(&self) -> DateTime<Utc>; fn today(&self) -> NaiveDate; }` in `midata-domain` (P0-2); `SystemClock` in the composition root, `FixedClock` in `midata-testkit` | Dates drive age, latest-value, alerts and analytics; tests must not depend on wall time (`docs/testing.md` determinism rule) |
| Store handle | `Store` owns `Mutex<Connection>` (P0-3); `Session` carries `&Store`/`Arc<Store>`; no module-level connection, no `static` | One handle, WAL, shared by UI/CLI/AI workers; tests open a tempfile DB and inject it |
| LLM provider | `trait LlmProvider` in `midata-ai` (P5), streaming events over a channel; a stub provider in tests | `ai.deterministic-provider` anti-pattern: no network, no timing assumptions |
| Actor + batch token | `Session { actor, batch_token }`, set by each surface (`user`/`cli`/`ai`/`seed`) | Audit rows are a consequence of the injected session, not a global “current user” |
| Filesystem roots | data/exports/backups/log dirs resolved once in the composition root and passed down | Portable mode and tests need different roots without env-var mutation |

Rules that make this natural from the first crate:

- **No `static mut`, no interior-mutable globals, no `OnceLock`/`lazy_static` for state.** A constant
  is fine; a process-wide handle is not.
- `DateTime::now`/`Utc::now` may be called **only** inside `SystemClock`; otherwise the `Clock` is passed.
- `midata-app` never receives a store handle (layer rule); it dispatches commands and renders output.
- The composition roots (`apps/midata`, `apps/midata-cli`, `midata-testkit`) are the only places that
  construct real implementations; every other crate accepts its dependencies as arguments.
- `xtask` follows the same rule today: it takes `root: &Path` explicitly and reads the workspace once
  per gate — no current-directory globals.

## 10. What this file is not

It is not a phase plan (`docs/roadmap.md`), not the work queue (`docs/gaps.md`), and not the parity
checklist (`docs/inventory.md`). If this file and `REBUILD_PLAN.md` ever disagree, the plan's §15
Decision Log wins — fix this file in the same commit.
