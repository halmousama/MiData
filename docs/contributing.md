# Contributing — task loop, gates, git workflow

> **Status:** binding process. Agents and humans follow the same loop.
> **Source:** `REBUILD_PLAN.md` §11 (process), §13.4 (git workflow, D25); AGENTS.md §5–§7, §10–§11.
> This file is the operational form of those rules; if they disagree, `REBUILD_PLAN.md` §15 wins.

## 1. The five-step task loop

1. **Orient** — pick the highest-ranked unclaimed entry in `docs/gaps.md` (check `ROADMAP.md` for the
   current phase first). Read only what the orientation table points at: **≤5 source files + ≤3 docs**.
   If a task needs more, it is mis-sized — split it and record the split in `docs/gaps.md`.
2. **Acceptance test first** — write the failing test or snapshot that defines "done", transcribing the
   oracle scenario from `docs/testing.md` (or marking it `fresh` when no oracle exists yet).
3. **Implement** — in the lowest layer that fits: pure rules → `midata-domain`; persistence →
   `midata-store`; computation → `midata-analytics`; multi-step workflow → `midata-ops`; pixels →
   `midata-app`. The command stays thin dispatch.
4. **Verify** — run the gate (§2). For any UI change, render offscreen and **look at the PNG** (both
   themes if tokens/layout changed).
5. **Record** — update `docs/inventory.md` rows with evidence + date, close/update the `docs/gaps.md`
   entry, run `cargo xtask stats` when numbers moved, append a terse `log/devlog.md` entry, commit.

**Definition of done:** behaviour exists as a registered, documented command (when user-visible) ·
tests including graceful-failure and the **test-integrity checklist (§3)** · wired into the UI (and CLI
where relevant) · snapshots viewed for UI work · local gate (`cargo xtask ci`) green on the reviewed head ·
a both-OS dispatch on the exact head being merged for **every non-docs-only change**, cited by run id in
the merge report (docs-only changes cite the local gate only — D32 owner amendment, §2) ·
inventory/gaps/scorecard/devlog updated · **`ROADMAP.md` header stamp and current-focus line updated at
task closure** (and the matching `Updated:` stamp in `docs/roadmap.md`).

