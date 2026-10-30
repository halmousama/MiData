# Decision log (ADR-lite)

> **Status:** living document · **Seeded:** 2026-10-26 (BOOTSTRAP-1) from `REBUILD_PLAN.md` §15
> **Authority:** `REBUILD_PLAN.md` §15 is the binding source. Entries here restate it in ADR form and
> add implementation-level decisions made inside the rules. Where the two disagree, §15 wins until the
> owner amends it. Owner decisions are never edited by agents — they are quoted and dated.
>
> **Rules for this file**
> - One entry per architectural decision: id · date · decision · consequence · source.
> - Only the owner adds, changes or reverses a `D*` decision.
> - Agent decisions taken under AGENTS.md §7 autonomy ("don't block on choices you can make yourself")
>   are recorded with an `A*` id, and the devlog entry references them.
> - A reversal keeps the original entry and adds a new one that supersedes it — decisions are never
>   rewritten (the same rule that makes command ids stable forever).

## 1. Owner decisions (REBUILD_PLAN §15, binding)

### D1 — Rebuild, don't port
**2026-10-27 (owner).** MiData Native is a fresh implementation in Rust + egui. The legacy system is
consumed as *specification only*: behaviour comes from the inventory rows, oracle scenarios and the
legacy behaviour report — never from legacy source. **Consequence:** no Python/TypeScript is read,
copied or translated; a behaviour question is answered by `docs/inventory.md` →
`docs/testing.md` → `docs/legacy-spec/PROJECT_REPORT.md` → owner, and the answer is written back into
the docs. (See also D15, and AGENTS.md rule 5.)

### D2 — No browser UI, mobile later
**2026-10-27 (owner).** The desktop app does not serve a browser UI. Mobile/remote access is a
documented future feature after the first stable release; the access method stays open.
**Consequence:** no HTTP server, no webview, no axum in v1; the contract for the future is written in
Phase 7 (`docs/mobile-access-strategy.md`, C8/§13.3). The legacy LAN/ngrok exposure is not rebuilt.

### D3 — English + French UI only
**2026-10-27 (owner).** Product UI languages are English (source) and French. No Arabic UI and no RTL
work in v1. **Consequence:** every user-facing string goes through `midata-i18n`; FR coverage is a
ratchet and reaches 100% (including engine-side messages) by end of Phase 6.

### D4 — Phased scope
**2026-10-27 (owner).** Scope is delivered in phases: core herd / entry / analytics / dashboard first;
AI and extensions later. **Consequence:** `docs/roadmap.md` and `ROADMAP.md` drive priorities; a task
outside the current phase needs an owner redirect.

### D5 — Fresh data, no legacy migration
**2026-10-27 (owner).** Data starts fresh with a redesigned schema. There is no migration tool or
importer for legacy databases, and none may be added. **Consequence:** `midata-store` migrations are
forward-only from an empty file; legacy DB files are irrelevant to this product.

### D6 — EAV core preserved
**2026-10-27 (consultation).** The Entity–Attribute–Value model stays as the product bet: any farm can
define its own measurable attributes without a schema migration. **Consequence:** `field_definitions`
+ `observations` are central; dynamic forms, modes, analytics, widgets and packs are built on them.

### D7 — Identity attributes promoted to columns
**2026-10-27 (owner + architect delegation).** Universal animal facts are structural columns with
CHECK/FK constraints and Rust enums: `custom_id`, `species_id`, `sex`, `breed` (nullable text),
`birth_date` (age computed, never stored), `sire_id`/`dam_id` plus phantom `sire_tag`/`dam_tag`,
`status` (Alive/Sold/Dead) with `exit_date`/`exit_reason`, `notes`, timestamps. The exact universal set
was delegated to the architect and is frozen in `REBUILD_PLAN.md` §5.2/§5.5.
**Consequence:** identity facts are never stored as dynamic fields; adding an identity-like column
requires a new owner decision here.

