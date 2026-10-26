# Proposal — `REBUILD_PLAN.md` §3 anti-pattern register extension

> **Status:** PROPOSED — owner review only. **Not binding.** `REBUILD_PLAN.md` §3 remains the
> authoritative register until the owner merges rows from this file (A5, D27).
> **Source:** the `## Anti-patterns — never reproduce` sections of the eight oracle files
> (`docs/oracles/*.md`, P0-11), deduplicated against the 15 rows already in §3.
> **Rule:** one row per defect class. A row enters §3 only if it adds a countermeasure §3 does not
> already enforce; otherwise the oracle finding is folded into the existing row (listed in §3 below).

## 1. Proposed new rows

| # | Legacy defect class (where it was observed) | Countermeasure in MiData Native | Enforced by |
|---|---|---|---|
| P1 | **Insights as free-form English prose**, asserted by substring ("not found", "requires") — analytics, assistant | Insights are structured (template key + params + severity) resolved through i18n; tests assert key and params, never text | `midata-analytics` types + i18n tests |
| P2 | **Presentation formatting inside the analytics engine** (histogram bins shipped as range *strings*) — analytics | The engine returns numeric bounds/counts; labels and formatting exist only at the UI edge | typed result structs + snapshot |
| P3 | **One polymorphic, null-filled result envelope** for every analysis (`trend_line: null`, `outliers: {high:[],low:[]}`) — analytics | Typed per-analysis result variants: absence is a variant, not a null field, so a caller cannot read a meaningless key | Rust enums (exhaustive match) |
| P4 | **Contradictory empty/error signalling** — same condition returns data in one path and an empty chart in another; silent "Unknown" buckets; unknown root returns empty success — analytics, pedigree, storage | One contract: typed `CommandError` for invalid input; structured empty result **with a reason code** for valid input with no data; a missing dimension is named in the error, never bucketed as "Unknown" | domain `CommandError` + per-command tests |
| P5 | **Two representations of "no value"** — absent row vs present-but-uncomputable; division by zero and missing input both collapse to null — storage, formula | One vocabulary: `missing`, `not_evaluated`, `error{reason}` are distinct variants; "not applicable / data missing" is never silently `false` | `midata-domain` result types + rule tests |
| P6 | **Validation and evaluation disagree** — a formula accepts a field key its validator rejects, so a typo reads as "no animals have data" — storage | One key-resolution path shared by validation and evaluation; an unresolved placeholder is an error, never a silent empty result | shared resolver + tests |
| P7 | **Identity and pedigree facts stored as observations** (sex/species/breed/birth-date/parents as "just measurements"; ancestry changed by re-recording) — analytics, pedigree, exchange | Identity is columns on `entities`; parent links are `sire_id`/`dam_id`; age is computed (D7). Never an EAV identity fact | schema + AGENTS rule 2 |
| P8 | **A second dispatch surface / second analytics engine** with its own string dispatch and column lists — analytics, assistant, query | One command registry and one analytics engine (D16/D23); a new computation is a new function inside `midata-analytics`, never a parallel executor | `xtask check-layers` + review |
| P9 | **Magic sentinel values encode meaning** (`mode_id = 0` = "no mode"; a sentinel string in an id column; a "role" value that secretly means authorization) — analytics, retention, storage | Absence is `Option`; ownership/purpose are explicit typed columns; protection policy and purpose are separate concepts | domain types + schema |
| P10 | **Query safety by first-keyword / substring matching** — no AST, wildcard and aliases bypass the whitelist, hardcoded source pair, free-string aggregate names, statement type inferred from an English error — query, exchange | `midata-queryguard` only: single SELECT/WITH decided on the AST; wildcard and aliases resolved into the whitelist; sources registry-driven; aggregate functions allowlisted | queryguard suite (AI-6…AI-9) |
| P11 | **Values interpolated into query text** instead of bound — the "injection" test asserted the payload appeared verbatim — query | Every value is a bound parameter; the injection suite asserts refusal *and* an untouched database, never quote-counting | queryguard + AI-19 |
| P12 | **Caps declared but never exercised** — the only cap test asserts the flag is off; size/timeout caps absent — query | Each cap has a test that reaches it (5,000 rows · 50 KB · 500 aggregated · 200-char strings · query timeout) | queryguard cap tests |
| P13 | **Two query languages in one surface** (SQL *and* a DSL) — assistant, query | One language: SQL through the validation lab. No second parser, no second documentation set | architecture + review |
| P14 | **Database identifiers in the public/agent contract** — row ids in a graph payload, internal table names as model enum values — pedigree, assistant, query | One stable animal identity (farm tag); agents see command ids and field keys, never storage ids or table names | command-output types + tests |
| P15 | **Display strings used as identity keys** — import reactivates a widget by title / a mode by name, so renaming or translating duplicates it — retention, exchange | Machine keys are identity; labels are presentation. Match on `key`, never on `label` | store lookups + rename test |
| P16 | **Silent row-level data loss in import/onboarding** — duplicate identifiers merged, rows without an identifier dropped, unknown bundle ids skipped, only aggregate counters reported — exchange, onboarding, pedigree | Per-row diagnostics with source-row index + reason code (`CommandError`), and an explicit refusal for duplicate identifiers inside one file (DE-19/HM-2) | import pipeline tests |
| P17 | **Two install paths with two safety levels** — a field-key collision fatal for a pack but silently tolerated in onboarding — exchange, onboarding | Exactly one field-creation path used by packs, onboarding and the AI proposal flow | `midata-ops` API + test |
| P18 | **Split completion state** — a marker file *and* database rows must agree, and both disagreement cases report "not onboarded" while data exists — onboarding | One source of truth: state is derived from the database; no on-disk marker | `onboarding.status` + test |
| P19 | **Demo or destructive seed on live data without a guard** — demo animals appended beside a real herd — onboarding | Seed is an explicit, confirmed command (`--force` to rebuild), refuses on a non-empty non-seed herd, and stamps `source='seed'` | `seed.demo` tests |
| P20 | **State changed by direct row mutation** — tests flip an active flag or reactivate a record without a command: no audit, no validation, divergent error behaviour — retention, storage | Every state change goes through a registered command; exactly one command per resource class; tests dispatch the real command | `command_hunt` + audit assertions |
| P21 | **Transport concepts as the domain contract** — 400/404 carry domain meaning, errors are English text, refusal classes differ per resource — storage, pedigree, retention | Domain errors are typed `CommandError`s rendered through i18n at the edge; no HTTP/transport type below the app; every rejection carries a machine-readable reason class so agents can branch | domain error tests |
| P22 | **Destructive-ish operations with no audit trail** — deactivate/reactivate/install/uninstall record nothing — retention | Every mutating command writes exactly one `audit_log` row (actor + batch token where applicable), including lifecycle operations | audit-count tests |
| P23 | **Retention with no single canonical active filter** — soft-deleted fields/modes still leak into pickers and re-scoping; neighbouring entities use inconsistent delete semantics | One canonical active predicate used by *every* read surface, and a written per-resource delete policy (soft vs hard) tested on the surface the user sees | store queries + list-surface tests |
| P24 | **Configuration inputs accepted and never exercised** — health toggles no code reads, a custom-species list ignored, widgets created despite the feature being off — onboarding, exchange | Every settings/config input is read by shipped code in the same phase or deleted; feature toggles gate the artefacts they claim to gate | activation test per input + `file-budget` review |
| P25 | **Species defaults hardcoded in code and tests** — gestation literals in Rust, every other species silently gets nothing — pedigree, onboarding | Per-species defaults are registry data; tests read them from the registry, never pin literals (AGENTS rule 4) | no-species-enumeration test |
| P26 | **Closed business vocabularies in code** — twelve fixed purpose tags (one per field), "normal" meaning none, a pregnancy status literal reset inside an action — pedigree | Purpose tags are data-driven, multi-valued and stored in field configuration; status value sets live in configuration, not in an action body | field-config tests + review |
| P27 | **Presentation attributes inside data payloads** — a stroke style proving an edge is dashed — pedigree | Presentation is theme tokens at the UI edge; the analytics/graph payload carries semantics only | payload types + theme review |
| P28 | **Unbounded or non-deterministic traversal** — depth-only guard lets a wide component exceed the node cap; truncation asserts only an upper bound with no selection rule — pedigree | Every traversal carries a visited set, generation bound (1–10) and the 500-node cap **on every path**, with a deterministic truncation rule | pedigree tests on wide fixtures |
| P29 | **Write-time link validation absent** — self-parenting accepted and rendered; blank parent tags normalised silently; unknown root returns empty success — pedigree | Validate sex and self-reference at write time with actionable `CommandError`s; pre-existing bad data is a surfaced data-quality warning; a blank tag is reported, not silently converted | `animal.*`/birth tests |
| P30 | **Typed settings modelled as stringly key/value rows** (temperature stored as text) — assistant, storage | Persisted settings are typed serde structs with `#[serde(default)]`; no locale-dependent parsing of persisted numbers/dates | settings types + round-trip tests |
| P31 | **Inconsistent storage typing papered over with per-query casts** (`CAST(weight AS DOUBLE)` everywhere) — assistant | Observations are typed at write time through domain constructors (one non-NULL column per `data_type`); money is integer minor units end-to-end | value-encoding tests |
| P32 | **Hidden global state and cross-module fixtures** — a module-level cache patched by tests, fixtures inferred from another file, private helpers imported — assistant, query, storage | Dependencies injected (store handle, clock, provider); fixtures come only from `midata-testkit`; behaviour is tested through the public command surface | crate API + review |
| P33 | **Non-deterministic streaming tests** — "there might be an error event (no LLM running)" — assistant | The provider is stubbed; the test asserts the terminal event deterministically, with no network and no timing assumptions | `midata-ai` stub-provider tests |

