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

    section("banned-name grep");
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

fn is_missing_binary(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        let text = cause.to_string();
        text.contains("No such file or directory")
            || text.contains("os error 2")
            || text.contains("The system cannot find the file")
    })
}
