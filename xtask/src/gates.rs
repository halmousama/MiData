//! Repository-wide naming gate.
//!
//! Two lists live behind `cargo xtask banned-names`: this module's legacy product-name scan and the
//! D30 attribution scan in `attribution.rs` (term assembly in `terms.rs`).
//!
//! The legacy product name is banned from every shipped surface. The constitution, the plan and the
//! legacy behaviour specification necessarily *discuss* the ban (and the spec quotes legacy
//! artefacts), so they are the only allowlisted homes; anywhere else is a violation. The pattern is
//! assembled at runtime so this file cannot match itself.

use crate::{attribution, source};
use anyhow::{bail, Context, Result};
use std::path::Path;

/// Text-like extensions scanned by the gate. Binary files are never branding surfaces.
///
/// Shared with the attribution gate so both lists cover exactly the same surfaces.
pub(crate) const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "md", "toml", "yml", "yaml", "tsv", "json", "txt", "html", "css", "js", "ts", "csv",
];

/// Documents whose job is to state or quote a ban, or to record its allowlist. Everything else must
/// be clean.
///
/// `ARTCRAFT_INVESTIGATION.md` is additive for P0-14 (d): it is a study of external projects and
/// legitimately quotes their attribution lines and vendor names. `docs/banned-names.baseline` is the
/// D30 allowlist, which must name the terms it allows and is therefore never a violation surface.
const ALLOWED_PATHS: &[&str] = &[
    "AGENTS.md",
    "ARTCRAFT_INVESTIGATION.md",
    "REBUILD_PLAN.md",
    "docs/banned-names.baseline",
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

/// The `cargo xtask banned-names` command: both D30 lists, the legacy product name first.
pub fn banned_names(root: &Path) -> Result<()> {
    legacy_product_name(root)?;
    attribution::check(root)
}

fn legacy_product_name(root: &Path) -> Result<()> {
    let term = banned_term();
    let mut hits: Vec<String> = Vec::new();
    let mut scanned = 0_usize;
    let mut files: Vec<source::SourceFile> = Vec::new();

    for extension in TEXT_EXTENSIONS {
        files.extend(source::files_with_extension(root, extension)?);
    }
    // Extensionless text files too: the same surfaces as the attribution scan (P0-14 review R3).
    files.extend(source::extensionless_text_files(root)?);

    for file in files {
        scanned = scanned.saturating_add(1);
        if is_allowed_path(&file.relative) {
            continue;
        }
        let content = std::fs::read_to_string(&file.absolute)
            .with_context(|| format!("reading {}", file.relative))?;
        for (index, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(&term) {
                hits.push(format!("{}:{}", file.relative, index.saturating_add(1)));
            }
        }
    }

    let violations: Vec<&String> = hits.iter().collect();

    if violations.is_empty() {
        println!(
            "banned-name gate: OK — 0 occurrence(s) outside the documented allowlist ({} text file(s) scanned)",
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
        assert!(is_allowed_path("ARTCRAFT_INVESTIGATION.md"));
        assert!(is_allowed_path("docs/banned-names.baseline"));
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
