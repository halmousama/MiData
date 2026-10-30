//! Acceptance tests for the string layer: both catalogues parse, every EN key has a FR row, no
//! placeholder is dropped in translation, lookup never panics, and coverage data is measurable.
//!
//! These tests run against the **real** catalogues, so they are the local half of the
//! `cargo xtask i18n-coverage` gate (the gate additionally enforces the ratchets and the orphan
//! rule, and reads the files from the repository tree).

// `clippy.toml` allows `expect`/`panic` in tests; the lint can only see `#[test]` functions, so the
// fixtures below opt in for the whole test binary.
#![allow(clippy::expect_used)]

use midata_i18n::{
    t, tf, Catalogue, CatalogueError, Coverage, Language, Translator, EN_TSV, FR_TSV,
};
use std::collections::{BTreeMap, BTreeSet};

fn embedded() -> Catalogue {
    Catalogue::embedded().expect("the embedded catalogues are valid")
}

/// The rows of a catalogue, as `(key, text)`.
fn rows(tsv: &str) -> Vec<(String, String)> {
    tsv.lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (key, text) = line.split_once('\t').expect("every row is key<TAB>text");
            (key.trim().to_string(), text.to_string())
        })
        .collect()
}

/// The `{placeholder}` names used by a template, in a deterministic order.
fn placeholder_names(text: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut rest = text;
    while let Some(open) = rest.find('{') {
        let after = rest.get(open..).unwrap_or_default();
        let Some(close) = after.find('}') else { break };
        if let Some(name) = after.get(1..close) {
            names.insert(name.to_string());
        }
        rest = after.get(close.saturating_add(1)..).unwrap_or_default();
    }
    names
}

#[test]
fn the_embedded_catalogues_parse_and_are_not_empty() {
    assert!(
        embedded().en_keys().count() > 0,
        "the source catalogue is not empty"
    );
}

#[test]
fn every_en_key_has_a_fr_row() {
    let coverage = embedded().coverage();
    assert_eq!(
        coverage.missing_fr,
        Vec::<String>::new(),
        "FR must be complete for every key that exists today (coverage is a ratchet)"
    );
    assert_eq!(coverage.en_total, coverage.fr_total);
}

#[test]
fn neither_catalogue_has_orphan_or_duplicate_rows() {
    let coverage = embedded().coverage();
    assert_eq!(
        coverage.orphan_fr,
        Vec::<String>::new(),
        "an FR row without an EN row is dead code, not a translation"
    );

    for (name, tsv) in [("en.tsv", EN_TSV), ("fr.tsv", FR_TSV)] {
        let mut seen = BTreeSet::new();
        for (key, _) in rows(tsv) {
            assert!(seen.insert(key.clone()), "{name} repeats the key {key}");
        }
    }
}

#[test]
fn keys_follow_the_documented_prefixes() {
    let allowed = [
        "screen.", "cmd.", "err.", "field.", "insight.", "species.", "nav.",
    ];
    for key in embedded().en_keys() {
        assert!(
            allowed.iter().any(|prefix| key.starts_with(prefix)),
            "key `{key}` has no documented prefix (docs/i18n.md §3)"
        );
        assert_eq!(key.to_lowercase(), key, "keys are lowercase: `{key}`");
    }
}

#[test]
fn fr_rows_are_translated_and_keep_every_placeholder() {
    let en = rows(EN_TSV);
    let fr: BTreeMap<String, String> = rows(FR_TSV).into_iter().collect();
    assert_eq!(en.len(), fr.len(), "every EN row has exactly one FR row");

    for (key, en_text) in &en {
        let Some(fr_text) = fr.get(key) else {
            panic!("FR row for `{key}` is missing");
        };
        assert!(
            !fr_text.trim().is_empty(),
            "FR row `{key}` is empty — a blank translation is a missing translation"
        );
        assert!(
            !en_text.trim().is_empty(),
            "EN row `{key}` is empty — the source catalogue cannot carry blank strings"
        );
        assert_eq!(
            placeholder_names(en_text),
            placeholder_names(fr_text),
            "`{key}` has different placeholders in EN and FR: {en_text:?} vs {fr_text:?}"
        );
    }
}