## 2. The gate

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run -p <touched crates>            # full: cargo nextest run --workspace
cargo xtask check-layers
cargo xtask file-budget
cargo xtask i18n-coverage                        # no regression
cargo xtask banned-names                         # legacy name + D30 attribution terms, ratcheted
cargo test -p midata-app -- snapshot             # UI tasks: goldens updated AND viewed
cargo test -p midata-engine --test command_hunt -- --ignored   # command changes
cargo xtask bench --quick                        # hot paths (flatten, dashboard, import)
cargo xtask stats                                # regenerate scorecard and commit it
```

`cargo xtask ci` runs the block. **Never commit red.** Every commit keeps the workspace building and the
touched crates' gates green. Mid-task commits are session checkpoints, not releases.

**CI still exists — but only when asked (D32).** Since the owner's 2026-10-30 ruling the GitHub workflow
has **no `push` or `pull_request` trigger**: it is `workflow_dispatch` only, and the ubuntu + windows
matrix, jobs and steps are unchanged. So a green local gate is the only thing that verifies your branch
unless you dispatch it. To run the both-OS matrix on your branch:

```sh
gh workflow run ci.yml --ref <your-branch>
gh run list --workflow ci.yml --branch <your-branch> --limit 5
gh run watch <run-id>          # wait for the ubuntu and windows jobs
```

**Dispatch is required** for (a) **every non-docs-only change** — it is cited by run id in the merge report
on the exact head being merged (D32 owner amendment, 2026-10-30); (b) any change to a **platform-sensitive
surface** — Windows path handling, file locks, process spawning, case-insensitive filesystems — which is a
subset of (a) but also binds a direct push, not only a merge; (c) every **phase gate**; (d) every
**release tag**; and (e) any change to `.github/` itself (a change to CI must be observed running). For an
ordinary **docs-only** change the local gate is the proof, but say so explicitly in the PR: "docs-only,
local gate green on head `<sha>`" with the final line quoted. If you quote a dispatch, paste the run URL
and both job conclusions.

**A dispatched run is evidence, not a merge gate (D32).** GitHub does not attach a `workflow_dispatch`
run to the PR: the checks can be green on the exact head while `gh pr checks` reports "no checks reported"
and the merge state reads `BLOCKED`. That is why the required-status-checks rule was removed from
`main`'s protection (force-push and deletion blocks stay) as part of landing D32 — before that merge,
because the rule itself blocked it. A **red** dispatch is
therefore the supervisor's stop signal — noticed, reviewed and cited — not a merge-button block; a green
one is cited as the (2b) proof. Nothing about this weakens the local gate: it stays the always-on proof.

## 3. Test-integrity checklist (`REBUILD_PLAN.md` §3.8)

A test that cannot fail is worse than no test: it inflates the ratchet and hides the defect it was written
to catch. The six checks below are the operational form of the `test-integrity.*` rows; a reviewer may
reject a change for one missing check, and *"the test passes"* is never the same claim as *"the behaviour is
proven"*.

- **Atomicity tests inject a failure *and* assert zero partial rows.** "All-or-nothing" is proven by
  forcing the failure (constraint violation, injected error, full disk), then asserting the before-state
  is intact: zero offspring, zero observations, unchanged status. A test that injects nothing and asserts
  success proves nothing — it would pass with the transaction removed
  (`test-integrity.no-vacuous-assertions`).
- **Every test names the boundary it exercises and asserts the negative case.** The name states the rule
  and the edge it probes — empty input, out-of-range, wrong type, forbidden identifier, duplicate — and
  the test asserts the refusal, not only the happy path (`test-integrity.named-for-boundary`).
- **Acceptance asserts values and invariants, never key presence or magic totals.** Money is asserted in
  minor units with the formula documented; a pack install asserts "every declared field is installed,
  every mode non-empty", never "14 fields / 2 modes / 5 widgets"; "type supported" is proven by the
  capability, never by whichever pack happens to use it (`test-integrity.assert-values`).
- **Tests dispatch the command surface, not storage internals.** A user-visible exclusion is asserted on
  the surface the user or an agent actually sees (command output, snapshot), not against a raw table read;
  fixtures come from `midata-testkit`, never from another module's private fixture
  (`test-integrity.command-surface`, `test-integrity.injected-dependencies`).
- **A test file's name states the behaviour under test.** A module named for a surface it does not test is
  a defect, not a convention; the two most misleading artefacts in the legacy suite were exactly this
  (`test-integrity.named-for-boundary`).
- **No vacuous clause anywhere.** No empty test bodies, no `assert!(a || b)` that is true by
  construction, no assertion on a constant, no duplicate scenario under a new name. If the test still
  passes when the feature is deleted, delete the test and write the real one
  (`test-integrity.no-vacuous-assertions`).

## 4. Adding a command

1. **Id:** lowercase `area.action` (`animal.create`, `observation.batch`, `data.importCommit`). Check the
   registry first — ids are **stable forever**, never renamed, never reused. A dropped behaviour leaves
   documentation, not a recycled id.
2. **Logic in the lowest crate that fits** (see step 3 of the loop). The command module stays ~30–60
   lines of dispatch, not business logic.
3. **Command module:** new or existing `midata-engine/src/<area>_cmds.rs` exposing `specs()`; register in
   `commands.rs` with a small surgical edit (re-read the file first — it is shared).
4. **Fill every field:** `id`, `title_key` (`cmd.<area>.<action>.title`, EN + FR), `params_doc`
   (what an agent reads via `midata commands`), `enabled` predicate, `run` returning
   `Result<CommandOutput, CommandError>`.
5. **Effects:** exactly one `audit_log` row per mutation (actor `user`/`cli`/`ai`/`seed`); invertible
   operations push one undo step; multi-write operations are transactional (all-or-nothing).
6. **Wire:** the screen/menu dispatches the id; the CLI exposes it automatically through `run`; an AI
   tool is generated from registry metadata (Phase 5) — no extra wiring needed.
7. **Tests:** happy path · **graceful failure** (empty/out-of-range/wrong-type/adversarial params →
   `Err`, never panic) · undo/audit assertions where applicable · oracle scenario mapped in
   `docs/testing.md`.
8. **Record:** inventory/gaps rows, `cargo xtask stats`, `command_hunt` green, devlog entry.

## 5. Adding a screen

1. One file `crates/midata-app/src/screens/<screen>.rs` + a `Screen` enum variant + nav wiring
   (core nav = Dashboard, Animals, Entry, Analytics, Assistant; advanced surfaces behind Developer Mode
   once Phase 6 lands).
2. State: a serde struct in the app-state module — inspectable, never scattered flags. Persistence keys
   go in the single constants module, prefixed `midata_`.
3. Build only from the widget kit and theme tokens: `DataTable`, `DynamicFieldRenderer`, modals, toasts,
   confirmed-destructive dialogs. **No hard-coded colours, sizes or strings.** `widgets/` never imports
   from `screens/`.
4. Behaviour through commands only — no domain logic, no direct store access from `midata-app`.
5. Design the empty, loading and error states (all i18n'd).
6. Snapshots: light + dark where layout matters, EN + FR spot check — **look at the PNG**.
7. Update the inventory row and the devlog.

## 6. Git workflow (D25, `REBUILD_PLAN.md` §13.4)

**Branches.** One short-lived branch per task: `t/<task-id>-<slug>` (`t/p0-1-workspace-ci`,
`t/p1-3-animal-create`). Parallel agents use one worktree each and one task branch each. **Never push to
`main`.** Push the task branch, open a PR; the **owner squash-merges** — or, inside the D31 envelope
(`AGENTS.md` §12), the SUPERVISOR does — → `main` holds exactly one green
commit per closed task. The branch is deleted after merge; the PR page keeps the detail. `--no-ff` is an
owner call, only when intermediate commits have archaeology worth keeping.

**Commits** (branch commits and the main squash commit alike):

```
<type>(<area>): <imperative subject ≤72 chars> [<task-id>]

