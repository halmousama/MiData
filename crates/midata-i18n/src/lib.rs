//! `midata-i18n` — the string layer (L0: pure, no I/O).
//!
//! Two TSV catalogues are **embedded at build time** (`include_str!`), so there is no file I/O at
//! runtime and no "translation file not found" failure mode: `en.tsv` is the source, `fr.tsv` is
//! first-class (D3, D24). The same lookup serves the egui shell, the CLI and engine-side
//! `CommandError` messages — that is why the crate exists instead of a UI-local table.
//!
//! ## Shape
//!
//! ```text
//! Catalogue::embedded()        → both catalogues, parsed once by the composition root
//! Translator::new(&catalogue, Language::Fr)
//! t!(translator, "err.value.type_mismatch")          → &str (the key itself on a miss)
//! tf!(translator, "err.pedigree...", "min" => "1")   → String with placeholders substituted
//! ```
//!
//! ## No globals
//!
//! The language lives in settings and the catalogue is constructed by the composition root; the crate
//! holds **no `static` state**, so a test can render any language without touching a process-wide
//! switch (`docs/architecture.md` §9). `t!` is a thin method call, not a hidden lookup table.
//!
//! A missing key renders as the **key itself** — never a panic, never an empty string — and shows up in
//! [`Catalogue::coverage`] and in the `cargo xtask i18n-coverage` gate.

mod catalogue;
mod language;
mod translator;

pub use catalogue::{Catalogue, CatalogueError, CatalogueName, Coverage};
pub use language::Language;
pub use translator::Translator;

/// The English source catalogue, embedded at build time.
pub const EN_TSV: &str = include_str!("../catalogues/en.tsv");

/// The French catalogue, embedded at build time.
pub const FR_TSV: &str = include_str!("../catalogues/fr.tsv");

/// Look up a key in the translator's language, falling back to the key itself on a miss.
///
/// ```ignore
/// let text = t!(translator, "err.value.type_mismatch");
/// ```
#[macro_export]
macro_rules! t {
    ($translator:expr, $key:literal) => {
        ($translator).t($key)
    };
}

/// Look up a key and substitute `{placeholder}` params.
///
/// Values must be `&str` (a numeric param is formatted by the caller, so the catalogue never decides
/// how a number looks).
///
/// ```ignore
/// let text = tf!(translator, "err.pedigree.generations_out_of_range", "min" => "1", "max" => "10");
/// ```
#[macro_export]
macro_rules! tf {
    ($translator:expr, $key:literal $(, $name:literal => $value:expr)* $(,)?) => {
        ($translator).format($key, &[$(( $name, $value )),*])
    };
}