#[test]
fn lookup_returns_the_language_string() {
    let catalogue = embedded();
    let en = Translator::new(&catalogue, Language::En);
    let fr = Translator::new(&catalogue, Language::Fr);

    let key = "err.value.type_mismatch";
    assert_eq!(
        en.t(key),
        catalogue.get(Language::En, key).expect("EN text")
    );
    assert_ne!(
        en.t(key),
        fr.t(key),
        "the FR catalogue is first-class, not a copy of EN"
    );
    assert!(fr.has(key));
    assert!(!fr.has("err.missing.row"));
}

#[test]
fn a_missing_key_returns_the_key_itself_never_an_empty_string() {
    let catalogue = embedded();
    let translator = Translator::new(&catalogue, Language::Fr);
    assert_eq!(translator.t("err.missing.row"), "err.missing.row");
    assert_eq!(translator.format("err.missing.row", &[]), "err.missing.row");
    assert_eq!(translator.t(""), "");
}

#[test]
fn the_t_macro_looks_up_an_embedded_key() {
    let catalogue = embedded();
    let translator = Translator::new(&catalogue, Language::En);
    assert!(t!(translator, "err.value.type_mismatch").starts_with("The value"));
    assert_eq!(t!(translator, "err.missing.row"), "err.missing.row");
}

#[test]
fn format_substitutes_every_known_placeholder() {
    let catalogue = embedded();
    let translator = Translator::new(&catalogue, Language::En);
    let text = tf!(
        translator,
        "err.pedigree.generations_out_of_range",
        "min" => "1",
        "max" => "10"
    );
    assert!(text.contains("between 1 and 10"), "{text}");
    assert!(
        !text.contains('{'),
        "no placeholder survives substitution: {text}"
    );

    let unknown = translator.format(
        "err.pedigree.generations_out_of_range",
        &[("unrelated", "x")],
    );
    assert!(
        unknown.contains("{min}") && unknown.contains("{max}"),
        "an unknown param must never eat a placeholder: {unknown}"
    );
}

#[test]
fn coverage_is_measurable_from_a_synthetic_pair() {
    let catalogue = Catalogue::from_tsv_pair(
        "err.a\tA\nerr.b\tB\nerr.c\tC\n",
        "err.a\tA-fr\nerr.b\tB-fr\nerr.d\tD-fr\n",
    )
    .expect("a well-formed pair parses");
    let coverage: Coverage = catalogue.coverage();
    assert_eq!(coverage.en_total, 3);
    assert_eq!(coverage.fr_total, 3);
    assert_eq!(coverage.missing_fr, vec!["err.c".to_string()]);
    assert_eq!(coverage.orphan_fr, vec!["err.d".to_string()]);
}

#[test]
fn malformed_catalogues_are_refused_instead_of_silently_dropped() {
    let no_tab = Catalogue::from_tsv_pair("err.a\tA\nbroken row\n", "err.a\tA-fr\n");
    assert!(matches!(no_tab, Err(CatalogueError::MalformedRow { .. })));

    let duplicate = Catalogue::from_tsv_pair("err.a\tA\nerr.a\tAgain\n", "err.a\tA-fr\n");
    assert!(matches!(
        duplicate,
        Err(CatalogueError::DuplicateKey { .. })
    ));

    let empty_key = Catalogue::from_tsv_pair(" \tA\n", " \tA-fr\n");
    assert!(matches!(
        empty_key,
        Err(CatalogueError::MalformedRow { .. })
    ));
}

#[test]
fn language_round_trips_through_its_settings_code() {
    for language in Language::ALL {
        assert_eq!(Language::from_code(language.code()), Some(language));
    }
    assert_eq!(Language::default(), Language::En);
    assert_eq!(Language::Fr.code(), "fr");
    assert_eq!(Language::from_code("de"), None);
    assert_eq!(Language::from_code("EN"), None);
}