<body: what & why; numbers when perf-relevant; any deviation noted>

Refs: gaps <task-id> · inventory <row-ids>
```

- `type` ∈ `feat · fix · refactor · perf · test · docs · chore · ci · build · bootstrap · integration ·
  revert`; `area` = crate or surface (`engine`, `store`, `app`, `ops`, `analytics`, `docs`, `ci`…).
- The `[task-id]` in the subject makes `git log --grep '\[P1-3\]'` a complete task archaeology tool.
- **No attribution trailer (D30):** no `Co-Authored-By:` line and no AI-vendor/model name in a commit
  message, in code, or in any doc, devlog or gap entry. Owner commits may be signed. Session and model
  notes live only in the gitignored `plan/` kit.
- **Green commits only.** Mid-task commits are checkpoints; a red commit is never pushed.

**Commit-message hook (D30).** `.git/hooks/` is never committed, so installing the hook is one
documented step per clone:

```sh
ln -sf ../../tools/hooks/commit-msg .git/hooks/commit-msg     # or copy it there, then chmod +x
```

`tools/hooks/commit-msg` runs `cargo xtask commit-msg <message-file>`, which rejects a message carrying
an attribution trailer or a banned AI vendor/model name (the term list lives in `xtask/src/terms.rs`,
assembled from fragments so the gate never flags its own definition). The hook **fails closed**: if the
checker cannot run (no `cargo`, a broken symlink, no message file), the commit is refused rather than
passed unverified. It needs no TTY and never prompts. `git commit --no-verify` still bypasses any hook,
which is why the same rule is enforced by `cargo xtask banned-names` over the tree in CI: the hook is
the fast local guard, the gate is the fact.

The in-tree half of the rule is the same command: `cargo xtask banned-names` runs both term lists and
fails on a new hit. Pre-existing hits are triaged in `docs/banned-names.baseline`
(`path<TAB>term<TAB>reason`); `reason` is `reference` (an integration/compatibility target, or a
document describing the ban) and `attribution` is never allowlisted. The allowlist is a ratchet — it
may only shrink, and a stale entry (no matching hit) fails until the baseline is regenerated with
`cargo xtask banned-names --write-baseline`.

Example:

```
feat(store): canonical latest-value flatten [P0-3]

