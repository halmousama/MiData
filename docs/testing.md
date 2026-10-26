# Testing — oracles, fixtures, snapshots, fuzz

> **Version:** v1 · **Updated:** 2026-10-26 (P0-11, oracle extraction)
> **Upstream:** `REBUILD_PLAN.md` §12 (strategy), §17 (metrics), Appendix D (oracle method);
> AGENTS.md rule 6 ("tests are the gate").
>
> ## ✅ Status: legacy-scenario transcription **COMPLETE** (2026-10-26)
> The one dedicated, owner-authorized extraction session (P0-11) read **only** the legacy test files, once,
> and transcribed test **names + asserted behaviours** (input → expected behaviour) into
> `docs/oracles/`. **No code, API shape, endpoint path, table/column name or legacy file name was copied.**
> **The firewall exception was used and expired with that session** (AGENTS.md rule 5) — the legacy tree is
> closed again for good. Behaviour questions now resolve through `docs/inventory.md` → this file and
> `docs/oracles/` → `docs/legacy-spec/PROJECT_REPORT.md` → owner. Never re-open the legacy tree.
>
> A scenario marked **fresh** has no legacy counterpart — that is the normal case for a rebuild that
> redesigns behaviour (D1), not a defect.

## 1. What a test must prove

| Layer | What it proves | Where it lives |
|---|---|---|
| Unit | Value rules, date/age maths, money arithmetic, formula evaluation, statistics known-values, pedigree layout maths, i18n lookup | `#[cfg(test)]` per crate |
| Integration | Store repositories, **migration idempotency (run twice)**, latest-value semantics vs a brute-force computation, every command's happy path, birth atomicity under injected failure, backup/restore round-trip, import strategies, restart-survival of staged AI writes | `crates/*/tests/` |
| Fuzz | `command_hunt`: registry-wide adversarial params (empty, absurd, negative, wrong type, malformed UTF-8 dates) → `Err`, never a panic | `midata-engine` (`--ignored`, run in the gate) |
| UI snapshots | Offscreen `egui_kittest` goldens per screen/state; English + French spot checks; both themes when tokens/layout change | `midata-app` / `midata-testkit` |
| Behaviour oracles | Legacy scenarios mapped to inventory rows as acceptance criteria — **scenarios, never code** | `docs/oracles/` (index below) |
| Perf | Flatten 10k animals < 500 ms · dashboard hydrate (10 widgets, 1k animals) < 300 ms · cold start < 2 s · import 1k rows < 10 s | `cargo xtask bench`; baselines in `docs/scorecard.md` |

**Determinism is mandatory:** seeded randomness, seeded demo data, no wall-clock dependence in assertions
(inject the date), no network in tests.

## 2. Oracle set — index

The extracted set is **470 test functions → 485 collected cases** across 18 modules (the legacy suite's own
per-module figures sum to 485 cases; `test_extensions` contributes 32 functions / 33 cases via one
parametrised test). Scenarios live in per-area files so no single file breaks the 500-line budget and an
agent loads only what its task needs (AGENTS.md §5 context budget).

| Oracle file | Legacy modules (functions) | Scenarios | Kept | Waived | Inventory blocks |
|---|---|---|---|---|---|
| `oracles/storage-and-config.md` | `test_database` 8 · `test_settings` 16 · `test_formula_service` 18 | 42 | 39 | 3 | SY, OR, HM, IX |
| `oracles/analytics-and-dashboard.md` | `test_analytics` 30 · `test_tool_analyze` 21 | 51 | 51 | 0 | AN, DB, AI |
| `oracles/pedigree-and-reproduction.md` | `test_pedigree` 26 · `test_health_reproduction` 34 | 60 | 59 | 1 | PD, OR, HM, EX, DE |
| `oracles/exchange-and-onboarding.md` | `test_import` 24 · `test_extensions` 32 · `test_onboarding` 25 | 81 | 81 | 0 | DE, IX, EX, OB, OR |
| `oracles/retention-and-soft-delete.md` | `test_soft_delete` 36 | 36 | 36 | 0 | OR, EX, DB, HM, DE |
| `oracles/query-validation.md` | `test_tool_query` 71 | 71 | 52 | 19 | AI-6…AI-9, AI-19, DB-9 |
| `oracles/assistant.md` | `test_ai_chat_service` 24 · `test_ai_settings` 20 · `test_ai_conversations` 19 · `test_tool_widget` 12 | 75 | 73 | 2 | AI, DB |
| `oracles/launcher-waivers.md` | `test_launcher` 17 · `test_local_sharing` 37 | 54 | 42 | 12 | SY, AI, OR, EX |
| **Total** | **18 modules / 470 functions** | **470** | **433** | **37** | 11 blocks |

The 37 waivers are 12 launcher/desktop-bootstrap tests, 19 from the legacy dual query-language surface, 3
transport-mechanism tests, 2 encryption-mechanism tests, and 1 skipped legacy placeholder. Every one carries
a written reason in its oracle file (§7 summarises them by class).