## 2. Proposed test-integrity rows (cross-cutting)

The largest single defect class in the legacy suite is **coverage that cannot fail**. It is worth its
own register row rather than living only in review culture.

| # | Legacy defect class (where observed) | Countermeasure | Enforced by |
|---|---|---|---|
| P34 | **Vacuous or tautological assertions** — empty test bodies, `assert a or b`, "atomicity" tests that inject no failure and assert success, a "does not partially commit" test asserting both rows commit, duplicate tests under different names, mismatched names/behaviour (asserted in six oracle files) | Every test states the boundary it exercises in its name and asserts the negative case; an atomicity test injects a failure **and** asserts zero rows; duplicates are merged, not counted | review + `command_hunt` + ratchet on test count with named evidence |
| P35 | **Structure-only acceptance** — profitability/dashboard assert key presence only; "type supported" proven by whichever pack happens to use it; pack totals pinned to magic numbers | Acceptance asserts the value (money in minor units, metric formula documented) and invariants ("every declared field installed"), never key presence or magic totals | oracle-mapped tests |
| P36 | **Tests coupled to internals** — driver/table names, a specific registered executor, private whitelist helpers, storage-layer-only assertions for user-visible exclusion | Tests go through the command surface and assert what the user/agent sees; fixture DB comes from `midata-testkit` | review + layer check |
| P37 | **Module named for a surface it does not test** (the single most misleading artefact in the legacy suite) | A test file's name states the behaviour under test; a mismatch is a defect, not a convention | review |

