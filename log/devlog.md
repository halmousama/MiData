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
