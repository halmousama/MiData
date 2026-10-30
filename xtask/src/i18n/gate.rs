//! The i18n gate's decision logic, kept pure so every rule is unit-tested.
//!
//! The gate has four teeth, in this order:
//! 1. **orphan FR keys are fatal** — a row present in `fr.tsv` but absent from `en.tsv` is dead code
//!    (`docs/i18n.md` §5; round-1 finding S2). It was a printed report line until P0-2; the moment the
//!    catalogues exist, it fails the build.
//! 2. **unused EN keys are fatal** — a source string that no screen, command or domain error references
//!    is a speculative key, and speculative keys are dead keys by another name. "Seed a string when a
//!    screen or command needs it", not before.
//! 3. **the EN total may not shrink** — deleting source rows lowers the coverage denominator (R4).
//! 4. **missing FR keys may not grow** — the coverage ratchet.
//!
//! A key counts as used when it appears as a plain quoted literal (`"cmd.animal.create.title"`) in a
//! Rust source file under `crates/` or `apps/`. Composition by `format!` is therefore not supported by
//! design: a key is a literal at its call site, which is also what makes it greppable for agents.

use super::{Baseline, BaselineState, CatalogueState};
use anyhow::Result;
use std::collections::BTreeSet;

/// What the gate decided: the human report, plus every problem found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// The report printed either way (numbers first, so a failure is readable).
    pub report: String,
    /// One entry per violated rule; empty means the gate passes.
    pub problems: Vec<String>,
}

impl Verdict {
    /// True when no rule was violated.
    pub fn is_ok(&self) -> bool {
        self.problems.is_empty()
    }
}

/// Evaluate the gate for one measured state.
pub fn evaluate(baseline: &Baseline, state: &CatalogueState, used: &BTreeSet<String>) -> Verdict {
    match state {
        CatalogueState::NotBorn => evaluate_not_born(baseline),
        CatalogueState::Born(coverage) => {
            let mut problems = Vec::new();

            if baseline.state != BaselineState::Catalogues {
                problems.push(format!(
                    "catalogues now exist but the baseline still says `no-catalogues` — regenerate it with `cargo xtask i18n-coverage --write-baseline` so the gate becomes a real measurement (en_total {})",
                    coverage.en_total
                ));
            }

            if coverage.en_total == 0 {
                problems.push(
                    "en.tsv is empty — an empty source catalogue cannot pass a coverage gate"
                        .to_string(),
                );
            }

            if !coverage.orphan_fr.is_empty() {
                problems.push(format!(
                    "orphan FR key(s) (present in FR, absent in EN — delete them, do not park them): {}",
                    coverage.orphan_fr.join(", ")
                ));
            }

            let unused: Vec<&String> = coverage
                .en_keys
                .iter()
                .filter(|key| !used.contains(*key))
                .collect();
            if !unused.is_empty() {
                let listed: Vec<&str> = unused.iter().map(|key| key.as_str()).collect();
                problems.push(format!(
                    "EN key(s) that no product source references (a speculative key is a dead key — seed a string only when a screen, command or error needs it): {}",
                    listed.join(", ")
                ));
            }

            if let Some(problem) = en_total_problem(coverage.en_total, baseline.en_total) {
                problems.push(problem);
            }
            if let Some(problem) = ratchet_problem(coverage.missing_fr.len(), baseline.fr_missing) {
                problems.push(problem);
            }

            Verdict {
                report: born_report(coverage, baseline, &unused, problems.is_empty()),
                problems,
            }
        }
    }
}

fn evaluate_not_born(baseline: &Baseline) -> Verdict {
    let mut problems = Vec::new();
    if baseline.state != BaselineState::NoCatalogues {
        problems.push(
            "catalogues are absent but the baseline expects `catalogues` — the baseline is stale or the catalogue was deleted accidentally"
                .to_string(),
        );
    }
    Verdict {
        report: "i18n-coverage: NOT RUN — no catalogues exist yet.\n  expected: crates/midata-i18n/catalogues/en.tsv (source) + fr.tsv, born with midata-i18n in P0-2 (D24)\n  docs/i18n-coverage.baseline records this state explicitly, so this is a loud, tracked skip — not a silent pass.\n  it becomes a real measurement the moment the catalogues appear (a stale `no-catalogues` baseline fails).\n".to_string(),
        problems,
    }
}

