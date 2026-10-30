# Dev log

Append-only. Terse. Newest entry at the bottom. Every entry: what landed, the numbers that moved, what
is still open. This file plus a green tree is how the next session picks up after a crash or a context
limit (AGENTS.md §5). Task ids match `docs/gaps.md` and the commit subject.

---

## 2026-10-26 — B0 · Repository bootstrap and documentation system

**Landed**
- Git repository initialised on `main`; `.gitignore` for `target/`, `plan/`, `*.db` (+ WAL sidecars),
  `midata_data/`, `logs/`, secrets. `rust-toolchain.toml` pins `stable` with `rustfmt` + `clippy`.
- Dual licence applied per `REBUILD_PLAN.md` §16 O1 default: `LICENSE-MIT` + `LICENSE-APACHE`
  (copyright "MiData contributors" — owner may change holder or licence; that would be a §15 amendment).
- Documentation system created: `docs/decisions.md` (D1–D25 restated as ADRs + open items O1–O7 +
  four bootstrap decisions A1–A4), `docs/gaps.md` v0 (11 ranked Phase-0 entries, 76 agent-hours),
  `docs/inventory.md` v0 (**195 rows** across HM/DE/OR/AN/DB/PD/IX/SY/AI/EX/OB, all `planned`),
  `docs/testing.md` (oracle table skeleton, extraction marked **pending**), `docs/architecture.md`,
  `docs/development.md`, `docs/ui-design.md`, `docs/i18n.md`, `docs/contributing.md`,
  `docs/scorecard.md` (GENERATED stub), `docs/roadmap.md` (mirror of plan §14), `ROADMAP.md` v0.
- **No Rust written, and that is the point:** crates are born with P0-1, so `crates/`, `apps/` and
  `xtask/` were deliberately not created (an empty crate is dead schema by another name).

**Numbers**
- Inventory: 0 / 195 rows done (0%) · tests 0 · crates 0 · snapshots 0 · i18n coverage n/a.
- Phase-0 queue: 10 open tasks + 1 blocked (`P0-11`, needs owner authorization for the oracle session).
- `AGENTS.md` measured at 32.7 KB against the ≤ 8 KB target → **new gap P0-10** (rank 2: cheap, and it
  taxes every session until fixed).

**Decisions taken under autonomy (AGENTS.md §7)** — recorded in `docs/decisions.md` §3:
A1 licence files created under the O1 default; A2 `docs/roadmap.md` created as a faithful mirror of
plan §14 (the orientation table referenced it but Appendix C omitted it); A3 no crate versions pinned
yet (first `[workspace.dependencies]` lands in P0-1, with the dependency-escalation rule applying);
A4 this task is a single root commit on `main` because a fresh repo has no base commit to branch from
and the owner's remote does not exist yet.

**Still open (in rank order)**
1. **P0-1** workspace + `xtask` + CI — everything else waits on it.
2. **P0-10** `AGENTS.md` diet to ≤ 8 KB (owner sign-off on the split before commit).
3. **P0-3** needs a one-line owner ruling before migrations are written: does Phase 0 migrate the
   **full** v1 schema (plan §14/§5.2) or only the tables P0–P2 code actually reads/writes (plan §5.1.5 +
   AGENTS.md §9 "no dead schema")? Migrations are escalation item #1, so this cannot be guessed.
4. **P0-11** oracle extraction is blocked on owner authorization; until it runs, `docs/testing.md`
   carries zero legacy scenarios and every criterion is marked `fresh`.
5. `docs/scorecard.md` is a stub — every `ROADMAP.md` number is hand-counted until `xtask stats` exists.

**Firewall note:** the legacy tree was not read, opened or grepped in this session; the only legacy
material consulted was `docs/legacy-spec/PROJECT_REPORT.md` (already inside this repository, reference
only).

---

## 2026-10-26 — P0-11 · Legacy oracle extraction session (firewall exception)

**Firewall exception used and expired with this session.** Owner authorized, for this session only, the
single exception to AGENTS.md rule 5: read **only** the legacy test files (`../MiData-back/backend/tests/`),
nothing else in that repo. The legacy tree is closed again for good — no later session may re-open it.
Behaviour questions now resolve through `docs/inventory.md` → `docs/testing.md` + `docs/oracles/` →
`docs/legacy-spec/PROJECT_REPORT.md` → owner.

**Landed**
- `docs/oracles/` — **8 new files**, one per area, so no file breaks the 500-line budget and an agent loads
  only the oracles its task needs: `storage-and-config.md` (42), `analytics-and-dashboard.md` (51),
  `pedigree-and-reproduction.md` (60), `exchange-and-onboarding.md` (81),
  `retention-and-soft-delete.md` (36), `query-validation.md` (71), `assistant.md` (75),
  `launcher-waivers.md` (54). Every scenario is `input → expected behaviour`, implementation-agnostic, with
  a stable id (`STO-nn` … `WAV-nn`) to cite in a test's doc comment and in an inventory `evidence` cell.