### D8 — Species is an open registry
**2026-10-27 (owner).** Presets are Cow, Sheep and Goat; the user may add any species (Camel, Horse,
Dog, …). No code enumerates species; per-species defaults (gestation days, field packs, labels, icons)
are data. **Consequence:** adding a species requires zero code changes; a Phase-0 test asserts the
seed generator emits only registry presets.

### D9 — Pig policy
**2026-10-27 (owner).** The species is never included in presets, seed/demo data, tests, fixtures,
documentation or screenshots, and no code path specially blocks it. **Consequence:** the codebase
simply never mentions it — the product stays neutral. Enforced by review and by the seed-content test
(P0-6), not by a runtime block.

### D10 — AI language behaviour
**2026-10-27 (owner).** Assistant system instructions are always in English; the response language is
never forced — the model mirrors the user's language, with one stabilizing prompt line allowed.
**Consequence:** insight/analytics text is never LLM-generated (see D16); only chat prose mirrors the
user.

### D11 — Widgets: curated first, dynamic via the validation lab
**2026-10-27 (owner + architect).** The owner prefers dynamic widgets from day one; the agreed middle
path is curated widgets in Phase 3 with query-as-data-source first-class in the schema, then
user-built and AI-generated widgets in Phases 5–6 on top of the SQL validation lab.
**Consequence:** `widgets.data_config_json` exists from the first migration that ships widgets; dynamic
widget creation is gated on `midata-queryguard`.

### D12 — Linux dev, Windows release priority
**2026-10-27 (owner).** Development happens on Linux; Windows is the release priority; CI runs both
from Phase 0; macOS packaging is optional later. **Consequence:** no unix-only assumptions (paths, file
locks, separators, line endings, locale-dependent number formats in persisted data). "Done" means the
gate is green on both OSes.

### D13 — Agent-first is a first-class principle
**2026-10-27 (owner).** *"Software in this era must be oriented toward agents as much as toward
humans."* The command registry exists from Phase 0, state is serde-inspectable, the CLI is
first-class, control-channel/MCP seams exist from day one, and the audit log doubles as agent
observability. **Consequence:** every user-visible behaviour is a registered command; UI, CLI, AI tools
and (Phase 6) the control channel dispatch by id.

### D14 — Methodology imported from ArtCraft
**2026-10-27 (consultation).** `AGENTS.md` constitution (adapted, attributed, ≤8 KB), never-crash
golden rules (stricter than the reference: `unsafe` forbidden with no exceptions), file budgets and
ratchets, the gaps/inventory/scorecard/devlog doc system, the five-step task loop, tests-first,
definition of done, the escalation list, parallelism rules, agent-hour estimates, offscreen snapshot
verification and generated numbers. **Consequence:** the process artifacts are part of the product and
are CI-enforced.

### D15 — Legacy test scenarios are behaviour oracles
**2026-10-27 (consultation).** The legacy 485-test suite is harvested *once* as acceptance criteria
(test names, documented behaviours, edge cases) and never as code. **Consequence:** `docs/testing.md`
is the mapping table; a behaviour without a mapped scenario is written fresh from the plan prose and
marked as such.

### D16 — Single analytics engine
**2026-10-27 (architect).** One pure-Rust analytics engine over SQLite serves UI, CLI, widgets and AI
tools. DuckDB and polars are excluded. **Consequence:** no second engine, no parallel implementation of
an existing computation; insights are deterministic i18n templates and the LLM is never in the
analytics path.

### D17 — Secrets in the OS keyring
**2026-10-27 (architect).** Secrets (the LLM API key) live in the OS keyring only; no crypto derived
from hard-coded constants, no API keys in binaries, settings files, SQLite or logs.
**Consequence:** the legacy embedded-key class of defect is structurally impossible here.

### D18 — chrono for time, integer minor units for money
**2026-10-27 (architect).** All date/time handling goes through `chrono`; money is `i64` minor units
plus a currency code, default **TND** (selectable TND/USD/EUR). **Consequence:** no hand-rolled
calendar math, no floats for money, ISO-8601 text in storage, conversion/formatting only at UI edges.

