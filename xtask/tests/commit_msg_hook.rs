//! Acceptance tests for the checked-in `commit-msg` hook (`tools/hooks/commit-msg`, P0-14).
//!
//! The hook is POSIX `sh` and Git runs it through its own `sh` on every OS, but this process-level
//! test needs a `sh` on PATH, so it is gated to unix. The gate logic it invokes is tested
//! cross-platform as a unit test in `xtask/src/terms.rs` and `xtask/src/attribution.rs`.
//!
//! The forbidden terms are assembled from fragments here for the same reason the gate assembles
//! them: this source file is inside the tree the gate scans (D30 self-reference trap).
//!
//! Helpers return `Result` on purpose: the workspace denies `expect`/`unwrap` in non-test code and
//! clippy's "in tests" exemption does not cover free helper functions, so a helper that failed would
//! otherwise have to panic.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn hook_path() -> PathBuf {
    repo_root().join("tools").join("hooks").join("commit-msg")
}

/// A unique temp file outside the repository — the hook reads the message path it is given, exactly
/// as Git passes `.git/COMMIT_EDITMSG`.
fn message_file(name: &str, content: &str) -> std::io::Result<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "midata-commit-msg-{}-{name}.txt",
        std::process::id()
    ));
    std::fs::write(&path, content)?;
    Ok(path)
}

fn run_hook(message: Option<&Path>) -> std::io::Result<Output> {
    let mut command = Command::new("sh");
    command.arg(hook_path());
    if let Some(message) = message {
        command.arg(message);
    }
    command
        .current_dir(repo_root())
        .env("XTASK_BIN", env!("CARGO_BIN_EXE_xtask"))
        .output()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn hook_rejects_an_attribution_trailer() -> TestResult {
    // Assembled: the literal must not appear in a scanned file.
    let trailer = concat!("Co-autho", "red-by");
    let message = message_file(
        "trailer",
        &format!("feat(xtask): sample [P0-14]\n\n{trailer}: a helper <helper@example.invalid>\n"),
    )?;
    let output = run_hook(Some(&message));
    let _ = std::fs::remove_file(&message);
    let output = output?;
    assert!(
        !output.status.success(),
        "the hook must reject an attribution trailer; stderr: {}",
        stderr(&output)
    );
    assert!(
        stderr(&output).contains("D30"),
        "the rejection must name the rule it enforces; stderr: {}",
        stderr(&output)
    );
    Ok(())
}

#[test]
fn hook_rejects_a_vendor_term() -> TestResult {
    // Assembled for the same reason: this file is gate-scanned.
    let vendor = concat!("open", "ai");
    let message = message_file(
        "vendor",
        &format!("docs: note the {vendor} compatible endpoint [P0-14]\n"),
    )?;
    let output = run_hook(Some(&message));
    let _ = std::fs::remove_file(&message);
    let output = output?;
    assert!(
        !output.status.success(),
        "the hook must reject a vendor/model term; stderr: {}",
        stderr(&output)
    );
    Ok(())
}

#[test]
fn hook_accepts_a_clean_message() -> TestResult {
    let message = message_file(
        "clean",
        "feat(xtask): add the attribution gate [P0-14]\n\nWhat & why.\n\nRefs: gaps P0-14\n",
    )?;
    let output = run_hook(Some(&message));
    let _ = std::fs::remove_file(&message);
    let output = output?;
    assert!(
        output.status.success(),
        "a clean message must pass; stderr: {}",
        stderr(&output)
    );
    Ok(())
}

/// P0-14 review R1: `git commit -v` puts the diff below the scissors line, uncommented. The diff of
/// a devlog change mentions the D30 terms, so scanning it refused a clean commit. The hook must stop
/// at the scissors line, exactly as Git does.
#[test]
fn hook_accepts_a_verbose_message_whose_diff_mentions_a_term() -> TestResult {
    let vendor = concat!("open", "ai");
    let message = message_file(
        "verbose",
        &format!(
            "docs(devlog): record the policy line [P0-14]\n\nWhat & why.\n\n\
             # ------------------------ >8 ------------------------\n\
             diff --git a/log/devlog.md b/log/devlog.md\n\
             --- a/log/devlog.md\n\
             +++ b/log/devlog.md\n\
             +the policy line quotes the {vendor} ban\n"
        ),
    )?;
    let output = run_hook(Some(&message));
    let _ = std::fs::remove_file(&message);
    let output = output?;
    assert!(
        output.status.success(),
        "a verbose diff below the scissors line must not fail the hook; stderr: {}",
        stderr(&output)
    );
    Ok(())
}

#[test]
fn hook_fails_closed_without_a_message_file() -> TestResult {
    let output = run_hook(None)?;
    assert!(
        !output.status.success(),
        "a hook that cannot read the message must refuse the commit, never pass silently"
    );
    Ok(())
}

#[test]
fn hook_fails_closed_when_the_check_cannot_run() -> TestResult {
    let message = message_file("unrunnable", "feat(xtask): sample [P0-14]\n")?;
    let output = Command::new("sh")
        .arg(hook_path())
        .arg(&message)
        .current_dir(repo_root())
        .env("XTASK_BIN", repo_root().join("no-such-xtask-binary"))
        .output();
    let _ = std::fs::remove_file(&message);
    let output = output?;
    assert!(
        !output.status.success(),
        "a missing checker must fail closed; stderr: {}",
        stderr(&output)
    );
    Ok(())
}
