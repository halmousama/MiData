# Scorecard

> # ⚠ GENERATED FILE — DO NOT EDIT
>
> This file is produced by `cargo xtask stats` and diffed by CI. A hand edit is a failing build.
> **Current state:** the generator does not exist yet (`xtask` arrives with P0-1). Everything below is
> a **placeholder** and must be replaced by real output in the same task that adds `stats`.

| Metric | Value | Source | Ratchet |
|---|---|---|---|
| Inventory coverage | 0 / 195 rows (0%) | `docs/inventory.md` (hand-counted v0) | ↑ only |
| Tests | 0 | nextest (not yet available) | ↑ only |
| Crates | 0 | workspace (not yet created) | — |
| Screens with ≥1 snapshot golden | 0 | `crates/midata-app` goldens | ↑ only |
| i18n coverage (EN) | n/a | `cargo xtask i18n-coverage` | = 100% |
| i18n coverage (FR) | n/a | `cargo xtask i18n-coverage` | ↑ to 100% by P6 |
| File-budget violations (soft 500 / alarm 800) | n/a | `cargo xtask file-budget` | ↓ to 0 |
| Clippy warnings | n/a | `cargo clippy -D warnings` | = 0 |
| `unsafe` occurrences | n/a | workspace lint `forbid` | = 0 |
| Flatten 10k animals | not measured | `cargo xtask bench` | < 500 ms |
| Dashboard hydrate (10 widgets, 1k animals) | not measured | `cargo xtask bench` | < 300 ms |
| Cold start | not measured | `cargo xtask bench` | < 2 s |
| Import 1k rows | not measured | `cargo xtask bench` | < 10 s |

## Notes

- Until `stats` exists, the row counts come from the hand-maintained table at the end of
  `docs/inventory.md`; once the generator lands, it reads the inventory table directly and the hand
  table is removed. Generated output is the single source of these numbers (`ROADMAP.md` quotes it,
  never re-counts by hand).
- A met perf budget flips to `enforce = true` and is never un-enforced (`REBUILD_PLAN.md` §11.8).
- Floors never lower. If a number would decrease, the change is wrong, not the number.