## 3. Already covered by existing §3 (fold in, do not duplicate)

| Oracle finding | Existing §3 row that already enforces it |
|---|---|
| Secret-encryption key from a hard-coded constant; API key encrypted into the app DB; credential file on disk | Row 8 — secrets in the OS keyring only |
| In-memory confirmation tokens with no persistence/expiry/single-use | Row 9 — `pending_changes` persisted with expiry |
| Inert `rule_type` variants; dead configuration/health toggles as schema | Row 4 — every enum variant implemented; row 3 — no dead anything |
| Date-blind flattening; N+1 listing | Rows 1–2 |
| Business logic in controllers; empty crates | Row 5 — registered layer DAG |
| Hand-maintained numbers drifting; docs ≠ code | Row 6 — generated scorecard, CI-diffed |
| Committed SPA bundle / launcher console as UI | Row 7 + D2 (no browser UI, no launcher) |
| Monolith modules | Row 11 — file budget |
| Mixed lockfiles / dependency sprawl | Row 12 |
| Storage/theme key mismatch | Row 13 |
| Zero tags, no CI, tolerated lint | Row 14 |
| Agent context exhaustion | Row 15 |

## 4. How to merge (owner action)

1. Accept, reword or reject each row — rows are numbered `P1…P37` for this review only; on merge they
   are appended to §3 in thematic order (analytics → query → AI → data I/O → pedigree → storage →
   test integrity) with stable, non-numbered keys.
2. The merge is a `REBUILD_PLAN.md` §15 amendment (bump header, add a decision row) because §3 is
   binding.
3. After merge, each oracle file's `## Anti-patterns` section keeps its provenance text but gains a
   pointer to the §3 row it feeds — one authoritative home, no duplication (D27).
