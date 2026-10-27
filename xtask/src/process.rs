//! Small, non-interactive process helpers. No shell, no pager, no prompts.

use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;

/// Run a command, streaming its output, and fail if it exits non-zero.
pub fn run(program: &str, args: &[&str], cwd: &Path) -> Result<()> {
    eprintln!("xtask: + {program} {}", args.join(" "));
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .with_context(|| format!("spawning `{program}` (is it installed and on PATH?)"))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "`{program} {}` failed with {status}",
            args.join(" ")
        ))
    }
}

/// Run a command and capture stdout. Used for `cargo metadata` and `git diff`.
pub fn capture(program: &str, args: &[&str], cwd: &Path) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .with_context(|| format!("spawning `{program}` (is it installed and on PATH?)"))?;

    if output.status.success() {
        String::from_utf8(output.stdout).context("command output was not UTF-8")
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!(
            "`{program} {}` failed with {}: {}",
            args.join(" "),
            output.status,
            stderr.trim()
        ))
    }
}

/// True when a tool answers `--version`. Used to skip external gates loudly, never silently.
pub fn is_available(program: &str, args: &[&str], cwd: &Path) -> bool {
    Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn capture_returns_stdout_for_a_real_command() {
        let cwd = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // `cargo --version` is guaranteed wherever this test runs; no shell, no prompts.
        let version = capture("cargo", &["--version"], &cwd).expect("cargo is available");
        assert!(version.starts_with("cargo "));
    }

    #[test]
    fn run_reports_failure_for_a_nonzero_exit() {
        let cwd = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let result = run("cargo", &["definitely-not-a-cargo-command"], &cwd);
        assert!(result.is_err(), "a failing command must surface as Err");
    }

    #[test]
    fn missing_tool_is_reported_not_panicked() {
        let cwd = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let result = run("midata-tool-that-does-not-exist", &["--version"], &cwd);
        assert!(
            result.is_err(),
            "a missing tool must be an error, never a panic"
        );
        assert!(!is_available(
            "midata-tool-that-does-not-exist",
            &["--version"],
            &cwd
        ));
    }
}