One window-function statement replaces per-entity lookups; brute-force
cross-check test on the 2k fixture; flatten of 10k animals measured at 41 ms.

Refs: gaps P0-3 · inventory DE-7, HM-3
```

**What travels with what.** `Cargo.lock`, snapshot goldens and generated docs (`docs/scorecard.md`) are
committed **in the same commit** as the change that motivates them. An artifact never drifts from its
code.

**Never committed:** `target/`, `plan/`, `*.db` (+ WAL sidecars), `midata_data/`, logs, secrets/tokens,
large real-file corpora.

**Tags.** Annotated `vX.Y.Z` for every release (signed when a key exists) plus milestone tags
`m/phase-0` … `m/phase-7` at each owner UAT pass, as named rollback points. Tag message = the phase
exit-criteria summary. The legacy repo shipped 80 commits and zero tags; not once here.

**Traceability chain:**
`gaps.md task-id → branch t/<id> → PR → one green main commit [<id>] → devlog entry (<id>) →
inventory-row evidence (test names / snapshot paths) → scorecard numbers.`
Every artifact links back to its origin; every task is reconstructable from git alone.

## 7. Escalation — stop and ask the owner

1. Any DB schema change (new table/column/index/migration) — migrations are owner-approved, never
   smuggled into a feature task.
2. Deleting or overwriting data beyond your own test fixtures (user data, the seeded demo baseline, mass
   golden regeneration).
3. Adding, removing or upgrading a **dependency** (licence allowlist + maintenance review).
4. Publishing, pushing to `main`, or tagging releases.
5. Spending money or committing to external services.
6. Anything involving real personal data.
7. Destructive git operations (force-push, history rewrite).
8. Anything the rules do not clearly cover — **when in doubt, leave it out.**

Human-reserved: overall planning and roadmap ownership, release cutting, signing, branding/icon, UAT
sign-off. Everything else is yours to decide — decide it and record it in the devlog.

## 8. Parallel agents

One git worktree per agent; own `CARGO_TARGET_DIR=target/agent-<name>`; one owner per crate/area at a
time; edit only files you own. Shared files (the `commands.rs` list, app-state struct, `en.tsv`/`fr.tsv`,
migration list, workspace `Cargo.toml`) get small surgical edits — **re-read before editing**. New
commands and screens go in new modules. Write `Cargo.toml` changes atomically (temp file outside
`crates/`, then `mv`). If another agent's in-progress edit breaks the build, wait and retry — never fix
their files. Delete a finished agent's `target/agent-*` directory.

## 9. Review loop (independent reviewer per PR)

- Every PR that touches `crates/`, `apps/` or `xtask/` receives one independent REVIEWER session
  before owner merge. Docs-only PRs (< 50 changed lines, no code) get the fast path: checklist
  sections A + D only.
- The reviewer is a fresh session with no memory of the implementing session. It audits the diff
  against the contract (AGENTS.md, §3 slugs, D-numbers, the task row's acceptance criteria, DoD)
  using the fixed template in `docs/review-templates.md` (REVIEWER).
- Findings carry severities: BLOCKER · MAJOR · MINOR · SUGGESTION. A finding that cites no binding
  clause or oracle id is a SUGGESTION by definition. Verdict: APPROVE iff zero BLOCKER + zero MAJOR.
- The implementing agent receives the report (FIX-ROUND template) and either fixes each
  BLOCKER/MAJOR (new commits on the task branch — never history rewriting) or contests it with a
  counter-citation; contests are owner-arbitrated.
- Maximum two review rounds per PR; a third round means the dispute is architectural — owner
  arbitrates directly. MINOR/SUGGESTION never block; cheap ones are fixed, the rest become gaps.md
  follow-ups.
- The reviewer never modifies code, never merges. The owner merges after APPROVE (D25); inside the
  D31 envelope the SUPERVISOR merges under the six D31 conditions (`AGENTS.md` §12).
- Reports live as PR comments (canonical) + one devlog line each round.