- Each oracle file ends with **`## Fresh`** (behaviours the legacy tests never covered — write them from
  `REBUILD_PLAN`) and **`## Anti-patterns — never reproduce`** (binding input to the §3 register).
- `docs/testing.md` v1 — extraction marked complete; §2 is now an index of modules → files → counts →
  inventory blocks; §7 waivers corrected; §8 mapping discipline rewritten around scenario ids.
- `docs/inventory.md` v1 — **222 rows** (195 + **27 added**, because the extraction proved behaviours the v0
  checklist had no row for): DE-16…19, OR-31…35, AN-27…29, PD-21, SY-13…16, AI-21…23, EX-13…16, OB-13…15.
  Per-block oracle pointers added to every block's source line.
- `docs/gaps.md` — P0-11 closed and moved to §Closed; queue now 10 open tasks / **73 agent-hours**.

**Numbers**
- Transcribed: **18 modules, 470 test functions → 485 collected cases** (matches the calibration figure in
  `PROJECT_REPORT` §17.2, which counts cases, not functions). Kept **433** scenarios, waived **37**.
- Largest modules: `test_tool_query` 71 · `test_local_sharing` 37 · `test_soft_delete` 36 ·
  `test_health_reproduction` 34 · `test_extensions` 32 (33 cases) · `test_analytics` 30.
- Inventory: 0 / 222 rows done (0%) — unchanged, because extraction produces *criteria*, not evidence.

**Two corrections to the existing docs (both material)**
1. **`test_local_sharing` was never a sharing surface.** The previous waiver (testing.md §7, inventory
   Waivers) assumed 37 tests covering LAN/tunnel/QR/tray. It contains none: it is read-only query validation
   (16), setup-template export (7), import (11) and round-trip (3) — **42 of its tests are kept**. Waiving it
   would have silently discarded the strongest existing precedents for `midata-queryguard` and the
   pack/template contract. The waiver now applies to **12 launcher tests, not 49**.
2. **`test_extensions` is 32 functions / 33 cases**, not 33 functions — the extra grep hit was a fixture.

**Decisions taken under autonomy (AGENTS.md §7)**
- Oracles were split into `docs/oracles/` rather than one 470-row `testing.md`: rule 13's 500-line soft cap
  would have been breached by ~60 lines, and a single file defeats the ≤3-doc context budget.
- The 27 missing inventory rows were **added** (AGENTS.md §8: "add it rather than letting the test float")
  rather than left as proposals. They are `planned` with `—` evidence, so no coverage number is inflated,
  and they are open to owner re-wording — never to deletion (ratchet).
- Anti-pattern lists were transcribed into each oracle file instead of straight into `REBUILD_PLAN.md` §3,
  which the owner alone amends. Consolidating them into §3 is an owner action.

**Still open (in rank order)**
1. **P0-1** workspace + `xtask` + CI — everything else still waits on it.
2. **P0-10** `AGENTS.md` diet to ≤ 8 KB (owner sign-off on the split before commit).
3. **P0-3** needs a one-line owner ruling: Phase 0 migrates the **full** v1 schema or only what P0–P2 code
   reads/writes. Migrations are escalation item #1.
4. **Owner action:** fold the eight oracle files' anti-pattern sections into `REBUILD_PLAN.md` §3, and
   sanity-check the 27 added inventory rows' wording.
5. `docs/scorecard.md` is still a stub — every `ROADMAP.md` number is hand-counted until `xtask stats`.

**Highest-value fresh work surfaced by the extraction** (no legacy coverage at all, so it is design work,
not transcription): AI staged-write persistence/restart-survival/single-use token/expiry (AI-10…AI-14) ·
`SELECT *` bypassing the column whitelist and values spliced instead of bound (AI-7, AI-19) · birth-action
rollback proof (PD-17) · import-batch atomicity (DE-16) · onboarding rollback · a backup verify that fails on
a corrupt file (IX-11).

---

## 2026-10-27 — P0-12 · Owner rulings on the nine bootstrap ambiguities

**Landed (amendments M1–M8, branch `t/p0-12-owner-amendments`)**
- `REBUILD_PLAN.md` → **0.4-draft**. §5.2 carries the **birth-gated schema** note (D26) and the
  `widgets` table is labelled a P3 addition; §14 Phase 0 now says "migrations v1 → birth-gated schema
  per §5.2 (D26)" and "species/fields/modes/system-mode presets; default widgets seeded in P3".
  §6.1 inventory provenance corrected (B0 + P0-8, not a phantom P0-D3). §7.4 i18n location fixed to
  `midata-i18n` (D24 authoritative). §1.3/§11.1/Appendix C/§14 exit now state the **lean-bootloader**
  policy (rules not procedures · one authoritative home per topic · ≤ ~12 KB). §15 gains **D26**
  (schema birth-gating) and **D27** (bootstrap rulings batch); header stamps bumped.
