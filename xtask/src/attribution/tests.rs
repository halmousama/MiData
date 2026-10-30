//! Unit tests for the D30 attribution gate.
//!
//! Split out of `attribution.rs` for the 500-line file budget; a child module still sees every
//! private helper through `use super::*`.

use super::*;

fn hit(path: &str, term: &str, line: usize) -> Hit {
    Hit {
        path: path.to_string(),
        term: term.to_string(),
        kind: terms::TermKind::Vendor,
        line,
    }
}

fn entry(path: &str, term: &str) -> AllowlistEntry {
    AllowlistEntry {
        path: path.to_string(),
        term: term.to_string(),
    }
}

fn first(needle_kind: terms::TermKind) -> String {
    terms::terms()
        .into_iter()
        .find(|candidate| candidate.kind == needle_kind)
        .map(|candidate| candidate.id.to_string())
        .unwrap_or_default()
}

#[test]
fn a_new_hit_fails_even_when_the_path_is_otherwise_allowlisted() {
    let vendor = first(terms::TermKind::Vendor);
    let problems = evaluate(&[hit("docs/architecture.md", &vendor, 15)], &[]);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("not in docs/banned-names.baseline"));
}

#[test]
fn an_allowlisted_reference_passes() {
    let vendor = first(terms::TermKind::Vendor);
    let problems = evaluate(
        &[hit("docs/architecture.md", &vendor, 15)],
        &[entry("docs/architecture.md", &vendor)],
    );
    assert!(problems.is_empty(), "{problems:?}");
}

/// P0-14 (c): the stored count may not exceed the measured one. A baseline row with no matching
/// hit is a stale entry, so the allowlist can only shrink — exactly the i18n/file-budget ratchet.
#[test]
fn the_stored_allowlist_count_may_not_exceed_the_measured_hits() {
    let vendor = first(terms::TermKind::Vendor);
    let allowlist = vec![
        entry("docs/architecture.md", &vendor),
        entry("docs/elsewhere.md", &vendor),
    ];
    let current = vec![hit("docs/architecture.md", &vendor, 15)];
    let problems = evaluate(&current, &allowlist);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("stale allowlist entry"));
}

#[test]
fn a_stale_entry_fails_so_the_allowlist_shrinks() {
    let vendor = first(terms::TermKind::Vendor);
    let problems = evaluate(&[], &[entry("docs/architecture.md", &vendor)]);
    assert_eq!(problems.len(), 1);
    assert!(problems[0].contains("stale allowlist entry"));
}

#[test]
fn baseline_parser_reads_reference_rows() {
    let parsed = parse_baseline("# header\ndocs/architecture.md\tterm\treference\n\n")
        .expect("valid baseline");
    assert_eq!(parsed, vec![entry("docs/architecture.md", "term")]);
}

#[test]
fn baseline_parser_rejects_malformed_rows() {
    assert!(parse_baseline("only-one-field").is_err());
    assert!(parse_baseline("path\tterm").is_err());
    assert!(parse_baseline("path\tterm\treference\textra").is_err());
}

/// P0-14 (d): an attribution claim is never allowlisted, in either direction.
#[test]
fn baseline_parser_rejects_an_attribution_reason() {
    let error = parse_baseline("docs/x.md\ttrailer\tattribution\n")
        .expect_err("an attribution reason must be rejected");
    assert!(error.to_string().contains("never allowlisted"));
}

#[test]
fn baseline_parser_rejects_an_unknown_reason() {
    assert!(parse_baseline("docs/x.md\tterm\tbecause\n").is_err());
}

#[test]
fn allowlisted_paths_are_never_measured() {
    assert!(gates::is_allowed_path("ARTCRAFT_INVESTIGATION.md"));
    assert!(gates::is_allowed_path(BASELINE));
    assert!(!gates::is_allowed_path("docs/architecture.md"));
}

#[test]
fn scan_text_reports_the_path_and_line() {
    let vendor = first(terms::TermKind::Vendor);
    let text = format!("line one\nline two names {}\n", vendor);
    let hits = scan_text("docs/example.md", &text);
    assert_eq!(hits, vec![hit("docs/example.md", &vendor, 2)]);
}

#[test]
fn a_commit_message_with_a_trailer_is_rejected() {
    let trailer = first(terms::TermKind::Attribution);
    let message = format!("feat(x): y [P0-14]\n\n{}: a helper\n", trailer);
    let error = check_message(&message).expect_err("the trailer must be rejected");
    assert!(error.to_string().contains("D30"));
}

#[test]
fn a_commit_message_with_a_vendor_name_is_rejected() {
    let vendor = first(terms::TermKind::Vendor);
    let message = format!("docs: note the {vendor} dialect [P0-14]\n");
    assert!(check_message(&message).is_err());
}

#[test]
fn a_clean_commit_message_passes() {
    assert!(check_message("feat(xtask): add the gate [P0-14]\n\nRefs: gaps P0-14\n").is_ok());
}

#[test]
fn git_template_comments_are_not_part_of_the_message() {
    let vendor = first(terms::TermKind::Vendor);
    let message =
        format!("feat(xtask): add the gate [P0-14]\n\n# a template line may mention {vendor}\n");
    assert!(check_message(&message).is_ok());
}

/// P0-14 review R1: `git commit -v` writes the diff below the scissors line uncommented, so a
/// clean message whose diff mentions a term was refused. Git discards that region; so do we.
#[test]
fn a_verbose_diff_below_the_scissors_line_is_not_scanned() {
    let vendor = first(terms::TermKind::Vendor);
    let message = format!(
        "docs(devlog): record the policy line [P0-14]\n\nWhat & why.\n\n\
         # ------------------------ >8 ------------------------\n\
         diff --git a/log/devlog.md b/log/devlog.md\n\
         +++ b/log/devlog.md\n\
         +the policy line quotes the {vendor} ban\n"
    );
    assert!(
        check_message(&message).is_ok(),
        "a verbose diff must not fail the message check"
    );
}

/// The cut is at the scissors line only: a trailer the user actually wrote still fails, even in
/// a verbose message.
#[test]
fn a_trailer_above_the_scissors_line_is_still_rejected() {
    let trailer = first(terms::TermKind::Attribution);
    let message = format!(
        "feat(x): y [P0-14]\n\n{trailer}: a helper\n\n\
         # ------------------------ >8 ------------------------\n\
         diff --git a/x b/x\n"
    );
    assert!(check_message(&message).is_err());
}

/// P0-14 review R2: the generator used to flatten every hit to `reference`, so an in-tree trailer
/// could be laundered into the allowlist by `--write-baseline`.
#[test]
fn the_baseline_generator_refuses_an_attribution_claim() {
    let trailer = first(terms::TermKind::Attribution);
    let hits = vec![Hit {
        path: "docs/example.md".to_string(),
        term: trailer,
        kind: terms::TermKind::Attribution,
        line: 4,
    }];
    let error = render_baseline(&hits).expect_err("an attribution claim must not be allowlisted");
    assert!(error.to_string().contains("never allowlisted"));
}

#[test]
fn the_baseline_generator_renders_reference_rows() {
    let vendor = first(terms::TermKind::Vendor);
    let content = render_baseline(&[hit("docs/architecture.md", &vendor, 15)])
        .expect("a reference hit is allowlistable");
    assert!(content.starts_with(BASELINE_HEADER));
    let rows: Vec<&str> = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();
    assert_eq!(
        rows,
        vec![format!("docs/architecture.md\t{vendor}\treference")]
    );
    assert_eq!(entries_in(&content), 1);
}
