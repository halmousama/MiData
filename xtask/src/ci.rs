//! `cargo xtask ci` — the full pre-completion gate in one command (REBUILD_PLAN.md §11.4).
//!
//! The same code runs locally and in GitHub Actions, so "green locally" and "green on the PR" mean
//! the same thing. External tools (nextest, deny, machete) are required where they can run; a tool
//! that is missing locally is reported as a **skip**, never as a pass, and CI installs all three.

use crate::{file_budget, gates, i18n, layers, process, stats};
use anyhow::{bail, Result};
use std::path::Path;

pub fn run(root: &Path) -> Result<()> {
    section("format");
    process::run("cargo", &["fmt", "--all", "--check"], root)?;

    section("clippy");
    process::run(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        root,
    )?;

    section("nextest");
    if !process::is_available("cargo", &["nextest", "--version"], root) {
        bail!(
            "cargo-nextest is not installed — the gate cannot run and must not pretend to pass.\n\
             Install it with: cargo install cargo-nextest --locked"
        );
    }
    process::run(
        "cargo",
        &["nextest", "run", "--workspace", "--no-fail-fast"],
        root,
    )?;

    section("check-layers");
    layers::run(root)?;

    section("file-budget");
    file_budget::check(root)?;

    section("i18n-coverage");
    i18n::check(root)?;

    section("banned-names");
    gates::banned_names(root)?;

    section("stats");
    stats::run(root, false)?;

    section("scorecard is committed");
    process::run(
        "git",
        &["diff", "--exit-code", "HEAD", "--", stats::SCORECARD],
        root,
    )
    .map_err(|_| {
        anyhow::anyhow!(
            "{SCORECARD} changed — run `cargo xtask stats` and commit the generated file with the change that moved the numbers",
            SCORECARD = stats::SCORECARD
        )
    })?;

    section("external tools");
    // Invoke the installed binaries directly: `cargo <sub>` inherits CARGO_PKG_* from the outer
    // cargo invocation, and cargo-machete mis-reads that environment (it treats its own name as a
    // path). Direct invocation is identical on both OSes and avoids the nested-cargo quirk.
    run_external_tool(
        root,
        "cargo-deny",
        &["check", "licenses", "bans", "sources"],
    )?;
    run_external_tool(root, "cargo-machete", &[])?;

    println!("\nxtask ci: OK — every gate that can run has run and passed");
    Ok(())
}

fn section(name: &str) {
    println!("\n== xtask ci: {name} ==");
}

/// Run a tool that CI installs, or skip loudly when it is absent locally.
///
/// A skip is never silent, and never allowed in CI: there the tool is a required gate, so a missing
/// binary is an error. Any non-zero exit from an installed tool is always an error.
fn run_external_tool(root: &Path, program: &str, args: &[&str]) -> Result<()> {
    match process::capture(program, args, root) {
        Ok(stdout) => {
            print!("{stdout}");
            Ok(())
        }
        Err(error) => {
            if is_missing_binary(&error) {
                if std::env::var_os("CI").is_some() {
                    anyhow::bail!(
                        "`{program}` is required in CI but was not found on PATH — the gate cannot run, so it fails"
                    );
                }
                println!(
                    "xtask ci: SKIP {program} — not installed locally (CI installs it; this run is not proof that it passes)"
                );
                Ok(())
            } else {
                Err(error)
            }
        }
    }
}

/// True only when the binary itself could not be spawned.
///
/// The test is the `io::ErrorKind` of the spawn failure, never the text of the message. A tool that
/// *did* run and then failed can legitimately print "No such file or directory" itself (a missing
/// `deny.toml`, an absent input file); classifying that by string match would downgrade a failing
/// gate to a local skip, which is a gate that does not bite (`REBUILD_PLAN.md` §3.8
/// `test-integrity.no-vacuous-gates`). `process::capture` keeps the spawn `io::Error` as the error
/// source, so it is found in the chain; a tool's stderr is folded into a plain message and is not.
fn is_missing_binary(error: &anyhow::Error) -> bool {
    error
        .chain()
        .filter_map(|cause| cause.downcast_ref::<std::io::Error>())
        .any(|io_error| io_error.kind() == std::io::ErrorKind::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn manifest_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// The loophole this guards: an *installed* tool that fails and mentions a missing file in its
    /// own stderr must fail the gate. Before the kind-based check this was reported as `SKIP` with
    /// exit 0 locally, so `cargo xtask ci` could pass while `cargo-deny` had actually failed.
    #[test]
    fn a_failed_run_is_never_downgraded_to_a_missing_binary() {
        let error = anyhow::anyhow!(
            "`cargo-deny check` failed with exit status: exit status: 1: \
             error: failed to open deny.toml: No such file or directory (os error 2)"
        );
        assert!(
            !is_missing_binary(&error),
            "a tool that ran and failed must fail the gate, never skip it"
        );
    }

    /// The other direction, end to end through `process::capture`: a binary that cannot be spawned
    /// stays a loud local skip candidate, because the spawn `io::Error` is `NotFound`.
    #[test]
    fn a_binary_that_cannot_be_spawned_is_classified_missing() {
        let error = process::capture(
            "midata-tool-that-does-not-exist",
            &["--version"],
            &manifest_dir(),
        )
        .expect_err("spawning a nonexistent binary must be an error");
        assert!(
            is_missing_binary(&error),
            "a genuinely missing binary must still be skippable, with its io::Error cause intact"
        );
    }
}
