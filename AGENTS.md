# AGENTS.md: guide for AI agents and contributors

> **Version:** 0.4-draft · **Last updated:** 2026-10-30 — seed artifact for the MiData Native repository (Phase 0); lives at the repo root once initialized. Change: §12 autonomous cycle / delegated merge authority (REBUILD_PLAN §15 D31, superseding the owner-only merge in §13.4/D25).
> **Provenance:** structure and several rules adapted from storytold/photocraft's `AGENTS.md` (MIT OR Apache-2.0). The study behind this adaptation: `ARTCRAFT_INVESTIGATION.md`.
> **Companions:** `REBUILD_PLAN.md` (master plan — its §15 Decision Log is **binding**) · `docs/legacy-spec/PROJECT_REPORT.md` (legacy behaviour specification, reference only).

MiData is a **local-first livestock management and analysis desktop application** for small and medium herds, written in **Rust only** (no JavaScript or TypeScript anywhere). **No Tauri, Electron, webview shells, HTTP servers or browser UI:** the app is native egui/eframe on wgpu. Never add a webview/JS framework or a server framework (axum and friends) as a dependency, build step or feature — mobile/remote access is a *documented future feature* (`docs/mobile-access-strategy.md`, written in Phase 7), not present work. The product name is always written **MiData** in user-facing text: UI, window titles, About, installers, release names, docs prose. Machine names stay lowercase: crates (`midata-*`), binaries (`midata`, `midata-cli`), command ids (`area.action`), storage keys (`midata_*`). The legacy name **"Rumetrics" is banned** — zero occurrences anywhere, CI greps for it.

**This is a specification-driven rebuild, not a port.** The legacy Python/FastAPI + React system is consumed as *specification only* (`docs/inventory.md`, oracle scenarios in `docs/testing.md`, `REBUILD_PLAN.md`). **Never copy, translate or port legacy Python/TypeScript code**, and never reproduce its documented defects — the anti-pattern register in `REBUILD_PLAN.md` §3 is binding. When implementing a behaviour, read its inventory row and oracle scenarios, not the old source. There is **no data migration from the legacy database** and none may be added (owner decision D5).

