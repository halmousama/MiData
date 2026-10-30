//! The D30 term lists: attribution trailers and AI vendor/model names.
//!
//! **The self-reference trap.** The banned-names gate scans the repository, so this file must not
//! contain the terms it bans — a literal here would make the gate flag its own definition and the
//! clean full-tree run impossible. Every term is therefore assembled from fragments at runtime, the
//! same device the legacy product-name gate uses (`gates.rs`), and
//! `term_literals_never_appear_in_their_own_source` is the acceptance test that keeps it true.
//!
//! The list mirrors the two halves of D30 (`REBUILD_PLAN.md` §15): (i) a trailer that claims
//! authorship/provenance and (ii) an AI vendor or model name. What a hit *means* is triaged in
//! `attribution.rs` and `docs/banned-names.baseline`: a vendor name naming an integration target is a
//! **reference**, not an attribution (P0-14 (d)).

/// What a term means for triage in `docs/banned-names.baseline`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermKind {
    /// A trailer claiming authorship/provenance. Never a legitimate surface term: an in-tree hit is
    /// allowlisted only as a documented `reference` to the rule (D30), and a commit message carrying
    /// one is rejected outright.
    Attribution,
    /// An AI vendor or model name. A hit may be a genuine reference (an API dialect, a compatibility
    /// target, a document describing the ban) and is allowlisted with that reason.
    Vendor,
}

/// One banned term: a stable id, its triage kind, and the lower-case needle matched against a line.
///
/// `id` doubles as the needle because every term is lower-case ASCII; both come from `concat!`, so
/// neither the id nor the needle is written literally in this file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub id: &'static str,
    pub kind: TermKind,
    pub needle: &'static str,
}

const fn term(id: &'static str, kind: TermKind) -> Term {
    Term {
        id,
        kind,
        needle: id,
    }
}

/// One matched term at one line (1-based) of the scanned text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermHit {
    pub term_id: &'static str,
    pub kind: TermKind,
    pub line: usize,
}

/// The full D30 term list.
///
/// Deliberately narrow: the common vendors and flagship model families that appear as attribution,
/// not every tool that ever spoke an LLM protocol (local runtimes and editors are compatibility
/// targets, i.e. references by definition). Animal names that are also model names are omitted on
/// purpose: this product's species registry is open, so such a word is a legitimate livestock term
/// long before it is an attribution, and the list must stay free of predictable false positives.
pub fn terms() -> Vec<Term> {
    vec![
        term(concat!("co-autho", "red-by"), TermKind::Attribution),
        term(concat!("open", "ai"), TermKind::Vendor),
        term(concat!("anthro", "pic"), TermKind::Vendor),
        term(concat!("clau", "de"), TermKind::Vendor),
        term(concat!("chat", "gpt"), TermKind::Vendor),
        term(concat!("gpt", "-4"), TermKind::Vendor),
        term(concat!("gpt", "-5"), TermKind::Vendor),
        term(concat!("co", "pilot"), TermKind::Vendor),
        term(concat!("gem", "ini"), TermKind::Vendor),
        term(concat!("deep", "seek"), TermKind::Vendor),
        term(concat!("mis", "tral"), TermKind::Vendor),
        term(concat!("per", "plexity"), TermKind::Vendor),
        term(concat!("gr", "ok"), TermKind::Vendor),
        term(concat!("co", "dex"), TermKind::Vendor),
    ]
}

/// Every term hit in `text`, one entry per (line, term). Matching is case-insensitive and
/// boundary-aware: a term must not be embedded inside a larger word, so a tunnel product whose name
/// merely ends in a model name is not a hit.
pub fn find(text: &str) -> Vec<TermHit> {
    let needles = terms();
    let mut hits: Vec<TermHit> = Vec::new();

    for (index, line) in text.lines().enumerate() {
        let line_number = index.saturating_add(1);
        let lowered = line.to_lowercase();
        for candidate in &needles {
            if !contains_term(&lowered, candidate.needle) {
                continue;
            }
            let already_reported = hits
                .iter()
                .any(|hit| hit.term_id == candidate.id && hit.line == line_number);
            if !already_reported {
                hits.push(TermHit {
                    term_id: candidate.id,
                    kind: candidate.kind,
                    line: line_number,
                });
            }
        }
    }

    hits
}

