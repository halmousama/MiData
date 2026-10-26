//! Repository-wide naming gate.
//!
//! The legacy product name is banned from every shipped surface. The constitution, the plan and the
//! legacy behaviour specification necessarily *discuss* the ban (and the spec quotes legacy
//! artefacts), so they are the only allowlisted homes; anywhere else is a violation. The pattern is
//! assembled at runtime so this file cannot match itself.

use crate::source;
use anyhow::{bail, Context, Result};
use std::path::Path;

/// Text-like extensions scanned by the gate. Binary files are never branding surfaces.
const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "md", "toml", "yml", "yaml", "tsv", "json", "txt", "html", "css", "js", "ts", "csv",
];

/// Documents whose job is to state or quote the ban. Everything else must be clean.
const ALLOWED_PATHS: &[&str] = &[
    "AGENTS.md",
    "REBUILD_PLAN.md",
    "docs/decisions.md",
    "docs/gaps.md",
];

pub fn is_allowed_path(relative: &str) -> bool {
    ALLOWED_PATHS.contains(&relative) || relative.starts_with("docs/legacy-spec/")
}

/// The banned token, split so the gate source never contains its own pattern.
fn banned_term() -> String {
    concat!("ru", "metrics").to_string()
}

pub fn banned_names(root: &Path) -> Result<()> {
    let term = banned_term();
    let mut hits: Vec<String> = Vec::new();
    let mut scanned = 0_usize;

    for extension in TEXT_EXTENSIONS {
        for file in source::files_with_extension(root, extension)? {
            scanned = scanned.saturating_add(1);
            let content = std::fs::read_to_string(&file.absolute)
                .with_context(|| format!("reading {}", file.relative))?;
            for (index, line) in content.lines().enumerate() {
                if line.to_lowercase().contains(&term) {
                    hits.push(format!("{}:{}", file.relative, index.saturating_add(1)));
                }
            }
        }
    }

    let violations: Vec<&String> = hits
        .iter()
        .filter(|hit| {
            let relative = hit.split(':').next().unwrap_or_default();
            !is_allowed_path(relative)
        })
        .collect();

    if violations.is_empty() {
        println!(
            "banned-name gate: OK — 0 occurrence(s) outside the documented allowlist ({} text file(s) scanned; allowlist: AGENTS.md, REBUILD_PLAN.md, docs/decisions.md, docs/gaps.md, docs/legacy-spec/)",
            scanned
        );
        return Ok(());
    }

    eprintln!(
        "banned-name gate: {} occurrence(s) outside the allowlist:",
        violations.len()
    );
    for hit in &violations {
        eprintln!("  - {hit}");
    }
    bail!("legacy product name found in a shipped surface")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_policy_and_legacy_spec_documents_are_allowlisted() {
        assert!(is_allowed_path("AGENTS.md"));
        assert!(is_allowed_path("docs/legacy-spec/PROJECT_REPORT.md"));
        assert!(!is_allowed_path("xtask/src/main.rs"));
        assert!(!is_allowed_path("docs/scorecard.md"));
        assert!(!is_allowed_path("ROADMAP.md"));
    }

    #[test]
    fn gate_source_does_not_contain_its_own_pattern() {
        let source = include_str!("gates.rs").to_lowercase();
        assert!(
            !source.contains(&banned_term()),
            "the gate must not flag itself; assemble the pattern instead of writing it literally"
        );
    }
}
