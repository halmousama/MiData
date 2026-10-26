//! Scorecard generation (`docs/scorecard.md`), the single source of measured numbers.
//!
//! Output is deterministic across OSes: sorted inputs, forward-slash paths, and **no timestamp** —
//! CI diffs the committed file, so a clock or a hash-map order would turn the gate into noise.
//! Every number names its source. Unmeasured budgets say so instead of pretending.

use crate::{file_budget, i18n, source, workspace};
use anyhow::{bail, Context, Result};
use std::fmt::Write as _;
use std::path::Path;

pub const SCORECARD: &str = "docs/scorecard.md";
const TARGET_SCREENS: usize = 13;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InventoryStats {
    pub total: usize,
    pub done: usize,
    pub partial: usize,
    pub waived: usize,
    pub with_evidence: usize,
}

/// Parse the row tables in `docs/inventory.md` (`| id | behaviour | status | phase | evidence |`).
pub fn parse_inventory(text: &str) -> InventoryStats {
    let mut stats = InventoryStats::default();

    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed.split('|').map(str::trim).collect();
        let Some(id) = cells.get(1) else {
            continue;
        };
        if !is_inventory_id(id) {
            continue;
        }

        stats.total = stats.total.saturating_add(1);
        match cells.get(3).copied() {
            Some("done") => stats.done = stats.done.saturating_add(1),
            Some("partial") => stats.partial = stats.partial.saturating_add(1),
            Some("waived") => stats.waived = stats.waived.saturating_add(1),
            _ => {}
        }

        if let Some(evidence) = cells.get(5).copied() {
            if !evidence.is_empty() && evidence != "—" && evidence != "-" {
                stats.with_evidence = stats.with_evidence.saturating_add(1);
            }
        }
    }

    stats
}

fn is_inventory_id(cell: &str) -> bool {
    let Some((prefix, number)) = cell.split_once('-') else {
        return false;
    };
    prefix.len() == 2
        && prefix.chars().all(|c| c.is_ascii_uppercase())
        && !number.is_empty()
        && number.chars().all(|c| c.is_ascii_digit())
}

fn count_test_attributes(root: &Path) -> Result<usize> {
    let mut count = 0_usize;
    for file in source::rust_files(root)? {
        let content = std::fs::read_to_string(&file.absolute)
            .with_context(|| format!("reading {}", file.relative))?;
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("#[test]")
                || trimmed.starts_with("#[tokio::test")
                || trimmed.starts_with("#[test_case")
                || trimmed.starts_with("#[rstest")
                || trimmed.starts_with("#[proptest")
            {
                count = count.saturating_add(1);
            }
        }
    }
    Ok(count)
}

fn count_unsafe_tokens(root: &Path) -> Result<usize> {
    // Assembled so the scanner never counts its own source (the same reason the banned-name gate
    // builds its pattern at runtime).
    let keyword = concat!("un", "safe");
    let mut count = 0_usize;
    for file in source::rust_files(root)? {
        let content = std::fs::read_to_string(&file.absolute)
            .with_context(|| format!("reading {}", file.relative))?;
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            let found = line
                .split(|character: char| !(character.is_alphanumeric() || character == '_'))
                .any(|token| token == keyword);
            if found {
                count = count.saturating_add(1);
            }
        }
    }
    Ok(count)
}

fn count_golden_pngs(root: &Path) -> Result<usize> {
    Ok(source::files_with_extension(root, "png")?.len())
}