- `AGENTS.md` §5 Git discipline: **no session ends with uncommitted work** (commit the checkpoint;
  verify with `git status` before asserting repo state). The diet itself stays P0-10.
- `docs/decisions.md`: D26/D27 mirrored; A2 ratified with the mirror stamp; A4 ratified (remote +
  branch protection live); O1 holder "MiData contributors" recorded as **provisional**; A5 added.
- `docs/roadmap.md`: explicit header stamp — **MIRROR of `REBUILD_PLAN.md` §14; live status only**.
- `docs/gaps.md` v1: P0-10 re-scoped to the lean-bootloader policy (33.5 KB → ≤ ~12 KB, dependency
  cleared); P0-3 schema ambiguity **resolved** by D26; P0-6 de-widgetized + onboarding-rollback
  criterion; P0-1 CI evidence now requires the PR's ubuntu+windows jobs (protection live); P0-8
  re-scoped 3 h → 2 h (row verification only; generation stays with `xtask stats`/P0-9); M8 criteria
  forward-wired in a dedicated table; P0-12 closed.
- `docs/proposals/anti-pattern-register-extension.md`: **M7 draft only** — 37 deduplicated rows
  (P1–P33 behaviour, P34–P37 test integrity) plus a fold-in map against the 15 existing §3 rows.
  **Not merged into §3** (owner-only amendment, A5).

**Numbers**
- Phase-0 queue: 10 open tasks · **72 agent-hours** (was 73; P0-8 −1 h). P0-12 (1.5 h) closed same day.
- `AGENTS.md`: 33.5 KB — now measured against the amended **≤ ~12 KB** lean-bootloader target.
- Inventory: 0 / 222 rows done (unchanged — this task changes the contract, not coverage).
- Anti-pattern proposal: 37 rows from 8 oracle files, 12 findings folded into existing §3 rows.

**Decisions taken under autonomy (AGENTS.md §7)**
- P0-8's re-scope is written as *verification-first* (rows, phase assignment, oracle-link column) with
  generation left to `xtask stats` — recorded so the owner can reverse it in one edit if the intended
  re-scope was different.
- `docs/proposals/` is created for owner-review-only documents that must not become binding by
  accident; A5 records the rule.
- The `widgets` table gained a `-- Phase 3 addition (D26)` comment inside the §5.2 catalogue instead of
  being physically moved: §5.2 is a spec catalogue, and moving tables would obscure the v1 shape.

**Still open (in rank order)**
1. **P0-1** workspace + `xtask` + CI — now with branch protection live, the PR must be green on
   ubuntu **and** windows before merge.
2. **P0-10** `AGENTS.md` → lean bootloader ≤ ~12 KB (policy approved; owner reviews the final shape).
3. **P0-2** `midata-domain` + `midata-i18n` — i18n lives in the crate, catalogues embedded at build.
4. ~~Owner action (M7): review the anti-pattern proposal and merge/reject rows.~~ **Done 2026-10-27** —
   all 37 rows merged into `REBUILD_PLAN.md` §3 as slug-keyed rows (D28); proposal file deleted.
5. `docs/scorecard.md` is still a stub — every `ROADMAP.md` number is hand-counted until `xtask stats`.

---

## 2026-10-27 — P0-12 (cont.) · M7 owner review merged: §3 gains 37 slug-keyed rows

**Landed (owner review of M7, branch `t/p0-12-owner-amendments`)**
- `REBUILD_PLAN.md` → **0.5-draft**. §3 now has two parts: **§3.1 rows 1–15** (original register, numbers
  kept as stable citation handles) and **§3.2–§3.8** — 37 owner-approved oracle-extraction rows grouped
  thematically (analytics results · query safety · AI contract · data I/O & onboarding · pedigree ·
  storage & settings · test integrity), each with a **stable slug** (`analytics.typed-results`,
  `query.bound-params`, `test-integrity.no-vacuous-assertions`, …). Owner rewords applied **verbatim** for
  P14 → `contract.stable-keys`, P20 → `state.command-only-mutations`, P26 →
  `config.business-vocabularies`, P30 → `settings.typed-facade`; P12's "query timeout" is defined as the
  SQLite progress-handler interrupt guard. Twelve fold-in findings added **one provenance clause** to the
  existing rows they belong to (row 8 now reads "…including credential files on disk") — no duplicate rows.
  §15 gains **D28** (with sub-decisions a/b/c) and header/footer stamps bumped.
