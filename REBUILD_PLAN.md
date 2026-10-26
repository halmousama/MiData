# MiData Native — Master Rebuild Plan

> **Version:** 0.3-draft (awaiting owner approval)
> **Last reviewed:** 2026-10-27 · **Last updated:** 2026-10-27 · **Change:** Git workflow & traceability added (§13.4, D25)
> **Owner:** project owner (decision authority) · **Executor:** AI coding agent(s) under this plan + `AGENTS.md`
> **Inputs:** `uploads/PROJECT_REPORT.md` (legacy behaviour specification) · `ARTCRAFT_INVESTIGATION.md` (methodology source, storytold/photocraft)
> **Language policy:** this document and every repo artifact are written in **English** (execution language of agents). Product UI strings: **English (source) + French**. No Arabic UI in v1.
> **Naming:** product name is always **MiData** (user-facing, PascalCase). Machine names are lowercase: crates `midata-*`, binary `midata`, command ids `area.action`, storage keys `midata_*`. The legacy name "Rumetrics" must appear **zero** times anywhere.

---

## 0. How to use this document

- **For the owner:** this is the approval artifact. §15 (Decision Log) records every decision taken during consultation; §16 lists the few open items with recommended defaults. **No implementation starts until this plan is approved** — the first implementation artifact after approval is `AGENTS.md` (Phase 0).
- **For agents:** this is the master plan, not the task list. Once Phase 0 exists, day-to-day work is picked from `docs/gaps.md` (ranked work list) and tracked in `docs/inventory.md` (feature coverage). Read `AGENTS.md` first, always. Where this plan and later docs conflict, this plan's §15 Decision Log wins until the owner amends it.
- **Stamp discipline:** every update to this file bumps the header stamps and appends a line to §15, mirroring the ArtCraft progress-docs standard.

---

## 1. Vision

### 1.1 Product definition

MiData is a **local-first livestock management and analysis desktop application** for small and medium herds (a few dozen to a few thousand head). It is offline, account-free, single-SQLite-file, native (Rust + egui), and oriented to the Maghreb market (default currency **TND**, French/English UI, cattle/sheep/goat presets with an **open species registry**).

The core product bet is preserved from the legacy system: an **EAV (Entity–Attribute–Value) data model** — every farm defines its own measurable attributes without migrations — combined with **Modes** (workspace lenses), a **single analytics engine**, **dashboard widgets**, **pedigree**, **import/export**, **extension packs**, and a **local LLM assistant** with staged, confirmed writes.

### 1.2 Non-negotiable constraints

| # | Constraint |
|---|---|
| C1 | Native desktop app. **No HTTP server, no browser UI, no webview** in v1. |
| C2 | Offline-first. No cloud SDK, no telemetry, no accounts. The only network calls: the user-configured OpenAI-compatible LLM endpoint (Phase 5). |
| C3 | UI languages: **English + French** only. No Arabic UI, no RTL work in v1. |
| C4 | Species registry is **open**: presets Cow/Sheep/Goat; the user can add any species (Camel, Horse, Dog, …). See §5.7 for the pig policy. |
| C5 | Single SQLite file (WAL, foreign keys ON) in the platform data directory; optional portable mode next to the executable. |
| C6 | Default currency TND; all money as **integer minor units** (never floats). |
| C7 | Licence: open source (recommended **MIT OR Apache-2.0** dual, Rust-ecosystem standard; owner to confirm — see §16 O1). |
| C8 | Mobile/remote access is a **documented future feature** (contract written in Phase 7), not built in v1. |
| C9 | No translation of legacy code. Legacy is consumed **as specification only** (§2). |

### 1.3 Design philosophy: agent-first (owner's principle)

> *"Software in this era must be oriented toward agents as much as toward humans. Users today are humans **and** agents, not humans only."* — project owner, 2026-10-27

Architectural consequences (all mandatory, from Phase 0):

1. **Everything is a command.** Every domain operation is a registered command (`id`, documented params, `enabled` predicate, `run → Result<JSON>`). The egui UI, the CLI, the Phase-5 AI assistant, and the Phase-6 control channel/MCP server are all **thin dispatchers over the same registry**.
2. **All state is inspectable.** App and UI state are serde-serializable structs, never scattered flags — an automated consumer can read and drive them.
3. **Commands are self-documenting.** `midata commands` (and later `engine.commands` over the control channel) lists ids, params schemas and docs — an agent discovers capabilities without reading source.
4. **CLI is a first-class citizen** (`apps/midata-cli`), shipped from Phase 1, growing with the registry.
5. **Control-channel + MCP seams exist from day one** (the registry is the seam); the loopback JSON-lines server and MCP wrapper are implemented in Phase 6.
6. **The audit log doubles as agent observability:** every command execution records actor (user/ai/cli), params digest, and results.
7. **`AGENTS.md` is part of the product,** kept under ~7–8 KB (bootloader + orientation table), with detail pushed to `docs/*` read on demand.

### 1.4 Honest positioning

egui yields a clean, professional **tool aesthetic** — not Tailwind/React gloss. The earthy/olive design system (§7.2) is reproduced via design tokens; expectations are set accordingly. Building large data tables in egui is real work; it is budgeted once, in Phase 1, as a reusable component.

---

## 2. Method: specification-driven rebuild (no code translation)

| Source | Role in the rebuild |
|---|---|
| `PROJECT_REPORT.md` | **Behaviour specification.** Feature inventory (§3 of the report) becomes `docs/inventory.md`; documented bugs (§19) become the anti-pattern register (§3 below). |
| Legacy backend test suite (485 tests) | **Behaviour oracles.** Test *scenarios* (names + assertions described in the report) are transcribed into acceptance criteria attached to inventory rows. **Zero Python code is translated or copied.** |
| Legacy UI | UX reference only (flows, screen inventory). No React patterns carried over. |
| `ARTCRAFT_INVESTIGATION.md` | Process/methodology source (AGENTS.md structure, task loop, gates, ratchets, agent-drivable design). |
| `docs/DESIGN.md` (legacy) | Palette/shape/motion tokens → re-expressed as egui design tokens (§7.2). |

---

## 3. Anti-pattern register (legacy lessons → enforced countermeasures)

| Legacy defect (PROJECT_REPORT §19–20) | Countermeasure in MiData Native | Enforced by |
|---|---|---|
| Flattening ignores `record_date` → "current value" arbitrary | Latest-value semantics defined: max `recorded_on`, tie-break max id (§5.4) | store tests + oracle scenarios |
| N+1 entity listing (10k animals → 10k queries) | Single window-function query flattens the whole list (§5.4) | perf budget test on 10k-animal fixture (§12) |
| Dead schema (`pending_changes` unused) & phantom deps (`alembic`) | "No dead anything" rule: every table/flag/dep is used; deprecated code deleted immediately | `cargo machete` (unused deps) + review rule in AGENTS.md |
| Half-implemented validation rules (inert `rule_type`s) | `rule_type` is a Rust enum; every variant implemented and tested; no inert options exist | exhaustive match (compiler) + tests |
| Layering leaks (business logic in controllers) | Layered crates L0–L5; UI/CLI may not contain domain logic | `cargo xtask check-layers` (CI gate) |
| Docs drift (README ≠ code; 12 documented divergences) | Numbers are **generated** (`xtask stats` → `docs/scorecard.md`); decisions logged; docs carry review stamps | CI regenerates & diffs scorecard |
| Stale committed SPA bundle ("Rumetrics" title shipped) | No committed build artifacts, ever; generated docs are marked and CI-verified | `.gitignore` policy + CI |
| Secret crypto derived from hardcoded `SECRET_KEY`; API key in bundle | Secrets in **OS keyring** (`keyring` crate); no server, no embedded keys | architecture (nothing to embed) |
| In-memory AI confirmation tokens (lost on restart) | `pending_changes` persisted in SQLite with expiry + single-use (§9) | integration test: restart-survival |
| Unauthenticated LAN/ngrok exposure; invalid CORS | Removed entirely (no server). Future remote access goes through the documented Phase-7 strategy with auth designed in | architecture |
| Monolith files (`statistical_engine.py` 1,720 lines; `ChatStream.tsx` 897) | File budget: soft cap 500 lines, alarm 800; "new area = new file"; no `utils.rs` dumps | `cargo xtask file-budget` ratchet |
| Mixed/stale lockfiles, deps mixing prod/test/desktop | Single `Cargo.lock`; `[workspace.dependencies]` central list; dev-deps separated | workspace policy + cargo-deny |
| Theme/storage key mismatch (`rumetrics-ui-theme` vs `midata-ui-theme`) | One naming convention; all persistence keys defined in a single constants module | code review + grep gate (`rumetrics` = 0 hits) |
| Zero tags, no CI, red lint tolerated | CI from Phase 0 on Linux+Windows; clippy `-D warnings`; **every release is tagged** | branch protection + release checklist (§13) |
| Agent context exhaustion as codebase grows | Context-economy system: boundaries=compression, crate READMEs, orientation table, devlog continuity, ≤5-files-per-task rule | §11 process + file budget |

---

## 4. Target architecture

### 4.1 Workspace layout