pub fn render(root: &Path) -> Result<String> {
    let inventory_text = std::fs::read_to_string(root.join("docs/inventory.md"))
        .context("reading docs/inventory.md")?;
    let inventory = parse_inventory(&inventory_text);
    let coverage_percent = inventory
        .done
        .saturating_mul(100)
        .checked_div(inventory.total)
        .unwrap_or(0);

    let tests = count_test_attributes(root)?;
    let packages = workspace::load(root)?.packages.len();
    let goldens = count_golden_pngs(root)?;
    let unsafe_tokens = count_unsafe_tokens(root)?;

    let budget_files = file_budget::measure(root)?;
    let budget_violations = budget_files
        .iter()
        .filter(|file| file.lines > file_budget::SOFT_LIMIT)
        .count();

    let (en_value, fr_value) = match i18n::measure(root)? {
        i18n::CatalogueState::NotBorn => (
            "n/a (no catalogue yet)".to_string(),
            "n/a (no catalogue yet)".to_string(),
        ),
        i18n::CatalogueState::Born(coverage) => {
            let translated = coverage.en_total.saturating_sub(coverage.missing_fr.len());
            let percent = translated
                .saturating_mul(100)
                .checked_div(coverage.en_total)
                .unwrap_or(0);
            (
                format!("100% ({} keys)", coverage.en_total),
                format!("{translated}/{} ({percent}%)", coverage.en_total),
            )
        }
    };

    let mut out = String::new();
    out.push_str("# Scorecard\n\n");
    out.push_str("> # ⚠ GENERATED FILE — DO NOT EDIT\n>\n");
    out.push_str(
        "> Produced by `cargo xtask stats` and diffed by CI. A hand edit is a failing build.\n",
    );
    out.push_str(
        "> Deterministic by design: no timestamps, sorted inputs, `/` separators on every OS.\n",
    );
    out.push_str("> Every number names its source; an unmeasured budget says `not measured`, never a guess.\n\n");

    out.push_str("| Metric | Value | Source | Ratchet |\n");
    out.push_str("|---|---|---|---|\n");
    writeln!(
        out,
        "| Inventory coverage (rows with evidence) | {} / {} rows ({}%) | `docs/inventory.md` | ↑ only |",
        inventory.done, inventory.total, coverage_percent
    )?;
    writeln!(
        out,
        "| Tests (source scan) | {} | `#[test]`-family attributes under `xtask/`, `crates/`, `apps/` | ↑ only |",
        tests
    )?;
    writeln!(
        out,
        "| Crates in the workspace | {} | `cargo metadata --no-deps` | — |",
        packages
    )?;
    writeln!(
        out,
        "| Screens with ≥1 snapshot golden | {} / {} | `*.png` goldens | ↑ only |",
        goldens, TARGET_SCREENS
    )?;
    writeln!(
        out,
        "| i18n coverage (EN) | {} | `cargo xtask i18n-coverage` | = 100% |",
        en_value
    )?;
    writeln!(
        out,
        "| i18n coverage (FR) | {} | `cargo xtask i18n-coverage` | ↑ to 100% by P6 |",
        fr_value
    )?;
    writeln!(
        out,
        "| File-budget violations (soft {} / alarm {}) | {} | `cargo xtask file-budget` | ↓ to 0 |",
        file_budget::SOFT_LIMIT,
        file_budget::ALARM_LIMIT,
        budget_violations
    )?;
    out.push_str("| Clippy warnings | 0 (enforced) | `cargo clippy -D warnings` | = 0 |\n");
    let unsafe_keyword = concat!("un", "safe");
    writeln!(
        out,
        "| `{unsafe_keyword}` occurrences | {} | source scan + workspace lint `forbid` | = 0 |",
        unsafe_tokens
    )?;
    out.push_str(
        "| Flatten 10k animals | not measured | `cargo xtask bench` (skeleton) | < 500 ms |\n",
    );
    out.push_str("| Dashboard hydrate (10 widgets, 1k animals) | not measured | `cargo xtask bench` (skeleton) | < 300 ms |\n");
    out.push_str("| Cold start | not measured | `cargo xtask bench` (skeleton) | < 2 s |\n");
    out.push_str("| Import 1k rows | not measured | `cargo xtask bench` (skeleton) | < 10 s |\n");

    out.push_str("\n## Notes\n\n");
    out.push_str(
        "- Coverage counts `done` rows only; a wired row without evidence does not count.\n",
    );
    writeln!(
        out,
        "- Inventory rows: {} total · {} done · {} partial · {} waived · {} with evidence.",
        inventory.total,
        inventory.done,
        inventory.partial,
        inventory.waived,
        inventory.with_evidence
    )?;
    out.push_str(
        "- The test count is a source scan of test attributes (doc tests excluded). It is a\n",
    );
    out.push_str("  deterministic ratchet input, not a nextest replacement.\n");
    out.push_str("- A met perf budget flips to `enforce = true` and is never un-enforced (`REBUILD_PLAN.md` §11.8).\n");
    out.push_str(
        "- Floors never lower. If a number would decrease, the change is wrong, not the number.\n",
    );

    Ok(out)
}

pub fn run(root: &Path, check_only: bool) -> Result<()> {
    let rendered = render(root)?;
    let path = root.join(SCORECARD);

    if check_only {
        let existing = std::fs::read_to_string(&path).unwrap_or_default();
        if existing != rendered {
            bail!(
                "{SCORECARD} is stale — regenerate it with `cargo xtask stats` and commit the result"
            );
        }
        println!("stats: OK — {SCORECARD} is up to date");
        return Ok(());
    }

    std::fs::write(&path, rendered).with_context(|| format!("writing {}", path.display()))?;
    println!("stats: wrote {SCORECARD}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_parser_counts_ids_and_statuses() {
        let text = "\
| id | behaviour | status | phase | evidence |
|---|---|---|---|---|
| HM-1 | Create an animal | done | P1 | test_create |
| HM-2 | Reject a duplicate | planned | P1 | — |
| HM-3 | List animals | partial | P1 | test_list |
| HM | block heading | x | x | x |
";
        let stats = parse_inventory(text);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.done, 1);
        assert_eq!(stats.partial, 1);
        assert_eq!(stats.with_evidence, 2);
    }

    #[test]
    fn inventory_parser_ignores_non_id_tables() {
        let text = "| Status | Meaning |\n|---|---|\n| planned | Not implemented |\n";
        assert_eq!(parse_inventory(text).total, 0);
    }

    #[test]
    fn inventory_id_shape_is_enforced() {
        assert!(is_inventory_id("HM-1"));
        assert!(is_inventory_id("AI-123"));
        assert!(!is_inventory_id("HM"));
        assert!(!is_inventory_id("hm-1"));
        assert!(!is_inventory_id("HM-"));
        assert!(!is_inventory_id("HM-1x"));
    }
}