- `docs/decisions.md`: D28 mirrored in ADR form; A5 marked **closed** (proposal merged, file deleted).
- `docs/contributing.md`: new **§3 Test-integrity checklist** operationalising §3.8 in six bullets (failure
  injected *and* zero partial rows · boundary named + negative case · values/invariants never key presence
  or magic totals · command surface not storage internals · file name states the behaviour · no vacuous
  clause); §1 Definition of Done now references it; later sections renumbered 4–8.
- `docs/architecture.md` §4: the business-vocabulary vs structural-identity distinction (P26) written where
  the row promises it — purpose tags/pregnancy-like values/suggestion lists = config data; sex, lifecycle
  status, `data_type`, `cardinality` = typed code; adding a business value is data, adding a sex is a §15
  decision.
- `docs/oracles/*.md` (8 files): every `## Anti-patterns` section opens with a **register-mapping pointer
  line** (finding number → §3 slug, or `§3 row N` for folded findings, or the governing decision for
  non-register items).
- `docs/proposals/anti-pattern-register-extension.md`: **deleted** — one authoritative home; provenance
  survives in D28 and in the oracle pointer lines. `docs/gaps.md` and `docs/testing.md` references updated.

**Numbers**
- §3 register: 15 rows → **52** (15 original + 37 slug-keyed). Slugs: **37**, none renamed later.
- Oracle files: **8 / 8** carry a register mapping covering **111** numbered findings.
- `REBUILD_PLAN.md` version 0.4-draft → **0.5-draft**; decisions D27 → **D28**.
- Inventory 0 / 222 rows done, queue 72 agent-hours — unchanged: this task changes the contract, not
  coverage.

**Decisions taken under autonomy (AGENTS.md §7)**
- Rows 1–15 keep their numbers instead of being absorbed into the thematic groups: the fold-in instruction
  itself cites rows by number ("row 8"), and other docs/proposal history do too. Slugs are the citation
  handle for the extension only — both are contract.
- The oracle pointer line maps **every** numbered finding, not just the ones that became rows: folded items
  point at `§3 row N`, non-register items at the governing decision (D9 pig policy, D21/D5 waived surfaces,
  AGENTS rule 12 i18n). Provenance is complete, §3 stays the only home.
- Test-integrity checklist placed as a new §3 (before the how-to checklists) and later sections renumbered;
  nothing referenced those section numbers by number.

**Still open (in rank order)**
1. **P0-1** workspace + `xtask` + CI — the register is now the reference the first crates must satisfy.
2. **P0-10** `AGENTS.md` → lean bootloader ≤ ~12 KB (owner reviews the final shape).
3. **P0-2** `midata-domain` + `midata-i18n` — `value.distinct-missing-states` and `value.one-resolver` are
   its acceptance criteria.

---

## 2026-10-27 — P0-1 · Workspace, `xtask` gates, ubuntu+windows CI

**Landed**
- Root workspace (`resolver = "2"`, `unsafe_code = "forbid"`, clippy deny-list, `panic = "unwind"`,
  `MIT OR Apache-2.0`), `Cargo.lock`, `.cargo/config.toml` (`cargo xtask` alias), `clippy.toml`,
  `deny.toml`, `.gitattributes` (LF everywhere, binary exceptions).
- **`xtask` only** — product crates stay birth-gated per task (D26). Dependencies: `anyhow` +
  `serde_json` only; no pre-populated workspace dependency table.
- Gates: `check-layers` (14-crate registered DAG with birth phase + manifest home, internal/external
  edges, dev-only testkit, app-as-dependency ban, egui/tokio/sqlparser/file-format homes),
  `file-budget` (500 soft / 800 alarm, shrink-only baseline), `i18n-coverage` (explicit
  `no-catalogues` state, FR-missing ratchet, orphan report), `stats` (deterministic scorecard, no
  timestamps), `banned-names`, `ci` (aggregator; missing local tools are loud skips, never passes),
  and `new-command`/`new-screen`/`seed`/`bench` skeletons that return an unsupported error.
- `.github/workflows/ci.yml`: `ci (ubuntu-latest)` + `ci (windows-latest)`, both run `cargo xtask ci`
  (fmt · clippy `-D warnings` · nextest · all gates · scorecard diff · cargo-deny · cargo-machete).
- Docs: `docs/architecture.md` §9 records the clock/store-handle/provider/actor injection seams for
  P0-2+ (no globals, no module-level connection); development/roadmap/ROADMAP/gaps updated.