fn born_report(
    coverage: &super::Coverage,
    baseline: &Baseline,
    unused: &[&String],
    passed: bool,
) -> String {
    // Denominator for the percentage is the EN total, never the FR row count: an orphan FR row would
    // otherwise print a fraction and a percentage that disagree.
    let mut report = format!(
        "i18n-coverage: EN {} key(s); FR {}/{} translated ({}%); FR rows {}; missing FR {}; orphan FR {}; unused EN {} (baseline: en_total {}, fr_missing {})\n",
        coverage.en_total,
        coverage.translated_fr(),
        coverage.en_total,
        coverage.fr_percent(),
        coverage.fr_total,
        coverage.missing_fr.len(),
        coverage.orphan_fr.len(),
        unused.len(),
        baseline.en_total,
        baseline.fr_missing
    );
    if passed {
        report.push_str(&format!(
            "i18n-coverage: OK — EN {} key(s) within the baseline of {}; {} missing FR key(s) within the baseline of {}; every key is referenced by product code\n",
            coverage.en_total, baseline.en_total, coverage.missing_fr.len(), baseline.fr_missing
        ));
    }
    report
}

/// The ratchet rule in one place: missing FR keys may not increase.
fn ratchet_problem(current_missing: usize, baseline_missing: usize) -> Option<String> {
    if current_missing > baseline_missing {
        Some(format!(
            "FR coverage regressed: {current_missing} missing key(s), baseline allows {baseline_missing}"
        ))
    } else {
        None
    }
}

/// The second ratchet: the EN source catalogue may not shrink.
///
/// Deleting EN keys lowers the denominator, so coverage can rise without a string being translated. A
/// genuinely dead key is deleted together with its FR row **and** a regenerated baseline.
fn en_total_problem(current_total: usize, baseline_total: usize) -> Option<String> {
    if current_total < baseline_total {
        Some(format!(
            "EN catalogue shrank: {current_total} key(s), baseline records {baseline_total} — deleting EN keys shrinks the coverage denominator; remove the FR row and regenerate the baseline in the same change if the key is genuinely dead"
        ))
    } else {
        None
    }
}

/// Which catalogue keys are actually referenced by product source.
///
/// Only `crates/` and `apps/` are scanned: `xtask` is tooling, and a key mentioned there is not a
/// consumer. Reading is bounded by the repository, and every file is read once.
pub fn scan_usage(root: &std::path::Path, keys: &BTreeSet<String>) -> Result<BTreeSet<String>> {
    let mut corpora: Vec<String> = Vec::new();
    for file in crate::source::rust_files(root)? {
        if !is_product_source(&file.relative) {
            continue;
        }
        let text = std::fs::read_to_string(&file.absolute)
            .map_err(|error| anyhow::anyhow!("reading {}: {error}", file.relative))?;
        corpora.push(text);
    }

    let mut used = BTreeSet::new();
    for key in keys {
        let literal = format!("\"{key}\"");
        if corpora.iter().any(|text| text.contains(&literal)) {
            used.insert(key.clone());
        }
    }
    Ok(used)
}

