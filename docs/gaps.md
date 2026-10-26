# Gaps — the ranked work queue

> **Version:** v1 · **Updated:** 2026-10-27 (P0-12 owner rulings) · **Phase:** 0 — Foundations
> **How to use it** (AGENTS.md §4): take the **highest-ranked unclaimed** entry, follow the five-step
> task loop (AGENTS.md §5), and close it with evidence. Estimates are **agent-hours** — one agent,
> sequential, including tests and docs — recalibrated after Phase 1 (`REBUILD_PLAN.md` §11.7).
> **Claiming is done by branch:** a task is claimed once `t/<task-id>-<slug>` exists on the remote.
> **Nothing here is invented scope:** every entry traces to `REBUILD_PLAN.md` §14 Phase 0 / Appendix C
> or to a measured shortfall found while bootstrapping (stated in the entry's *Evidence* field).
>
> **Ranking rule:** unblocking infrastructure first, then high-value behaviour, then polish — and a
> *cheap, measured* defect may outrank a large feature when it degrades every future session.

## Queue

| Rank | id | Task | Kind | Impact | Est. (h) | Depends on | Status |
|---|---|---|---|---|---|---|---|
| 1 | P0-1 | Workspace + `xtask` + CI | infra | Unblocks every later task; the gate must exist before code does | 10 | — | open |
| 2 | P0-10 | `AGENTS.md` diet to a lean bootloader ≤ ~12 KB | docs | The bootloader is read every session; it is currently 33.5 KB against a ~12 KB target | 3 | — (policy approved 2026-10-27, D27) | open |
| 3 | P0-2 | `midata-domain` + `midata-i18n` | feature | Types, value rules and the string layer that everything else consumes | 8 | P0-1 | open |
| 4 | P0-3 | `midata-store` + migrations | feature | Schema v1, latest-value query, rebuild's data foundation | 12 | P0-2 | open |
| 5 | P0-4 | `midata-engine` skeleton | feature | Command registry + audit hook — the agent-first seam | 8 | P0-2, P0-3 | open |
| 6 | P0-5 | `midata-app` shell + theme | feature | First pixels: window, router, tokens, settings persistence | 12 | P0-2, P0-4 | open |
| 7 | P0-6 | Onboarding + demo seed | feature | Makes the app usable on first run; exit-gate dependency | 8 | P0-3, P0-4, P0-5 | open |
| 8 | P0-7 | `midata-cli` skeleton | feature | `commands` / `run` / `seed` — agent drivability from a terminal | 3 | P0-4, P0-6 | open |
| 9 | P0-8 | Inventory v0 reconciliation (re-scoped) | docs | Turns the hand-written checklist into a machine-checkable contract | 2 | — | open |
| 10 | P0-9 | Snapshots + gates polish | quality | Locks the Phase-0 exit criteria and the ratchets in place | 6 | P0-5, P0-6 | open |

**Sum:** 72 agent-hours (Phase-0 estimate band is 40–80 h — `REBUILD_PLAN.md` §14). Phase 0 closes when
every row above is `done` and the exit criteria in `docs/roadmap.md` are met. P0-11 (oracle extraction,
3 h) closed on 2026-10-26 and P0-12 (owner rulings, 1.5 h) on 2026-10-27 — see §Closed.

---

## P0-1 — Workspace, `xtask`, CI  ·  10 h  ·  rank 1

**Kind:** infra · **Source:** `REBUILD_PLAN.md` §4.1/§4.2/§13.1, Appendix C bullets 1, 3, 5.

**Why now.** Nothing can be built or gated until the workspace, the layer checker and CI exist. The
first crate birth also fixes the dependency set (`Cargo.lock`) and the workspace lints that make
"never crash" a compiler fact rather than a review opinion.

**Scope.**
- Root `Cargo.toml` workspace: `resolver = "2"`; `[workspace.dependencies]` with the Phase-0 shortlist
  only (`REBUILD_PLAN.md` Appendix A: rusqlite/serde/serde_json/chrono/thiserror/anyhow/egui stack/
  egui_kittest/crossbeam-channel/directories/keyring/tracing/tracing-subscriber/tracing-appender);
  `[workspace.lints.rust] unsafe_code = "forbid"`; `[workspace.lints.clippy]` deny list from
  AGENTS.md; `panic = "unwind"` in the release profile; `licence = "MIT OR Apache-2.0"`.
- `clippy.toml` (tests exempted), `deny.toml` (licence allowlist MIT/Apache-2.0/BSD-2/3/ISC/Zlib/
  Unicode), `rustfmt.toml` if non-default settings are needed.
- **Crate births (P0 set only):** `midata-domain`, `midata-i18n`, `midata-store`, `midata-ops`,
  `midata-engine`, `midata-app`, `midata-testkit`, `apps/midata`, `apps/midata-cli`, `xtask`. Each with
  a `README.md` public-API stub and the `#![deny(...)]` header. **No analytics/queryguard/exchange/ai
  yet** (born in P3/P5).
  *(Alternative: P0-1 creates the workspace + `xtask` only and each later task births its crates. That
  is the stricter reading of "no crate before the phase that first uses it" and is the recommended
  split if P0-1 runs long — decide at task start and record it.)*
- `xtask` with `check-layers` (registered DAG in `xtask/src/layers.rs`, intra-layer edges included),
  `file-budget` (500 soft / 800 alarm), `i18n-coverage`, `stats` (writes `docs/scorecard.md`, marked
  GENERATED), `ci` (runs the gate), plus `new-command`, `new-screen`, `seed`, `bench` skeletons that
  fail with an "unsupported" message rather than a panic.
- `.github/workflows/ci.yml`: ubuntu + windows matrix; fmt, clippy `-D warnings`, nextest, check-layers,
  file-budget, i18n-coverage, cargo-deny, cargo-machete, scorecard regeneration diff, snapshots on Linux.
- `rumetrics` grep gate (zero occurrences) as a `xtask ci` step.

**Evidence to record.** `cargo xtask ci` green locally; the PR's **both workflow jobs** (ubuntu +
windows) green on the owner's remote (live since 2026-10-27, D27 — branch protection means this is a
merge gate, not a nice-to-have); `docs/scorecard.md` committed and CI-diffed; layer check demonstrably
fails on a deliberately illegal edge (temporary, reverted — captured in the devlog, not committed).