### D19 — Printing becomes PDF export
**2026-10-27 (consultation).** Direct printing is out of scope; animal cards and report sheets are
exported as PDF. **Consequence:** `midata-exchange` owns the PDF dependency (crate choice is §16 O4);
the legacy "print cards" screen becomes an export action.

### D20 — Naming
**2026-10-27 (consultation).** Product name is always **MiData** in user-facing text; machine names are
lowercase (`midata-*` crates, `midata`/`midata-cli` binaries, `area.action` command ids, `midata_*`
storage keys). The legacy name **"Rumetrics" is banned** — zero occurrences anywhere, CI greps for it.
**Consequence:** all persistence keys live in one constants module; renames are reviewed against the
grep gate.

### D21 — One-step onboarding
**2026-10-27 (consultation).** Onboarding collapses to a single step (species presets + demo-data
toggle), honouring the legacy ADR-004 intent; Developer Mode is implemented for real in Phase 6 with
its flag plumbed from Phase 1. **Consequence:** no wizard, no five-step flow; onboarding is merge-only
and idempotent.

### D22 — Synchronous core, isolated async
**2026-10-27 (architect).** Domain, store, analytics, engine and app are synchronous; long work runs on
std background threads with `crossbeam-channel`; `tokio` exists only inside `midata-ai` for
`reqwest`/SSE. **Consequence:** no async runtime leaks into the core; DB access is a single
`Mutex<Connection>` on WAL (single-user app, negligible contention).

### D23 — Crate layout L0–L5 with a dedicated engine
**2026-10-27 (architect).** Layers L0–L5 including a dedicated `midata-engine` (command registry) plus
`apps/midata`, `apps/midata-cli` and `xtask`. **Consequence:** see D24 for the refined set.

### D24 — Crate architecture refined (registered DAG)
**2026-10-27 (owner consultation).** Added `midata-i18n` (L0, shared by app and CLI), `midata-ops`
(L2 workflows — keeps engine commands thin dispatch), `midata-queryguard` (L2 SQL-validation lab placed
*below* the engine so AI and dynamic widgets share one validator), `midata-exchange` (L2 file formats —
isolates heavy deps) and `midata-testkit` (dev fixtures). Layers form a **registered DAG**, not a strict
chain; `midata-app` depends only on domain + engine + i18n (+ the egui stack); crates are born in the
phase that first uses them; split triggers are documented in §4.8.
**Consequence:** `cargo xtask check-layers` reads `xtask/src/layers.rs`; an undeclared edge is a build
failure, so a layer violation is a CI fact rather than a review opinion.

### D25 — Git workflow and traceability
**2026-10-27 (owner consultation).** Private GitHub remote; protected `main` (no direct pushes — the
owner squash-merges PRs); one branch per task `t/<task-id>-<slug>`; conventional commit subjects
carrying `[task-id]`; green commits only, with
mid-task commits as session checkpoints; goldens, generated docs and `Cargo.lock` committed with their
motivating change; annotated release tags `vX.Y.Z` plus milestone tags `m/phase-N`; the full
traceability chain of `REBUILD_PLAN.md` §13.4. **Consequence:** `docs/contributing.md` §Git is the
operational form; tagging, force-push and history rewriting are escalation items.
**Partly superseded (D30):** the mandatory `Co-Authored-By` attribution clause is repealed.

### D26 — Schema is birth-gated
**2026-10-27 (owner).** Each table is created by the migration of the phase that first uses it. Phase 0
creates species…`audit_log` **except** `widgets`; P3 creates `widgets` (+ default-widget seeding with
the dashboard); P5 creates `conversations`, `messages`, `pending_changes`; P6 adds the pack-provenance
columns (`origin`/`pack_id` on `field_definitions`/`modes`/`widgets`) through an owner-approved
migration. **Consequence:** "no dead schema" (§5.1.5) outranks "full v1 schema at once"; P0-3 migrates
only the Phase-0/1/2 tables, and every later table arrives wired to code in the same task.