```text
midata/
├── AGENTS.md                  # agent constitution: iron rules + orientation table
├── ROADMAP.md                 # one page: stage, headline numbers, current focus (stamped)
├── docs/
│   ├── architecture.md        # crate DAG, layers, engine/UI seam, data model summary
│   ├── development.md         # build/run/test/drive-app, faster loops, snapshots
│   ├── contributing.md        # task loop, gates, add-a-command/add-a-screen checklists
│   ├── gaps.md                # ranked work list (the queue agents pick from)
│   ├── inventory.md           # feature coverage vs PROJECT_REPORT (ids: HM-*, DE-*, AN-*, …)
│   ├── scorecard.md           # GENERATED by `xtask stats` — never hand-edited
│   ├── decisions.md           # one paragraph per architectural decision (ADR-lite)
│   ├── ui-design.md           # design tokens, themes, widget usage
│   ├── i18n.md                # TSV format, key naming, coverage rule
│   ├── testing.md             # oracle mapping, fixtures, snapshots, fuzz
│   ├── legacy-spec/PROJECT_REPORT.md  # legacy behaviour spec (reference ONLY — never implementation source)
│   └── mobile-access-strategy.md      # Phase 7: documented future feature (C8)
├── log/devlog.md              # terse entry after every task (external memory)
├── plan/                      # local, gitignored: research, estimates, execution notes
├── crates/                    # born in the phase that first uses them — never earlier
│   ├── midata-domain/         # L0 pure types & value semantics — zero I/O; deps: serde/chrono/thiserror   [P0]
│   ├── midata-i18n/           # L0 TSV catalogues (en source, fr), t!() lookup, coverage data — pure;
│   │                          #    shared by app AND cli                                                   [P0]
│   ├── midata-store/          # L1 rusqlite: engine (WAL, FK ON), migrations (user_version), repositories,
│   │                          #    canonical latest-value/flatten queries, validated-query execution       [P0]
│   ├── midata-analytics/      # L2 statistics, studies, dashboard hydration, pedigree graph — single engine [P3]
│   ├── midata-queryguard/     # L2 SQL validation lab (sqlparser AST): SELECT-only, column whitelist,
│   │                          #    caps — pure (SQL + whitelist in); shared by AI and widget commands      [P5]
│   ├── midata-exchange/       # L2 file exchange: xlsx/csv read (calamine), xlsx write (rust_xlsxwriter),
│   │                          #    PDF render — heavy format deps isolated here                            [P3]
│   ├── midata-ops/            # L2 domain workflows: birth event, onboarding merge, pack install/uninstall,
│   │                          #    import pipeline, backup ops, seed generation (keeps commands thin)      [P0]
│   ├── midata-engine/         # L3 Session + Command Registry (thin dispatch into ops/store/analytics)
│   │                          #    + audit hooks + undo stack                                              [P0]
│   ├── midata-ai/             # L4 LLM client (SSE), tool loop over the registry — the ONLY crate
│   │                          #    allowed tokio/reqwest                                                   [P5]
│   ├── midata-app/            # L5 egui shell: screens (thin), widget kit, serde app state, theme tokens;
│   │                          #    depends ONLY on domain + engine + i18n (+ egui stack)                   [P0]
│   └── midata-testkit/        # dev-only shared fixtures: temp-DB harness, demo-seed builders,
│                              #    snapshot helpers, command-test harness                                  [P0]
├── apps/
│   ├── midata/                # desktop binary (eframe/wgpu) — composition root; control server (P6) lives here
│   └── midata-cli/            # headless binary: commands | run | seed | import | export | backup | restore |
│                              #    stats | mcp (P6)
└── xtask/                     # check-layers | file-budget | ci | stats | i18n-coverage | bench |
                               #    new-command | new-screen | seed
```

### 4.2 Layer rules (enforced by `cargo xtask check-layers`)

- Dependencies form a **registered DAG** — not a strict chain — enforced by `cargo xtask check-layers`: every crate and every allowed edge (including intra-layer edges such as `ops → analytics/exchange/store`) is declared in `xtask/src/layers.rs`. Spine: `domain, i18n (L0) ← store (L1) ← {analytics, queryguard, exchange} (L2) ← ops (L2) ← engine (L3) ← ai (L4) ← app (L5)`. `apps/*` may depend on anything; nothing else may depend on `app`/`apps`; `midata-testkit` is a dev-dependency only.
- **`midata-app` depends only on `midata-domain`, `midata-engine`, `midata-i18n`** (+ the egui stack). All data reaches the UI as `CommandOutput` JSON, typed at the edges — "thin UI" is a Cargo fact, not a convention.
- **Nothing below `midata-app` may import `egui`, `eframe`, `egui_plot`, `egui_extras`, `winit` or `rfd`.**
- **No crate before the phase that first uses it** (birth phases annotated in §4.1) — an empty crate is dead schema by another name.
- One-way rule inside `midata-app`: `widgets/` never imports from `screens/` — keeps the future `midata-widgets` graduation mechanical (§4.8).
- Each crate carries a `README.md` documenting its **public API** (the interface an agent reads instead of the implementation); kept current with every public-API change.

### 4.3 The Command Registry (core of the agent-first principle)

```text
Command {
  id:          "animal.create"                 // lowercase dot-notation, stable forever
  title_key:   "cmd.animal.create.title"       // i18n key (EN/FR)
  params_doc:  {"custom_id":"string", "species_id":"int", "sex":"Male|Female|null",
                "birth_date":"YYYY-MM-DD|null", "sire_id":"int|null", "dam_id":"int|null",
                "values":"{field_key: value}"}  // what agents read via `commands`
  enabled:     |ctx| -> bool                    // greys UI entry points when false
  run:         |ctx, params| -> Result<CommandOutput, CommandError>
               // CommandOutput: serde_json::Value + optional user-facing summary i18n key
               // records exactly one audit_log row; invertible commands push an undo step
}
```

- Registry lives in `midata-engine`; command implementations live in **per-area modules** (`engine/src/animal_cmds.rs`, `observation_cmds.rs`, `field_cmds.rs`, `mode_cmds.rs`, `analytics_cmds.rs`, `data_cmds.rs`, `system_cmds.rs`, `ai_cmds.rs`…), each exposing `specs()`, merged in one `commands.rs` list. **Shared files never grow** (ArtCraft pattern).
- `run` never panics on any params/state (§11.2); returns typed `CommandError` with an i18n message key.
- UI screens, CLI verbs, AI tools, and the future control channel all dispatch **by id** — one behaviour, one implementation, four surfaces.

### 4.4 Concurrency model

- **Everything is synchronous** in domain/store/analytics/engine/app. No async runtime leaks into the core.
- Long work (imports, heavy analytics, AI streaming) runs on **std background threads**; results flow to the UI via `crossbeam-channel`. The UI polls channels each frame (egui `repaint` request).
- `tokio` exists **only inside `midata-ai`** (isolated runtime on its worker thread) for `reqwest`/SSE.
- DB access: one `Store` handle wrapping `Mutex<rusqlite::Connection>` (WAL). Single-user app; contention is negligible; AI worker and UI share the handle safely.

### 4.5 Storage & configuration locations

| What | Where |
|---|---|
| Database | platform data dir via `directories` crate: Linux `~/.local/share/midata/midata.db`, Windows `%APPDATA%\midata\midata.db`, macOS `~/Library/Application Support/midata/midata.db` |
| Portable mode | env `MIDATA_PORTABLE=1` (or flag) → `midata_data/` next to the executable (farm-USB friendly) |
| Exports / backups | `…/midata/exports/`, `…/midata/backups/` (rotation: newest 10) |
| Logs | `…/midata/logs/` (`tracing` + file appender, rotated) |
| App settings | `settings` table (K/V JSON) — language, theme, currency, mode prefs, AI provider config |
| Secrets (LLM API key) | **OS keyring** (`keyring` crate). Never in DB, never derived from a hardcoded key |

### 4.6 Errors, time, money

- Errors: `thiserror` typed errors per crate; `anyhow` only in binaries. User-facing messages are i18n keys + params, never raw exception strings.
- Time: **all dates/times via `chrono`** (workspace dep). No hand-rolled calendar math anywhere (ArtCraft golden rule adopted verbatim). Dates stored ISO-8601 text; `recorded_on` is a date.
- Money: **integer minor units** (`i64`) + currency code from settings (default `TND`; selectable TND/USD/EUR in v1). Never floats for money.

### 4.7 Dependency shortlist (full table in Appendix A)

`rusqlite` (bundled) · `serde`/`serde_json` · `chrono` · `thiserror`/`anyhow` · `egui`/`eframe`/`egui_plot`/`egui_extras`/`egui_kittest`(dev) · `crossbeam-channel` · `directories` · `keyring` · `evalexpr` (in `midata-ops`) · `calamine` + `csv` + `rust_xlsxwriter` + `printpdf`|`genpdf` (Phase 3, `midata-exchange` only) · `tracing` · `sqlparser` (Phase 5, `midata-queryguard` only) · `reqwest` + `reqwest-eventsource` + `tokio` (Phase 5, `midata-ai` only) · dev-tools: `cargo-deny` (licence allowlist), `cargo-machete` (unused deps), `cargo-nextest`.
**Explicitly not used:** DuckDB, polars, any webview/Tauri/Electron, any ORM. (Single-engine analytics decision — §8.)

### 4.8 Planned crate graduations (split triggers, not dates)

| Graduation | Trigger (measurable) |
|---|---|
| `midata-app` → extract `midata-widgets` | ≥2 agents working on UI in parallel, **or** `widgets/` exceeds ~5k lines, **or** kit API stable while screens churn |
| Control-channel/MCP protocol types → small `midata-control` crate | protocol types + handlers outgrow the file budget inside `engine` (P6); the TCP server itself lives in `apps/midata`, the MCP wrapper in `midata-cli` |
| Any `<area>_cmds.rs` outgrows the budget | split **files/modules inside `engine`** (new area = new file) — crate splits are the last resort |
| Post-v1 phone/web companion | reuses L0–L4 wholesale (UI-agnostic by design); the companion is a new shell, not a fork |

Graduations are owner-approved decisions (recorded in `docs/decisions.md`); a split is mechanical — no behaviour change in the same commit.

---

## 5. Data model v1 (fresh schema, no migration from legacy)

### 5.1 Design principles

1. **Identity is structural.** Universal facts about every animal are real columns with CHECK/FK constraints and Rust enums — not dynamic fields (owner decision 2026-10-27, "full promotion").
2. **EAV for everything else.** Farm-specific attributes remain fully dynamic field definitions + dated observations. This is the product bet; it is preserved intact.
3. **Cardinality is explicit.** Fields are `single` (one value per animal — upsert semantics, e.g. breed notes, ear-tag colour) or `series` (time-series, e.g. weight, milk yield).
4. **Latest-value semantics are defined and tested** (§5.4) — the legacy "arbitrary current value" bug is structurally impossible.
5. **No dead schema.** Every table is used by shipped code in the phase that introduces it. `pending_changes` arrives in Phase 5 **already wired**.
6. **Migrations:** `PRAGMA user_version` + ordered forward-only migration functions in `midata-store`; idempotency-tested; no table is created before the phase that uses it.

### 5.2 Table catalogue (spec-level)