/// Product code means library/binary source under `crates/` or `apps/`, tests included.
fn is_product_source(relative: &str) -> bool {
    relative.starts_with("crates/") || relative.starts_with("apps/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{CatalogueState, Coverage};

    fn baseline(state: BaselineState, en_total: usize, fr_missing: usize) -> Baseline {
        Baseline {
            state,
            en_total,
            fr_missing,
        }
    }

    fn keys(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|name| (*name).to_string()).collect()
    }

    fn born(en: &[&str], fr: &[&str]) -> CatalogueState {
        let en_keys = keys(en);
        let fr_keys = keys(fr);
        CatalogueState::Born(Coverage {
            en_total: en_keys.len(),
            fr_total: fr_keys.len(),
            missing_fr: en_keys.difference(&fr_keys).cloned().collect(),
            orphan_fr: fr_keys.difference(&en_keys).cloned().collect(),
            en_keys,
        })
    }

    #[test]
    fn a_complete_pair_used_by_product_code_passes() {
        let state = born(&["err.a", "err.b"], &["err.a", "err.b"]);
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 2, 0),
            &state,
            &keys(&["err.a", "err.b"]),
        );
        assert!(verdict.is_ok(), "problems: {:?}", verdict.problems);
        assert!(verdict.report.contains("FR 2/2 translated (100%)"));
    }

    /// S2 (round-1 finding): an FR row with no EN row used to be a printed line. It is now fatal.
    #[test]
    fn an_orphan_fr_key_fails_the_gate() {
        let state = born(&["err.a"], &["err.a", "err.orphan"]);
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 1, 0),
            &state,
            &keys(&["err.a"]),
        );
        assert!(!verdict.is_ok());
        assert!(
            verdict
                .problems
                .iter()
                .any(|problem| problem.contains("orphan FR key(s)")
                    && problem.contains("err.orphan")),
            "problems: {:?}",
            verdict.problems
        );
    }

    #[test]
    fn an_orphan_fails_even_when_fr_coverage_is_complete() {
        let state = born(&["err.a", "err.b"], &["err.a", "err.b", "err.c"]);
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 2, 0),
            &state,
            &keys(&["err.a", "err.b"]),
        );
        assert!(!verdict.is_ok(), "completeness never excuses an orphan row");
    }

    #[test]
    fn an_en_key_no_source_uses_fails_the_gate() {
        let state = born(&["err.a", "err.b"], &["err.a", "err.b"]);
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 2, 0),
            &state,
            &keys(&["err.a"]),
        );
        assert!(!verdict.is_ok());
        assert!(
            verdict
                .problems
                .iter()
                .any(|problem| problem.contains("no product source references")
                    && problem.contains("err.b")),
            "problems: {:?}",
            verdict.problems
        );
    }

    #[test]
    fn a_growing_gap_in_fr_coverage_fails_the_gate() {
        let state = born(&["err.a", "err.b"], &["err.a"]);
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 2, 0),
            &state,
            &keys(&["err.a", "err.b"]),
        );
        assert!(!verdict.is_ok());
        assert!(verdict
            .problems
            .iter()
            .any(|problem| problem.contains("FR coverage regressed")));
    }

    #[test]
    fn a_shrinking_en_catalogue_fails_the_gate() {
        let state = born(&["err.a"], &["err.a"]);
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 2, 0),
            &state,
            &keys(&["err.a"]),
        );
        assert!(!verdict.is_ok());
        assert!(verdict
            .problems
            .iter()
            .any(|problem| problem.contains("EN catalogue shrank")));
    }

    #[test]
    fn an_empty_source_catalogue_cannot_pass() {
        let verdict = evaluate(
            &baseline(BaselineState::Catalogues, 1, 0),
            &CatalogueState::Born(Coverage {
                en_total: 0,
                fr_total: 0,
                missing_fr: Vec::new(),
                orphan_fr: Vec::new(),
                en_keys: BTreeSet::new(),
            }),
            &BTreeSet::new(),
        );
        assert!(!verdict.is_ok());
        assert!(verdict
            .problems
            .iter()
            .any(|problem| problem.contains("en.tsv is empty")));
    }

    #[test]
    fn the_not_born_state_still_skips_loudly_with_its_baseline() {
        let verdict = evaluate(
            &baseline(BaselineState::NoCatalogues, 0, 0),
            &CatalogueState::NotBorn,
            &BTreeSet::new(),
        );
        assert!(verdict.is_ok(), "the tracked skip is not a failure");
        assert!(verdict.report.contains("NOT RUN"));
        assert!(verdict.report.contains("not a silent pass"));
    }

    #[test]
    fn a_stale_baseline_fails_in_both_directions() {
        let stale_skip = evaluate(
            &baseline(BaselineState::NoCatalogues, 0, 0),
            &born(&["err.a"], &["err.a"]),
            &keys(&["err.a"]),
        );
        assert!(!stale_skip.is_ok(), "catalogues exist: stop skipping");

        let stale_measurement = evaluate(
            &baseline(BaselineState::Catalogues, 1, 0),
            &CatalogueState::NotBorn,
            &BTreeSet::new(),
        );
        assert!(
            !stale_measurement.is_ok(),
            "a missing catalogue behind a measurement baseline is a regression"
        );
    }

    #[test]
    fn only_product_sources_count_as_consumers() {
        assert!(is_product_source("crates/midata-domain/src/value.rs"));
        assert!(is_product_source("apps/midata-cli/src/main.rs"));
        assert!(!is_product_source("xtask/src/i18n.rs"));
        assert!(!is_product_source("docs/i18n.md"));
    }
}