### D27 — Bootstrap rulings batch
**2026-10-27 (owner).** The nine bootstrap ambiguities are ruled: (1) `docs/roadmap.md` carries the
mirror stamp — `REBUILD_PLAN.md` §14 is authoritative, the file holds live status only (A2 ratified);
(2) A4 ratified — the root commit on `main` was legal, task branches from P0-1 onward; (3) the `P0-n`
task-id scheme is canonical; (4) P0-8's inventory reconciliation is rescoped; (5) i18n lives in
`midata-i18n` (D24 authoritative); (6) `AGENTS.md` becomes a **lean bootloader** — rules not
procedures, one authoritative home per topic, target ≤ ~12 KB (P0-10 amended); (7) the licence holder
"MiData contributors" stays provisional and O1 remains open; (8) remote + branch protection are live,
so P0-1 must prove CI on both OSes; (9) new Git rule — **no session ends with uncommitted work**.
**Consequence:** P0-12 records the amendments; P0-10 is re-scoped (not dropped); migrations and later
documentation follow D26.

### D28 — Anti-pattern register extended by the oracle extraction (M7 review)
**2026-10-27 (owner review of M7).** `REBUILD_PLAN.md` §3 is extended by **37 owner-approved rows** drawn
from the eight oracle files' `## Anti-patterns` sections. They are grouped thematically — analytics
results · query safety · AI contract · data I/O & onboarding · pedigree · storage & settings · test
integrity — and keyed by a **stable slug** (`analytics.typed-results`, `query.bound-params`,
`test-integrity.no-vacuous-assertions`) instead of the proposal's review numbers P1–P37. Rows 1–15 keep
their numbers as stable citation handles; findings an existing row already covered were folded in as
one provenance clause, never duplicated. Four rows were reworded by the owner and merged verbatim (`P14`
→ `contract.stable-keys`, `P20` → `state.command-only-mutations`, `P26` →
`config.business-vocabularies`, `P30` → `settings.typed-facade`). Sub-decisions: **(a)** a single query
language — the legacy DSL is dropped, SQL through `midata-queryguard` only; **(b)** the public and
agent-facing contract carries stable machine keys only — no storage row ids, no table names; **(c)**
settings are a typed serde facade with `#[serde(default)]` over the K/V table.
**Consequence:** `docs/proposals/anti-pattern-register-extension.md` is deleted — §3 is the single
authoritative home, and provenance survives here and in each oracle file's slug pointers. Test integrity
(§3.8) is operationalised as a checklist in `docs/contributing.md` §3. Slugs are contract: never renamed,
never reused.

### D29 — Independent reviewer loop per PR
**2026-10-27 (owner consultation).** Every PR that touches `crates/`, `apps/` or `xtask/` receives one
independent REVIEWER session before owner merge; docs-only PRs (under 50 changed lines, no code) take the
fast path (checklist sections A + D). The reviewer is a fresh session with no memory of the implementing
one, audits the diff against the fixed template in `docs/review-templates.md`, and reports findings as
BLOCKER · MAJOR · MINOR · SUGGESTION — a finding that cites no binding clause or oracle id is a SUGGESTION
by definition. The verdict is APPROVE iff zero BLOCKER + zero MAJOR. The implementer fixes BLOCKER/MAJOR on
the task branch (never by rewriting history) or contests with a counter-citation, and the owner arbitrates
contests. Two rounds maximum; a third means the dispute is architectural. The reviewer never modifies code
and never merges.
**Consequence:** `docs/contributing.md` §9 is the operational form; reports live as PR comments (canonical)
plus one devlog line per round.