**Risks.** Dependency approval: every crate here is already named in Appendix A, so no new escalation —
but the versions chosen must still pass `cargo deny`. Windows runner availability is the first thing to
verify, since D12 makes it binding.

---

## P0-10 — `AGENTS.md` diet to a lean bootloader  ·  3 h  ·  rank 2

**Kind:** docs · **Source:** `REBUILD_PLAN.md` §1.3 bullet 7, §11.1, Appendix C bullet 2; amended policy
`REBUILD_PLAN.md` §15 **D27** (owner ruling 2026-10-27).

**Policy (amended, owner-approved).** `AGENTS.md` is a **lean bootloader**: iron rules + orientation
only — **rules not procedures; one authoritative home per topic**; target **≤ ~12 KB** (raised from the
original 8 KB). `AGENTS.md` may summarize and point at a doc; it must never duplicate that doc's
content. Every rule keeps exactly one authoritative home — usually `docs/contributing.md`,
`docs/development.md` or `docs/architecture.md`, which already carry most of the mechanical detail, so
this stays mostly deletion.

**Evidence of the gap.** Measured 2026-10-26: `AGENTS.md` is **33.5 KB**, ~2.8× the amended target. Every
session pays that cost before doing any work — the exact failure mode §1.3 bullet 7 was written to
prevent.

