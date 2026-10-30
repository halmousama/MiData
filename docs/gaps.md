# Gaps — the ranked work queue

> **Version:** v1 · **Updated:** 2026-10-30 (P0-2 re-land closed: `midata-domain` + `midata-i18n` are on `main`, the i18n gate is fatal; P0-14 and the P0-2f docs truth pass closed earlier the same day) · **Phase:** 0 — Foundations
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
| 1 | P0-1 | Workspace + `xtask` + CI | infra | Unblocks every later task; the gate must exist before code does | 10 | — | **done** (2026-10-27) |
| 2 | P0-2f | Docs truth pass: roadmap focus line, closure-stamp DoD rule, P0-14 promotion | docs | Every session boots from a stale "current focus" | 1.5 | — (D31 merged) | **done** (2026-10-30, PR #6) |
| 3 | P0-14 | D30 mechanical enforcement: `commit-msg` hook + vendor/model term list | tooling | D30 is honour-system until the hook and the second term list exist | 3 | — (D30 adopted 2026-10-27) | **done** (2026-10-30, PR #7) |
| 4 | P0-2 | `midata-domain` + `midata-i18n` (re-land) | feature | Types, value rules and the string layer that everything else consumes | 8 | P0-1 | **done** (2026-10-30, PR #8) |
| 5 | P0-3 | `midata-store` + migrations | feature | Schema v1, latest-value query, rebuild's data foundation | 12 | P0-2 | open |
| 6 | P0-4 | `midata-engine` skeleton | feature | Command registry + audit hook — the agent-first seam | 8 | P0-2, P0-3 | open |
| 7 | P0-5 | `midata-app` shell + theme | feature | First pixels: window, router, tokens, settings persistence | 12 | P0-2, P0-4 | open |
| 8 | P0-6 | Onboarding + demo seed | feature | Makes the app usable on first run; exit-gate dependency | 8 | P0-3, P0-4, P0-5 | open |
| 9 | P0-7 | `midata-cli` skeleton | feature | `commands` / `run` / `seed` — agent drivability from a terminal | 3 | P0-4, P0-6 | open |
| 10 | P0-8 | Inventory v0 reconciliation (re-scoped) | docs | Turns the hand-written checklist into a machine-checkable contract | 2 | — | open |
| 11 | P0-9 | Snapshots + gates polish | quality | Locks the Phase-0 exit criteria and the ratchets in place | 6 | P0-5, P0-6 | open |
| 12 | P0-10 | `AGENTS.md` diet to a lean bootloader ≤ ~12 KB | docs | The bootloader is read every session; it is currently 36.4 KB (35.6 KiB) against a ~12 KB target | 3 | — (policy approved 2026-10-27, D27) | open |

**Status vocabulary:** `open` · `done` (always with its closure date and evidence) · `escalated` (a task
carrying an `AGENTS.md` §12 condition-3 escalation flag (D31): this row's Status reads `escalated`, or a
local `plan/ESCALATE` marker exists alongside `plan/HOLD`).

**Sum:** 54 agent-hours open (72 at queue start; P0-1 closed 10 h; P0-2f 1.5 h and P0-14 3 h added and
closed 2026-10-30; **P0-2 re-landed and closed 8 h**, 2026-10-30). Of the eight open rows, three are
ready (`P0-3`, `P0-8`, `P0-10`) and five are blocked behind the store → engine → app chain. Phase 0
closes when every open row is `done` and the exit criteria in `docs/roadmap.md` are met. P0-11 (oracle
extraction, 3 h) closed on 2026-10-26, P0-12 (owner rulings, 1.5 h) on 2026-10-27, P0-1 on 2026-10-27,
P0-2f and P0-14 and P0-2 on 2026-10-30 — see §Closed.

---

## P0-1 — Workspace, `xtask`, CI  ·  10 h  ·  rank 1  ·  **done 2026-10-27**

**Kind:** infra · **Source:** `REBUILD_PLAN.md` §4.1/§4.2/§13.1, Appendix C bullets 1, 3, 5.

**Scope decision (taken under autonomy, recorded here).** The recommended split in §Scope was chosen:
P0-1 creates the workspace + `xtask` **only**; product crates are birth-gated per task (D26). Only
`anyhow` and `serde_json` are workspace dependencies because only `xtask` uses them — no pre-populated
dependency table, which would be a dead manifest by another name.

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
  **Snapshots are not delivered by P0-1** (no snapshot step exists; the earlier bullet was aspirational):
  tracked as known-shortfall 7, target P0-9, not left in prose.
- `rumetrics` grep gate (zero occurrences) as a `xtask ci` step.

**Delivered (evidence).**
- `cargo xtask ci` green locally: `cargo fmt --all --check` · `clippy --workspace --all-targets -D
  warnings` · `nextest run --workspace` (**43 tests**) · `check-layers` · `file-budget` ·
  `i18n-coverage` · banned-name grep · `stats` + committed-scorecard diff · `cargo deny` ·
  `cargo machete`. The PR's **ubuntu + windows jobs are green** — PR #2, run
  [40419756246](https://github.com/halmousama/MiData/actions/runs/40419756246) (ubuntu 22 s,
  windows 56 s) — the first real two-OS proof; P0-12 could not provide it. Re-verified on the round-1
  fix commit `b7035e7`: run
  [40421245292](https://github.com/halmousama/MiData/actions/runs/40421245292) green on both OSes
  (ubuntu 20 s, windows 43 s).
- **Registered DAG:** 14 crates registered in `xtask/src/layers.rs` (13 planned + `xtask`), with birth
  phase and manifest home per crate; intra-layer edges (`ops → analytics/exchange`,
  `engine → ops`, `ai → queryguard`) included. Unregistered crates and unregistered external
  dependencies fail. Application roots are not libraries: `midata-cli` has an explicit allowlist
  (domain, i18n, store, engine) and can never link the egui shell, while `apps/midata → midata-app` is
  the one declared app-to-app edge (round-1 **R1**, `APP_EDGES`).
- **Every gate shown to bite** (intentional violation, caught, reverted — output quoted in the PR body):
  layer check (unregistered `clap` edge; app-edge rules proven by `ui_composition_root_may_depend_on_the_app_shell`,
  `headless_cli_may_not_depend_on_the_ui_shell`, `application_crate_cannot_be_a_dependency`),
  file budget (501-line file + stale-baseline unit test), i18n coverage (stale `no-catalogues`
  baseline; FR-missing ratchet regression; **EN-total shrink**, round-1 R4), stats (hand-edited
  scorecard fails `--check`; golden filter and done-with-evidence numerator unit-tested, R2/R3),
  banned-name grep (term in a shipped file), `ci` (format violation stops the run before any later
  step), `cargo-deny` (temporary GPL-licensed path dependency → licence not allowed) and
  `cargo-machete` (temporary unused path dependency → unused dependency), both appended to the PR body
  for round-1 R5.
- **Baselines committed with zero violations:** `docs/file-budget.baseline` (empty),
  `docs/i18n-coverage.baseline` (`state: no-catalogues`, loud skip until P0-2),
  `docs/scorecard.md` (GENERATED, 0/222 inventory, **43 tests**, 1 crate, 0 golden screens of the 11
  in `REBUILD_PLAN.md` §7.1, 0 budget violations, 0 clippy/unsafe). No timestamps in generated output,
  so ubuntu and windows diff identically.
- **Missing-tool skip is kind-based, not text-based** (self-check, owner-approved 2026-10-27):
  `ci::is_missing_binary` classifies a `SKIP` from the spawn `io::ErrorKind::NotFound` only, so an
  installed tool that fails and prints "No such file or directory" itself (a missing `deny.toml`) can
  no longer be downgraded to a local skip — §3.8 `test-integrity.no-vacuous-gates`. Regression tests:
  `ci::tests::a_failed_run_is_never_downgraded_to_a_missing_binary` (fails against the string-matching
  predicate — the negative proof) and `ci::tests::a_binary_that_cannot_be_spawned_is_classified_missing`.
- **Round-1 review fixes (2026-10-27, PR #2 comment
  [6430093550](https://github.com/halmousama/MiData/pull/2#issuecomment-6430093550)):** R1 app-edge rule
  + explicit CLI allowlist (three new layer tests, 37 → 43 tests overall); R2 coverage numerator =
  `done` rows with non-empty evidence (parser test); R3 goldens counted only under
  `crates/midata-app/**/snapshots/**` with the target set to the 11 §7.1 screens; R4 EN-total ratchet
  enforced (no stored-but-ignored baseline field); R5 deny + machete negative proofs; R6 snapshot
  deferral moved to known-shortfall 7; S1 birth is "declared & printed, enforced by owner review
  (D26)"; S2 orphan FR keys fatal once catalogues exist, wired into P0-2 acceptance; S3 scorecard says
  "enforced by the gate"; S4 PR body stale count fixed; S5 CI actions pinned to commit SHAs and the
  architecture header aligned with §9.
- **Design seams planned for P0-2+:** `docs/architecture.md` §9 records clock / store-handle /
  provider / actor injection through `Session`, no globals and no module-level connection.
- **Owner TODO (also in the devlog):** after this PR merges, enable **Require status checks** in branch
  protection for `ci (ubuntu-latest)` and `ci (windows-latest)`.

**Risks.** Dependency approval: only `anyhow` + `serde_json` were added (both in Appendix A), so no new
escalation — `cargo deny` runs in CI. Windows parity is verified by the matrix on every PR; the P0-1 PR
is the first run of that matrix. The Linux snapshot step is deferred to P0-9 (known-shortfall 7), not
claimed here.

---

## P0-2f — Docs truth pass: roadmap focus line, closure-stamp DoD rule, P0-14 promotion  ·  1.5 h  ·  rank 2  ·  **done 2026-10-30 (PR #6)**

**Kind:** docs · **Source:** measured defect found while bootstrapping the D31 cycle — the roadmap header
had been stale since PR #3 was closed unmerged. (PR #3 carries **no review record at all**: 0 reviews,
0 review comments, 0 issue comments, and its timeline holds only commits plus `closed` — so no review
finding is cited here, and the re-land needs a fresh review.)

**Why now.** Every session reads `docs/roadmap.md` first to learn the current focus. Since PR #3 was
closed unmerged, that line named P0-10 then P0-2 as "next" while `main` contained no product crates at
all, so each session booted from a false picture of the repository. A wrong boot state costs every
future task; it is cheap to correct and it is the precondition for the re-land chain.

**Scope.**
- `docs/roadmap.md`: truthful `Updated:` stamp and "Current focus" — `main` has **no `crates/`
  directory**, P0-2 lives on `t/p0-2-domain-i18n` (remote tip `cb4405c`), PR #3 was **closed unmerged on
  2026-10-27T09:35:38Z**; focus order P0-2f → P0-14 → P0-2 re-land → P0-3, with P0-3's block on P0-2
  stated.
- `docs/contributing.md`: the DoD gains the closure-stamp rule (header stamp + current-focus line are
  updated at task closure).
- `docs/gaps.md`: P0-14 promoted from known-shortfall 8; ranks renumbered; P0-2's Status corrected;
  Status vocabulary documented; the P0-10/§12 pointer follow-up wired in; shortfall 8 traced to its
  discovery on the P0-2 branch.

**Non-goals.** No product code, no schema, no dependency change. The task record declared four docs
(`docs/roadmap.md`, `docs/contributing.md`, `docs/gaps.md`, `log/devlog.md`); the **scope amendment
declared by the supervisor in review round 1** adds root `ROADMAP.md`, because it is the first doc every
session reads (AGENTS.md §4 point 0) and it still carried a wrong boot state ("next task **P0-10**",
"9 tasks queued, none blocked"). `ROADMAP.md` is therefore re-stamped in the same task, not left as
debt; the amendment is stated in the PR body and is not silent creep.

**Evidence.** The three state facts re-verified at write time with `gh pr view 3 --json
state,mergedAt,closedAt`, `git ls-tree --name-only origin/main crates/` (empty) and `git ls-remote
origin refs/heads/t/p0-2-domain-i18n` (`cb4405c`); `cargo xtask ci` green; devlog entry 2026-10-30.

---

## P0-14 — D30 mechanical enforcement: `commit-msg` hook + vendor/model term list  ·  3 h  ·  rank 3  ·  **done 2026-10-30 (PR #7)**

**Kind:** tooling · **Source:** measured shortfall 8 (found 2026-10-27 during P0-2 on branch
`t/p0-2-domain-i18n`, promoted here 2026-10-30); `REBUILD_PLAN.md` §15 **D30**; `AGENTS.md` §5.

**Evidence of the gap.** `AGENTS.md` §5 forbids attribution trailers and AI-vendor/model names anywhere,
and then states plainly: "Review-enforced today: the `commit-msg` hook and the vendor/model term list in
the banned-names gate **do not exist yet**" (D30: "Review-enforced **until** … land"). Neither exists:
`.git/hooks/` holds no `commit-msg` hook, and `xtask/src/gates.rs::banned_names` greps only the legacy
product name, not trailers or vendor/model names. The rule is honour-system today (it holds only because
reviewers apply it by hand).

**Scope.**
- (a) **Checked-in hook + documented install.** A `commit-msg` hook template under `tools/hooks/` that
  rejects a message containing an attribution trailer or a banned vendor/model name. `.git/hooks/` is
  never committed, so the same change adds the activation step to the contributor docs (copy/symlink the
  template and make it executable) — documented, not assumed.
- (b) **Second term list in the banned-names gate.** `xtask/src/gates.rs` grows a list covering
  (i) `Co-Authored-By` trailers in commit messages and (ii) AI vendor/model names in operational docs.
  **D30 self-reference trap:** the gate scans the tree, so the literal terms must not appear in a file
  the gate scans. The literals are assembled/split in code (e.g. `xtask/src/terms.rs`, built from
  fragments) so the source that defines them cannot trip the gate itself; the docs describe the
  mechanism only and never print the names. This is an acceptance test, not a comment. **Precedent in
  the tree:** the legacy-name gate already does exactly this — `xtask/src/gates.rs:31`
  `concat!("ru", "metrics")` assembles the term, and `xtask/src/gates.rs:92`
  `gate_source_does_not_contain_its_own_pattern` is the test that keeps it true. The new list follows
  that pattern rather than inventing one.
- (c) **Baseline sweep + committed allowlist.** Run the new list over the whole tree once, commit the
  pre-existing violations in a named baseline file — `docs/banned-names.baseline`, mirroring
  `docs/i18n-coverage.baseline` and `docs/file-budget.baseline` — and make the gate fatal from day one.
  The gate prints a count of allowlisted entries; that count may only shrink (a `cargo xtask
  banned-names` step inside `xtask ci`, with a unit test that fails if the stored count exceeds the
  measured one, exactly as the i18n and file-budget ratchets do), and every new violation fails.
- (d) **Exemptions.** The path allowlist **mirrors the existing gate's own allowlist** —
  `xtask/src/gates.rs::is_allowed_path` / `ALLOWED_PATHS`: `AGENTS.md`, `REBUILD_PLAN.md`,
  `docs/decisions.md`, `docs/gaps.md`, all of `docs/legacy-spec/` — **plus `ARTCRAFT_INVESTIGATION.md`**
  (which legitimately quotes external projects but is not in today's list, so adding it is an additive
  change to `ALLOWED_PATHS`, stated in the P0-14 PR). Everything else in the tree is in scope.
  **Reference-vs-attribution rule:** the term list targets *attribution* — trailers and claims of
  authorship/provenance — not *reference*. A vendor name naming an integration target is a reference:
  `docs/architecture.md:15` names an LLM vendor only to define the API dialect the app speaks, which is
  a compatibility target, not an authorship claim, and it must not fail the gate. New hits are triaged
  into the baseline with the reason recorded: `reference` entries are allowed with that stated reason,
  attribution entries fail outright. Without this rule the "clean full-tree run" criterion is unpassable
  as scoped — it would fail on day one at that line.

**Acceptance criteria.**
- The hook rejects a synthetic bad message (attribution trailer, or a banned vendor/model term) and
  accepts a synthetic good one; the negative proof is in the PR body.
- The gate fails on a synthetic introduced violation and passes clean once it is removed.
- The allowlist file is committed; the gate reports its count, and a test asserts the count only shrinks.
- The term literals live where they cannot trip the gate (assembled/split in code), proven by the clean
  full-tree run.

**Evidence.** Hook tests, gate negative proof, allowlist file + count, `cargo xtask ci` green, install
paragraph in the contributor docs.

**Delivered (2026-10-30, PR #7).**
- (a) `tools/hooks/commit-msg` — POSIX `sh`, no TTY, no prompt; it runs
  `cargo xtask commit-msg <message-file>` (or `$XTASK_BIN` when a built binary is supplied) and **fails
  closed** when it cannot run. Install step documented in `docs/contributing.md` §6.
- (b) `xtask/src/terms.rs` — the D30 term list, every literal assembled from fragments, with
  `term_literals_never_appear_in_their_own_source` as the acceptance test; the scan lives in
  `xtask/src/attribution.rs` and is wired into `cargo xtask banned-names` (run by `xtask ci`).
  Matching is case-insensitive and word-bounded (the first baseline sweep caught a tunnel product's
  name ending in a model name — a substring match was a false positive).
- (c) `docs/banned-names.baseline` — **5 allowlisted entries** at birth (`path<TAB>term<TAB>reason`),
  all triaged `reference`: `docs/architecture.md` (the API dialect the app speaks),
  `docs/contributing.md` and `docs/review-templates.md` (documents that state the ban), and
  `log/devlog.md` twice (the history-scrub record and the policy line it quotes). The gate prints the
  count on every run; new hits fail; stale entries fail until
  the baseline is regenerated, and an `attribution` reason is rejected by the parser, so the list can
  only shrink.
- (d) `ARTCRAFT_INVESTIGATION.md` added to `ALLOWED_PATHS` (additive; stated in the PR body), and the
  allowlist file itself is exempt because it must name the terms it allows. Reference hits are
  allowlisted with their reason; attribution hits are never allowlisted.
- Negative proofs in the PR body: the hook rejects a synthetic bad message (and accepts a good one),
  and the gate fails on a synthetic introduced violation and passes once it is removed.
- `cargo xtask ci` green locally (26 new tests: 21 unit + 5 hook) and both PR jobs green on
  [run 41129162523](https://github.com/halmousama/MiData/actions/runs/41129162523) (ubuntu + windows).
- **Owner clarification requested 2026-10-30 (PR #6 R7a); implemented per this row pending
  confirmation.** The reference-vs-attribution rule as written narrows binding D30 to attribution
  contexts; this task implements that narrowing exactly and does not amend D30's text.

**Review #7 round 1 (REQUEST_CHANGES, 0B/1M/4m/4s) — fixed on the same branch.** R1 (major): the hook
now truncates the message at Git's scissors line, so `git commit -v` no longer rejects a clean message
whose diff mentions a term. R2: the baseline generator keeps `TermKind` and refuses to allowlist an
attribution hit. R3: both name scans also cover bounded extensionless UTF-8 files. R4: the vendor stem
is matched word-bounded, so model suffixes (`-4o`) are caught. R5: the test count is corrected to 21
unit + 5 hook = 26. R6–R9 recorded (run id, docs-truth and Windows-hook shortfalls, baseline-header
note).

---

## P0-2 — `midata-domain` + `midata-i18n` (re-land)  ·  8 h  ·  rank 4  ·  **done 2026-10-30 (PR #8)**

**Kind:** feature (L0) · **Source:** `REBUILD_PLAN.md` §4.1, §5.3, §5.5, §7.4; Appendix C bullets 5–7.

**Status (measured 2026-10-30).** The implementation exists on branch
`t/p0-2-domain-i18n` (remote tip `cb4405c`, a single commit — the remote branch carries no separable
"review fix" commits), but PR #3 was **closed without being merged**
(`closedAt` 2026-10-27T09:35:38Z, `mergedAt` null), so **`main` has no `crates/` directory** and none of
this work is on `main`. The 8 h stays on the open sum; the task is queued for **re-land as a fresh PR**
(this entry is the queue record, not a new scope). Everything in the scope below was implemented and
reviewed on that branch.

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
- **Acceptance carried from round-1 S2:** once both catalogues exist, an **orphan FR key** (present in
  FR, absent in EN) is a **fatal** gate failure, not a printed report line — `docs/i18n.md` §5: orphan
  keys are deleted, not parked. The first real baseline written in P0-2 must record `en_total`, so the
  R4 non-decrease ratchet is live from the day the catalogues appear.

**Evidence.** Unit test names in the crate; `cargo xtask i18n-coverage` reporting a real number;
per-crate README public-API sections filled in; `docs/i18n.md` updated with the convention actually
enforced.

**Delivered — re-land (2026-10-30, PR #8).** The original implementation (`cb4405c`) was merged
forward, not rewritten: `git merge origin/main` brought D31, the P0-2f docs truth pass and P0-14's D30
enforcement onto the branch, and the fresh PR is the re-land. Conflicts were resolved keeping both
intents — `xtask/src/ci.rs` runs **both** P0-2's fatal `i18n-coverage` step and P0-14's
`banned-names` step; `docs/gaps.md`/`docs/scorecard.md`/`log/devlog.md` took main's newer content plus
the branch's entries. Each acceptance bullet was re-verified on the merged tree (not assumed):
- **Domain:** `SpeciesKey` is a shape-only newtype (`any_species_key_is_accepted_as_data` parses
  `camel`/`horse`/`yak-2`, so an enum of presets would fail the test); `Sex` (nullable at the schema
  level), `Status` + `validate_exit_date` (both directions), `DataType`, `Cardinality`;
  `Value`/`EncodedValue` exactly-one-column with type matching (`value_rules.rs`, 17 tests incl.
  hostile `multi_select` payloads that must not panic); `CommandError` reason + i18n key + params
  (`seams_and_errors.rs`); `Age::between` leap-day clamp (`time_money_pedigree.rs`); `Money` integer
  minor units, checked add/sub, currency conflict (`money_is_integer_minor_units`,
  `money_rejects_mixed_currencies_and_overflow`); `PedigreeBounds` 1–10 / 500 constants.
- **i18n:** `en.tsv` + `fr.tsv` embedded; a miss returns the key itself; `Catalogue::coverage()`
  exposed; `every_en_key_has_a_fr_row`, `neither_catalogue_has_orphan_or_duplicate_rows`,
  `keys_follow_the_documented_prefixes` (the naming-convention test).
- **Fatal gate (round-1 S2):** `xtask/src/i18n/gate.rs` makes an orphan FR key, an unused EN key, an
  `en_total` decrease and a growing FR gap each a **build break**; 11 unit tests, including
  `an_orphan_fr_key_fails_the_gate` and `an_orphan_fails_even_when_fr_coverage_is_complete`.
  `docs/i18n-coverage.baseline` records the real `state: catalogues / en_total: 19 / fr_missing: 0`,
  so the R4 non-decrease ratchet was live from the day the catalogues appeared.
- **Three negative proofs on the merged tree** (temporary catalogue edits, each reverted): missing FR
  row → `FR coverage regressed: 1 missing key(s), baseline allows 0`; orphan FR key → `orphan FR
  key(s) …: err.orphan.row`; key removed from both → `EN catalogue shrank: 18 key(s), baseline
  records 19`.
- **Gate:** `cargo fmt --check`, `clippy -D warnings`, `nextest` (163 tests), `check-layers`,
  `file-budget` (40 files, 0 over cap), `i18n-coverage` (EN 19, FR 19/19), `banned-names` (0 new
  hits) and `cargo xtask ci` all green.
- **Windows gate fix (found by this PR's CI, same task):** after the merge, `windows-latest` failed in
  the `nextest` step with `failed to remove file target\debug\xtask.exe — Access is denied`: `cargo
  xtask ci` *is* that executable, and clippy running before the test step dirtied the bin fingerprint
  so `cargo test` tried to replace a locked running file. `xtask/src/ci.rs` now runs nextest before
  clippy (the bin `cargo run` just built stays fresh), with a source-scan regression test
  (`tests_run_before_clippy_so_the_running_binary_is_never_relinked_on_windows`). Both OS jobs green.
- **D30 compliance:** the one branch text that printed a banned trailer literal in the devlog was
  rephrased; no baseline entry was added.

---

## P0-3 — `midata-store` v0: engine, migrations, latest-value queries  ·  12 h  ·  rank 5

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

## P0-4 — `midata-engine` skeleton: registry, session, audit  ·  8 h  ·  rank 6

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

## P0-5 — `midata-app` shell: window, router, theme, settings  ·  12 h  ·  rank 7

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

## P0-6 — One-step onboarding + deterministic demo seed  ·  8 h  ·  rank 8

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

## P0-7 — `midata-cli` skeleton  ·  3 h  ·  rank 9

**Kind:** feature (`apps/midata-cli`) · **Source:** `REBUILD_PLAN.md` §10, Appendix C bullet 11.

**Scope.** `midata commands` (list ids + params docs + enabled state), `midata run <id> --params <json>`
(actor `cli`, audit recorded), `midata seed --demo [--force]`. Human-readable output by default, `--json`
for agents. Exit codes distinguish usage error from command error. No domain logic in the binary — all
through the registry.

**Evidence.** CLI integration test running a read and a write command against a temp DB; `midata
commands` output snapshot; audit rows show actor `cli`; README with the verb list.

---

## P0-8 — Inventory v0 reconciliation (re-scoped)  ·  2 h  ·  rank 10

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

## P0-9 — Snapshots + gates polish  ·  6 h  ·  rank 11

**Kind:** quality · **Source:** `REBUILD_PLAN.md` §7.5, §11.4, §12, Phase 0 exit criteria.

**Scope.** `egui_kittest` snapshot harness in `midata-testkit` (offscreen, no window/focus stealing);
goldens for shell + onboarding in EN **and** FR, light and dark where layout matters; a documented
"look at the PNG" step in `docs/development.md`; the full gate block (`cargo fmt --check` → clippy →
nextest → check-layers → file-budget → i18n-coverage → snapshots → bench --quick → stats) wired into
`xtask ci` and into CI; **a Linux-only CI snapshot job** (the step P0-1 deferred — known-shortfall 7);
ratchet baselines recorded (test count, coverage, file-budget violations = 0).

**Evidence.** Goldens committed and visually reviewed (devlog notes "viewed"); `cargo xtask ci` green on
both OSes; baseline numbers in `docs/scorecard.md`.

---

## P0-10 — `AGENTS.md` diet to a lean bootloader  ·  3 h  ·  rank 12

**Kind:** docs · **Source:** `REBUILD_PLAN.md` §1.3 bullet 7, §11.1, Appendix C bullet 2; amended policy
`REBUILD_PLAN.md` §15 **D27** (owner ruling 2026-10-27).

**Why rank 12 (re-ranked 2026-10-30).** P0-2f removes the acute wrong-focus tax, and the unblocking
chain (P0-14 → P0-2 re-land → P0-3 → …) now outranks the diet; this stays queued, not dropped.

**Policy (amended, owner-approved).** `AGENTS.md` is a **lean bootloader**: iron rules + orientation
only — **rules not procedures; one authoritative home per topic**; target **≤ ~12 KB** (raised from the
original 8 KB). `AGENTS.md` may summarize and point at a doc; it must never duplicate that doc's
content. Every rule keeps exactly one authoritative home — usually `docs/contributing.md`,
`docs/development.md` or `docs/architecture.md`, which already carry most of the mechanical detail, so
this stays mostly deletion.

**Evidence of the gap.** Measured 2026-10-30: `AGENTS.md` is **36.4 KB** (35.6 KiB), ~3.0× the amended
target. Every session pays that cost before doing any work — the exact failure mode §1.3 bullet 7 was
written to prevent.

**Scope.** Keep: the non-negotiable owner policies, the never-crash rules, the command/EAV/latest-value/
identity rules, the escalation list, the Git discipline block, the orientation table. Move out (with a
pointer left behind): the workspace map, the add-a-command / add-a-screen checklists, the gate command
block, the parallel-agent rules, the record-keeping prose. Where a doc lacks the authoritative text,
move it there *first* (one home), then collapse AGENTS.md to a reference. One further move belongs in
this diet: §12's six-condition merge list should collapse to a pointer at `docs/review-templates.md`
(the bootloader is now **36 436 B** = 3.0× P0-10's target, and the conditions already have an
authoritative home in the template).

**Constraint.** AGENTS.md is the constitution; the owner approves the final shape (owner-review diff
before commit, not a §7 escalation item).

**Acceptance criteria.** `wc -c AGENTS.md` ≤ ~12 288 bytes · **every rule survives** (rule-by-rule
traceability table in the PR body, each row naming its new authoritative home) · no topic has two
homes (grep for the moved headings) · orientation table still points at every doc · before/after byte
counts in the devlog.

**Evidence.** `wc -c AGENTS.md`; traceability table in the PR body; devlog entry with the two byte
counts.

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
| P0-1 | Workspace + `xtask` gates + ubuntu/windows CI | 2026-10-27 | `Cargo.toml`, `Cargo.lock`, `xtask/` (43 tests), `xtask/src/layers.rs` (registered DAG), `docs/file-budget.baseline`, `docs/i18n-coverage.baseline`, `docs/scorecard.md`, `.github/workflows/ci.yml`; PR body carries the per-gate negative proofs; `log/devlog.md` |
| P0-2f | Docs truth pass: roadmap focus line, closure-stamp DoD rule, P0-14 promotion | 2026-10-30 | `docs/roadmap.md` (focus + `Updated:`), `docs/contributing.md` (closure-stamp DoD), `docs/gaps.md` (P0-14 promoted, vocabulary, shortfall 8), root `ROADMAP.md` re-stamp under the declared round-1 scope amendment; PR #6; `log/devlog.md` |
| P0-2 | `midata-domain` + `midata-i18n` (re-land) | 2026-10-30 | `crates/midata-domain/` (identity/value rules/age/money/pedigree/errors/clock + tests), `crates/midata-i18n/` (EN+FR catalogues, `t!`/`tf!`, coverage), `xtask/src/i18n/` (fatal gate, 11 tests), `docs/i18n-coverage.baseline` (`en_total: 19`), `docs/i18n.md` §1/§5/§6, `docs/architecture.md` §9 seams, `xtask/src/layers/rules.rs`; PR #8 (re-lands `cb4405c`; predecessor PR #3 closed unmerged); `log/devlog.md` |
| D29 | Independent reviewer loop per PR | 2026-10-27 | `REBUILD_PLAN.md` §15 D29; `docs/decisions.md` D29; `docs/review-templates.md` (REVIEWER, FIX-ROUND); `docs/contributing.md` §9 |
| D30 | Attribution repealed + `main` history scrubbed | 2026-10-27 | `REBUILD_PLAN.md` §15 D30; `docs/decisions.md` D30; `docs/contributing.md` §6; `log/devlog.md` |
| D31 | Delegated merge authority and the autonomous cycle | 2026-10-27 | `REBUILD_PLAN.md` §15 D31; `AGENTS.md` §12; `docs/decisions.md` D31; `docs/review-templates.md` (SUPERVISOR, IMPLEMENTER kickoff); `log/devlog.md` |

---

## Known shortfalls discovered during bootstrap (not yet tasks)

Items that are real but too small — or too dependent — to queue on their own. Promote to a `P<phase>-n`
entry when they grow.

1. ~~**`docs/scorecard.md` is a stub.**~~ **Resolved 2026-10-27 (P0-1):** `cargo xtask stats` generates
   the scorecard deterministically and CI diffs it. `ROADMAP.md` quotes it.
2. ~~**No `deny.toml` / `Cargo.lock` yet**~~ **Resolved 2026-10-27 (P0-1):** both committed; `cargo
   deny check` is a CI gate.
3. **`docs/mobile-access-strategy.md` is absent by design** (Phase 7, C8/D2). Any early attempt to add
   axum or a server is a scope violation, not a gap.
4. ~~**Oracle mapping is empty** (`docs/testing.md` skeleton).~~ **Resolved 2026-10-26 (P0-11):** 470
   scenarios extracted into `docs/oracles/`, 433 kept + 37 waived, mapped to 222 inventory rows. What
   remains is the *other* half of the parity measurement: no test yet cites a scenario id in an
   inventory `evidence` column, so coverage still reads 0% until Phase 0/1 code lands.
5. **`docs/legacy-spec/PROJECT_REPORT.md` is committed as the behaviour spec.** It is reference-only and
   must never become an implementation source; if any agent starts quoting its Python/React/Git layout
   content as a build guide, that is a rule-5 violation to stop immediately.
6. **The declared MSRV is unverified** (found 2026-10-27, self-check while clearing PR #2). The root
   workspace declares `rust-version = "1.80"`, but nothing proves it: `rust-toolchain.toml` pins the
   moving `stable` track and CI runs whatever stable resolves to, so a dependency or language feature
   newer than 1.80 could land and only be caught when someone builds on 1.80. *Evidence:* `Cargo.toml`
   (`rust-version`), `rust-toolchain.toml` (`channel = "stable"`), `.github/workflows/ci.yml` (no MSRV
   job). *Fix options:* a pinned `1.80` CI job, or drop the claim and state "stable only" explicitly —
   both are cheap; the owner picks one when the first product crate adds a real dependency risk.
7. **CI has no snapshot step** (found 2026-10-27, round-1 review of PR #2, R6). The P0-1 Scope bullet
   promised "snapshots on Linux" and the workflow header repeated it, but `.github/workflows/ci.yml`
   runs only `cargo xtask ci` and `xtask/src/ci.rs` has no snapshot section — the deferral lived in PR
   prose and a YAML comment, which is where deviations go to die. *Target:* **P0-9** (snapshot harness
   goldens + a Linux-only CI step). *Evidence:* `.github/workflows/ci.yml` (no snapshot step),
   `xtask/src/ci.rs` (no snapshot section), P0-9 scope in this file.
8. **D30 is not enforced mechanically yet** (found 2026-10-27 during P0-2 on branch
   `t/p0-2-domain-i18n`; **promoted to P0-14 on 2026-10-30** — it was never on `main` before this
   promotion). `AGENTS.md` §5 forbids attribution trailers and AI vendor/model names in operational
   docs, and says of the enforcement: "Review-enforced today: the `commit-msg` hook and the vendor/model
   term list in the banned-names gate **do not exist yet**". Neither does: `.git/hooks/` holds no
   `commit-msg` hook and `xtask/src/gates.rs::banned_names` still greps only the legacy product name.
   The rule is honour-system until P0-14 lands: a checked-in hook template plus a
   second term list, with the term literals assembled in code so the gate cannot trip on its own
   definition and a committed allowlist (`docs/banned-names.baseline`) of pre-existing hits so it can be
   fatal from day one. *Evidence:*
   `ls .git/hooks/`, `xtask/src/gates.rs`, `AGENTS.md` §5, P0-14 scope in this file. **Resolved
   2026-10-30 (P0-14, PR #7):** both exist and `xtask ci` runs them; item 9 records the one remaining
   stale sentence in `AGENTS.md`.
9. **`AGENTS.md` §5 still denies that the D30 enforcement exists** (found 2026-10-30, PR #7 review round
   1 R7; docs-truth). The sentence is: "Review-enforced today: the `commit-msg` hook and the vendor/model
   term list in the banned-names gate **do not exist yet**." P0-14 makes it false: `tools/hooks/commit-msg`
   and `xtask/src/terms.rs` exist and `xtask ci` runs them. `AGENTS.md` is owner-reserved, so the fix is
   an owner edit (drop the sentence or reword it to "mechanically enforced: …"), not an agent one. The
   same sentence is quoted as history in the P0-14 row above and must stay quoted there.
10. **The end-to-end hook tests never run on Windows CI** (found 2026-10-30, PR #7 review round 1 R9).
    `xtask/tests/commit_msg_hook.rs` is `#![cfg(unix)]` because it launches `sh` to run the hook; the
    *gate* logic it exercises (`terms::find`, `check_message`, the scissors cut) is cross-platform and
    covered by unit tests, but no test proves the hook script itself under `windows-latest`. *Fix
    option:* a Windows job that runs the hook through Git's own `sh` (`git -c core.hooksPath=… commit`),
    or a Rust-side test that feeds Git's message file shape without a shell. Cheap; needs a CI matrix
    decision (P0-1 territory), so recorded rather than done.
