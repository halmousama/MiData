//! Lookup and placeholder substitution.
//!
//! A [`Translator`] is a cheap, `Copy` view of "these catalogues, this language". It is passed down —
//! the shell holds one, a command handler receives one when it needs to render — so the language never
//! lives in a process-wide switch (`docs/architecture.md` §9).
//!
//! Two rules make the renderer safe:
//! - a missing key returns the **key itself** (visible in the UI, caught by the coverage gate);
//! - a placeholder with no matching param is left **as written**, so a partial render still tells a
//!   developer which param is missing instead of printing a hole.

use crate::catalogue::Catalogue;
use crate::language::Language;

/// A language-bound view over the embedded catalogues.
#[derive(Debug, Clone, Copy)]
pub struct Translator<'a> {
    catalogue: &'a Catalogue,
    language: Language,
}

impl<'a> Translator<'a> {
    /// Bind a catalogue to a language.
    pub const fn new(catalogue: &'a Catalogue, language: Language) -> Self {
        Self {
            catalogue,
            language,
        }
    }

    /// The bound language.
    pub const fn language(&self) -> Language {
        self.language
    }

    /// The same catalogue in another language (a live language switch re-binds, it does not reload).
    pub const fn with_language(self, language: Language) -> Self {
        Self {
            catalogue: self.catalogue,
            language,
        }
    }

    /// True when the key exists in the bound language.
    pub fn has(&self, key: &str) -> bool {
        self.catalogue.get(self.language, key).is_some()
    }

    /// The text for `key`, or `key` itself when it is missing.
    pub fn t<'b>(&'b self, key: &'b str) -> &'b str {
        self.catalogue.get(self.language, key).unwrap_or(key)
    }

    /// The text for `key` with `{name}` placeholders replaced by `params`.
    ///
    /// Unknown placeholders stay in the output; a param that the template does not use is ignored.
    pub fn format(&self, key: &str, params: &[(&str, &str)]) -> String {
        let template = self.t(key);
        let mut out = String::with_capacity(template.len());
        let mut rest = template;
        while let Some(open) = rest.find('{') {
            out.push_str(rest.get(..open).unwrap_or_default());
            let after = rest.get(open..).unwrap_or_default();
            let Some(close) = after.find('}') else {
                out.push_str(after);
                return out;
            };
            let name = after.get(1..close).unwrap_or_default();
            let written = after.get(..=close).unwrap_or_default();
            match params.iter().find(|(param, _)| *param == name) {
                Some((_, value)) => out.push_str(value),
                None => out.push_str(written),
            }
            rest = after.get(close.saturating_add(1)..).unwrap_or_default();
        }
        out.push_str(rest);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalogue() -> Catalogue {
        Catalogue::from_tsv_pair(
            "err.a\tA {one} and {two}\nerr.b\tNo params\n",
            "err.a\tA-fr {one} et {two}\nerr.b\tSans paramètre\n",
        )
        .expect("valid pair")
    }

    #[test]
    fn a_missing_key_is_its_own_placeholder() {
        let catalogue = catalogue();
        let translator = Translator::new(&catalogue, Language::En);
        assert_eq!(translator.t("err.unknown"), "err.unknown");
        assert!(!translator.has("err.unknown"));
    }

    #[test]
    fn switching_language_keeps_the_catalogue() {
        let catalogue = catalogue();
        let english = Translator::new(&catalogue, Language::En);
        let french = english.with_language(Language::Fr);
        assert_eq!(english.t("err.b"), "No params");
        assert_eq!(french.t("err.b"), "Sans paramètre");
        assert_eq!(french.language(), Language::Fr);
    }

    #[test]
    fn placeholders_are_substituted_once_and_never_eaten() {
        let catalogue = catalogue();
        let translator = Translator::new(&catalogue, Language::En);
        assert_eq!(
            translator.format("err.a", &[("one", "1"), ("two", "2")]),
            "A 1 and 2"
        );
        assert_eq!(
            translator.format("err.a", &[("one", "1")]),
            "A 1 and {two}",
            "a missing param stays visible"
        );
        assert_eq!(translator.format("err.b", &[("unused", "x")]), "No params");
    }

    #[test]
    fn a_broken_brace_never_panics() {
        let catalogue =
            Catalogue::from_tsv_pair("err.b\tunclosed {brace\n", "err.b\taccolade {ouverte\n")
                .expect("valid pair");
        let translator = Translator::new(&catalogue, Language::En);
        assert_eq!(translator.format("err.b", &[]), "unclosed {brace");
        assert_eq!(
            translator.format("err.b", &[("brace", "x")]),
            "unclosed {brace"
        );
    }
}