```sql
-- v1 (Phase 0) --------------------------------------------------------------
CREATE TABLE species (
  id INTEGER PRIMARY KEY,
  key TEXT NOT NULL UNIQUE,             -- machine key: 'cow','sheep','goat', user-added 'camel'…
  label_en TEXT NOT NULL, label_fr TEXT NOT NULL,
  is_builtin INTEGER NOT NULL DEFAULT 0,
  gestation_days INTEGER,               -- per-species default, editable (data, not code)
  icon TEXT,                            -- icon/font-glyph name
  sort_order INTEGER NOT NULL DEFAULT 100,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL
);

CREATE TABLE entities (
  id INTEGER PRIMARY KEY,
  custom_id TEXT NOT NULL UNIQUE,       -- farm tag/serial
  species_id INTEGER NOT NULL REFERENCES species(id),
  sex TEXT CHECK (sex IN ('Male','Female')),          -- NULL = not recorded
  breed TEXT,                           -- free text; per-species suggestion lists in settings
  birth_date TEXT,                      -- ISO date, NULL allowed (approximate/unknown)
  sire_id INTEGER REFERENCES entities(id) ON DELETE SET NULL,
  dam_id  INTEGER REFERENCES entities(id) ON DELETE SET NULL,
  sire_tag TEXT, dam_tag TEXT,          -- phantom-parent external tags when sire/dam_id IS NULL
  status TEXT NOT NULL DEFAULT 'Alive' CHECK (status IN ('Alive','Sold','Dead')),
  exit_date TEXT, exit_reason TEXT,     -- set when status != Alive
  notes TEXT,
  created_at TEXT NOT NULL, updated_at TEXT NOT NULL
);
CREATE INDEX idx_entities_species ON entities(species_id);
CREATE INDEX idx_entities_status  ON entities(status);

CREATE TABLE field_definitions (
  id INTEGER PRIMARY KEY,
  key TEXT NOT NULL UNIQUE,
  label_en TEXT NOT NULL, label_fr TEXT NOT NULL,
  category TEXT NOT NULL,               -- e.g. Health, Production, Reproduction, Finance
  data_type TEXT NOT NULL CHECK (data_type IN ('number','text','select','boolean','multi_select','date')),
  cardinality TEXT NOT NULL DEFAULT 'series' CHECK (cardinality IN ('single','series')),
  unit TEXT,                            -- 'kg','L','days'…
  options_json TEXT,                    -- {"choices":[…]} for select/multi_select; breed suggestions etc.
  purpose TEXT,                         -- semantic tag replacing legacy 'role' for non-identity concepts:
                                        -- 'production_milk','withdrawal_days','mating_date','pregnancy_status',
                                        -- 'gestation_period','expected_birth_date','birth_date_evt','offspring_count','calving_ease'…
  validation_json TEXT,                 -- {"min":…,"max":…} inline numeric bounds
  is_active INTEGER NOT NULL DEFAULT 1, -- soft delete
  sort_order INTEGER NOT NULL DEFAULT 100,
  created_at TEXT NOT NULL, updated_at TEXT NOT NULL
);
CREATE INDEX idx_fields_category ON field_definitions(category);

CREATE TABLE observations (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
  field_id  INTEGER NOT NULL REFERENCES field_definitions(id),
  recorded_on TEXT NOT NULL,            -- ISO date
  value_num REAL,                       -- number & boolean (0/1)
  value_text TEXT,                      -- text, select, multi_select (JSON array), date (ISO)
  source TEXT NOT NULL DEFAULT 'manual' CHECK (source IN ('manual','import','ai','computed','seed')),
  created_at TEXT NOT NULL
);
CREATE UNIQUE INDEX uq_obs_e_f_d ON observations(entity_id, field_id, recorded_on);
CREATE INDEX idx_obs_latest ON observations(entity_id, field_id, recorded_on DESC);

CREATE TABLE modes (
  id INTEGER PRIMARY KEY,
  name_en TEXT NOT NULL, name_fr TEXT NOT NULL,
  icon TEXT, is_system INTEGER NOT NULL DEFAULT 0,   -- system mode ("General Husbandry") shows all fields, protected
  is_active INTEGER NOT NULL DEFAULT 1, sort_order INTEGER NOT NULL DEFAULT 100,
  created_at TEXT NOT NULL
);
CREATE TABLE mode_fields (
  mode_id INTEGER NOT NULL REFERENCES modes(id) ON DELETE CASCADE,
  field_id INTEGER NOT NULL REFERENCES field_definitions(id) ON DELETE CASCADE,
  sort_order INTEGER NOT NULL DEFAULT 100,
  PRIMARY KEY (mode_id, field_id)
);

CREATE TABLE computed_fields (
  id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE,
  expression TEXT NOT NULL,             -- {{field_key}} placeholders, evaluated by evalexpr sandbox
  unit TEXT, category TEXT, is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL, updated_at TEXT NOT NULL
);

CREATE TABLE validation_rules (
  id INTEGER PRIMARY KEY,
  scope TEXT NOT NULL DEFAULT 'entity',         -- 'entity' | 'field'
  field_key TEXT,                               -- when scope='field'
  rule_type TEXT NOT NULL CHECK (rule_type IN
    ('min','max','range','required','date_expiry','date_expression')),  -- ALL variants implemented
  rule_json TEXT NOT NULL,                      -- {"min":…,"max":…,"days":…,"expr":"date_diff({{treatment_date}}, current_date) < {{withdrawal_days}}"}
  severity TEXT NOT NULL CHECK (severity IN ('info','warning','critical')),
  message_key TEXT NOT NULL,                    -- i18n template key
  is_active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE widgets (
  id INTEGER PRIMARY KEY,
  widget_type TEXT NOT NULL,            -- 'kpi','line','bar','table','alert_board','progress_list','donut','profitability'
  title_key TEXT NOT NULL,              -- i18n key or user text
  source TEXT NOT NULL DEFAULT 'builtin' CHECK (source IN ('builtin','user','ai','pack')),
  data_config_json TEXT NOT NULL,       -- query-as-data-source is a first-class type from day one (owner decision)
  viz_config_json TEXT,
  mode_id INTEGER REFERENCES modes(id) ON DELETE SET NULL,
  sort_order INTEGER NOT NULL DEFAULT 100, is_active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE settings (
  key TEXT PRIMARY KEY, value_json TEXT NOT NULL, updated_at TEXT NOT NULL
);

CREATE TABLE audit_log (
  id INTEGER PRIMARY KEY,
  command_id TEXT NOT NULL, actor TEXT NOT NULL,   -- 'user' | 'ai' | 'cli' | 'seed'
  entity_id INTEGER, field_key TEXT,
  old_value TEXT, new_value TEXT,
  batch_token TEXT, created_at TEXT NOT NULL
);
CREATE INDEX idx_audit_entity ON audit_log(entity_id);

-- Phase 5 additions ---------------------------------------------------------
CREATE TABLE conversations (id INTEGER PRIMARY KEY, title TEXT, message_count INTEGER NOT NULL DEFAULT 0,
                            created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
CREATE TABLE messages (id INTEGER PRIMARY KEY, conversation_id INTEGER NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
                       role TEXT NOT NULL, content TEXT NOT NULL,
                       tool_calls_json TEXT, tool_results_json TEXT, created_at TEXT NOT NULL);
CREATE TABLE pending_changes (id INTEGER PRIMARY KEY, token TEXT NOT NULL UNIQUE,
                              kind TEXT NOT NULL CHECK (kind IN ('modify','batch')),
                              description TEXT NOT NULL, operations_json TEXT NOT NULL,
                              mode TEXT NOT NULL CHECK (mode IN ('transactional','per_item')),
                              consumed INTEGER NOT NULL DEFAULT 0,
                              created_at TEXT NOT NULL, expires_at TEXT NOT NULL);  -- PERSISTED (legacy bug fixed)
```

### 5.3 Value encoding rules

| `data_type` | Column | Format |
|---|---|---|
| number | `value_num` | REAL |
| boolean | `value_num` | 0.0 / 1.0 |
| text / select | `value_text` | plain string (select ∈ `options_json.choices`) |
| multi_select | `value_text` | JSON array string |
| date | `value_text` | ISO-8601 `YYYY-MM-DD` |

Exactly one of `value_num`/`value_text` is non-NULL per row, matching the field's `data_type` (enforced in domain constructors + tested). `cardinality='single'` fields keep **at most one row per (entity, field)** — service-layer upsert invariant, covered by tests.

### 5.4 Latest-value semantics (the legacy P0 fix)

"Current value" of a field for an entity ≔ the observation with **max `recorded_on`**, tie-break **max `id`**. The whole-list flatten uses **one** query (no N+1):

```sql
SELECT entity_id, field_id, value_num, value_text FROM (
  SELECT o.*, ROW_NUMBER() OVER (
    PARTITION BY o.entity_id, o.field_id
    ORDER BY o.recorded_on DESC, o.id DESC) AS rn
  FROM observations o
) WHERE rn = 1;
```

This "logical flattened view" (identity columns + latest values for active fields) is the **single source** for: animal list, exports, AI query whitelist surface, dashboard hydration, analytics filtering. Perf budget: flatten 10k animals < 500 ms on CI hardware (§12).

### 5.5 Invariants & enforcement

- Sire must be Male, dam must be Female — enforced at `animal.create`/`animal.update`/`animal.recordBirth` command level (typed enums; invalid states unrepresentable) with graceful `CommandError`.
- Duplicate `custom_id` rejected (UNIQUE + friendly error i18n key).
- Phantom parents: `sire_id/dam_id = NULL` **plus** `sire_tag/dam_tag` free text; pedigree renders them distinctly.
- Deleting an entity cascades observations; deleting a field is **soft** (`is_active=0`); modes/extensions soft-toggle.
- Every invariant has an integration test in `midata-store`/`midata-engine`.

### 5.6 Species registry & presets policy

- Presets (seeded at onboarding, `is_builtin=1`): **Cow, Sheep, Goat** — each with a curated default field pack (identity/growth/dairy/health/reproduction/nutrition categories, condensed from the legacy onboarding bundles) and default `gestation_days` (data column, editable in Settings).
- The user adds any species (Camel, Horse, Dog, Rabbit, …) via Settings → Species; new species get the generic default field pack; extension packs may target species by key.
- **No code path enumerates species.** All species-dependent behaviour reads the registry (gestation defaults, field packs, analytics filters). Adding Camel = zero code changes.
- **Pig policy (owner directive, 2026-10-27):** the species is never included in presets, seed/demo data, tests, documentation examples, or screenshots. There is **no technical block** against a user manually adding it — the product stays neutral; the codebase simply never mentions it.