### D30 — No attribution trailers, and the history that carried them was scrubbed
**2026-10-27 (owner ruling).** D25's mandatory `Co-Authored-By:` trailer is repealed. No `Co-Authored-By`
line and no AI-vendor/model name may appear in a commit message, in code, or in any doc, devlog or gap
entry. Session and model traceability, if the owner wants it at all, lives only in the gitignored `plan/`
kit — never on a published surface.
**Consequence:** the banned-names gate grows a vendor/model term list and a `commit-msg` hook enforces the
rule locally (both still open); until they land the rule is review-enforced, and
`AGENTS.md` says so rather than claiming a gate that does not exist. The trailers already in `main`'s
history were removed on 2026-10-27 by rewriting every ref with `git-filter-repo` using a message-only
callback: every tree stayed byte-identical (`main`'s tree hash `b886fe2f` before and after, all 7 commits
preserved in order), and the push was `--force-with-lease` against the verified old tip. Pre-scrub SHAs
`8979bc0`/`123ffb2`/`bf9813d`/`b65e614` survive only as closed-PR archive; the pre-scrub backup bundle sits
outside the repo at `../midata-pre-scrub.bundle` and is never committed.

### D31 — Delegated merge authority and the autonomous cycle
**2026-10-27 (owner consultation).** A SUPERVISOR agent may run consecutive task cycles without owner
action. It never implements and never reviews: it picks the top `docs/gaps.md` entry, spawns a
fresh-context IMPLEMENTER, then a fresh-context REVIEWER that receives parameters only (no summaries, no
opinions, no implementer devlog claims), arbitrates at most two review rounds, and squash-merges with
`gh pr merge --squash --delete-branch` — GitHub-side. Since D32 the *proof* behind that merge is the
self-reported local gate plus, where §3 rule 9 requires it, a cited manual dispatch; the required-check
gate cannot be satisfied by a dispatch, so it was removed from `main`'s protection (supervisor, before
that PR merged — the rule itself blocked the merge) and the supervisor cites evidence rather than relying
on the merge button.
**Merge conditions (all six required; any failure = stop and escalate):** (1) reviewer APPROVE with
0 BLOCKER/0 MAJOR · (2) **CI proof in two parts — see D32**: (2a) the local gate (`cargo xtask ci`) green
on the reviewed head, its final line quoted in the PR and the reviewer's re-run on that head; (2b) a
manual both-OS dispatch green on **the exact head being merged**, required for **every non-docs-only
merge** and cited by run id in the merge report — docs-only/fast-path merges cite the local gate only,
and a merge without its cited run id is a record defect (D32 owner amendment, 2026-10-30) ·
(3) **no escalation flag on the
task** — the `docs/gaps.md` row's Status reads `escalated`, or a local `plan/ESCALATE` marker exists
alongside `plan/HOLD` · (4) the diff touches no owner-reserved surface (schema migrations, the
dependency set, deletion of committed data, the security-adjacent §3 rows — §3.1 rows 8–10, §3.3,
§3.4 — branding) · (5) a **merge report paragraph in the devlog** — owed by stage 6 of the same
cycle; the cycle is not complete until that paragraph exists and its absence is an escalation, which
is how the condition is checked · (6) no `plan/HOLD` file.
**Superseded (CI clause): D32, 2026-10-30.** The original condition 2 ("both required checks green on the
PR head") and the "required checks are enforced server-side" guarantee above no longer hold; D32 replaced
them with (2a)/(2b) and removes the required-status-checks rule from `main`'s protection (supervisor,
before this PR merged — the rule itself blocked the merge). The rest of D31 —
the six conditions, the budget, the revocation path and the fresh-context rationale — stands unchanged.
**Limits:** default budget of 3 tasks then an owner digest; a task failing twice escalates instead of
being retried; a round-3 review escalates; the owner revokes at any time by creating `plan/HOLD` or by
amending this row. The owner keeps merge authority on escalations, phase gates and releases.
**Why fresh contexts rather than one long-lived session:** a single session accumulates drift and loses
exact decision text to compaction, and a supervisor that merges its own work removes the owner's last
systematic checkpoint. Fresh workers make the repository the only memory — `AGENTS.md`, `docs/gaps.md`,
`docs/decisions.md` and `log/devlog.md` are re-read every cycle, so no binding rule can survive only
inside a conversation summary.
**Precondition verified 2026-10-27:** a spawned subagent reports shell access, `git 2.53.0`, an
authenticated `gh` (scopes `repo`/`workflow`) and filesystem write access — the loop's tooling premise
holds on this machine. If a future harness cannot give workers git/`gh`, D31 is void and the owner runs
the cycles by hand.
**Consequence:** D25's "the owner squash-merges every PR" becomes "the owner squash-merges every PR
outside the D31 envelope"; `AGENTS.md` §12 is the operating procedure, `docs/review-templates.md` gains
the SUPERVISOR template, and every delegated merge leaves a report paragraph in the devlog.