**Numbers**
- Tests: 0 → **35** (all xtask gate unit tests; nextest green).
- CI: PR #2 — `ci (ubuntu-latest)` and `ci (windows-latest)` both **pass** (22 s / 56 s,
  run 40419756246). First real two-OS proof.
- Crates in workspace: **1** (`xtask`); 13 registered but not yet born.
- File-budget violations: **0** across 12 Rust files. `unsafe` tokens: 0. Clippy warnings: 0.
- Inventory coverage unchanged at 0 / 222; i18n coverage `n/a` (loud tracked skip until P0-2).
- Scorecard is **generated, committed and CI-diffed** — `ROADMAP.md` now quotes it instead of hand counts.

**Negative proofs recorded in the PR body (each violation temporary + reverted)**
| Gate | Intentional violation | Result |
|---|---|---|
| check-layers | `xtask` → unregistered `clap` | `unregistered external dependency` |
| file-budget | 501-line file | `new violation at 501 lines` |
| i18n-coverage | catalogues appear over a `no-catalogues` baseline | stale-baseline failure |
| i18n-coverage | EN key without FR over baseline `fr_missing: 0` | `FR coverage regressed` |
| stats | hand-edited scorecard | `--check` reports stale |
| banned-names | banned term in a shipped `.md` | occurrence reported |
| ci | unformatted target file | stops at the format step |

**Decisions taken under autonomy (AGENTS.md §7)**
- Adopted the recommended P0-1 split (workspace + `xtask` only). Creating ten empty crates to match
  Appendix C's older bullet would violate the birth-gating reading of D26.
- `stats` counts tests by a deterministic source scan of test attributes (doc tests excluded) rather
  than invoking nextest: the scorecard must be identical on ubuntu and windows and stable without
  compiling test binaries. Documented in the scorecard itself.
- Banned-name scope: shipped surfaces must be clean; the constitution/plan/decisions/gaps and the
  legacy spec are allowlisted because their job is to state or quote the ban.

**Owner TODO**
- After this PR merges: enable **Require status checks** in branch protection for
  `ci (ubuntu-latest)` and `ci (windows-latest)`. Until then the two jobs are informative, not merge
  gates.

**Still open (in rank order)**
1. **P0-10** `AGENTS.md` → lean bootloader ≤ ~12 KB (owner reviews the final shape).
2. **P0-2** `midata-domain` + `midata-i18n` — the first crates; run `cargo xtask i18n-coverage
   --write-baseline` when the catalogues are born so the skip becomes a real measurement.
3. **P0-3** `midata-store` + birth-gated migrations (D26).

## 2026-10-27 — P0-1 · Review round 1: no findings delivered; one gate-honesty fix

**What happened.** The fix-round session was launched with an unfilled template (`<branch>`, `PR #<N>`,
`round <k>`, and no findings block). Verified before acting: PR #2 on `t/p0-1-workspace-ci` has
**0 reviews, 0 review comments, 0 issue comments** via the GitHub API, and no findings file exists
locally, so **zero external findings exist to fix or contest**. Per owner ruling, the implementer did
**not** self-audit in the reviewer role (reviewer/implementer separation, `docs/contributing.md` §9);
an independent REVIEWER session runs next.

**Gates verified green locally** on the branch before and after the change (`cargo xtask ci`: fmt ·
clippy `-D warnings` · 37 nextest · check-layers · file-budget · i18n loud skip · banned-names ·
stats + committed-scorecard diff · cargo-deny · cargo-machete).

**Observation 01 (owner-approved fix, §3.8 `test-integrity.no-vacuous-gates`).** `ci::is_missing_binary`
classified a missing tool by string-matching `No such file or directory` / `os error 2` /
`The system cannot find the file` anywhere in the error chain. `process::capture` folds a tool's
**stderr** into that same error, so an *installed* tool that failed while naming a missing file (e.g.
`cargo-deny` without `deny.toml`, or `git -C /nonexistent status` → `fatal: cannot change to
'/nonexistent': No such file or directory`) was printed as `SKIP` and returned **0** locally — a gate
that silently did not bite. Now classified from the spawn `io::ErrorKind::NotFound` only. Negative
proof: the new regression test fails against the old predicate
(`a tool that ran and failed must fail the gate, never skip it`) and passes with the fix; the opposite
direction is covered end to end through `process::capture`. Tests 35 → **37**.

**Observation 02 (recorded, not fixed).** The declared MSRV (`rust-version = "1.80"`) is unverified —
no pinned-toolchain job exists. Row 6 of gaps "Known shortfalls".

**Observation 03 (reported to the owner, untouched).** `docs/review-templates.md` is **untracked** on
this branch (author unknown; it is the reviewer-agent contract). Left uncommitted rather than adopted.

