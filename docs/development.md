# Development

> **Status:** P0-1 landed — the workspace, `xtask` gates and the ubuntu+windows CI matrix exist.
> Only `xtask` is a crate; product crates arrive with their birth tasks (D26). The everyday commands
> below become runnable as each crate lands.
> **Source:** `REBUILD_PLAN.md` §11.4 (gate), §12 (testing), §13 (CI/release), §4.5 (storage),
> §7.5 (snapshots); AGENTS.md §5.

## 1. Prerequisites

- **Rust stable** — pinned by `rust-toolchain.toml` (`channel = "stable"`, components `rustfmt` +
  `clippy`). No nightly features; `unsafe_code = "forbid"` is a workspace lint (D14).
- **Linux is the development OS; Windows is the release priority** (D12). A task is done only when CI is
  green on **both**. Paths use `std::path` + `directories`; no unix-only assumptions, no locale-dependent
  number formats in persisted data.
- CI-only tools (not needed to code, needed to close a task): `cargo-nextest`, `cargo-deny`,
  `cargo-machete`.

## 2. Everyday commands

```sh
cargo build                                   # debug build of the whole workspace
cargo run -p midata                           # desktop app (eframe/wgpu window)
cargo run -p midata-cli -- commands           # list every registered command + params docs
cargo run -p midata-cli -- run settings.get --params '{}'
cargo run -p midata-cli -- seed --demo        # deterministic demo data (~50 animals)
cargo nextest run --workspace                 # full test suite
cargo fmt                                     # format
cargo clippy --workspace --all-targets -- -D warnings
```

### Faster test loops (while iterating)

```sh
cargo nextest run -p midata-store --lib                     # one crate, lib tests only
cargo nextest run -p midata-engine --test command_hunt -- --ignored
cargo nextest run -p midata-store --test migrations         # one integration file
cargo test -p midata-app -- snapshot                        # UI goldens (offscreen)
cargo xtask bench --quick                                   # hot-path perf smoke
```

Keep the final gate (below) for the end of the task — it is the thing CI runs.

## 3. The pre-completion gate (exact commands)

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run -p <crates you touched>        # full: cargo nextest run --workspace
cargo xtask check-layers
cargo xtask file-budget
cargo xtask i18n-coverage                        # must not regress
cargo test -p midata-app -- snapshot             # UI tasks: goldens updated AND viewed
cargo test -p midata-engine --test command_hunt -- --ignored   # if you added/changed commands
cargo xtask bench --quick                        # if you touched a hot path (flatten, dashboard, import)
cargo xtask stats                                # regenerate scorecard; commit it (CI diffs)
```

`cargo xtask ci` runs the whole block. **Never commit red**: every commit keeps the workspace building
and the touched crates' gates green (D25) — mid-task commits are checkpoints, and a crashed session is
recovered from the branch plus `log/devlog.md`.

## 4. Where data lives while developing

| What | Path |
|---|---|
| Database | platform data dir: Linux `~/.local/share/midata/midata.db`, Windows `%APPDATA%\midata\midata.db` |
| Portable mode | `MIDATA_PORTABLE=1` → `midata_data/` next to the executable (gitignored) |
| Exports / backups | `…/midata/exports/`, `…/midata/backups/` (newest 10 kept) |
| Logs | `…/midata/logs/` (`tracing`, rotated) |

Tests never touch the user database: store tests run on tempfile DBs, and the demo seed is
deterministic. Never point a test at a real herd file, and never commit `*.db`, `midata_data/` or logs
(`.gitignore` enforces it).

## 5. Driving the app without a mouse (agent workflow)

The command registry is the seam, so anything the UI can do is scriptable:

```sh
cargo run -p midata-cli -- commands                       # discover ids + params docs
cargo run -p midata-cli -- run animal.list --params '{}'  # read
cargo run -p midata-cli -- run animal.create --params '{...}'
```

- CLI runs record actor `cli` in `audit_log`; UI runs record `user`; seed records `seed`; assistant
  writes record `ai` with a `batch_token`.
- Output is human-readable by default and `--json` for machine consumption.
- Phase 6 adds `engine.execute` / `app.inspect` / `ui.screenshot` over a loopback control channel; Phase 6
  also exposes the same surface through `midata-cli mcp --bridge <port>`.

## 6. Snapshots ("look at the PNG")

- `cargo test -p midata-app -- snapshot` renders screens offscreen through `egui_kittest` — no window,
  no focus stealing — and compares against committed goldens.
- Regenerate intentionally: update the golden, **open the PNG**, then note the result in the task
  record / devlog. Both themes when tokens or layout change; a French spot check when strings change.
- Goldens travel in the same commit as the change that caused them (never a separate "update goldens"
  commit that drifts from its code).

## 7. Debugging notes

- **DB problems:** the app must show an i18n error screen, not crash. Reproduce with a deliberately
  corrupt/empty file in a temp dir rather than the real data dir.
- **Panics:** a panic escaping a command is a bug, not a fallback path. The shell's last-resort catch
  exists so the herd survives — it reports an `internal` error and keeps the process alive.
- **Windows-only surprises:** separator handling, case-insensitive paths, file locks, line endings,
  and `%APPDATA%` resolution. CI catches them; do not "fix" them with unix assumptions.
- **Slow queries:** use `xtask bench --quick` before and after; the flatten path is the hot spot and has a
  hard budget (10k animals < 500 ms).

## 8. Release (Phase 7 — recorded here so the path is known)

Semver from `0.1.0`; **every release is tagged** `vX.Y.Z`, plus internal checkpoint tags `m/phase-N`.
Windows installer first (built on a windows runner — no local Windows machine needed), then Linux
AppImage + `.deb`, macOS optional/deferred. Signing is a human-reserved task. `CHANGELOG.md` is kept per
release; `docs/roadmap.md` holds the exit criteria. Publishing, tagging and pushing to `main` are
escalation items (AGENTS.md §7).
