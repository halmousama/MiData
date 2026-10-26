# Testing — oracles, fixtures, snapshots, fuzz

> **Version:** v0 (skeleton) · **Updated:** 2026-10-26 (BOOTSTRAP-1)
> **Upstream:** `REBUILD_PLAN.md` §12 (strategy), §17 (metrics), Appendix D (oracle method);
> AGENTS.md rule 6 ("tests are the gate").
>
> ## ⚠ Status: legacy-scenario transcription is **PENDING**
> The oracle set is produced by **one dedicated, owner-authorized extraction session**
> (`REBUILD_PLAN.md` Appendix D; queued as **P0-11**, blocked on owner authorization). That session
> reads **only** the legacy test files, once, and writes test **names + asserted behaviours**
> (input → expected behaviour) into the mapping table below. **No code is copied.** Afterwards the
> legacy tree is closed again for good (AGENTS.md rule 5) and behaviour questions resolve through
> `docs/inventory.md` → this file → `docs/legacy-spec/PROJECT_REPORT.md` → owner.
>
> Until then: every acceptance criterion is written **fresh** from the plan/inventory prose and is
> marked `fresh` in the *Scenario* column. A row without a legacy counterpart is not a defect — it is
> the normal case for a rebuild that redesigns behaviour (D1).

## 1. What a test must prove

| Layer | What it proves | Where it lives |
|---|---|---|
| Unit | Value rules, date/age maths, money arithmetic, formula evaluation, statistics known-values, pedigree layout maths, i18n lookup | `#[cfg(test)]` per crate |
| Integration | Store repositories, **migration idempotency (run twice)**, latest-value semantics vs a brute-force computation, every command's happy path, birth atomicity under injected failure, backup/restore round-trip, import strategies, restart-survival of staged AI writes | `crates/*/tests/` |
| Fuzz | `command_hunt`: registry-wide adversarial params (empty, absurd, negative, wrong type, malformed UTF-8 dates) → `Err`, never a panic | `midata-engine` (`--ignored`, run in the gate) |
| UI snapshots | Offscreen `egui_kittest` goldens per screen/state; English + French spot checks; both themes when tokens/layout change | `midata-app` / `midata-testkit` |
| Behaviour oracles | Legacy scenarios mapped to inventory rows as acceptance criteria — **scenarios, never code** | this file |
| Perf | Flatten 10k animals < 500 ms · dashboard hydrate (10 widgets, 1k animals) < 300 ms · cold start < 2 s · import 1k rows < 10 s | `cargo xtask bench`; baselines in `docs/scorecard.md` |

**Determinism is mandatory:** seeded randomness, seeded demo data, no wall-clock dependence in
assertions (inject the date), no network in tests.

## 2. Oracle mapping table

One row per scenario. `inventory` lists the row ids the scenario guards; `our test` names the exact
test function that must exist before the row can be marked `done`.

| Legacy module | Scenario (input → expected behaviour) | inventory | our test | status |
|---|---|---|---|---|
| *(pending extraction)* | — | — | — | pending |
| `test_database` | fresh DB → migrations apply; applying twice → no change | SY-1, SY-12 | — | pending |
| `test_import` | — | DE-10…DE-14, IX-1…IX-4 | — | pending |
| `test_analytics` | — | AN-1…AN-26, DB-1…DB-13 | — | pending |
| `test_health_reproduction` | — | PD-11…PD-20, OR-18…OR-28 | — | pending |
| `test_pedigree` | — | PD-1…PD-10 | — | pending |
| `test_soft_delete` | — | HM-22, OR-2, EX-9 | — | pending |
| `test_extensions` | — | EX-1…EX-12 | — | pending |
| `test_formula_service` | — | OR-13…OR-17 | — | pending |
| `test_onboarding` | — | OB-1…OB-12 | — | pending |
| `test_ai_chat_service` | — | AI-1…AI-4, AI-17 | — | pending |
| `test_ai_settings` | — | AI-15, AI-16 | — | pending |
| `test_ai_conversations` | — | AI-3 | — | pending |
| `test_tool_query` | — | AI-6…AI-9, AI-19 | — | pending |
| `test_tool_analyze` | — | AN-15…AN-26 | — | pending |
| `test_tool_widget` | — | AI-18, DB-9 | — | pending |
| `test_settings` | — | OR-9…OR-12, SY-1…SY-4 | — | pending |
| `test_launcher`, `test_local_sharing` | — | — (waived, see §7) | — | waived |