**Two-OS proof after the fix.** Commit `b7035e7` re-ran CI: `ci (ubuntu-latest)` and
`ci (windows-latest)` both **green** (run 40421245292, 20 s / 43 s).

**Still open (rank order).** 1. Independent REVIEWER session on PR #2. 2. P0-10 `AGENTS.md` diet.
3. P0-2 `midata-domain` + `midata-i18n`.

**Review #2 round 1 (independent REVIEWER, separate session):** REQUEST_CHANGES — 0 BLOCKER / 1 MAJOR / 6 MINOR / 5 SUGGESTION; R1 is the shipped-contract defect (`APP_CRATES` rejects the required `midata → midata-app` edge, proven by a throwaway-copy test); full report in PR #2 comment <https://github.com/halmousama/MiData/pull/2#issuecomment-6430093550>; gate re-run green on this machine (37/37), no code touched, nothing pushed — this line left uncommitted for the implementing agent.

## 2026-10-27 — P0-1 · PR #2 round-1 fix session (R1–R6, S1–S5)

**Review record reconciliation (bookkeeping).** The round-1 report header claimed **6 MINOR** but listed
five (R2–R6). The report is otherwise consistent: 1 MAJOR (R1) + 5 MINOR + 5 SUGGESTION = 11 findings.
This entry treats the header as a counting slip, not a missing item: **R1–R6 and S1–S5 are all applied**,
and the PR body's fix-round table carries all eleven.

**R1 (MAJOR) — app roots are not libraries.** `xtask/src/layers/rules.rs` gained `APP_EDGES`, the only
place an app-to-app edge may be declared; today it holds exactly `midata → midata-app` (the eframe
composition root needs the egui shell). `check_workspace` rejects an app crate as a dependency unless
the depending package declares that edge, so `midata-cli → midata-app` stays forbidden. `midata-cli`
lost `any_internal` and now has an explicit allowlist — domain, i18n, store, engine (testkit as dev).
Three new layer tests: `ui_composition_root_may_depend_on_the_app_shell` (positive),
`headless_cli_may_not_depend_on_the_ui_shell` (negative), `application_crate_cannot_be_a_dependency`
(negative, engine → cli), plus `headless_cli_has_an_explicit_internal_allowlist`. Negative proof run:
temporarily moving the declared edge to `midata-cli` failed **both** the positive and the negative test
(`left: [Violation { package: "midata", message: "depends on application crate \`midata-app\` …" }]`),
then reverted.

**R2 — coverage numerator.** `docs/scorecard.md` now counts `done` rows with a **non-empty** evidence
cell (`—`/`-` do not count); `InventoryStats.done_with_evidence` is the ratchet input, asserted by
`inventory_numerator_requires_done_and_evidence` (three `done` rows, two with evidence, one numerator).