**Agent-first product (owner's principle):** *"Software in this era must be oriented toward agents as much as toward humans — users today are humans **and** agents."* Concretely: every user-visible behaviour is a **command** in the engine registry; the egui UI, the CLI, the AI assistant and (Phase 6+) the control channel/MCP server are thin dispatchers by command id; all app state is serde-serializable and inspectable; commands are self-documenting via `midata commands`. Build every feature so an agent can write it, test it, drive it and audit it.

**Owner policies (non-negotiable):**

- **Offline-first:** no telemetry, no accounts, no cloud services. The only permitted network peer is the user-configured OpenAI-compatible LLM endpoint (Phase 5+).
- **UI languages: English (source) + French.** Every user-facing string — including error messages and generated insights — goes through i18n. No Arabic UI and no RTL work in v1. AI assistant: system instructions in English; **never force a response language** — the model mirrors the user's language (one stabilizing prompt line is allowed).
- **Species: open registry.** Presets: Cow, Sheep, Goat; the user can add any species (Camel, Horse, Dog, …). Never enumerate species in code. **Pig policy:** the species pig never appears in presets, seed/demo data, tests, fixtures, documentation or examples; and no code path specially blocks it — the product stays neutral, the codebase simply never mentions it.
- **Money:** integer minor units (`i64`), never floats. Default currency **TND** (selectable: TND/USD/EUR).
- **Data:** a single SQLite file (WAL, foreign keys ON) in the platform data directory (`directories` crate); portable mode via `MIDATA_PORTABLE=1` (data next to the executable). Herd records are sacred: every mutation is audited, destructive operations require explicit confirmation, backup/restore must round-trip.
- **Licence:** contributions are dual **MIT OR Apache-2.0** (pending owner confirmation — `REBUILD_PLAN.md` §16 O1).

Read this file first, then `docs/`.

## 1. Orientation (5 minutes)

| Read | Why |
|---|---|
| `REBUILD_PLAN.md` | Master plan: architecture, data model v1, phases, **§15 binding Decision Log**, §16 open items |
| `docs/architecture.md` | Crate map, dependency layers, the engine/UI seam, data flow |
| `docs/development.md` | Build, test, run, snapshots, driving the app via CLI, debug tricks, faster test loops |
| `docs/contributing.md` | Task-loop details, commit style, add-a-command / add-a-screen checklists (summaries in §10–11 below) |
| `docs/gaps.md` | Every known shortfall and open task, ranked: **the work list to pick from** |
| `docs/inventory.md` | Feature coverage vs the legacy spec (blocks HM/DE/OR/AN/DB/PD/IX/SY/AI/EX/OB) — our parity checklist; rows carry evidence |
| `docs/scorecard.md` | Generated numbers: inventory %, test counts, i18n coverage, file-budget, perf budgets — measured, never hand-edited |
| `docs/roadmap.md` | Phases 0–7: scope, exit criteria, **current focus** |
| `ROADMAP.md` | One page: stage, headline numbers, honest "Why not 1.0 yet"; review stamps |
| `docs/decisions.md` | ADR-lite log, seeded from `REBUILD_PLAN.md` §15 |
| `docs/ui-design.md` | Design tokens (earthy/olive palette), themes, widget kit usage |
| `docs/i18n.md` | TSV catalogue format, key naming, coverage rule |
| `docs/testing.md` | Oracle mapping: legacy scenario → inventory row → our test; fixtures; snapshot policy; waivers |
| `docs/legacy-spec/PROJECT_REPORT.md` | Legacy behaviour specification (reference **only** — never an implementation source) |
| `crates/<name>/README.md` | Public API of that crate — read this instead of the implementation |
| `log/devlog.md` | What landed recently, numbers, **"Still open"** bullets |

## 2. Workspace map

```text
crates/                          # born in the phase that first uses them — never earlier
  midata-domain     L0 pure types & value semantics (species, sex, status, data types, cardinality,
                       value rules, CommandError) — zero I/O; deps limited to serde/chrono/thiserror  [P0]
  midata-i18n       L0 TSV catalogues (en.tsv source, fr.tsv), t!() lookup, coverage data — pure;
                       shared by app AND cli                                                          [P0]
  midata-store      L1 rusqlite: engine (WAL, FK ON), migrations (PRAGMA user_version), repositories,
                       canonical latest-value / flatten queries, validated-query execution            [P0]
  midata-analytics  L2 statistics, studies, dashboard hydration, pedigree graph — the single
                       analytics engine                                                               [P3]
  midata-queryguard L2 SQL validation lab (sqlparser AST): SELECT-only, column whitelist, caps —
                       pure; shared by midata-ai AND dynamic-widget commands; never duplicated        [P5]
  midata-exchange   L2 file exchange: xlsx/csv read (calamine), xlsx write (rust_xlsxwriter),
                       PDF render — heavy format deps isolated here                                   [P3]
  midata-ops        L2 domain workflows: birth event, onboarding merge, pack install/uninstall,
                       import pipeline, backup ops, seed generation — keeps engine commands thin      [P0]
  midata-engine     L3 Session + Command Registry (thin dispatch into ops/store/analytics),
                       audit hooks, undo stack                                                        [P0]
  midata-ai         L4 LLM client (SSE), tool loop over the command registry — the ONLY crate
                       allowed to depend on tokio/reqwest                                             [P5]
  midata-app        L5 egui shell: screens (thin), widget kit, serde app state, theme tokens —
                       depends ONLY on domain + engine + i18n (+ egui stack)                          [P0]
  midata-testkit    dev-only shared fixtures: temp-DB harness, demo-seed builders, snapshot
                       helpers, command-test harness                                                  [P0]
apps/
  midata            desktop binary (eframe/wgpu) — composition root; control server (P6) lives here
  midata-cli        headless binary: commands | run | seed | import | export | backup | restore |
                       stats | mcp (P6)
xtask/              cargo xtask check-layers | file-budget | ci | stats | i18n-coverage | bench |
                       new-command | new-screen | seed
```

**Layering is enforced** by `cargo xtask check-layers` against a **registered DAG** in `xtask/src/layers.rs`: every crate and every allowed edge is declared there, including intra-layer edges (`ops → analytics/exchange/store`). Spine: `domain, i18n (L0) ← store (L1) ← {analytics, queryguard, exchange} (L2) ← ops (L2) ← engine (L3) ← ai (L4) ← app (L5)`. `apps/*` may depend on anything; nothing else may depend on `app`/`apps`; `midata-testkit` is a dev-dependency only. **`midata-app` depends only on `midata-domain`, `midata-engine` and `midata-i18n`** (+ the egui stack) — all data reaches the UI as command outputs; the thin UI is a Cargo fact, not a convention. **Nothing below `midata-app` may import `egui`, `eframe`, `egui_plot`, `egui_extras`, `winit` or `rfd`.** **No crate before the phase that first uses it** — an empty crate is dead schema by another name. Inside `midata-app`: `widgets/` never imports from `screens/` (one-way rule; keeps the future `midata-widgets` extraction mechanical). Each crate's `README.md` is its API contract: agents read the README, not the implementation — keep READMEs current with every public-API change.

## 3. Golden rules

### Never crash (outranks feature work)

Farmers trust MiData with their herd's only record; a crash can lose it. A malformed import file, a bad command param, a corrupt settings row, an odd keystroke or a full disk must produce an error the user or agent can act on, **never a panic**. Don't ship a feature by adding a panic path; fix a crash before building on top of it.

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or `unimplemented!`. Return the crate's error type and propagate with `?`; use `ok_or(..)?`, `let .. else { return Err(..) }`, `if let`, or `unwrap_or*` where a fallback is truly correct (never one that silently corrupts data). Unfinished features return an "unsupported" `CommandError`. The only exception is a provably infallible literal: `#[allow(clippy::expect_used)]` plus `.expect("why it can't fail")`.
- **No `unsafe` — no exceptions.** The workspace sets `unsafe_code = "forbid"`. (Stricter than our reference project, which needed one isolated platform crate; MiData has no such need. If you ever believe you do, that is an escalation conversation, not a code change.)
- **Input-derived numbers are hostile.** Use `get()` rather than `[i]`/`[a..b]` for indices from files, params or arithmetic on them; slice strings only at char boundaries; use `checked_*`/`saturating_*` for lengths, offsets and counts; guard division by zero and NaN/inf casts; cap allocations sized by input (import files can be arbitrarily large).
- **Bound recursion and iteration.** Pedigree traversal always carries a visited-set + generation bounds (1–10) + the 500-node global cap; formula evaluation has depth/step limits.
- **Don't cascade.** Handle lock poisoning (`lock().unwrap_or_else(PoisonError::into_inner)`), treat thread joins as `Result`s, handle closed channels gracefully.
- **Last-resort guard.** The app shell catches an escaped panic around command dispatch and import/export, reports it as an i18n error, and keeps the app and database alive. It's a safety net, not a licence to panic. Keep `panic = "unwind"`.
- **Prove it.** Every crash fix ships with a small synthetic regression test that panicked before the fix.
- **Enforced by clippy.** `clippy.toml` allows `unwrap`/`expect`/`panic`/indexing in tests only. Every crate carries `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`; new crates start with it.

### The rules

1. **Everything is a command.** New user-visible behaviour = a command in `midata-engine` (`src/<area>_cmds.rs` exposing `specs()`, merged in the `commands.rs` list) with: `id` (lowercase `area.action`, **stable forever** — a published surface for agents), `title_key` (i18n), `params_doc` (the JSON-ish string agents read via `midata commands`), an `enabled` predicate, and `run(ctx, params) -> Result<CommandOutput, CommandError>` writing **exactly one `audit_log` row** (invertible commands also push one undo step). The UI, CLI, AI tools and (later) control channel/MCP all dispatch by id. Only pure view/window state (navigation, theme, panel layout, language) belongs to the shell (`app.*` commands). **Never rename or reuse a command id.**
2. **Identity vs EAV boundary.** Identity attributes are columns on `entities` (`custom_id`, `species_id`, `sex`, `breed`, `birth_date`, `sire_id`/`dam_id` + phantom `sire_tag`/`dam_tag`, `status`/`exit_date`/`exit_reason`). Everything else is `field_definitions` + `observations` (EAV — the product bet). Never store identity facts as dynamic fields; never add identity-like columns without an owner decision recorded in `docs/decisions.md`. Age is **computed** from `birth_date`, never stored. Sire must be Male, dam must be Female — enforced in the commands with graceful errors.
3. **Latest-value semantics.** "Current value" = the observation with max `recorded_on`, tie-break max `id`. Use the canonical store query (single window-function statement); **never re-invent flattening, never assume insertion order, never loop per-entity (N+1)**. `cardinality='single'` fields keep at most one row per (entity, field) via upsert; `series` fields enforce `UNIQUE(entity_id, field_id, recorded_on)`. Value encoding follows `data_type` (number/boolean → `value_num`; text/select/multi_select-as-JSON-array/date-as-ISO → `value_text`; exactly one non-NULL) — values are constructed only through domain constructors.
4. **Species are data, never code.** No match arms, constants or branches enumerating species anywhere. Per-species defaults (`gestation_days`, field packs, icons, labels) live in the `species` registry and pack manifests. Adding "Camel" must require **zero code changes**. The **pig policy** (Owner policies above) applies to seeds, tests, fixtures, docs, screenshots and examples — a Phase-0 test asserts the seed generator emits only registry presets.
5. **Clean rebuild from spec.** Legacy code is never an implementation source; behaviours come from `docs/inventory.md` rows + `docs/testing.md` oracle scenarios + `REBUILD_PLAN.md`. **The legacy codebase (MiData-back, outside this repo) is off-limits: never open, read or grep it — not its implementation, not its tests.** Behaviour questions resolve via `docs/inventory.md` → `docs/testing.md` → `docs/legacy-spec/PROJECT_REPORT.md` → ask the owner; write every answer back into the docs. The single exception was the one-time Phase-0 oracle extraction, whose output already lives in `docs/testing.md`. No legacy-DB migration importer (D5). The anti-pattern register (REBUILD_PLAN §3) is binding — these must never reappear: date-blind flattening, N+1 listing, dead schema, inert enum variants, logic leaking into UI/CLI layers, committed build artifacts, secrets derived from constants, in-memory confirmation tokens, drifting hand-written numbers, mixed naming ("Rumetrics").
6. **Tests are the gate.** Every change ships with tests. Every command gets: happy path + **graceful-failure** tests (empty/out-of-range/wrong-type/adversarial params → `Err`, never panic) + undo/audit assertions where applicable. Store tests run on tempfile DBs and cover migration idempotency (run twice). Analytics tests assert known-value oracles. Everything is deterministic (seeded randomness, seeded demo data). **`command_hunt`** — the registry-wide adversarial fuzz test (our `panic_hunt`) — must stay green. Legacy oracle scenarios never regress; floors and ratchets never lower.
7. **The UI is thin and data-driven.** Screens dispatch commands and render state — **no domain logic in `midata-app`** (layer check + review). App/UI state lives in serde structs so any automation surface can read and drive it. Colours, radii and spacing come from `theme` tokens, never hard-coded; light **and** dark themes must work. All persistence keys live in one constants module, prefixed `midata_`.
8. **Verify UI changes visually.** Render offscreen through the snapshot harness (`cargo test -p midata-app -- snapshot`, egui_kittest — no window, no focus stealing). **Look at the PNG.** Update goldens intentionally and attach before/after to the task record; spot-check FR when strings change; both themes when tokens/layout change. Fixtures, screenshots and docs use **seeded demo data only — never real personal or farm data**.
9. **Never break Windows.** Development happens on Linux; the CI matrix (ubuntu + windows) is binding. Paths via `std::path` + `directories`; no unix-only assumptions (file locks, separators, line endings, locale-dependent number formats in persisted data). A task is done only when the full gate is green on both OSes.
10. **Time goes through `chrono`** (workspace dependency). Never hand-roll calendar/date math (days-to-date, leap years, age computation). Persisted dates are ISO-8601 text. The date-rule sandbox (validation `date_expression`) exposes only `date_diff`, `date_add`, `current_date` — a failing expression evaluates to false, never crashes.
11. **Money is integer minor units** (`i64`) plus a currency code from settings (default TND). Never floats for money; conversion/formatting happens only at UI edges. Profitability math (prices, feed costs) uses minor units end-to-end.
12. **Every string goes through i18n.** `t!("key")` against TSV catalogues (`en.tsv` source, `fr.tsv` first-class) — UI, command titles, `CommandError` messages, insight templates, seed-pack labels. `cargo xtask i18n-coverage` never regresses; FR reaches 100% (including engine-side messages) by end of Phase 6.
13. **Hygiene: file budget, no dead anything.** Soft cap **500 lines/file**, alarm at 800 — split the file or record a gaps entry; `cargo xtask file-budget` lists violations and **the list may only shrink**. New areas go in new files/modules; no `utils.rs`/`helpers.rs`/`common.rs` dumps. Unused code, tables, flags and dependencies are deleted immediately (history lives in git); `cargo machete` runs in CI. Comments explain *why*; every `pub` item carries a doc comment; `cargo fmt` + `clippy -D warnings`, always.
14. **Secrets & privacy.** The only secret store is the OS keyring (`keyring` crate) — never SQLite, settings files, logs, or values derived from hard-coded constants. No network calls except the user-configured LLM endpoint; no telemetry ever. No personal data in fixtures, docs, examples or commit messages. Every mutating command writes its `audit_log` row (actor: `user`/`ai`/`cli`/`seed`); AI-staged writes carry their `batch_token`.
15. **Dependencies are gated.** Versions centralize in `[workspace.dependencies]`. Adding, removing or upgrading a dependency is an **escalation item** (owner approval; licence allowlist MIT/Apache-2.0/BSD/ISC/Zlib/Unicode via `cargo-deny`; maintenance status checked). Banned by architecture: DuckDB, polars, any ORM, any webview/server framework, `tokio`/`reqwest` outside `midata-ai`, `sqlparser` outside `midata-queryguard`, file-format crates (calamine/csv/rust_xlsxwriter/pdf) outside `midata-exchange`.
16. **Analytics is single-engine and deterministic.** All statistics/studies/pedigree math live in `midata-analytics` — one code path serving UI, CLI, widgets and AI tools. Never add a second engine or a parallel implementation of an existing computation. Insights are deterministic i18n templates — **the LLM is never in the analytics path**.
17. **AI writes are staged, persisted, confirmed, audited** (Phase 5+). Assistant-proposed writes never execute during a chat turn: they persist in `pending_changes` (SQLite — **never an in-memory map**) with a single-use token + 5-minute expiry, appear as a confirmation card, execute only on explicit user confirmation, and land in `audit_log` with the `batch_token`. Restart-survival of staged tokens is a tested exit gate. All SQL (AI or user-built dynamic widgets, Phase 6+) passes the validation lab only — **`midata-queryguard`** (L2, below the engine, so both `midata-ai` and engine-level widget commands consume it; never duplicated, never housed inside `midata-ai`): single SELECT/WITH statement (sqlparser AST), columns whitelisted against active fields + identity columns, hard caps (5,000 rows / 50 KB result / 30 raw rows before summarization / 500 aggregated / 200-char strings / 32k-token conversation budget with a 24k warning).
18. **No committed build artifacts; generated docs stay generated.** Nothing built is committed. Generated docs (`docs/scorecard.md`, generated inventory stats) are marked GENERATED, produced by `cargo xtask stats`, and diffed by CI — never hand-edited. Releases are always tagged (the legacy repo shipped 80 commits with zero tags — not once here).

## 4. Picking work

Priorities: important infrastructure first, then high-value inventory rows, then the long tail.

0. **Read `ROADMAP.md`, then `docs/gaps.md`.** Take the highest-ranked unclaimed entry. `docs/inventory.md` measures spec coverage per block — a wired row without behaviour does not count; only behaviour **with evidence** (test names, snapshot paths) counts. When your work moves a number or closes a gap: update the gaps entry and the inventory rows **with evidence and the date**, and bump their status lines.
1. **Check `docs/scorecard.md`** before picking work: inventory %, missed perf budgets, i18n coverage, file-budget violations, test counts. Its numbers are measured; prefer them to estimates.
2. `docs/roadmap.md` → **Current focus** (the active phase and its scope). Stay inside the current phase unless the owner redirects.
3. `log/devlog.md` → the **"Still open"** bullets of recent entries — finishing an open loop beats starting a new one.
4. Estimates are recorded in gaps entries as **agent-hours** (one agent, sequential, including tests). After each phase, calibrate from actuals and restate the remaining estimates in `ROADMAP.md`.

**Ratchets** (monotonic, CI-enforced): inventory coverage ↑, i18n coverage ↑, test count ↑, snapshot coverage ↑; file-budget violations ↓, clippy warnings = 0 always. A met perf budget flips to `enforce = true` — never un-enforce. A raised floor never lowers.

## 5. Task lifecycle & before you finish

Every task runs the five-step loop:

1. **Orient** — pick the entry from `docs/gaps.md`; read only what the orientation table points to for that area. **Context budget: ≤5 source files + ≤3 docs.** If a task needs more, it is mis-sized: split it and record the split in gaps.
2. **Acceptance test first** — write the failing test/snapshot that defines "done" *before* implementation code (transcribe the oracle scenario from `docs/testing.md` where one exists).
3. **Implement** — in the lowest layer that fits (algorithm → domain/analytics, persistence → store, orchestration → engine command, pixels → app).
4. **Verify** — run the gate below; for UI work render offscreen and **look at the PNG** (both themes if tokens/layout changed).
5. **Record** — update inventory/gaps rows with evidence + date; run `cargo xtask stats` when numbers moved; append the terse `log/devlog.md` entry (what landed, numbers, still open); commit per the Git discipline below.

**Definition of done:** behaviour exists as a registered, documented command (if user-visible) · tests including graceful-failure · wired into the UI (and CLI where relevant) · snapshots viewed for UI work · full gate green (CI on both OSes) · inventory/gaps/scorecard/devlog updated.

**Git discipline** (full workflow: `REBUILD_PLAN.md` §13.4 / `docs/contributing.md`):
- One branch per task: `t/<task-id>-<slug>`. **Never push to `main`** — push your task branch, open the PR; the owner squash-merges (one green commit per closed task) — except inside the D31 envelope (`AGENTS.md` §12), where the SUPERVISOR merges under its six conditions.
- Commit message: `<type>(<area>): <subject> [<task-id>]`, body with what & why, `Refs:` line.
  **No Co-Authored-By trailers and no AI-vendor/model names anywhere** — not in commit messages,
  code, docs, devlog or gaps (D30, supersedes D25's trailer clause). Session/model traceability,
  if the owner wants it, lives only in the gitignored `plan/` kit. Review-enforced today: the
  `commit-msg` hook and the vendor/model term list in the banned-names gate do not exist yet.
- **Never commit red:** every commit keeps the workspace building and touched-crate gates green. Mid-task commits are your session checkpoints — commit early, commit often (a crashed session is recovered from the branch + devlog).
- Snapshot goldens, generated docs and `Cargo.lock` travel **in the same commit** as their motivating change — an artifact never drifts from its code.
- Tagging, force-push, history rewriting, pushing to main: escalation items (§7).
- **No session ends with uncommitted work** — commit the checkpoint before stopping; always verify repo state with `git status` before asserting it.
- Code-bearing PRs are audited by an independent reviewer session before owner merge (`docs/contributing.md` §9); APPROVE requires zero BLOCKER/MAJOR findings; contests go to the owner.

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run -p <crates you touched>        # full: cargo nextest run --workspace
cargo xtask check-layers
cargo xtask file-budget
cargo xtask i18n-coverage                        # must not regress
cargo test -p midata-app -- snapshot             # UI tasks: goldens updated AND viewed
cargo test -p midata-engine --test command_hunt -- --ignored   # if you added/changed commands
cargo xtask bench --quick                        # if you touched a hot path (flatten, dashboard, import)
cargo xtask stats                                # regenerate scorecard; commit it (CI diffs)
```

While iterating, run less and keep the block above as the final gate: `cargo nextest run -p <crate> --lib` skips examples and integration tests; `--test <file>` runs one integration file. See `docs/development.md` › Faster test loops.

Then append the terse entry to `log/devlog.md` (what landed, numbers, what's still open). **Sessions can end abruptly** (crashes, context limits) — the dev log plus a green tree is how the next agent picks up. Keep the tree building at every step; never leave a half-applied migration or a broken `Cargo.toml` behind.

## 6. Parallel agents

- One **git worktree** per agent; branches named `t/<task-id>-<slug>` — one task per branch (e.g. `t/p1-3-animal-create`). Integration merges to main are done by the owner or a designated integrator (commit style: `Integration: track tNN`).
- Use your own target dir (`CARGO_TARGET_DIR=target/agent-<name>`) to avoid the Cargo build lock; delete `target/agent-*` dirs of finished agents (each is several GB).
- **One owner per crate/area at a time; edit only files you own.**
- Shared files (the `v.extend(...)` list in `engine/src/commands.rs`, the app-state struct, `i18n/en.tsv`/`fr.tsv`, the migration list, the workspace `Cargo.toml`) get small, surgical edits; **re-read before editing**.
- Put new commands/screens in **new modules** (`<area>_cmds.rs`, `screens/<screen>.rs`) rather than growing a shared file.
- Keep every `Cargo.toml` valid at all times — **create or rewrite manifests atomically**: write to a temp file outside `crates/`, then `mv` into place.
- If someone else's in-progress edit breaks the build: wait and retry — **don't fix their files**.

## 7. Autonomy & escalation

**Don't block on choices you can make yourself.** Within these rules, decide — and record the decision in the devlog entry.

**Stop and ask the owner (closed list):**

1. Any DB **schema change** (new table/column/index/migration) — migrations are owner-approved, never smuggled into a feature task.
2. Deleting or overwriting data beyond your own test fixtures (user data, the seeded demo baseline, mass golden regeneration).
3. Adding, removing or upgrading a **dependency** (licence allowlist + maintenance review).
4. Publishing, pushing to main, or tagging releases — except the supervisor's squash-merges inside the D31 envelope (`AGENTS.md` §12).
5. Spending money or committing to external services.
6. Anything involving real personal data.
7. Destructive git operations (force-push, history rewrite).
8. Anything these rules don't clearly cover — **when in doubt, leave it out.**

**Human-reserved:** overall planning and roadmap ownership, release cutting, signing, branding/icon, UAT sign-off.

## 8. Where things are tracked

- `REBUILD_PLAN.md` — master plan; **§15 Decision Log is binding**; owner amends it, nobody else.
- `ROADMAP.md` — one page: stage, headline numbers (inventory coverage %, screens, tests, budgets), current focus, honest "Why not 1.0 yet"; stamps **Last reviewed / Last updated / Change**.
- `docs/roadmap.md` — phases 0–7 with scope and exit criteria.
- `docs/gaps.md` — the ranked work queue: entries with kind, impact, agent-hour estimate, evidence. **The list to pick from.**
- `docs/inventory.md` — feature coverage vs the legacy spec (our parity checklist); ids per block (HM/DE/OR/AN/DB/PD/IX/SY/AI/EX/OB); rows: behaviour · status · phase · evidence.
- `docs/scorecard.md` — **generated** by `cargo xtask stats` (never hand-edited; CI diffs it).
- `docs/decisions.md` — ADR-lite: one paragraph + date per architectural decision.
- `docs/testing.md` — oracle mapping table (legacy scenario → inventory row → our test names) + waivers with reasons.
- `log/devlog.md` — append after every task; the continuity record between sessions/agents.
- `plan/` (local, gitignored) — research, estimates, execution drafts.

## 9. Keeping the data & schema complete

(Our analogue of the reference project's "keep the native format complete" rule, for a SQLite + EAV product.)

- **Migrations** live in `midata-store`: `PRAGMA user_version`, forward-only, ordered, idempotent — each tested by running it twice. No downgrade paths. No table is created before the phase that uses it. **No dead schema:** every table/column is read and written by shipped code in the same phase (the legacy `pending_changes` failure never repeats).
- Adding a column updates, **in the same change**: migration + repository + domain type + export/import mapping + seed generator + affected snapshots. Nothing is silently dropped from exports or backups.
- Settings-persisted structs: new fields always `#[serde(default)]` so older settings and databases still load.
- Backup = WAL checkpoint + `PRAGMA integrity_check` + timestamped file copy; restore round-trip is tested; rotation keeps the newest 10.
- Exports (XLSX/CSV/PDF — PDF replaces the legacy "print cards"; there is no direct printing in v1) and imports round-trip the full field set: identity columns + EAV latest-value semantics identical across every surface.

## 10. Adding a command (checklist)

1. **Id:** lowercase `area.action` (`animal.create`, `observation.batch`, `data.importCommit`). Ids are stable forever — check the registry, never rename, never reuse.
2. **Logic in the lowest crate that fits:** pure rules → `midata-domain`; persistence → `midata-store`; computations/studies → `midata-analytics`; multi-step workflows (birth, onboarding, packs, import, backup) → `midata-ops`. The command itself stays **thin dispatch** (~30–60 lines). With unit tests; pure where possible; deterministic.
3. **Command module:** new or existing `midata-engine/src/<area>_cmds.rs` exposing `specs()`; register in `commands.rs` (surgical edit). Fill: `id`, `title_key` (i18n EN+FR), `params_doc`, `enabled` predicate, `run` → `Result<CommandOutput, CommandError>`.
4. **Effects:** exactly one `audit_log` row; invertible operations push one undo step; mutations are transactional (all-or-nothing — see the birth-event pattern).
5. **Wire:** the screen/menu dispatches the id; the CLI exposes it automatically through `run`.
6. **Tests:** happy path · graceful failure (adversarial params → `Err`) · undo/audit where applicable · oracle scenarios mapped in `docs/testing.md`.
7. **Record:** update inventory/gaps; run `cargo xtask stats`; `command_hunt` green; devlog entry.

## 11. Adding a screen (checklist)

1. One file `midata-app/src/screens/<screen>.rs` + a `Screen` enum variant + nav wiring (respecting Developer-Mode gating once Phase 6 lands: core nav = Dashboard, Animals, Entry, Analytics, Assistant; advanced surfaces gated).
2. State: a serde struct in the app-state module; persistence keys (if any) in the single constants module (`midata_*`).
3. Build only from the widget kit (`DataTable`, `DynamicFieldRenderer`, modals, toasts, tokens) — no hard-coded colors, sizes or strings. The widget kit is one-way: `widgets/` never imports from `screens/`.
4. Behaviour through commands only — no domain logic, no direct store calls from `midata-app`.
5. Empty state, loading state and error states designed (i18n).
6. Snapshot goldens (light + dark where layout-relevant; EN, FR spot) — **look at the PNG**.
7. Update inventory + devlog.

## 12. Autonomous cycle (supervisor mode, per D31)

- The SUPERVISOR never implements and never reviews: it picks the top gaps task, spawns a fresh
  IMPLEMENTER subagent (kickoff template, `docs/review-templates.md`), then a fresh REVIEWER
  subagent (REVIEWER template, parameters only — no summaries, no opinions), arbitrates rounds
  (max 2), and merges only when **all six** D31 conditions hold, via
  `gh pr merge --squash --delete-branch`:
  (1) reviewer APPROVE with **0 BLOCKER / 0 MAJOR**;
  (2) both required CI checks **green on the PR head**;
  (3) **no escalation flag on the task** — the `docs/gaps.md` row's Status reads `escalated`, or a
      local `plan/ESCALATE` marker exists alongside `plan/HOLD`;
  (4) the diff touches **no owner-reserved surface** — schema migrations, the dependency set,
      deletion of committed data, the security-adjacent §3 rows (§3.1 rows 8–10, §3.3, §3.4),
      branding;
  (5) a **merge report paragraph in the devlog** — owed by stage 6 of the same cycle, so the cycle
      is not complete until it exists; a cycle that merged without one is an escalation;
  (6) no `plan/HOLD` file.
- **Budget:** 3 tasks per run, then stop and hand the owner a digest (tasks merged, verdicts, open
  escalations, the next three gaps entries in rank order). A **round-3 review** or **two failures
  of the same task** stops the loop immediately with an escalation — never a silent retry.
- Every worker is a fresh context; binding constraints are re-read from this file each cycle,
  never carried in conversation memory. Compaction between cycles is safe precisely because the
  docs are the source of truth — but no binding rule may exist only in a compacted summary.
- `plan/HOLD` (gitignored) stops the loop at the next cycle boundary; any unmet merge condition
  stops it immediately with an escalation.
- Each cycle ends with: merge report paragraph + one supervisor devlog line (task, verdicts,
  CI run ids, deviations, next task).

---

**If you remember nothing else:** never crash · everything is a command · acceptance test first · the gate is non-negotiable · look at the PNG · update docs and devlog before committing · **when in doubt, leave it out.**

*Version 0.4-draft · 2026-10-30 · Structure and several rules adapted from storytold/photocraft's `AGENTS.md` (MIT OR Apache-2.0); see `ARTCRAFT_INVESTIGATION.md` for the study behind this adaptation.*