Expected legacy volume for calibration (`PROJECT_REPORT.md` §17.2, **not** a target for our test count):
71 tool-query · 37 local-sharing · 36 soft-delete · 34 health/reproduction · 33 extensions ·
30 analytics · 26 pedigree · 25 onboarding · 24 AI-chat · 24 import · 21 analyze · 20 AI-settings ·
19 AI-conversations · 18 formula · 17 launcher · 16 settings · 12 widget · 8 database = **485**.

## 3. Fixtures

| Fixture | Contents | Used by |
|---|---|---|
| `empty_db()` | migrated tempfile DB, no rows | every store/command test |
| `seeded_demo()` | the deterministic demo seed (`REBUILD_PLAN.md` §5.7): ~50 animals, ~2,000 observations, 5-generation pedigree, 2 phantom parents | analytics, pedigree, dashboard, snapshots |
| `flatten_2k()` | synthetic entity/field/observation matrix with deliberately interleaved dates, including same-date ties | latest-value vs brute-force cross-check |
| `perf_10k()` | 10k animals + observation set sized for the flatten budget | perf budgets only (`#[ignore]` by default) |
| `command_harness()` | session + registry + temp DB + audit assertions | every command test |

Rules: fixtures are **generated** (deterministic, seeded), never committed as binary blobs; the demo
seed is the only "realistic" data and contains **only registry preset species** (D9 policy test);
personal data never appears anywhere.

## 4. Snapshot policy ("look at the PNG")

- Harness: `egui_kittest`, offscreen — no window, no focus stealing; `cargo test -p midata-app --
  snapshot`.
- Goldens are committed and regenerated **intentionally**; a regenerated golden travels in the same
  commit as the change that caused it (AGENTS.md §5 git discipline).
- A UI task is not done until an agent has **opened the rendered PNG** and described the result; both
  themes when tokens/layout change; a French spot check when strings change.
- Snapshot-covered screens count as a ratchet (only increases).

## 5. Fuzz: `command_hunt`

Registry-wide adversarial sweep over **every** registered command: empty params, missing required
params, wrong JSON types, out-of-range numbers, absurd string lengths, malformed dates (including
invalid UTF-8 boundaries), unknown ids in foreign-key fields, and a DB whose file was deleted mid-run.
Every outcome must be `Ok` or a typed `CommandError` — never a panic, never a silent data corruption.
Run explicitly in the gate: `cargo test -p midata-engine --test command_hunt -- --ignored`.

## 6. Perf budgets

Measured by `cargo xtask bench`; recorded in `docs/scorecard.md`. A met budget flips to
`enforce = true` and **never un-enforces** (`REBUILD_PLAN.md` §11.8): flatten 10k animals < 500 ms ·
dashboard hydrate (10 widgets, 1k animals) < 300 ms · cold start < 2 s · import 1k rows < 10 s.

## 7. Waivers

| Scenario source | Why waived | Recorded in |
|---|---|---|
| `test_local_sharing` (37), `test_launcher` (17) | LAN/ngrok/tray/packaging surfaces are dropped by design — no server, no browser UI (D2) | `docs/decisions.md` D2, `REBUILD_PLAN.md` §6.2 |
| Legacy dual-engine (DuckDB) analytics tests | Single engine; the behaviour is kept, the second engine is not (D16) | `docs/decisions.md` D16 |
| Legacy AI forced-Arabic response assertions | Mirror-the-user language instead (D10) | `docs/decisions.md` D10 |

A waiver is recorded here **and** in `docs/decisions.md`; a waived scenario is never silently dropped.

## 8. Mapping discipline

- Every `done` inventory row must name at least one test or snapshot path in its `evidence` column.
- Every scenario here must point at inventory ids; a scenario with no row means a missing inventory row
  — add it rather than letting the test float.
- Floors never lower: an oracle scenario that was passing may be re-expressed but not deleted.