### 5.7 Seed / demo data

Deterministic generator (`xtask` or CLI verb `midata seed --demo [--force]`): ~50 animals across the three presets, ~2,000 observations (lactation curves, ADG series, health chains), 5-generation pedigree incl. 2 phantom parents, sample modes/widgets/rules/computed fields. Seeded rows carry `source='seed'`. Idempotent; `--force` rebuilds. Used by snapshot tests and development.

---

## 6. Feature inventory & triage

### 6.1 Tracking model

`docs/inventory.md` (created in Phase 0, task P0-D3) transcribes PROJECT_REPORT §3 into **one row per testable behaviour**, grouped in blocks with stable ids:

| Block | Ids | Legacy source |
|---|---|---|
| Herd management | HM-* | report §3.1 |
| Data entry | DE-* | §3.2 (batch, mass, dynamic forms, import) |
| Organization & config | OR-* | §3.3 (fields, modes, computed, rules, packs, templates) |
| Analytics | AN-* | §3.4 (stats, studio, deep dive, correlation, profitability) |
| Dashboard | DB-* | §3.4 (widget types) |
| Pedigree & reproduction | PD-* | §3.4 + §11.3 |
| Import/Export | IX-* | §3.2/§11.6 |
| System (backup/settings/i18n/theme) | SY-* | §3.6/§15 |
| AI assistant | AI-* | §3.5 |
| Extensions/packs | EX-* | §3.3 |
| Onboarding | OB-* | §11.1 |

Row columns: `id · behaviour · status(planned/partial/done) · phase · evidence(test name / snapshot path)`. This is our parity checklist: **coverage % is a headline ROADMAP number** and a ratchet (never decreases). Legacy oracle scenarios (from the 485-test suite described in the report) attach to rows as acceptance criteria.

### 6.2 Triage decisions (frozen; changes require a §15 decision-log entry)

**DROP (not in MiData Native at all):** LAN sharing server · ngrok tunnel · QR codes · system tray · PyInstaller packaging · browser SPA & static bundling · CORS/API-key machinery · DuckDB second engine · "Rumetrics" naming · onboarding as a 5-step wizard · pig presets (policy C4/§5.6).

**DEFER (documented, post-v1-stable):** mobile/remote access (Phase-7 strategy doc, §13.3) · MCP server + control channel implementation (seams from day one; built Phase 6) · macOS packaging (Phase 7 optional) · multi-user/sync.

**REDESIGN (kept as capabilities, rebuilt per this plan):** EAV schema (§5) · flattening (§5.4) · validation rules (all types implemented) · onboarding (1 step) · widgets (curated → dynamic in P5–6, query-source first-class) · extensions (local JSON packs, atomic) · templates (config-only export/import) · AI assistant (persisted tokens, keyring secrets, command-registry tools) · profitability (TND default, minor units) · printing (→ PDF export of animal cards/reports).

**KEEP (1:1 as behaviours):** dynamic fields · modes + protected system mode · batch/mass entry · analytics suite (trend/distribution/correlation/comparison/deep-dive/descriptive) · server-config-driven dashboard widgets · pedigree graph (BFS, phantoms, cycle-safe, 500-node cap) · atomic birth trigger · computed fields (formula sandbox) · Excel/CSV import with mapping + skip/update strategies · XLSX exports · backup/verify/restore/rotate-10 · conversation persistence · audit trail · demo seed · light/dark theme.

---

## 7. UI architecture & design system

### 7.1 Navigation & screens

Central `Screen` enum router (no URLs). Core nav (always visible): **Dashboard · Animals · Entry · Analytics · Assistant**(from P5). Advanced surfaces (Packs, Settings, mode switcher prominence, Deep Analysis, Pedigree) gated by **Developer Mode** (persisted `midata_developer_mode` flag) — ADR-004 progressive-disclosure spirit, implemented for real this time (Phase 6; flag plumbing from Phase 1).

Screens (one file each under `midata-app/src/screens/`, thin: dispatch commands, render state): `onboarding`, `dashboard`, `animal_list`, `animal_detail`, `entry` (single/batch/mass tabs), `analytics_studio`, `deep_analysis`, `pedigree`, `packs`, `settings` (tabs: Fields, Species, Modes, Formulas, Rules, AI Provider, Data & Backup, Appearance & Language, Developer), `assistant` (docked side panel, pinnable).

### 7.2 Design tokens & theme

- Token module `midata-app/src/theme.rs`: the legacy earthy/olive palette re-expressed as egui `Visuals` — light: warm sand bg `#f5f0ea`, olive primary `#759048`, cards white, warm-brown shadows; dark: `#261d14` family. Full earth/olive scales, radii (cards 16–28px, buttons 12px), spacing scale, semantic success/warning/destructive.
- **No hard-coded colors/radii anywhere** — everything via tokens (clippy-reviewable rule in AGENTS.md).
- Light/dark themes; persistence key `midata_theme` (single constants module owns all keys — legacy mismatch bug class eliminated).
- Typography: Inter (bundled, OFL-attributed) + tabular numerals for data columns.

### 7.3 Core widget kit (built once, Phase 1)

- **DataTable** (on `egui_extras::TableBuilder`): sortable columns, filter bar, pagination (default 25/50/100), multi-select, column visibility from active mode fields + identity columns, row context menu, keyboard nav. This is the highest-effort component; budgeted explicitly.
- **DynamicFieldRenderer**: renders an input per `field_definitions` row (number/text/select/boolean/multi_select/date via `egui_extras` DatePicker), driven by mode + cardinality.
- Modal dialogs (focus trap, ESC), toasts, confirm dialogs (destructive ops), empty states (audited per screen), skeletons.
- Dashboard grid: bento-style responsive `egui::Grid`/`horizontal` layout; widget cards hydrate from `widgets.data_config_json`.
- Charts: `egui_plot` (line, bar, histogram, scatter, box); **donut/pie via custom painter** (small effort, Phase 3; if it slips, ship horizontal-bar equivalent and log to gaps).

### 7.4 i18n (EN source + FR)

