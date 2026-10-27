# xtask — MiData workspace gates

`xtask` is the only crate in P0-1. It runs every gate locally and in CI from the same code, so
"green locally" and "green on the PR" cannot drift.

| Command | What it enforces |
|---|---|
| `cargo xtask check-layers` | The registered crate DAG in `src/layers.rs`: unknown crates, illegal internal edges, unregistered external dependencies, dev-only `midata-testkit`, app crates as dependencies, and restricted heavy crates (egui/tokio/sqlparser/file formats). |
| `cargo xtask file-budget` | Rust files over the 500-line soft cap (alarm 800). The violation list is a ratchet: `docs/file-budget.baseline` may only shrink. |
| `cargo xtask i18n-coverage` | EN/FR catalogue coverage. The not-born state is recorded explicitly in `docs/i18n-coverage.baseline`; when catalogues appear, a stale baseline fails and the gate becomes a real measurement. Missing FR keys may not increase. |
| `cargo xtask stats` | Regenerates `docs/scorecard.md` deterministically (no timestamps). `--check` verifies without writing. |
| `cargo xtask ci` | fmt · clippy `-D warnings` · nextest · every gate above · banned-name grep · scorecard-is-committed · cargo-deny · cargo-machete. Missing external tools are reported as **skips**, never passes. |
| `new-command`, `new-screen`, `seed`, `bench` | Registered skeletons: they fail with an actionable "unsupported" error, never a panic. |

## Baselines

Generated, committed, and never hand-edited:

- `docs/file-budget.baseline` — `path<TAB>lines` for files over 500 lines (currently empty).
- `docs/i18n-coverage.baseline` — `state: no-catalogues` until `midata-i18n` is born in P0-2.

Regenerate intentionally and commit the baseline in the same change that motivates it:

```sh
cargo xtask file-budget --write-baseline
cargo xtask i18n-coverage --write-baseline
cargo xtask stats
```

## Negative proof (a gate must be shown to bite)

Each gate has unit tests for its pure logic, and the PR for P0-1 records a manual violation per gate
(temporary, reverted): an illegal layer edge, a 501-line file, a catalogue with a missing FR row, a
hand-edited scorecard, a formatting break that stops `ci`, and the banned name in a source file.
A gate that has never failed is a gate assumed to work.

## Adding a crate

1. Create the crate in the phase that first uses it (D26) and add it to the workspace members.
2. Register it in `src/layers.rs`: `name`, `layer`, `birth`, `manifest`, allowed internal edges,
   allowed external dependencies.
3. Add its `README.md` public-API contract.
4. Run `cargo xtask check-layers` and `cargo xtask stats`.

Dependency changes are owner-approved (AGENTS.md rule 15); the registry is the machine-checked form
of that approval.