**Scope.** Keep: the non-negotiable owner policies, the never-crash rules, the command/EAV/latest-value/
identity rules, the escalation list, the Git discipline block, the orientation table. Move out (with a
pointer left behind): the workspace map, the add-a-command / add-a-screen checklists, the gate command
block, the parallel-agent rules, the record-keeping prose. Where a doc lacks the authoritative text,
move it there *first* (one home), then collapse AGENTS.md to a reference.

**Constraint.** AGENTS.md is the constitution; the owner approves the final shape (owner-review diff
before commit, not a §7 escalation item).

**Acceptance criteria.** `wc -c AGENTS.md` ≤ ~12 288 bytes · **every rule survives** (rule-by-rule
traceability table in the PR body, each row naming its new authoritative home) · no topic has two
homes (grep for the moved headings) · orientation table still points at every doc · before/after byte
counts in the devlog.

**Evidence.** `wc -c AGENTS.md`; traceability table in the PR body; devlog entry with the two byte
counts.

---

## P0-2 — `midata-domain` + `midata-i18n`  ·  8 h  ·  rank 3

**Kind:** feature (L0) · **Source:** `REBUILD_PLAN.md` §4.1, §5.3, §5.5, §7.4; Appendix C bullets 5–7.

**Scope — domain (pure, zero I/O).**
- Species (`SpeciesKey` as data, never an enum of known species — D8), `Sex` (Male/Female, nullable),
  `Status` (Alive/Sold/Dead with exit invariants), `DataType` (number/text/select/boolean/
  multi_select/date), `Cardinality` (single/series).
- Value encoding: constructors that guarantee "exactly one of `value_num`/`value_text` is set, matching
  the data type" (§5.3), multi-select as a JSON array string, dates as ISO-8601 text.
- `CommandError` (i18n key + params, never a raw message) and the graceful-failure vocabulary
  (`invalid_param`, `not_found`, `conflict`, `unsupported`, `internal`).
- Age computed from `birth_date` via chrono (never stored); money as minor units + currency code;
  pedigree traversal bounds as constants (1–10 generations, 500 nodes).
- Unit tests: value round-trips, encoding mismatches rejected, age across leap years, money arithmetic
  is integer-only, no-species-enumeration test.

**Scope — i18n (pure, shared by app and cli).**
- TSV catalogues: `crates/midata-i18n/catalogues/en.tsv` (source) + `fr.tsv`; `t!`/lookup API returning
  the key itself on miss (never a panic), and coverage data exposed for `xtask i18n-coverage`.
  (`REBUILD_PLAN.md` §7.4 was aligned to this location in P0-12/M1 — D24 is authoritative, no
  correction left to record.)
- Key-naming convention (`cmd.animal.create.title`, `err.animal.duplicate_custom_id`, `screen.x.y`) and
  a test that every key in `en.tsv` has a `fr.tsv` row or is listed as pending.

**Evidence.** Unit test names in the crate; `cargo xtask i18n-coverage` reporting a real number;
per-crate README public-API sections filled in; `docs/i18n.md` updated with the convention actually
enforced.

---

## P0-3 — `midata-store` v0: engine, migrations, latest-value queries  ·  12 h  ·  rank 4