- TSV catalogues `midata-app/src/i18n/en.tsv`, `fr.tsv` (ArtCraft pattern); lookup via `t!("key")`; **every** user-facing string — including `CommandError` messages and insights templates — goes through i18n from day one (avoiding ArtCraft's "engine messages still English" partial state).
- `cargo xtask i18n-coverage` measures key coverage; 100% FR is a Phase-6 exit gate; coverage is a ratchet.
- Language setting: `settings.language ∈ {en, fr}`, live-switchable.

### 7.5 UI verification ("Look at the PNG")

- `egui_kittest`-based offscreen snapshot tests per screen/state (`cargo test -p midata-app` renders headless, compares golden PNGs; goldens committed, regenerated intentionally).
- Any UI task's Definition of Done includes: snapshot updated + **agent viewed the rendered PNG** + before/after noted in the task record.
- Demo imagery: seeded demo data only; public-domain art if ever needed; never personal data.

---

## 8. Analytics (single engine)

- One code path in `midata-analytics` (pure functions over rows fetched by store) serving UI, CLI, widgets, and AI tools — **no second engine, no DuckDB** (legacy dual-engine divergence risk eliminated).
- Capabilities: descriptive stats (count/mean/median/std/min/max/percentiles) · trend (group-by-date means + linear regression slope/intercept/r²) · distribution (histogram bins, single-value edge case) · correlation (Pearson per group + matrix across numeric fields) · comparison (group stats) · deep-dive single-variable report (overview, histogram, time trend, by-breed, by-sex, top/bottom-5 outliers) · dashboard KPI hydration · profitability (production field resolved via `purpose='production_milk'`-style tags or explicit choice; prices/feed costs in minor units; currency from settings, **default TND**) · pedigree graph (bidirectional BFS from root, generations 1–10 default 3, visited-set cycle protection, phantom nodes from `sire_tag/dam_tag`, global cap 500 nodes, sex-consistency warnings).
- Categorical fields (select/text/boolean/multi_select) route to count-based distributions, never numeric math.
- Insights are **deterministic templates** (i18n keys), not LLM output — offline and reproducible.
- Determinism: any randomness is seeded. Heavy loops: rayon where measured-useful; perf budgets recorded in `docs/scorecard.md`.
- Legacy oracle scenarios (30 analytics tests, 26 pedigree tests, etc. per report §17.2) become the acceptance suite.

---

## 9. AI assistant subsystem (Phase 5)

| Aspect | Design |
|---|---|
| Provider | OpenAI-compatible endpoint (LM Studio default `http://localhost:1234/v1`, Ollama, any). Settings: base URL, model, temperature; **API key in OS keyring**; connectivity-test command. |
| Transport | SSE streaming via `reqwest-eventsource` on an isolated tokio runtime inside `midata-ai`'s worker thread; events → crossbeam channel → UI (chunk, tool_call, tool_status, tool_result, confirmation, done, error — legacy event set preserved). |
| Prompts | System instructions **in English**; **no forced response language**; one stabilizing line: "Respond in the language of the user's latest message." (owner decision 2026-10-27). |
| Tools | Thin wrappers over the **command registry** + a read-only query tool. Read tools execute inline; write tools **stage** (below). Tool list is generated from registry metadata — new commands are automatically offerable. |
| Query sandbox | Lives in **`midata-queryguard`** (L2, pure: SQL + whitelist in) — **below** the engine so both the AI tools (P5) and the dynamic-widget commands (P6) share one validator; never duplicated, never housed inside `midata-ai` (engine cannot depend upward). `sqlparser` AST validation: exactly one statement, SELECT/WITH only, columns whitelisted against the logical flattened view (§5.4) + identity columns, hard LIMIT (5,000 rows / 50 KB result / 30 raw rows before summarization / 500 aggregated / 200-char strings / 32k-token conversation budget with 24k warning — legacy governance defaults as starting point). Execution of validated SQL goes through `midata-store`. Injection-attempt scenarios from legacy `test_tool_query` (71 tests) are the acceptance oracle. |
| Staged writes | `modify`/`batch` plans persisted in **`pending_changes`** (SQLite) with single-use token + 5-minute expiry → confirmation card in UI → commit command executes transactionally or per-item → `audit_log` rows carry `batch_token`. **Restart-survival integration test is an exit gate** (legacy in-memory-token bug fixed). |
| Conversations | Persisted (`conversations`/`messages`), list/rename/delete, context window = last 20 messages + summarization. |
| Widget generation | AI proposes widget config (type + validated query + viz) → same validation lab → user preview → saved as `source='ai'` widget. Dynamic (user-built) widgets unlock in Phase 6 over the identical lab (owner decision: middle path). |
| UI | Docked assistant panel: streaming markdown, per-tool expandable blocks with status badges, confirmation cards, conversation list. |

---

## 10. Agent-facing surfaces (timeline)

| Surface | What | Phase |
|---|---|---|
| egui UI | human surface; thin dispatcher over commands | 0–7 |
| `midata-cli` | `commands` (list+docs), `run <id> --params <json>`, `seed`, `import`, `export`, `backup`, `restore`, `stats` | skeleton P1, grows P3+ |
| Audit log | every command execution: actor, params digest, outcome — human accountability + agent observability | P1 |
| Control channel | loopback TCP JSON-lines, token-file auth (ArtCraft protocol shape): `engine.execute`, `engine.commands`, `app.inspect` (serde state), `ui.screenshot` | P6 |
| MCP server | `midata-cli mcp --bridge <port>` wrapping the same protocol — external agents (Claude Desktop, Cursor…) drive MiData | P6 |
| Mobile/remote | **documented strategy only** (§13.3) | P7 doc |

---

## 11. Engineering process (ArtCraft-derived, scaled to one owner + agents)

### 11.1 Artifact set

Created in Phase 0 and kept alive thereafter: `AGENTS.md` (≤8 KB, English, header credit: *"Structure adapted from storytold/photocraft AGENTS.md (MIT OR Apache-2.0)"*) · `ROADMAP.md` (headline numbers: inventory coverage %, screens done, tests count, budget status; stamped Last reviewed/updated/Change; includes an honest "Why not 1.0 yet" section) · `docs/*` (§4.1) · `log/devlog.md` · local gitignored `plan/`.

### 11.2 Golden rules (authoritative text lives in AGENTS.md; summary here)

1. **Never crash (outranks features).** No `unwrap/expect/panic!/unreachable!/todo!/unimplemented!` in non-test code — enforced by workspace lints `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]` + `clippy.toml` (tests exempt). **`unsafe_code = "forbid"` workspace-wide, no exceptions.** Input-derived numbers are hostile (`get()`, `checked_*`, allocation caps, char-boundary slicing). Bound recursion. Every crash fix ships a regression test.
2. **File budget.** Soft cap 500 lines/file, alarm 800; `xtask file-budget` lists violations; **the list may only shrink** (ratchet). New area = new file. No `utils.rs`/`helpers.rs` dumps.
3. **Everything is a command** (§4.3) with the "add a command" checklist in `docs/contributing.md`.
4. **Tests are the gate.** No change without tests; commands get graceful-failure tests (bad params → `Err`, never panic); a registry-wide adversarial fuzz test (`command_hunt`, mirroring `panic_hunt`) must stay green.
5. **All strings via i18n; all colors via tokens; all dates via chrono; all money as minor units.**
6. **No dead anything.** Unused tables/flags/deps/code deleted immediately; `cargo machete` in CI.
7. **Comments explain *why*.** Docs stamped; generated files marked GENERATED.
8. **Species/data policies:** open registry, no hardcoded species lists; pig policy per §5.6.

### 11.3 Task lifecycle (every task, every agent)

1. **Orient** — pick from `docs/gaps.md` (highest-ranked unclaimed); read only the docs+crate READMEs the orientation table points to (≤5 files + 2–3 docs rule).
2. **Acceptance test first** — write the failing test / snapshot defining "done" (oracle scenarios from `docs/inventory.md` rows where applicable).
3. **Implement** — smallest layer that fits; run crate tests while iterating.
4. **Verify** — full gate (§11.4) + for UI: render offscreen and **look at the PNG**.
5. **Record** — update inventory/gaps/scorecard rows with evidence + date; append terse `log/devlog.md` entry (what landed, numbers, still-open); commit with task id. *"Sessions can end abruptly — the devlog plus a green tree is how the next agent picks up. Keep the tree building at every step."*

**Definition of Done:** behaviour is a registered documented command (where user-visible) · tests incl. graceful-failure · wired to UI (+CLI where relevant) · snapshot reviewed for UI work · full gate green · docs/inventory/devlog updated.

### 11.4 Pre-completion gate (exact commands)

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run -p <touched crates>          # full: cargo nextest run --workspace
cargo xtask check-layers
cargo xtask file-budget
cargo xtask i18n-coverage                      # no regression
cargo test -p midata-app -- snapshot           # UI tasks: goldens updated & viewed
cargo test -p midata-engine --test command_hunt -- --ignored   # command tasks
cargo xtask bench --quick                      # hot-path tasks (flatten, dashboard hydrate)
cargo xtask stats                              # scorecard regenerated; commit it (CI diffs)
```

### 11.5 Autonomy & escalation

- **"Don't block on choices you can make yourself."** The agent decides everything the rules cover.
- **Ask the owner (closed list):** any DB schema migration · deleting/overwriting user data or fixtures · adding/removing a dependency (licence & maintenance check) · publishing, pushing to main, tagging releases · spending money · personal data anywhere · destructive git operations · anything the rules don't clearly cover — **"when in doubt, leave it out."**
- **Human-reserved:** overall planning, release cutting, signing, UAT sign-off, branding.

### 11.6 Parallelism rules (documented from day one, active whenever >1 agent)

Git worktree per agent · `CARGO_TARGET_DIR=target/agent-<name>` · **one owner per crate/module area** · new commands/screens in new modules, never growing shared files · shared files (`commands.rs` list, `lib.rs`, state struct) get small surgical edits, re-read before editing · `Cargo.toml` changes written atomically (temp file + `mv`) · never fix another agent's in-progress files — wait and retry · integration branch merges (`Integration: track tNN`) · delete finished agents' target dirs.

### 11.7 Estimation & calibration

Estimates are **agent-hours** (one agent, sequential, incl. tests). Initial numbers below; after Phase 1, calibrate from actuals (merged-task count × measured durations) and restate Phase 2+ estimates in `ROADMAP.md` — the ArtCraft calibration method.

### 11.8 Quality ratchets (monotonic, CI-enforced)

file-budget violations (↓ only) · inventory coverage % (↑) · FR i18n coverage (↑, 100% by P6) · test count (↑) · snapshot-covered screens (↑) · clippy warnings (=0 always) · perf budgets (met → `enforce=true`) · seed-oracle floors (e.g. "flattened latest-values match brute-force computation on 2,000-observation fixture" — never disabled).

---

## 12. Testing strategy

| Layer | What | Where |
|---|---|---|
| Unit | domain value rules, analytics math (known-value oracles), formula eval, pedigree layout math, i18n lookup | per-crate `#[cfg(test)]` |
| Integration | store repos + migrations idempotency (tempfile DBs) · **every command**: happy path, bad params, unauthorized state (graceful `Err`) · latest-value semantics vs brute-force · birth atomicity under injected failure · backup/restore round-trip · import skip/update strategies · pending-changes restart survival (P5) | crates' `tests/` |
| Fuzz | `command_hunt`: registry-wide adversarial params (empty/absurd/negative/malformed-UTF8 dates) → `Err`, never panic | `midata-engine` |
| UI snapshots | `egui_kittest` offscreen goldens per screen/state (seeded demo data, EN+FR spot checks) | `midata-app` |
| Behaviour oracles | legacy scenarios (485-test inventory from PROJECT_REPORT §17.2) mapped to `docs/inventory.md` rows as acceptance criteria — **scenarios, never code** | `docs/testing.md` mapping table |
| Perf budgets | flatten 10k animals < 500 ms · dashboard hydrate (10 widgets, 1k animals) < 300 ms · app cold start < 2 s · import 1k rows < 10 s — measured by `xtask bench`, baselines in `docs/scorecard.md` | xtask |
| Determinism | seeded demo data; seeded randomness in analytics | everywhere |

CI runs the whole suite on **ubuntu-latest + windows-latest** from Phase 0. Snapshot goldens run on Linux (canonical); Windows runs logic suites.

---

## 13. CI/CD, packaging, release

### 13.1 CI (Phase 0)

GitHub Actions: `fmt` · `clippy -D warnings` · `nextest` (matrix: ubuntu, windows) · `xtask check-layers` · `xtask file-budget` · `xtask i18n-coverage` (diff vs baseline) · `cargo deny check` (licence allowlist: MIT/Apache-2.0/BSD/ISC/Zlib/Unicode; anything else → escalation) · `cargo machete` · scorecard regeneration diff · snapshots (linux). Branch protection: red = no merge. Owner develops on **Linux**; Windows parity guaranteed by CI from day one (owner decision 2026-10-27).

### 13.2 Release & packaging (Phase 7)

Semver from `0.1.0`; **every release tagged** (legacy had zero tags). Pipeline: `cargo-dist` (candidate) or bespoke GH Actions: **Windows installer first** (owner's release priority; built on windows runner — no local Windows machine needed), Linux AppImage + .deb, macOS bundle optional/deferred. Signing = human-reserved task. Release checklist in `docs/development.md`; CHANGELOG.md kept per release.

### 13.3 Mobile/remote access — documented future feature (C8)

Phase 7 writes `docs/mobile-access-strategy.md` as a **contract for the future**, covering at least: (A) embedded loopback/LAN HTTP server exposing the **command registry** (axum) + read-only mobile web companion; (B) DB-file sync (Syncthing-style) + separate mobile client; (C) full web client as a separate product. Prerequisite seams already shipped: command registry (§4.3), serde state, control channel + MCP (P6), audit log. No v1 code is written for this; the doc records the recommendation and the auth model it must carry (the legacy no-auth LAN exposure is the cautionary tale).

### 13.4 Git workflow & traceability

**Remote:** private GitHub repository (owner creates it; going public is an owner decision at
release time — publishing is an escalation item). Default branch `main`, protected: **no direct
pushes** — agents push task branches and open PRs; the owner reviews and merges.

**Branch model (task-centric):**
- One short-lived branch per `docs/gaps.md` task: `t/<task-id>-<slug>` (e.g. `t/p0-1-workspace-ci`,
  `t/p1-3-animal-create`). Parallel agents: one git worktree per agent, one task branch each (AGENTS.md §6).
- Mid-task commits on the branch are **session checkpoints** (crash / context-limit recovery):
  commit early, commit often — every commit keeps the workspace building and the touched-crate
  gates green (**never commit red**).
- Merge policy: **squash-merge by the owner** → `main` holds exactly one green commit per closed
  task; the branch is deleted after merge and the PR page preserves the detail. Use `--no-ff`
  merge instead only when a task's intermediate commits have archaeology worth keeping (owner's call).

**Commit convention** (branch commits and main squash commits alike):

```
<type>(<area>): <imperative subject ≤72 chars> [<task-id>]

<body: what & why; numbers when perf-relevant; any deviation noted>

Refs: gaps <task-id> · inventory <row-ids>
Co-Authored-By: <agent/model identity> <noreply@…>
```

- `type` ∈ feat · fix · refactor · perf · test · docs · chore · ci · build · bootstrap · integration · revert.
  `area` = crate or surface (engine, store, app, ops, analytics, docs, ci…).
- The task id in the subject makes `git log --grep '\[P1-3\]'` a full task archaeology tool.
- **AI attribution is mandatory:** every agent-authored commit carries the `Co-Authored-By:` trailer
  naming the agent/model; owner commits are optionally GPG/SSH-signed. The devlog entry records
  anything the trailer cannot (session notes, model switches, retries).

**Gitignore (never committed):** `target/` · `plan/` · `*.db` · `midata_data/` · logs · secrets/tokens ·
large real-file corpora (pinned & fetched per `docs/testing.md`, if any).

**Committed — and travelling in the same commit as their motivating change:** `Cargo.lock` ·
snapshot goldens · generated docs (`docs/scorecard.md`, regenerated inventory stats — CI diffs them) ·
`log/devlog.md` (append-only shared memory) · test fixtures under `tests/`.

**Tags:** annotated (signed when a key is available): `vX.Y.Z` for every release — the legacy repo
shipped 80 commits with **zero tags**; not once here — plus internal milestone tags `m/phase-0` …
`m/phase-7` at each owner UAT pass, as named rollback points. Tag message = the phase exit-criteria summary.

**Traceability chain (the trackability guarantee):**
`gaps.md task-id → branch t/<id> → PR → one green main commit [<id>] → devlog entry (<id>) →
inventory-row evidence (test names / snapshot paths) → scorecard numbers.`
Every artifact links back to its origin; every task is reconstructable from git alone.


---

## 14. Phase roadmap

> Estimates are initial agent-hours (sequential, incl. tests); recalibrate after Phase 1 (§11.7). Each phase ends with: gate green on both OSes · snapshots updated · inventory/gaps/scorecard/devlog current · owner UAT checkpoint.

| Phase | Name | Estimate |
|---|---|---|
| 0 | Foundations | 40–80 h |
| 1 | Core herd | 80–150 h |
| 2 | Organization & rules | 40–80 h |
| 3 | Analytics, dashboard, data I/O | 100–180 h |
| 4 | Pedigree & reproduction | 60–120 h |
| 5 | AI assistant | 100–200 h |
| 6 | Packs, dynamic widgets, agent surfaces, polish | 60–120 h |
| 7 | Release & documentation | 30–60 h |
| **Total** | | **~510–990 agent-hours** |

### Phase 0 — Foundations
**Scope:** repo + workspace + CI (§13.1) · `AGENTS.md` + full `docs/` skeleton + `ROADMAP.md` v0 + devlog · `xtask` (check-layers, file-budget, ci, stats, new-command/new-screen templates, seed, bench skeleton, i18n-coverage) · `midata-domain` v0 (types, enums, Value rules, CommandError) · `midata-i18n` (TSV catalogues en/fr, `t!()` lookup, coverage data) · `midata-store` v0 (engine, PRAGMAs, migrations → full v1 schema §5.2, repos, latest-value queries) · `midata-ops` v0 (onboarding-merge + seed workflows) · `midata-engine` v0 (Command trait, Session, registry, audit hook, undo seam) · `midata-testkit` (temp-DB harness, demo-seed builders, snapshot + command-test helpers) · `midata-app` shell (eframe window, Screen router, nav, theme tokens light/dark, settings load/save — i18n via `midata-i18n`) · **1-step onboarding** (species preset choice + optional demo seed → creates species/fields/modes/system-mode/widgets) · `midata-cli` skeleton (`commands`) · `docs/inventory.md` v0 generated from PROJECT_REPORT §3 · `docs/gaps.md` v0 populated with Phase-1 tasks · seed generator (§5.7).
**Exit criteria:** `xtask ci` green on ubuntu+windows · app boots to dashboard with demo data · onboarding idempotent · migration idempotency tests · AGENTS.md ≤ 8 KB · 0 file-budget violations · snapshots: shell + onboarding · devlog active.

### Phase 1 — Core herd
**Scope:** reusable **DataTable** kit + DynamicFieldRenderer + modal/toast/confirm kit (§7.3) · Settings→Fields manager (CRUD, toggle, categories, options editor, cardinality, units, inline min/max, EN/FR labels) · Settings→Species manager (add/edit custom species, gestation defaults) · Animals list screen (columns = identity + active-mode fields, sort/filter/paginate/select, context menu) · Animal detail (identity card, latest values, full history timeline + per-field charts, edit, delete-with-confirm, PDF card placeholder) · create/edit animal (dynamic form + identity + parent pickers with sex validation + phantom-tag support) · observations: single / **batch** (one animal, many fields) / **mass** (many animals, same fields) entry screens · all as commands (`animal.*`, `observation.*`, `field.*`, `species.*`) + graceful-failure tests + `command_hunt` v1 · audit log writing · CLI: `run`, `seed` · Developer-mode flag plumbing (persisted, gating nothing yet).
**Exit:** inventory HM-*/DE-core rows done with evidence · flatten perf budget passing · snapshots all screens · fuzz green · legacy oracle scenarios for entities/measurements/import-free entry transcribed & passing.

### Phase 2 — Organization & rules
**Scope:** Modes (CRUD, field links + ordering, protected system mode "General Husbandry", nav mode switcher, mode-scoped field visibility across list/detail/entry) · computed fields (`evalexpr` sandbox over `{{field_key}}`, save-time validation, herd-wide calculation, shown in detail/list, `computed_field.*` commands) · validation rules engine — **all six rule types implemented** (min/max/range/required/date_expiry/date_expression with chrono-only sandbox: `date_diff`, `date_add`, `current_date`; failures evaluate false, never crash) · Alerts center (nav badge, per-animal alerts panel, alerts list screen, severity styling) · session undo for invertible data-entry commands (engine undo stack; scope decision §16 O2).
**Exit:** demo-data alerts fire correctly (legacy oracle scenarios from `test_health_reproduction`/`test_formula_service` transcribed) · mode switching visibly re-scopes every data surface · rules of every type tested incl. malformed expressions.

### Phase 3 — Analytics, dashboard, data I/O
**Scope:** create crates `midata-analytics` (single engine) + `midata-exchange` (all file-format deps — calamine/csv/xlsx/PDF — isolated there) · `midata-analytics` full (§8: stats, trend+regression, distribution, correlation matrix, comparison, deep-dive, insights templates i18n) · Analytics Studio screen (study builder: type, fields, filters species/breed/sex/date, chart+insights panes) · Deep Analysis screen · Dashboard: widget kit (KPI, line, bar, table, alert_board, progress_list, donut-custom, profitability) hydrating from `data_config_json` (query-as-source first-class) · dashboard editor (enable/reorder/parameters per mode; bento grid) · profitability widget/screen (purpose-tag resolution, minor-unit prices, TND default — legacy USD-default bug fixed) · **Import wizard** (xlsx via calamine / csv: column-mapping UI to identity+fields, conflict strategy skip/update, per-row error report, progress + cancel, `data.import` command) · **XLSX exports** (rust_xlsxwriter: flattened wide table w/ filters + summary workbook) · **PDF export** animal cards & report sheets · Backup system (`backup.create/verify/restore/list`, WAL checkpoint, rotation-10) · CLI: `import/export/backup/restore/stats` · `xtask stats` → `docs/scorecard.md` generated (inventory %, tests, budgets, i18n, file-budget) · perf budgets v1 measured & baselined.
**Exit:** legacy analytics oracle scenarios (studio/deep-dive/stats/correlation/dashboard/profitability/export) transcribed & passing · 1,000-row xlsx round-trip with mapping + both conflict strategies · backup restore verified on a copy · scorecard committed & CI-diffed · donut decision resolved (ship or logged gap).

### Phase 4 — Pedigree & reproduction
**Scope:** pedigree computation in analytics (BFS, depth bounds, cycles, phantoms, 500-cap, warnings — pure & unit-tested, legacy `graphLayout` + `test_pedigree` scenarios as oracles) · **Pedigree screen**: layered layout algorithm (generations as columns, pure function, tested), canvas render with pan/zoom, node cards (sex/species/status styling, phantom styling), click-through to animal detail, warnings panel · **Birth event wizard** (`animal.recordBirth` — single transaction: validate dam[+sire] → record parturition observations via purpose tags (mating/pregnancy-status/gestation/expected-birth/birth-date/offspring-count/calving-ease) → set pregnancy status Open → create offspring entities (duplicate custom_id aborts all) → identity observations + sire/dam links → one commit) with failure-injection atomicity tests · reproduction views (pregnant animals, upcoming expected births, alerts integration) · sex/gender consistency warnings surfaced (not fatal).
**Exit:** atomicity test (kill mid-transaction → zero partial state) · 5-generation demo pedigree renders with phantoms · all PD-* oracle scenarios green.

### Phase 5 — AI assistant
**Scope:** create crates `midata-queryguard` (SQL validation lab at L2 — shared later by Phase-6 dynamic widgets) + `midata-ai` (§9 in full): provider settings UI + keyring key + connectivity test · SSE streaming worker + event channel · assistant panel UI (stream markdown, tool blocks w/ status, conversation list CRUD, abort) · tool framework generated from command registry · read-only query tool + **SQL validation lab** (sqlparser AST, whitelist, caps; `test_tool_query`'s 71 scenarios as oracle) · staged writes via persisted `pending_changes` + confirmation cards + commit/cancel + audit `batch_token` · context governance (row/size/token caps, summarization) · AI widget generation (propose → validate → preview → save `source='ai'`) · `ai.*` commands + `command_hunt` coverage.
**Exit:** end-to-end chat against LM Studio · injection-attempt suite green · **restart-survival test for staged tokens** · audit rows verified · assistant UI snapshots (idle/streaming/tool/confirmation states).

### Phase 6 — Packs, dynamic widgets, agent surfaces, polish
**Scope:** **Extension packs**: local JSON manifests (fields, modes w/ links, widgets, species defaults incl. gestation) · install atomic w/ rollback + data_type-conflict rejection + reactivation semantics · uninstall = soft deactivate · Packs screen (browse bundled + user-added manifests, install state) · **Setup templates**: config-only export/import (fields, modes, links, widgets, computed, rules — never entities/observations) · **Dynamic widget builder**: user-composed SELECT over the logical flattened view through **`midata-queryguard`** — the lab sits below the engine precisely so these engine-level commands can use it (whitelist + caps), preview, save as `source='user'` — owner's day-one preference delivered at the earliest safe point (owner decision 2026-10-27) · **Control channel** (loopback TCP JSON-lines, token file 0600, auth-first; `engine.execute/commands`, `app.inspect`, `ui.screenshot`) + **MCP server** (`midata-cli mcp --bridge`) · **Developer Mode ON**: progressive disclosure (core nav 5 items; Packs/advanced settings/mode-switcher prominence gated) · empty-state audit & pass · FR translations to 100% (incl. engine messages) · UI perf pass (list virtualization if budget missed).
**Exit:** pack install/uninstall atomicity tests · dynamic widget injection suite green · external MCP client (Claude Desktop or Cursor) lists commands and executes a read-only one against a running instance · developer-mode toggle verified by snapshots · i18n-coverage FR = 100%.

### Phase 7 — Release & documentation
**Scope:** packaging pipeline (§13.2: Windows installer priority, Linux AppImage+deb, macOS optional) · app icon + branding (original, licensed; attribution rows) · **user documentation** (getting-started, per-screen guides, EN+FR) · ops documentation (backup/restore, portable mode, keyring behaviour) · **`docs/mobile-access-strategy.md`** (§13.3 contract) · CHANGELOG + release checklist · tag `v1.0.0` candidate after owner UAT against inventory coverage ≥ 90% of KEEP list.
**Exit:** clean Windows 10/11 VM: install → onboard → demo data → core loop (list/entry/analytics/dashboard/pedigree/AI chat) without crash · clean Ubuntu AppImage run · packaged-build backup→restore verified · docs published · release tagged.

---

## 15. Decision log (consultation record — binding)

| # | Date | Decision | Source |
|---|---|---|---|
| D1 | 2026-10-27 | Rebuild from scratch in **native Rust + egui**; no code translation — legacy consumed as specification only | owner |
| D2 | 2026-10-27 | Native app does **not** serve browser UI; mobile access = documented future feature after first stable release (method open) | owner |
| D3 | 2026-10-27 | UI languages: **English + French** only; no Arabic UI/RTL in v1 | owner |
| D4 | 2026-10-27 | Scope: **phased** — core herd/entry/analytics/dashboard first; AI & extensions later phases | owner |
| D5 | 2026-10-27 | Data: **fresh start** — redesigned schema, no migration tool from legacy DB | owner |
| D6 | 2026-10-27 | EAV core preserved as the product bet | consultation |
| D7 | 2026-10-27 | **Identity attributes promoted to fixed columns** (full promotion): custom_id, species_id, sex, breed (nullable text), birth_date (age computed, never stored), sire_id/dam_id (+ phantom sire_tag/dam_tag), status Alive/Sold/Dead + exit_date/exit_reason, notes, timestamps. Universal-set definition delegated to architect | owner + delegation |
| D8 | 2026-10-27 | **Species = open registry**; presets Cow/Sheep/Goat; user may add any species (Camel, Horse, Dog…); no code enumerates species; per-species defaults (gestation) are data | owner |
| D9 | 2026-10-27 | **Pig policy:** never in presets, seeds, tests, docs, or examples; no technical block (product stays neutral) | owner |
| D10 | 2026-10-27 | AI responses: instructions always in English; **no forced response language** — model mirrors the user's language (one stabilizing prompt line allowed) | owner |
| D11 | 2026-10-27 | Widgets: owner prefers dynamic from day one; agreed **middle path** — curated widgets P3 (query-source first-class in schema), dynamic user-built + AI-generated in P5–6 atop the SQL validation lab | owner + architect |
| D12 | 2026-10-27 | OS: development on **Linux**; **Windows is the release priority**; CI matrix both from Phase 0; macOS optional later | owner |
| D13 | 2026-10-27 | **Agent-first philosophy adopted as first-class principle** (quote in §1.3): command registry from Phase 0, serde inspectable state, CLI first-class, control-channel/MCP seams day one, audit as agent observability | owner |
| D14 | 2026-10-27 | Methodology imported from ArtCraft investigation: AGENTS.md constitution (adapted, attributed, ≤8 KB), never-crash golden rules (stricter: `unsafe` forbidden with **no** exceptions), file budgets + ratchets, gaps/inventory/scorecard/devlog doc system, 5-step task loop, tests-first, DoD, escalation list, parallelism rules, agent-hour estimates, offscreen snapshot verification, generated numbers | consultation |
| D15 | 2026-10-27 | Legacy 485-test scenarios = behaviour oracles (acceptance criteria), zero code translation | consultation |
| D16 | 2026-10-27 | Single analytics engine (pure Rust on SQLite); **no DuckDB/polars** | architect (report §20 P1 risk) |
| D17 | 2026-10-27 | Secrets in OS keyring; no SECRET_KEY-derived crypto; no API keys in binaries | architect (report §16) |
| D18 | 2026-10-27 | chrono for all time; money as integer minor units; default currency TND (selectable TND/USD/EUR) | architect |
| D19 | 2026-10-27 | Printing replaced by **PDF export** (cards/reports) | consultation |
| D20 | 2026-10-27 | Naming: MiData (user-facing) / `midata-*` (machine); "Rumetrics" banned (grep gate) | consultation |
| D21 | 2026-10-27 | Onboarding collapsed to **1 step** (species + demo-data toggle), ADR-004 spirit; Developer Mode implemented for real in P6 | consultation |
| D22 | 2026-10-27 | All-sync core; tokio isolated inside `midata-ai` only; background threads + channels elsewhere | architect |
| D23 | 2026-10-27 | Crate layout L0–L5 incl. dedicated `midata-engine` (command registry) + `apps/{midata, midata-cli}` + `xtask` | architect (D13 consequence) |
| D24 | 2026-10-27 | Crate architecture refined: added `midata-i18n` (L0, shared by app+cli), `midata-ops` (L2 workflows — keeps engine commands thin dispatch), `midata-queryguard` (L2 SQL-validation lab — placed **below** the engine so AI (P5) and dynamic widgets (P6) share one validator; fixes the upward-dependency trap), `midata-exchange` (L2 file formats — isolates heavy deps), `midata-testkit` (dev fixtures). Layers = registered DAG, not a strict chain. `midata-app` restricted to domain+engine+i18n. Crates born in the phase that first uses them. Split triggers documented (§4.8) | owner consultation (crate-structure review) |
| D25 | 2026-10-27 | Git workflow adopted: private GitHub remote; protected `main` (no direct pushes — owner squash-merges PRs); one branch per task `t/<task-id>-<slug>`; conventional commit subjects carrying `[task-id]`; mandatory `Co-Authored-By` attribution on agent commits; green-commits-only with mid-task commits as session checkpoints; goldens/generated-docs/Cargo.lock committed with their motivating change; annotated release tags `vX.Y.Z` + milestone tags `m/phase-N`; full traceability chain (§13.4) | owner consultation (git organization) |

*Plan-document decisions (this file):* v0.1 initial draft — 2026-10-27 · v0.2 crate-architecture revision (D24) — 2026-10-27 · v0.3 git workflow (D25) — 2026-10-27.

## 16. Open items (non-blocking; defaults apply until owner rules)

| # | Item | Recommended default | Decide by |
|---|---|---|---|
| O1 | Licence | **MIT OR Apache-2.0 dual** (Rust standard; legacy was MIT) | Phase 0 |
| O2 | Undo scope | Session-scoped undo for invertible data-entry commands (engine undo stack); persistent audit remains the record | Phase 2 |
| O3 | Pie/donut charts | Custom painter donut in P3; fallback = horizontal bars + gap entry | Phase 3 |
| O4 | PDF crate | `printpdf` (lower-level, lighter) vs `genpdf` (templated); spike decides | Phase 3 |
| O5 | App icon/branding | Commission/generate original icon (no fox/Adobe echoes); owner approves | Phase 7 |
| O6 | macOS packaging | Defer unless owner acquires hardware/CI need | Phase 7 |
| O7 | Built-in species beyond three (e.g. Camel curated pack) | Stay at 3 presets; Camel etc. user-addable with generic pack; revisit if owner wants Maghreb-curated camel pack | any time |

## 17. Success metrics (v1.0 gate)

- Inventory coverage ≥ **90%** of KEEP list, 100% of DROP list absent (grep-verified: `rumetrics`=0, `ngrok`=0, `duckdb`=0, pig-term=0 in seeds/docs).
- CI green on ubuntu + windows; clippy warnings 0; `unsafe` occurrences 0; file-budget violations 0.
- i18n coverage: EN 100%, FR 100% (UI + engine messages).
- Snapshot coverage: every screen ≥1 golden; `command_hunt` green; every command has graceful-failure tests.
- Perf: cold start < 2 s · flatten 10k animals < 500 ms · dashboard hydrate < 300 ms · import 1k rows < 10 s.
- Tests: ≥ 300 (ratchet from Phase 1) · all legacy oracle scenarios mapped in `docs/testing.md` either passing or explicitly waived with reason in `docs/decisions.md`.
- Owner UAT: full core loop on packaged Windows build without crash; staged AI write survives app restart; MCP client drives one read-only command.
- Releases tagged; CHANGELOG current; docs stamps current.

## 18. Risks & mitigations

| Risk | Likelihood | Mitigation |
|---|---|---|
| egui DataTable effort underestimated (AG-Grid gap) | High | Built once in P1 as dedicated kit with its own budget & snapshots; pagination keeps render cost bounded; fallback scope: defer column-visibility UI, keep sort/filter/paginate |
| Aesthetic gap vs legacy React app disappoints owner | Medium | Token system from P0; owner reviews snapshot goldens at every phase gate (cheap feedback loop); ui-design.md is the contract |
| Context economy erodes as codebase grows (owner's stated nightmare) | Medium | File-budget ratchet + check-layers + crate READMEs + orientation table + devlog + ≤5-files rule + dead-code ban — all CI-enforced from P0 |
| Scope creep back toward legacy bloat (14 config surfaces) | Medium | Triage (§6.2) frozen; changes only via decision-log entry; Developer Mode hides advanced surfaces |
| egui version churn (0.37→…) | Medium | Pin in workspace; upgrades are standalone planned tasks (ArtCraft migrated live — precedent) |
| AI-phase security regressions | Medium | Validation lab reused verbatim by dynamic widgets & MCP; injection oracle suite from legacy 71 scenarios; persisted tokens with restart test |
| Solo-owner escalation bottleneck | Low-Medium | Escalation list is closed & small; batch review windows at phase gates; agents authorized to proceed on everything else |
| SQLite contention UI vs AI worker | Low | Single mutexed connection + WAL; commands are short; heavy reads batched |
| Windows-only bugs found late | Low | CI matrix both OSes from day one; packaging dry-runs from Phase 3 onward (nightly artifact builds) |

---

## Appendix A — Dependency shortlist

| Crate | Role | Phase | Notes / licence class |
|---|---|---|---|
| `rusqlite` (bundled) | SQLite engine | 0 | MIT |
| `serde`, `serde_json` | serialization everywhere | 0 | MIT/Apache |
| `chrono` | all date/time (golden rule) | 0 | MIT/Apache |
| `thiserror` / `anyhow` | typed errors / binaries | 0 | MIT/Apache |
| `egui`, `eframe`, `egui_plot`, `egui_extras` | UI shell, charts, tables/date-picker | 0 | MIT/Apache |
| `egui_kittest` (dev) | offscreen snapshot tests | 0 | MIT/Apache |
| `crossbeam-channel` | UI↔worker channels | 0 | MIT/Apache |
| `directories` | platform data dirs | 0 | MIT/Apache-2.0 |
| `tracing`, `tracing-subscriber`, `tracing-appender` | structured logs | 0 | MIT |
| `keyring` | OS keyring secrets | 0/5 | MIT/Apache |
| `evalexpr` | computed-field + date-rule sandbox (in `midata-ops`) | 2 | MIT — in `midata-ops` |
| `calamine` | xlsx/xls read (import) | 3 | MIT — `midata-exchange` only |
| `csv` | CSV import | 3 | MIT — `midata-exchange` only |
| `rust_xlsxwriter` | xlsx export | 3 | MIT/Apache — `midata-exchange` only |
| `printpdf` or `genpdf` | PDF cards/reports (O4) | 3 | MIT/Apache — `midata-exchange` only (O4) |
| `image` | snapshot/PDF raster glue | 3 | MIT/Apache — `midata-exchange` / `midata-app` |
| `rayon` | parallel analytics (only where measured) | 3 | MIT/Apache — `midata-analytics` |
| `sqlparser` | SQL AST validation lab (AI queries + dynamic widgets) | 5 | Apache-2.0 — `midata-queryguard` only |
| `reqwest`, `reqwest-eventsource`, `tokio` | LLM HTTP+SSE (ai crate only) | 5 | MIT/Apache |
| `pulldown-cmark` (or similar) | assistant markdown rendering | 5 | MIT/Apache |
| `axum` | **only if/when** mobile-access option A is built | post-v1 | MIT |
| dev-tools: `cargo-nextest`, `cargo-deny`, `cargo-machete` | CI gates | 0 | — |

Policy: `[workspace.dependencies]` centralizes versions; `deny.toml` licence allowlist (MIT, Apache-2.0, BSD-2/3, ISC, Zlib, Unicode); any new dependency = escalation item (§11.5).

## Appendix B — Initial command catalogue (sketch; grows via "add a command" checklist)

| Area | Command ids (v1 target) |
|---|---|
| animal | `animal.create` `animal.update` `animal.delete` `animal.get` `animal.list` `animal.recordBirth` `animal.setStatus` |
| observation | `observation.record` `observation.batch` `observation.mass` `observation.delete` `observation.latest` `observation.history` |
| field | `field.list` `field.create` `field.update` `field.toggle` `field.delete` `field.grouped` |
| species | `species.list` `species.add` `species.update` `species.toggle` |
| mode | `mode.list` `mode.create` `mode.update` `mode.delete` `mode.toggle` `mode.setFields` `mode.activate` |
| computed | `computed.list` `computed.create` `computed.update` `computed.delete` `computed.calculate` `computed.validate` |
| rule | `rule.list` `rule.create` `rule.update` `rule.delete` `rule.evaluate` (per entity / herd-wide) |
| analytics | `analytics.stats` `analytics.trend` `analytics.distribution` `analytics.correlation` `analytics.comparison` `analytics.deepDive` `analytics.profitability` `analytics.dashboard` |
| pedigree | `pedigree.graph` `pedigree.ancestors` `pedigree.descendants` |
| widget | `widget.list` `widget.create` `widget.update` `widget.delete` `widget.toggle` `widget.reorder` |
| data | `data.importPreview` `data.importCommit` `data.exportXlsx` `data.exportSummaryPdf` |
| system | `system.backupCreate` `system.backupVerify` `system.backupRestore` `system.backupList` `settings.get` `settings.set` `seed.demo` |
| onboarding | `onboarding.status` `onboarding.run` |
| pack | `pack.list` `pack.install` `pack.uninstall` `template.export` `template.import` |
| ai (P5) | `ai.chat` (streaming session) `ai.query` `ai.stageModify` `ai.stageBatch` `ai.commit` `ai.cancel` `ai.conversation.*` `ai.widget.generate` `ai.settings.*` `ai.testConnection` |
| app (UI-level, shell-owned) | `app.navigate` `app.setTheme` `app.setLanguage` `app.setDeveloperMode` `app.setMode` |

## Appendix C — Phase 0 artifact checklist

- [ ] Repo initialized, workspace builds, `Cargo.lock` committed
- [ ] `AGENTS.md` (≤8 KB, English, attribution header) + orientation table
- [ ] `docs/`: architecture · development · contributing (task loop, gates, add-a-command, add-a-screen) · gaps v0 · inventory v0 (from PROJECT_REPORT §3) · decisions (seeded with §15 of this plan) · ui-design · i18n · testing (oracle-mapping skeleton) · scorecard (generated stub)
- [ ] `ROADMAP.md` v0 (stamped; headline numbers defined) + `log/devlog.md` started
- [ ] `xtask`: check-layers · file-budget · ci · stats · i18n-coverage · new-command · new-screen · seed · bench (skeleton)
- [ ] CI workflows (ubuntu+windows) green incl. cargo-deny + machete + scorecard diff
- [ ] Crates born per §4.1 birth phases (P0 set: domain, i18n, store, ops, engine, app, testkit + apps/* + xtask) + crate READMEs (public-API stubs); `xtask check-layers` operational with the P0 crates and their allowed edges registered in `xtask/src/layers.rs`
- [ ] Schema v1 migrations + idempotency tests + repos + latest-value query tests (incl. 10k-row perf fixture)
- [ ] Command trait + registry + audit hook + first commands (`settings.*`, `onboarding.*`, `seed.demo`)
- [ ] App shell: window, router, nav, tokens (light/dark), i18n loader, settings persistence
- [ ] Onboarding 1-step + demo seed generator (deterministic; pig-policy verified by test: seed contains only preset species)
- [ ] CLI skeleton: `midata commands`, `midata run`, `midata seed`
- [ ] Snapshots: shell, onboarding (EN+FR)
- [ ] Owner sign-off checkpoint → Phase 1 tasks populated in `docs/gaps.md`

## Appendix D — Legacy oracle mapping (method)

The oracle set is produced by **one dedicated extraction session** in Phase 0 that performs an *extractive read* (read-only, one pass, no implementation study) of the legacy test files for each module listed in PROJECT_REPORT §17.2, harvesting test names, documented behaviours and edge cases into acceptance rows. That session's written output — not the legacy source — is the reference; **the wall closes the moment it is committed.** From then on the legacy tree is off-limits per `AGENTS.md` golden rule 5 (never open, read or grep it), and behaviour questions resolve through `docs/inventory.md` → `docs/testing.md` → `docs/legacy-spec/PROJECT_REPORT.md` → owner, with the answer written back into the docs.

Acceptance rows: e.g. `test_pedigree` (26) → PD-* rows (BFS depth bounds, truncation at 500, cycle safety, phantom parents, sex warnings); `test_tool_query` (71) → AI-* injection/limit scenarios (Phase 5 gate); `test_extensions` (33) → EX-* atomicity/type-conflict/rollback rows (Phase 6); `test_onboarding` (25) → OB-* (adapted to 1-step flow); `test_soft_delete` (36) → OR-*/HM-* is_active semantics; `test_import` (24) → IX-* skip/update/per-row-errors; `test_formula_service` (18) → OR-computed; `test_launcher`/`test_local_sharing` (54) → **dropped by design** (recorded waiver in `docs/decisions.md`). Rows without a legacy counterpart are written fresh from PROJECT_REPORT prose. The mapping table itself lives in `docs/testing.md` and is a Phase-0 exit artifact (skeleton) completed per phase.

---

*End of REBUILD_PLAN.md v0.3-draft. Next step upon owner approval: draft `AGENTS.md` (Phase 0 first artifact), then scaffold. No implementation occurs before approval.*