**R3 — golden metric.** Only PNGs under `crates/midata-app/**/snapshots/**` count
(`is_screen_golden`, unit-tested); the target is **11**, the screen list in `REBUILD_PLAN.md` §7.1, and
both facts are stated in the generated scorecard's source/notes. End-to-end proof: `touch
docs/logo.png` left the metric at 0/11, a probe at `crates/midata-app/snapshots/probe.png` moved it to
1/11; both removed.

**R4 — EN-total ratchet.** `i18n::check` now fails when the EN catalogue is smaller than
`docs/i18n-coverage.baseline` (`en_total`), so the field is no longer stored-and-ignored. `docs/i18n.md`
§5 documents both ratchets and the deletion rule (delete FR row + regenerate the baseline in one
change). Negative proof: temporary 1-key catalogues against `en_total: 2` failed with `EN catalogue
shrank: 1 key(s), baseline records 2 …`; reverted. Unit test
`ratchet_fails_when_the_en_catalogue_shrinks`.

**R5 — deny + machete negative proofs.** Temporary path dependencies under `/tmp/opencode`:
`temp-gpl` (`license = "GPL-3.0-only"`) made `cargo-deny check licenses` fail with `GPL-3.0-only …
rejected: license is not explicitly allowed`; `temp-unused` was reported by `cargo-machete` as an
unused dependency (exit 1). Both removed, `xtask/Cargo.toml` and `Cargo.lock` reverted.

**R6 — snapshot deferral tracked.** `docs/gaps.md` known-shortfall **7** records that CI has no snapshot
step, with the target **P0-9**, and P0-9's scope now names the Linux-only CI job; the P0-1 Scope bullet
and the workflow comment no longer imply it exists.

**S1** `birth` is documented as "declared and printed; enforced by owner review (D26)" in
`rules.rs`/`layers.rs`. **S2** orphan-FR-keys-fatal is P0-2 acceptance in `docs/gaps.md` and
`docs/i18n.md` §5. **S3** the scorecard says "0 (enforced by the gate)". **S4** the PR body's stale
`35` is corrected to `37` at the time of the fix (the branch now measures 43; the fix-round table states
both). **S5** all four CI actions are pinned to commit SHAs with the release tag in a trailing comment,
and `docs/architecture.md`'s header now matches §9 (updated when a binding decision is fixed ahead of
the crate).

**Numbers.** `xtask` tests **37 → 43**; workspace still 1 crate; inventory 0/222 (numerator now
`done`+evidence); goldens 0/11; i18n loud skip unchanged. `docs/scorecard.md` regenerated.
`cargo xtask ci` green locally after the changes; the two-OS re-run starts with the pushed commit —
no CI-green claim is made here until the PR checks report it.

**Still open (rank order).** 1. Round-2 reviewer session on PR #2. 2. P0-10 `AGENTS.md` diet.
3. P0-2 `midata-domain` + `midata-i18n` (with the S2 acceptance and the first real `en_total`).

**Review #2 round 2 (independent REVIEWER, separate session):** APPROVE — 0 BLOCKER / 0 MAJOR / 0 MINOR / 2 SUGGESTION (N1: line 287 cites a non-existent `docs/contributing.md` §9, N2: gaps records no run id for the fix commit `b1d52fe`); round-1 items R1–R6, S1 and S3–S5 verified fixed in the tree (R1 positive+negative layer tests, R2 `done_with_evidence`, R3 §7.1 11-screen golden filter, R4 EN-total ratchet, R5 deny/machete rows, R6 known-shortfall 7) and **S2 confirmed as a P0-2 acceptance wire, not a gate change** (orphans still report-only); round-1 header's "6 MINOR" reconciled to the 5 listed (11 findings total); full report in PR #2 comment <https://github.com/halmousama/MiData/pull/2#issuecomment-6437110096>; gate re-run green on this machine at `b1d52fe` (43/43) and both PR checks green; no code touched, nothing pushed — this line left uncommitted for the implementing agent.

## 2026-10-27 — P0-13 · D30 policy recorded; `main` history scrubbed

**What landed.** D29 (independent reviewer loop) and D30 (attribution repeal) are now recorded in
`REBUILD_PLAN.md` §15 and `docs/decisions.md` — D30 had been cited by `AGENTS.md`, `docs/gaps.md` and
this log while existing in no decision table at all. The four places that still mandated the
`Co-Authored-By` trailer (`REBUILD_PLAN.md` §13.4, `docs/contributing.md` §Git, `docs/decisions.md` D25,
`docs/review-templates.md` A) now state the ban instead, and the example commit in `docs/contributing.md`
no longer puts the owner's name and address on an agent-authored commit. Two defects in the in-flight D30
edit were fixed before it landed: a stray object-id fragment inside `unrepresentable` (`REBUILD_PLAN.md`
§5.5) and that misattributed example line.

**History.** `main`'s seven commits were rewritten with `git-filter-repo` (message-only callback,
`^Co-Authored-By:` stripped case-insensitively, trailing whitespace renormalised). Every tree stayed
byte-identical (`b886fe2f` before and after) and the subject and order of all seven are unchanged; the
push was `--force-with-lease` against the verified old tip `b65e614`. The squash commit of PR #2 carried
six trailer lines on its own — a squash-merge aggregates the PR's commit trailers into one message, which
is how the name reached `main` though no PR body mentioned it.

**Repo settings.** Squash-only: `allow_merge_commit` and `allow_rebase_merge` disabled. `main` had no
branch protection at all (the GET returned 404), so this session created it rather than restoring it:
required status checks `ci (ubuntu-latest)` and `ci (windows-latest)`, strict up-to-date required, force
pushes and deletions blocked, no review requirement (a solo owner cannot approve their own PR).

**Not done on purpose.** The scrub scope was `main` only, by owner choice. `t/p0-1-workspace-ci` and
`t/p0-12-owner-amendments` still carry the old commits on the remote, and `t/p0-2-domain-i18n`'s remote
tip still has `b65e614` in its ancestry — force-push that branch before opening its PR or the trailer
returns with the merge. D30 also stays honour-system: no `commit-msg` hook, and the banned-name gate
still greps only the legacy product name. The formal gap entry lives in P0-2's `docs/gaps.md` §8,
which quotes the sentence this commit replaces — reword it when that branch lands.

**Numbers.** commits on `main` 7 → 7 (rewritten, none added); `Co-Authored-By` occurrences 14 → 0 across
the rewritten local refs; tree `b886fe2f` unchanged; backup bundle `../midata-pre-scrub.bundle` (359 KB).

## 2026-10-27 — D31 · Supervisor mode: delegated merge authority

**What landed.** D31 is recorded as a row in `REBUILD_PLAN.md` §15 and a section in
`docs/decisions.md`; `AGENTS.md` gains §12 as the operating procedure and `docs/review-templates.md`
gains the SUPERVISOR template (stages 0–7). A SUPERVISOR agent may now run consecutive task cycles —
fresh implementer, fresh reviewer, at most two review rounds — and squash-merge through `gh pr merge`
without owner action, but only while all six conditions hold: APPROVE with 0 BLOCKER/0 MAJOR, both
required checks green on the PR head, no escalation flag, no owner-reserved surface in the diff, a
merge report paragraph in the devlog, and no `plan/HOLD`. Budget 3 tasks then an owner digest; a task
that fails twice escalates instead of being retried; the owner revokes by creating `plan/HOLD` or by
amending the row.

**Why it is not a self-merging implementer.** Delegation goes to an agent that never implements and
never reviews, and the merge itself happens on GitHub, so branch protection and the required checks are
enforced server-side rather than trusted. Workers start with fresh contexts, which makes the repository
the only memory: the compaction risk disappears because no binding rule lives in a conversation summary.

**Precondition, checked before adopting.** A probe subagent reported shell access, `git 2.53.0`, an
authenticated `gh` with `repo`/`workflow` scopes and filesystem write access.

**Urgent queue closed first.** The remote `t/p0-2-domain-i18n` tip was force-pushed to `cb4405c` (clean
ancestry on `f862d25`) before anything else — leaving it on `94d2fd5` would have kept `b65e614`
reachable and returned the trailers through the next squash. PR #4 was then squash-merged as `14a79ca`
after confirming the D30 row and the four flipped surfaces; its branch was deleted and `main` scans
clean.

**Numbers.** `main` 8 commits; trailer occurrences on `main` 0; PR #4 checks green (ubuntu 19 s, windows
45 s); the D30 session's four follow-up items are down to three.

**Owner authorisation for the two owner-reserved actions above.** The owner confirms they instructed
both owner-reserved actions recorded here (the force-push of the `t/p0-2-domain-i18n` remote tip and
the squash-merge of PR #4), and acknowledges that PR #4 carried no reviewer report, a deviation from
D29 accepted by the owner. Recorded 2026-10-30.

**Review #5 round 1 (independent REVIEWER, separate session):** REQUEST_CHANGES — 0 BLOCKER / 3 MAJOR / 5 MINOR / 2 SUGGESTION (R1: the binding D31 row lists six merge conditions, all four restatements say five and drop the devlog-report item, and `AGENTS.md` §12 names them only by count; R2: six superseded "owner squash-merges" statements left standing in `AGENTS.md` §5/§7.4, §13.4 and `docs/contributing.md` §6/§9, so the constitution contradicts itself; R3: the devlog records a remote force-push and the PR #4 squash-merge — both owner-reserved — without stating their authority, and PR #4 carries no reviewer report under D29); docs-only fast path (A+D), C skipped, B sampled on merge conditions/budget/escalation triggers/cross-references; every number in the entry verified against the tree and the remote (8 commits, 0 trailers, PR #4 timings 19 s/45 s, `cb4405c`, `git 2.53.0`, `gh` scopes) and the three xtask gates re-run green on this machine; full report in PR #5 comment <https://github.com/halmousama/MiData/pull/5#issuecomment-6532761853>; no code or docs touched, nothing pushed — this line left uncommitted for the supervisor.

**Review #5 round 1 fixes (fix round, same branch, 2026-10-30):** R1 the six D31 merge conditions are now
enumerated identically in `AGENTS.md` §12, `docs/decisions.md` D31, the SUPERVISOR template stage 5 and
the PR body — the devlog restatement was corrected too; R2 the owner-only-merge carve-out
"except inside the D31 envelope" landed at all six superseded statements (`AGENTS.md` §5, §7.4, §13.4
twice, `docs/contributing.md` §6 and §9); R3 the owner-authorisation paragraph is recorded in the D31
entry above; R4 budget/digest/round-3 stop added to §12; R5 the IMPLEMENTER (kickoff) and FIX-ROUND
templates now exist in `docs/review-templates.md`; R6 the escalation flag is defined (gaps Status
`escalated`, or `plan/ESCALATE`) in §12, D31 and the template; R7 "§3 security rows" is resolved to
§3.1 rows 8–10, §3.3, §3.4 wherever the conditions are restated (§15 untouched, per the owner);
R8 stamps: `AGENTS.md` 0.4-draft with the §12/D31 change line, P0-10 re-measured at **36.4 KB**
(35.6 KiB); R9 D29/D30/D31 rows added to the gaps Closed table; R10 the docs-only fast path stated in
the REVIEWER template. `cargo xtask ci` green (the i18n skip is the tracked no-catalogues state).
