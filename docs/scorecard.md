# Scorecard

> # ⚠ GENERATED FILE — DO NOT EDIT
>
> Produced by `cargo xtask stats` and diffed by CI. A hand edit is a failing build.
> Deterministic by design: no timestamps, sorted inputs, `/` separators on every OS.
> Every number names its source; an unmeasured budget says `not measured`, never a guess.

| Metric | Value | Source | Ratchet |
|---|---|---|---|
| Inventory coverage (done rows with evidence) | 0 / 222 rows (0%) | `docs/inventory.md` | ↑ only |
| Tests (source scan) | 76 | `#[test]`-family attributes under `xtask/`, `crates/`, `apps/` | ↑ only |
| Crates in the workspace | 1 | `cargo metadata --no-deps` | — |
| Screens with ≥1 snapshot golden | 0 / 11 | `crates/midata-app/**/snapshots/**/*.png`; target = `REBUILD_PLAN.md` §7.1 screens | ↑ only |
| i18n coverage (EN) | n/a (no catalogue yet) | `cargo xtask i18n-coverage` | = 100% |
| i18n coverage (FR) | n/a (no catalogue yet) | `cargo xtask i18n-coverage` | ↑ to 100% by P6 |
| File-budget violations (soft 500 / alarm 800) | 0 | `cargo xtask file-budget` | ↓ to 0 |
| Clippy warnings | 0 (enforced by the gate) | `cargo clippy -D warnings` | = 0 |
| `unsafe` occurrences | 0 | source scan + workspace lint `forbid` | = 0 |
| Flatten 10k animals | not measured | `cargo xtask bench` (skeleton) | < 500 ms |
| Dashboard hydrate (10 widgets, 1k animals) | not measured | `cargo xtask bench` (skeleton) | < 300 ms |
| Cold start | not measured | `cargo xtask bench` (skeleton) | < 2 s |
| Import 1k rows | not measured | `cargo xtask bench` (skeleton) | < 10 s |

## Notes

- Coverage counts `done` rows whose evidence cell is non-empty (`—`/`-` do not count); a row
  flipped to `done` without evidence cannot raise the ratchet.
- Inventory rows: 222 total · 0 done · 0 partial · 0 waived · 0 with evidence · 0 done with evidence.
- A screen golden is a PNG under `crates/midata-app/**/snapshots/**`; the target is the 11
  screens listed in `REBUILD_PLAN.md` §7.1, not every image in the repository.
- The test count is a source scan of test attributes (doc tests excluded). It is a
  deterministic ratchet input, not a nextest replacement.
- A met perf budget flips to `enforce = true` and is never un-enforced (`REBUILD_PLAN.md` §11.8).
- Floors never lower. If a number would decrease, the change is wrong, not the number.