Every oracle file ends with two sections that are **binding inputs to the build**, not commentary:
**`## Fresh`** (behaviours legacy never covered — write them from `REBUILD_PLAN`) and
**`## Anti-patterns — never reproduce`** (binding countermeasures in the `REBUILD_PLAN.md` §3 register;
each file's list opens with a pointer line mapping every finding to the §3 slug it feeds — D28).

### Scenario ids are the handle

Each scenario has a stable id — `STO-nn`, `ANA-nn`, `PED-nn`, `EXC-nn`, `RET-nn`, `QGV-nn`, `AST-nn`,
`WAV-nn`. When a test is written, cite the scenario id in its doc comment and in the inventory row's
`evidence` column (e.g. `evidence: tests/store.rs::migration_is_idempotent (STO-01)`). A scenario id is
never reused or renumbered; a scenario that stops applying is marked withdrawn in its file with a reason.

## 3. Fixtures

| Fixture | Contents | Used by |
|---|---|---|
| `empty_db()` | migrated tempfile DB, no rows | every store/command test |
| `seeded_demo()` | the deterministic demo seed (`REBUILD_PLAN.md` §5.7): ~50 animals, ~2,000 observations, 5-generation pedigree, 2 phantom parents | analytics, pedigree, dashboard, snapshots |
| `flatten_2k()` | synthetic entity/field/observation matrix with deliberately interleaved dates, including same-date ties | latest-value vs brute-force cross-check |
| `perf_10k()` | 10k animals + observation set sized for the flatten budget | perf budgets only (`#[ignore]` by default) |
| `command_harness()` | session + registry + temp DB + audit assertions | every command test |

Rules: fixtures are **generated** (deterministic, seeded), never committed as binary blobs; the demo seed is
the only "realistic" data and contains **only registry preset species** (D9 policy test); personal data never
appears anywhere. Fixtures live in `midata-testkit` only — a test never reaches into another module's fixture
(the legacy suite did, and two of its modules depended on values defined in a third).

## 4. Snapshot policy ("look at the PNG")

- Harness: `egui_kittest`, offscreen — no window, no focus stealing; `cargo test -p midata-app -- snapshot`.
- Goldens are committed and regenerated **intentionally**; a regenerated golden travels in the same commit as
  the change that caused it (AGENTS.md §5 git discipline).
- A UI task is not done until an agent has **opened the rendered PNG** and described the result; both themes
  when tokens/layout change; a French spot check when strings change.
- Snapshot-covered screens count as a ratchet (only increases).

## 5. Fuzz: `command_hunt`

Registry-wide adversarial sweep over **every** registered command: empty params, missing required params,
wrong JSON types, out-of-range numbers, absurd string lengths, malformed dates (including invalid UTF-8
boundaries), unknown ids in foreign-key fields, and a DB whose file was deleted mid-run. Every outcome must
be `Ok` or a typed `CommandError` — never a panic, never silent data corruption.
Run explicitly in the gate: `cargo test -p midata-engine --test command_hunt -- --ignored`.

## 6. Perf budgets

Measured by `cargo xtask bench`; recorded in `docs/scorecard.md`. A met budget flips to `enforce = true` and
**never un-enforces** (`REBUILD_PLAN.md` §11.8): flatten 10k animals < 500 ms · dashboard hydrate (10
widgets, 1k animals) < 300 ms · cold start < 2 s · import 1k rows < 10 s.

## 7. Waivers

| Scenario source | Count | Why waived | Recorded in |
|---|---|---|---|
| `test_launcher` — port probing, LAN address discovery, server manager state | 6 | D2: native desktop only, no server, no LAN surface | `docs/decisions.md` D2, `REBUILD_PLAN.md` §6.2 |
| `test_launcher` — tunnel credential, tunnel manager state, safe double-stop | 3 | D2: no public tunnel | same |
| `test_launcher` — frozen-bundle resource resolution, launcher console window | 3 | Not applicable to a native binary (packaging / process bootstrap) | same |
| `test_tool_query` — the structured query-plan (DSL) surface | 9 | One validated SQL path; the DSL was a second query language (D16 spirit) | `docs/oracles/query-validation.md` |
| Legacy dual-engine (DuckDB) analytics assertions | — | Single engine; the behaviour is kept, the second engine is not (D16) | `docs/decisions.md` D16 |
| Legacy AI forced-Arabic response assertions | — | Mirror-the-user language instead (D10) | `docs/decisions.md` D10 |
| Legacy transport envelopes (status codes, SSE framing, response key names, timing fields) | — | Typed command output, in-process events; formatting belongs at the UI edge | `docs/oracles/*` anti-pattern sections |

**⚠ Corrected 2026-10-26.** This table previously waived **`test_local_sharing` (37 tests)** as a LAN/tunnel
surface. That was wrong: the module contains **no** sharing, tunnel, QR-code or browser-UI assertion. It
tests read-only query validation (16), setup-template export (7), import (11) and round-trip (3) — all of
which the rebuild keeps (see `oracles/launcher-waivers.md`). The waiver applies to **12 tests, not 49**.
Waiving it would have silently discarded the strongest existing precedents for `midata-queryguard` and the
pack/template contract.

A waiver is recorded here **and** in `docs/decisions.md`; a waived scenario is never silently dropped.

## 8. Mapping discipline

- Every `done` inventory row must name at least one test or snapshot path in its `evidence` column, with the
  scenario id it implements.
- Every scenario points at inventory ids. A scenario with no row means a missing inventory row — add it
  rather than letting the test float. **27 rows were added by the extraction session** (SY-13…16, DE-16…19,
  OR-31…35, AN-27…29, PD-21, AI-21…23, EX-13…16, OB-13…15); they are marked `planned` with `—` evidence and
  are open to owner re-wording, never to deletion.
- Floors never lower: an oracle scenario that was passing may be re-expressed but not deleted. A scenario
  that no longer applies is marked **withdrawn** in its oracle file, with a reason.
- **The `fresh` sections are requirements, not suggestions.** A feature whose oracle file lists fresh
  behaviours ships tests for them or records a gap entry explaining why not.