### D32 — CI is manual-dispatch only
**2026-10-30 (owner ruling).** The owner's instruction: *"disable it without removing, i want to run it
only manually."* `.github/workflows/ci.yml` therefore loses its `push` and `pull_request` triggers and
keeps `workflow_dispatch`; the `ubuntu-latest` + `windows-latest` matrix, both jobs and every step stay
byte-identical. A run is started with `gh workflow run ci.yml --ref <branch>`, inspected with
`gh run list --workflow ci.yml --branch <branch>`, and awaited with `gh run watch <run-id>`. The
`concurrency` group stays as it was — `group: ci-${{ github.workflow }}-${{ github.ref }}`,
`cancel-in-progress: true` — so a manual run on a branch cancels only that branch's earlier run.
**Why.** Wall-clock: the gate already exists as a single local definition (`cargo xtask ci`) that runs the
same block on the same code, so an automatic run on every push and every PR bought latency at the cost of
runner minutes. The trigger is the only thing removed — revocation is one diff away.
**Supersession.** This amends D31's condition (2) and `AGENTS.md` §12: "both required CI checks green on
the PR head" is no longer satisfiable. The replacement is checkable in two parts — **(2a)** the local gate
ran green on the reviewed head, its final line is quoted in the PR description, and the reviewer re-ran
it (the self-reported half); **(2b)** a manual both-OS dispatch is green on that head for any change in
the platform-sensitive class of §3 rule 9 (Windows path handling, file locks, process spawning,
case-insensitive filesystems). **Accepted risk:** because (2a) is self-reported, a Windows-specific defect
can reach `main` with nothing server-side having seen it — run `41133273505`'s `failed to remove …
target\debug\xtask.exe … Access is denied (os error 5)` is exactly the class of failure only the Windows
job could catch, and it did once already. **Mitigations:** dispatch before any phase gate, release tag,
or merge touching a platform-sensitive surface — **widened by the amendment below (2026-10-30) to every
non-docs-only merge on the exact head being merged**; a release tag requires a green both-OS dispatch on record.
**Branch protection (supervisor, done before the merge).** A `workflow_dispatch` run is **not** a PR check: the
dispatched checks are green on the exact PR head while `gh pr checks` still reports "no checks reported"
and the merge state reads `BLOCKED`. With `ci (ubuntu-latest)` / `ci (windows-latest)` required on `main`,
every merge would therefore need the admin bypass (`enforce_admins: false`) — security theatre, not a
control. **Ruling:** the required-status-checks requirement is removed from `main`'s protection (the
force-push and deletion blocks stay), and the supervisor verifies condition (2b) by running and citing the
dispatch instead of letting GitHub gate the merge button. That settings change was made **before** the
merge, not after: `gh pr merge` was refused with "the base branch policy prohibits the merge", so the
documented order was impossible and the record states what actually happened.
**Consequence:** the honest posture changes from "CI verifies every PR" to "CI verifies what we ask it
to"; the local gate is now load-bearing for every merge, and its output must be quoted rather than
assumed. `docs/contributing.md` §2 carries the contributor-facing command.
**Amendment (owner, 2026-10-30):** every non-docs-only merge cites, in its merge report, a both-OS
`workflow_dispatch` run executed on the exact head being merged; docs-only/fast-path merges cite the
local gate only. A merge without its cited run id is a record defect.

### D33 — Rulings on the four items left open by the first delegated run
**2026-10-31 (owner ruling).** The first D31 run closed with four items awaiting the owner; all four are
now resolved. None of them changes a §3 iron rule — they settle how an existing rule is read, or move a
known defect into the queue.

1. **R7a (PR #6) — the D30 reading, accepted with one absolute-ban carve-out.** The owner accepts the
   **reference-vs-attribution** reading recorded in D30 and P0-14 (d): a vendor or model term naming an
   integration or compatibility target, or a document that describes the ban itself, is a **reference**,
   not an attribution — which is what lets `docs/architecture.md` say `OpenAI-compatible`. **Carve-out:**
   the two terms `DeepSeek` and `opencode` are **absolutely banned** — no reference exemption, no
   context carve-out, no allowlist path — because they identify this project's own tooling and model
   sessions rather than any integration target or third party. A hit is a violation everywhere,
   including a file that documents the ban.
   **Consequence (supervisor, measured 2026-10-31):** `deepseek` is already a `Vendor` term in
   `xtask/src/terms.rs` and currently has **zero** in-tree hits, so the carve-out costs nothing there;
   `opencode` is **not** in the term list and has **one** pre-existing hit — `log/devlog.md:355`, a
   historical note naming a temporary directory. Under an absolute ban no baseline entry may buy it, so
   that line is rephrased (the same remedy D30's own adoption used) and both terms gain a list entry the
   baseline generator can never allowlist. Enforcement is **P0-16**, which absorbs the carve-out.
2. **R10 (PR #7) → P0-16.** The `docs/banned-names.baseline` allowlist is keyed `(path, term)`, so a
   *second* attribution line inside an already-allowlisted file passes the gate. The fix is that an
   allowlist row carries its **exact hit count**, so a new hit in an allowlisted file is still a new
   violation. Queued as P0-16 and executed together with the D33 carve-out above because both change the
   same triage surface.
3. **S1 (PR #8) → P0-17.** `EncodedValue` exposes public variants, so `Num(NAN)` is constructible outside
   `encode`. Decode rejects it, but the invariant belongs in the type: **private variants plus smart
   constructors**. Queued as P0-17.
4. **D12/A4 stale ADR spans (PR #10/#11 round 2) → folded into P0-8, not a new row.** Two spans of ADR
   prose still describe the pre-D32 world; nothing operative asserts them, and the inventory row-level
   pass already owns "every ADR/inventory claim checked against the code". P0-8's acceptance criteria now
   name this explicitly rather than leaving it as an unowned remark.

**Why the queue and not a decision row each.** All four are small, mechanical and already traced to a
review finding; inventing binding clauses for them would duplicate what P0-16/P0-17/P0-8 acceptance
already say.

## 2. Open items with defaults (REBUILD_PLAN §16)

These are not decisions yet — they are defaults that hold until the owner rules. Listed here so the
"why is it like this?" question has one place to look.

| # | Item | Default in force | Decide by |
|---|---|---|---|
| O1 | Licence | **MIT OR Apache-2.0** dual (files created in BOOTSTRAP-1; copyright holder "MiData contributors" **provisional** — owner may change holder or licence) | Phase 0 — still open (D27) |
| O2 | Undo scope | Session-scoped undo for invertible data-entry commands; persistent audit remains the record | Phase 2 |
| O3 | Pie/donut charts | Custom-painter donut in P3; fallback = horizontal bars + gaps entry | Phase 3 |
| O4 | PDF crate | `printpdf` vs `genpdf` — spike decides | Phase 3 |
| O5 | App icon/branding | Original icon, owner-approved, no echoes of existing brands | Phase 7 |
| O6 | macOS packaging | Deferred unless hardware/CI need appears | Phase 7 |
| O7 | Species beyond three presets | Stay at 3 presets; others user-addable with the generic pack | any time |

## 3. Bootstrap decisions (A-series, agent-made under AGENTS.md §7)

### A1 — Licence files created under the O1 default
**2026-10-26.** `LICENSE-MIT` + `LICENSE-APACHE` were created with `Copyright (c) 2026 MiData
contributors`, because O1's recommended default is "MIT OR Apache-2.0 dual" with "Decide by: Phase 0".
**Consequence:** the repository is legally usable from the first commit; the owner may still choose a
single licence or a different copyright holder, which would be a §15 amendment plus a rewrite of both
files. **No `Cargo.toml` licence field exists yet** — it is set once, in P0-1, to `MIT OR Apache-2.0`.

### A2 — `docs/roadmap.md` mirrors §14 instead of inventing a roadmap
**2026-10-26.** `AGENTS.md` §1/§8 and `REBUILD_PLAN.md` §11.1 reference `docs/roadmap.md` (phases 0–7,
scope, exit criteria, current focus), but Appendix C's artifact list omits it. It was created as a
faithful mirror of `REBUILD_PLAN.md` §14 with a *Current focus* line, and carries no new scope.
**Consequence:** no dangling doc reference; phase scope changes still require a `REBUILD_PLAN.md`
amendment. **Ratified 2026-10-27 (D27)** with an explicit header stamp (mirror + "live status only").

### A3 — Dependency versions are not pinned anywhere yet
**2026-10-26.** BOOTSTRAP-1 writes no Rust, so `rust-toolchain.toml` pins only the *channel*
(`stable`). Crate versions are chosen and recorded in `[workspace.dependencies]` in P0-1, where the
dependency-escalation rule (AGENTS.md rule 15) applies for the first time.
**Consequence:** library versions are decided with licence/maintenance review at P0-1, not here.

### A4 — The first commit is a root commit on `main`
**2026-10-26.** D25 requires one task branch per task and forbids direct pushes to `main`, but a fresh
repository has no commit to branch from and the owner's remote does not exist yet.
**Consequence:** BOOTSTRAP-1 is a single root commit on `main` (`[B0]`); from P0-1 onward every task
starts from a `t/<task-id>-<slug>` branch, and `main` is protected once the owner creates the remote.
**Ratified 2026-10-27 (D27):** remote and branch protection are live, so the PR flow is active from
P0-1 and CI on both OSes is the merge gate.

### A5 — Anti-pattern consolidation stays a proposal file
**2026-10-27.** The eight oracle files' `## Anti-patterns — never reproduce` sections were folded into a
deduplicated draft at `docs/proposals/anti-pattern-register-extension.md` instead of editing
`REBUILD_PLAN.md` §3, which only the owner amends.
**Consequence:** the owner reviews one diff and merges it into §3 (or rejects rows); until then §3
remains binding and the proposal has no authority. **Closed 2026-10-27 (D28):** the owner approved all
37 rows, §3 became their home, and the proposal file was deleted. The `docs/proposals/` convention for
owner-review-only documents survives.

### A6 — `t!(translator, "key")`: the language is injected, never ambient
**2026-10-27.** `REBUILD_PLAN.md` §7.4 and early drafts of `docs/i18n.md` wrote the lookup as
`t!("key")`, which implies a process-wide "current language". `docs/architecture.md` §9 forbids state
globals, and a global language would also make a two-language snapshot render impossible. The macro
therefore takes the `Translator` explicitly (`t!(translator, "key")`, `tf!(translator, "key",
"name" => value)`), the catalogue is parsed once by the composition root, and `midata-i18n` holds no
`static`.
**Consequence:** every surface (egui shell, CLI, future control channel) passes its translator down,
exactly like the `Clock` and the store handle; a miss still renders the key itself. Written into
`docs/i18n.md` §1 and `docs/architecture.md` §9.
