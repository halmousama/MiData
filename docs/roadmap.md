# Roadmap — phases 0–7

> **MIRROR of `REBUILD_PLAN.md` §14 (authoritative); this file carries live status only; never edit
> scope/exit-criteria here.** It exists because the orientation table points here for "scope, exit
> criteria, current focus"; it adds no scope of its own. A phase-scope change is an amendment to the
> plan, then a re-sync here — never the reverse (D27/A2).
> **Updated:** 2026-10-30 (P0-2f docs truth pass) · **Current focus:** **Phase 0 — Foundations —
> P0-2f → P0-14 → P0-2 re-land → P0-3**

## Current focus

**Phase 0 — Foundations.** P0-1 (workspace + `xtask` + CI) is closed on `main`, so the gate exists on
ubuntu and windows and the scorecard is generated. But **`main` contains no `crates/` directory at
all**: P0-2's implementation and its round-1 review fixes exist only on branch `t/p0-2-domain-i18n`
(remote tip `cb4405c`), and **PR #3 was closed without being merged on 2026-10-27T09:35:38Z**
(`mergedAt` is null). Phase 0 has therefore shipped infrastructure, not one line of product code.
Next, in order: **P0-2f** (this docs truth pass — the focus line had been stale since PR #3, so every
session booted from a wrong "current focus") → **P0-14** (D30 mechanical enforcement: `commit-msg`
hook + vendor/model term list) → **P0-2 re-land** (put the already-reviewed domain + i18n branch on
`main` as a fresh PR) → **P0-3** (`midata-store`). **P0-3 is blocked until P0-2 is on `main`**, because
P0-3 depends on it. The `AGENTS.md` diet (P0-10) moved down the queue: P0-2f removes the acute
wrong-focus tax, and the unblocking chain now outranks the diet (still open, still owner-reviewed).

Phase 0 closes only when its exit criteria below are met **and** the owner signs off; Phase 1 tasks
are then populated in `docs/gaps.md`.

## Phase table

| Phase | Name | Estimate | Exit criteria (summary) |
|---|---|---|---|
| 0 | Foundations | 40–80 h | `xtask ci` green on ubuntu+windows · app boots to a dashboard with demo data · onboarding idempotent · migration idempotency tests · `AGENTS.md` lean bootloader ≤ ~12 KB · 0 file-budget violations · snapshots for shell + onboarding · devlog active |
| 1 | Core herd | 80–150 h | HM/DE-core inventory rows done with evidence · flatten perf budget passing · all screens snapshotted · fuzz green · entity/measurement oracle scenarios (transcribed) passing |
| 2 | Organization & rules | 40–80 h | demo-data alerts fire correctly · mode switching visibly re-scopes every data surface · every rule type tested including malformed expressions |
| 3 | Analytics, dashboard, data I/O | 100–180 h | analytics oracle scenarios passing · 1,000-row xlsx round-trip with both conflict strategies · backup restore verified on a copy · scorecard committed and CI-diffed · donut decision resolved |
| 4 | Pedigree & reproduction | 60–120 h | birth atomicity proven (mid-transaction failure → zero partial state) · 5-generation demo pedigree renders with phantoms · all PD-* oracle scenarios green |
| 5 | AI assistant | 100–200 h | end-to-end chat against a local LM endpoint · injection suite green · restart-survival test for staged tokens · audit rows verified · assistant UI snapshots |
| 6 | Packs, dynamic widgets, agent surfaces, polish | 60–120 h | pack install/uninstall atomicity · dynamic-widget injection suite green · external MCP client drives a read-only command · Developer Mode verified by snapshots · FR i18n = 100% |
| 7 | Release & documentation | 30–60 h | clean Windows 10/11 VM: install → onboard → core loop without a crash · Ubuntu AppImage run · packaged backup→restore verified · docs published · release tagged |

**Total:** ~510–990 agent-hours (initial, recalibrated after Phase 1 — `REBUILD_PLAN.md` §11.7).

## Phase scope notes that shape day-to-day work

- **Crates are born in the phase that first uses them** (§4.1, D26): P0-1 ships `xtask` only; P0-2
  births domain + i18n; P0-3 store + ops; P0-4 engine + testkit; P0-5 app + `apps/midata`; P0-7
  `apps/midata-cli`; P3 adds analytics + exchange; P5 adds queryguard + ai. No empty crates, ever.
- Every phase ends with: gate green on both OSes · snapshots updated and viewed · inventory/gaps/
  scorecard/devlog current · an owner UAT checkpoint · a milestone tag `m/phase-N`.
- Mobile/remote access is **not** built in any phase; Phase 7 writes the strategy document only (D2/C8).
- The legacy sharing/desktop surfaces (LAN server, tunnel, QR, tray, PyInstaller, SPA bundle) appear in
  no phase — they are dropped by design (`REBUILD_PLAN.md` §6.2).