/// True when `needle` occurs in `haystack` (both already lower-case) as a standalone word: the
/// characters immediately around it are not alphanumeric. All needles are ASCII, so every index
/// `find` returns is a char boundary.
fn contains_term(haystack: &str, needle: &str) -> bool {
    let mut search_from = 0_usize;
    while let Some(found) = haystack
        .get(search_from..)
        .and_then(|rest| rest.find(needle))
    {
        let start = search_from.saturating_add(found);
        let end = start.saturating_add(needle.len());
        let before = haystack
            .get(..start)
            .and_then(|prefix| prefix.chars().next_back());
        let after = haystack.get(end..).and_then(|suffix| suffix.chars().next());
        let bounded =
            !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric);
        if bounded {
            return true;
        }
        search_from = end;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The acceptance test behind P0-14 (b): the list cannot trip the gate that owns it. The literal
    /// is built by `terms()` and searched for in this module's own source.
    #[test]
    fn term_literals_never_appear_in_their_own_source() {
        let source = include_str!("terms.rs").to_lowercase();
        for candidate in terms() {
            assert!(
                !source.contains(candidate.id),
                "`{}` must be assembled from fragments, never written literally here",
                candidate.id
            );
        }
    }

    #[test]
    fn every_term_is_lower_case_and_non_empty() {
        for candidate in terms() {
            assert!(!candidate.id.is_empty());
            assert_eq!(candidate.id, candidate.id.to_lowercase());
        }
    }

    #[test]
    fn an_attribution_trailer_is_found_case_insensitively() {
        let trailer = terms()
            .into_iter()
            .find(|candidate| candidate.kind == TermKind::Attribution)
            .map(|candidate| candidate.id)
            .unwrap_or_default();
        let message = format!("feat(x): y [P0-14]\n\nCo-Autho{}: someone\n", "red-By");
        assert!(message.to_lowercase().contains(trailer));
        let hits = find(&message);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].kind, TermKind::Attribution);
        assert_eq!(hits[0].line, 3);
    }

    #[test]
    fn a_vendor_term_is_found_in_a_document_line() {
        let vendor = terms()
            .into_iter()
            .find(|candidate| candidate.kind == TermKind::Vendor)
            .map(|candidate| candidate.id)
            .unwrap_or_default();
        let text = format!("The endpoint speaks the {}-compatible dialect.\n", vendor);
        let hits = find(&text);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].kind, TermKind::Vendor);
        assert_eq!(hits[0].line, 1);
    }

    #[test]
    fn a_clean_message_has_no_hits() {
        assert!(find("feat(xtask): add the gate [P0-14]\n\nRefs: gaps P0-14\n").is_empty());
    }

    /// One line mentions two different terms: both are reported, so the allowlist can be exact.
    #[test]
    fn distinct_terms_on_one_line_are_both_reported() {
        let vendors: Vec<&str> = terms()
            .into_iter()
            .filter(|candidate| candidate.kind == TermKind::Vendor)
            .map(|candidate| candidate.id)
            .take(2)
            .collect();
        assert_eq!(
            vendors.len(),
            2,
            "the vendor list must hold at least two terms"
        );
        let text = format!("{} and {} are named here.\n", vendors[0], vendors[1]);
        assert_eq!(find(&text).len(), 2);
    }

    /// Found while building the first baseline: a tunnel product's name ends in a model name, so a
    /// plain substring match produced a false positive. Terms must match as whole words.
    #[test]
    fn a_term_inside_a_longer_word_is_not_a_hit() {
        let vendor = terms()
            .into_iter()
            .find(|candidate| candidate.id.ends_with("ok"))
            .map(|candidate| candidate.id)
            .unwrap_or_default();
        assert!(
            !vendor.is_empty(),
            "the list must hold the term this test probes"
        );
        let word = format!("n{vendor}");
        assert!(!find(&word).iter().any(|hit| hit.term_id == vendor));
        assert!(find(&format!("the {vendor} tunnel"))
            .iter()
            .any(|hit| hit.term_id == vendor));
    }
}