**Kind:** feature (L1) · **Source:** `REBUILD_PLAN.md` §5.2–§5.5, §9 ("keeping the data & schema
complete"), Appendix C bullet 7; `test_database` oracle (pending extraction).

**Scope.**
- `Store` handle over `Mutex<rusqlite::Connection>`, WAL, `foreign_keys=ON`, busy timeout; platform data
  dir via `directories` + `MIDATA_PORTABLE=1` (D22, C5).
- Migrations: `PRAGMA user_version`, ordered, forward-only, **each tested by running twice**
  (idempotency). **Schema scope resolved by D26 — see below.**
- Repositories for species/entities/field definitions/observations/settings/audit_log.
- The canonical latest-value flatten as **one** window-function statement (§5.4) — no second
  implementation anywhere; a brute-force cross-check test on a 2,000-observation fixture.
- Invariants: duplicate `custom_id` rejected; `cardinality='single'` upsert keeps at most one row;
  `UNIQUE(entity_id, field_id, recorded_on)` for series; entity delete cascades observations; field
  delete is soft.
- Perf fixture: flatten 10k animals measured and recorded (budget < 500 ms).

**Schema scope — resolved (D26, owner ruling 2026-10-27).** Birth-gated: P0-3 migrates **only** the
Phase-0 tables — species, entities, field_definitions, observations, modes, mode_fields,
computed_fields, validation_rules, settings, audit_log — **not** `widgets` (P3), nor `conversations` /
`messages` / `pending_changes` (P5). Pack-provenance columns (`origin`/`pack_id`) arrive in a P6
owner-approved migration. "No dead schema" outranks "full v1 at once"; §5.2 now carries the note.

**Evidence.** Migration idempotency tests; flatten-vs-brute-force test; 10k perf measurement in
`docs/scorecard.md`; graceful-failure tests for corrupt/absent DB file (open must return an error, not
panic).

---

## P0-4 — `midata-engine` skeleton: registry, session, audit  ·  8 h  ·  rank 5

**Kind:** feature (L3) · **Source:** `REBUILD_PLAN.md` §4.3, §11.2 rule 3; Appendix C bullet 8.

**Scope.**
- `Command` specification struct (id, title_key, params_doc, enabled, run) + the registry list in
  `commands.rs` merged from per-area modules (`system_cmds.rs`, `onboarding_cmds.rs`, `seed_cmds.rs`
  first).
- `Session` (store handle + actor + undo seam) and `CommandOutput` (serde JSON + optional summary i18n
  key); **exactly one `audit_log` row per mutating command**.
- First commands: `settings.get`, `settings.set`, `onboarding.status`, `onboarding.run`, `seed.demo`.
- Dispatch boundary: an escaped panic is caught, converted to an i18n `internal` error, and the app/DB
  stay alive (last-resort guard — a safety net, not a licence to panic).
- `command_hunt` v0: registry-wide adversarial params (empty/absurd/negative/malformed UTF-8 dates)
  → `Err`, never panic; `#[ignore]`d like the full suite, run explicitly in the gate.

**Evidence.** Registry listing test (ids unique, all have title keys and params docs — `midata
commands` readiness); per-command happy path + graceful-failure tests; audit-row count assertions;
`command_hunt` green.

---

## P0-5 — `midata-app` shell: window, router, theme, settings  ·  12 h  ·  rank 6

**Kind:** feature (L5 + `apps/midata`) · **Source:** `REBUILD_PLAN.md` §7.1–§7.5, Appendix C bullet 9.

**Scope.**
- `apps/midata` composition root: eframe/wgpu window, logging (`tracing` + rotating file appender),
  store open (error screen instead of a crash when the DB cannot open), session construction.
- `Screen` enum router + nav (Dashboard, Animals, Entry, Analytics; Assistant appears in P5), empty
  placeholder screens that dispatch nothing yet.
- `theme.rs` token module (light/dark, earthy/olive palette, radii, spacing, semantic colours);
  **no hard-coded colours anywhere**; `widgets/` one-way rule (never imports `screens/`).
- Settings load/save through `settings.*` commands; language switch wired to `midata-i18n`; persistence
  keys in one constants module (`midata_theme`, `midata_language`, `midata_developer_mode`, …).
- Empty/loading/error states designed for each placeholder screen.

**Evidence.** App boots; snapshots of the shell in light **and** dark (PNG viewed); settings persistence
across restart; a screenshot of the DB-open error state; crate README public API.

---

## P0-6 — One-step onboarding + deterministic demo seed  ·  8 h  ·  rank 7

**Kind:** feature (L2 ops + L5 screen) · **Source:** `REBUILD_PLAN.md` §5.6, §5.7, §7.1 (D21),
Appendix C bullets 9–10; `test_onboarding` oracle (pending extraction).

**Scope.**
- `midata-ops` onboarding merge workflow: species presets → field bundles → modes + links → system
  mode → optional demo seed; **merge-only and idempotent** (running twice duplicates nothing). **No
  widget seeding in Phase 0** — default widgets arrive in P3 with the dashboard (D26).
- Onboarding screen: one step (species preset multi-select, demo-data toggle, language, currency).
- `seed.demo` generator: ~50 animals, ~2,000 observations (lactation curves, ADG series, health
  chains), 5-generation pedigree including 2 phantom parents, sample modes/rules/computed fields;
  `source='seed'`; deterministic; `--force` rebuilds.
- **Pig policy test:** the seed emits only registry presets (D9) — an assertion on generated content,
  not a runtime block.

**Acceptance criteria (also M8-wired from the oracle extraction).**
- Idempotency test (run onboarding twice → identical row counts).
- Seeded-determinism test (two runs, byte-identical content modulo timestamps).
- Seed-content policy test (registry presets only, D9).
- Onboarding snapshot EN + FR; app boots to a dashboard with demo data.
- **Rollback proof (fresh oracle — no legacy coverage):** an injected mid-way failure during an
  onboarding run leaves **zero partial rows** (no species, no fields, no modes, no system mode, no
  links) — assert counts before/after, not just that an error was returned. Same transaction-boundary
  proof applies to a re-run (merge-only no-op).

---

## P0-7 — `midata-cli` skeleton  ·  3 h  ·  rank 8

**Kind:** feature (`apps/midata-cli`) · **Source:** `REBUILD_PLAN.md` §10, Appendix C bullet 11.

**Scope.** `midata commands` (list ids + params docs + enabled state), `midata run <id> --params <json>`
(actor `cli`, audit recorded), `midata seed --demo [--force]`. Human-readable output by default, `--json`
for agents. Exit codes distinguish usage error from command error. No domain logic in the binary — all
through the registry.

**Evidence.** CLI integration test running a read and a write command against a temp DB; `midata
commands` output snapshot; audit rows show actor `cli`; README with the verb list.

---

## P0-8 — Inventory v0 reconciliation (re-scoped)  ·  2 h  ·  rank 9

**Kind:** docs · **Source:** `REBUILD_PLAN.md` §6.1, Appendix C bullet 3; measured: B0/P0-11 already
transcribed §3 into **222** rows.

**Re-scope (owner-approved 2026-10-27, D27).** Verification-first: this task owns the **row-level**
correctness of the inventory, not the number pipeline. Generation stays with `cargo xtask stats`
(P0-1) and the ratchet baselines with P0-9 — no second generator, no duplicated plumbing. 3 h → 2 h.

**Scope.** Verify the 222 rows against `PROJECT_REPORT.md` §3/§11/§9 line by line; split rows that
bundle two behaviours; record/repair the phase assignment per row (P0–P7, D26-aware); add the
`docs/testing.md` row-link column so every row points at its oracle scenario id(s) or `fresh`.

**Out of scope (by re-scope).** Generated coverage numbers, removal of the hand table, scorecard
wiring — those land with `xtask stats` (P0-1) and P0-9.

**Evidence.** Row count before/after with a justification per change; oracle-link column populated for
every block; phase column consistent with D26 (no widget/conversation table credited to P0).

---

## P0-9 — Snapshots + gates polish  ·  6 h  ·  rank 10

**Kind:** quality · **Source:** `REBUILD_PLAN.md` §7.5, §11.4, §12, Phase 0 exit criteria.

**Scope.** `egui_kittest` snapshot harness in `midata-testkit` (offscreen, no window/focus stealing);
goldens for shell + onboarding in EN **and** FR, light and dark where layout matters; a documented
"look at the PNG" step in `docs/development.md`; the full gate block (`cargo fmt --check` → clippy →
nextest → check-layers → file-budget → i18n-coverage → snapshots → bench --quick → stats) wired into
`xtask ci` and into CI; ratchet baselines recorded (test count, coverage, file-budget violations = 0).

**Evidence.** Goldens committed and visually reviewed (devlog notes "viewed"); `cargo xtask ci` green on
both OSes; baseline numbers in `docs/scorecard.md`.

---

## P0-11 — Legacy oracle extraction session  ·  3 h  ·  ✅ **CLOSED 2026-10-26**

**Kind:** docs · **Source:** `REBUILD_PLAN.md` Appendix D, `plan/kickoffs.md` §3, AGENTS.md rule 5
single exception.

**Scope.** One dedicated session reads **only** the legacy test files, transcribes test names and
asserted behaviours (input → expected behaviour, never code shapes) into `docs/testing.md`, mapped to
inventory row ids; marks uncovered behaviours `fresh`; records per-module counts. The firewall
exception **expires with that session** and is noted in the devlog.

**Outcome (2026-10-26).** 18 modules / **470 test functions → 485 collected cases** transcribed into
`docs/oracles/` (8 files, indexed by `docs/testing.md` §2): 433 scenarios kept, 37 waived with reasons.
**27 inventory rows added** (222 total) for behaviours the v0 checklist had no row for. Each oracle file
carries a `## Fresh` section (behaviours legacy never covered) and an `## Anti-patterns` section feeding
`REBUILD_PLAN.md` §3. Two corrections to the existing docs: the legacy `test_local_sharing` module was
**not** a LAN/tunnel surface (it is query validation + setup-template exchange, all kept), and
`test_extensions` is 32 functions / 33 cases, not 33 functions.

**Evidence.** `docs/oracles/*` (8 files); `docs/testing.md` v1; `docs/inventory.md` v1 (222 rows); devlog
line: "firewall exception used and expired with this session".

---

## P0-12 — Owner rulings on the bootstrap ambiguities  ·  1.5 h  ·  ✅ **CLOSED 2026-10-27**

**Kind:** docs · **Source:** owner ruling message on the nine bootstrap ambiguities + oracle-extraction
follow-ups (2026-10-27), `REBUILD_PLAN.md` §15 **D26/D27**.

**Scope applied.**
- `REBUILD_PLAN.md` → **0.4-draft**: §5.2 birth-gated schema note + per-phase table comments (M3) ·
  §14 Phase 0 scope/exit wording (D26, lean bootloader) · §6.1 inventory provenance (M2) · §7.4 i18n
  location = `midata-i18n` (M1) · lean-bootloader policy in §1.3/§11.1/Appendix C (M4) · D26/D27 rows
  and header stamps (M6).
- `AGENTS.md` §5 Git discipline → "no session ends with uncommitted work" (M5).
- `docs/decisions.md` → D26/D27 mirrored; A2/A4 ratification notes; O1 holder provisional; A5 (M6).
- `docs/roadmap.md` → explicit MIRROR stamp (owner ruling 7 / M6).
- `docs/gaps.md` → P0-10 re-scoped to the lean-bootloader policy, P0-3 ambiguity resolved (D26), P0-6
  de-widgetized + rollback criterion, P0-1 CI evidence tightened, P0-8 re-scoped, M8 criteria wired.
- `docs/proposals/anti-pattern-register-extension.md` → M7 draft delivered for owner review, then
  **merged the same day as 37 slug-keyed rows in §3.2–§3.8 (D28)**; the proposal file is deleted —
  `REBUILD_PLAN.md` §3 is the single authoritative home and each oracle file points at the slugs it feeds.

**Evidence.** D26/D27 in `REBUILD_PLAN.md` §15 and `docs/decisions.md`; this entry; devlog
2026-10-27; branch `t/p0-12-owner-amendments`.

---

## Forward-wired acceptance criteria (M8, from the oracle extraction)

The extraction surfaced behaviours the legacy suite never covered. They are wired here so the phase
tasks cannot be written without them; each becomes a mandatory acceptance criterion of the task that
owns it (same rule as `docs/testing.md` §8: a scenario is evidence, not decoration).

| Phase / task | Acceptance criterion (fresh — no legacy oracle) | Oracle ids |
|---|---|---|
| **P3** — import wizard | An injected failure mid-import commits **zero** rows: assert the row count before, force the failure, assert the same count after — never accept "an error was returned" as atomicity proof | DE-16 |
| **P3** — backup system | `backup.verify` **fails** on a deliberately corrupted copy (truncated header / flipped bytes) and still succeeds on a clean copy; the error is an i18n `CommandError`, not raw engine text | IX-11 |
| **P4** — birth event | Birth is all-or-nothing: inject a failure after the offspring row is inserted → zero offspring, zero observations, unchanged dam pregnancy status; both invariant columns asserted | PD-17 |
| **P5** — query validation | `SELECT *` (and count-over-wildcard) cannot bypass the column whitelist; values are **bound, never spliced** into query text; multi-statement, comment, union and prompt-injected payloads are rejected by the AST lab | AI-7, AI-19 |
| **P5** — staged writes | Proposed writes execute **nothing** during the chat turn; the plan persists in `pending_changes` across an app restart; the token is single-use and expires after 5 minutes; commit is transactional or per-item as declared; every confirmed change writes an `audit_log` row carrying the `batch_token` | AI-10, AI-11, AI-12, AI-13, AI-14 |
| **P0-6** — onboarding | An injected mid-way failure leaves zero partial state (see the P0-6 entry) | fresh |

---

## Closed

| id | Task | Closed | Evidence |
|---|---|---|---|
| B0 | Bootstrap: git, `.gitignore`, `rust-toolchain.toml`, dual licence, doc system v0, gaps v0, inventory v0 | 2026-10-26 | this file; `docs/inventory.md` (195 rows); `log/devlog.md` |
| P0-11 | Legacy oracle extraction session (firewall exception, one session only) | 2026-10-26 | `docs/oracles/` (8 files, 470 scenarios); `docs/testing.md` v1; `docs/inventory.md` v1 (222 rows); `log/devlog.md` |
| P0-12 | Owner rulings on the nine bootstrap ambiguities + oracle follow-ups (M1–M8) | 2026-10-27 | `REBUILD_PLAN.md` 0.5-draft §3.1–§3.8 + §15 D26–D28; `docs/decisions.md`; `docs/contributing.md` §3; `log/devlog.md` |

---

## Known shortfalls discovered during bootstrap (not yet tasks)

Items that are real but too small — or too dependent — to queue on their own. Promote to a `P<phase>-n`
entry when they grow.

1. **`docs/scorecard.md` is a stub.** Every number in `ROADMAP.md` is hand-written until `xtask stats`
   exists (P0-1/P0-9). Risk: the "drifting hand-written numbers" anti-pattern. Mitigation: the stub is
   marked GENERATED and CI must diff it before Phase 0 closes.
2. **No `deny.toml` / `Cargo.lock` yet** — they arrive with the first crate (P0-1); licence compliance is
   therefore unverified until then.
3. **`docs/mobile-access-strategy.md` is absent by design** (Phase 7, C8/D2). Any early attempt to add
   axum or a server is a scope violation, not a gap.
4. ~~**Oracle mapping is empty** (`docs/testing.md` skeleton).~~ **Resolved 2026-10-26 (P0-11):** 470
   scenarios extracted into `docs/oracles/`, 433 kept + 37 waived, mapped to 222 inventory rows. What
   remains is the *other* half of the parity measurement: no test yet cites a scenario id in an
   inventory `evidence` column, so coverage still reads 0% until Phase 0/1 code lands.
5. **`docs/legacy-spec/PROJECT_REPORT.md` is committed as the behaviour spec.** It is reference-only and
   must never become an implementation source; if any agent starts quoting its Python/React/Git layout
   content as a build guide, that is a rule-5 violation to stop immediately.
