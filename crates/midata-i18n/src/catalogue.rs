//! The two catalogues, their parser and the coverage measurement.
//!
//! The TSV format is two columns, tab-separated, UTF-8, no header row, no quoting
//! (`docs/i18n.md` §2). Parsing is strict: a row without a tab or a repeated key is an error, never a
//! silent drop — a lost translation is exactly the failure the catalogue exists to prevent.

use crate::language::Language;
use std::collections::{BTreeMap, BTreeSet};

/// Which of the two catalogues a problem came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CatalogueName {
    /// The English source catalogue.
    En,
    /// The French catalogue.
    Fr,
}

impl CatalogueName {
    /// The file name as it sits under `catalogues/`.
    pub const fn file(self) -> &'static str {
        match self {
            Self::En => "en.tsv",
            Self::Fr => "fr.tsv",
        }
    }
}

impl std::fmt::Display for CatalogueName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.file())
    }
}

/// A catalogue that could not be loaded.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CatalogueError {
    /// A row is missing its tab, or has an empty key.
    #[error("{catalogue} line {line}: {detail}")]
    MalformedRow {
        /// Which catalogue the row is in.
        catalogue: CatalogueName,
        /// 1-based line number, for a human opening the file.
        line: usize,
        /// What is wrong with the row.
        detail: String,
    },
    /// The same key appears twice in one catalogue.
    #[error("{catalogue} repeats the key `{key}`")]
    DuplicateKey {
        /// Which catalogue the key repeats in.
        catalogue: CatalogueName,
        /// The repeated key.
        key: String,
    },
}

/// Both catalogues, parsed.
///
/// Construct it once in a composition root ([`Catalogue::embedded`]) and share it; lookups are binary
/// searches over sorted maps, so a rendered frame does no I/O and no allocation.
#[derive(Debug, Clone, Default)]
pub struct Catalogue {
    en: BTreeMap<String, String>,
    fr: BTreeMap<String, String>,
}

impl Catalogue {
    /// Parse the catalogues embedded in this build.
    pub fn embedded() -> Result<Self, CatalogueError> {
        Self::from_tsv_pair(crate::EN_TSV, crate::FR_TSV)
    }

    /// Parse a pair of TSV documents (used by [`Catalogue::embedded`] and by the tests).
    pub fn from_tsv_pair(en: &str, fr: &str) -> Result<Self, CatalogueError> {
        Ok(Self {
            en: parse(CatalogueName::En, en)?,
            fr: parse(CatalogueName::Fr, fr)?,
        })
    }

    /// The text for `key` in `language`, or `None` when the key is absent.
    pub fn get(&self, language: Language, key: &str) -> Option<&str> {
        self.rows(language).get(key).map(String::as_str)
    }

    /// Every source (EN) key, in deterministic order.
    pub fn en_keys<'a>(&'a self) -> impl Iterator<Item = &'a str> + 'a {
        self.en.keys().map(String::as_str)
    }

    /// The measured coverage of this pair.
    pub fn coverage(&self) -> Coverage {
        let en_keys: BTreeSet<&str> = self.en.keys().map(String::as_str).collect();
        let fr_keys: BTreeSet<&str> = self.fr.keys().map(String::as_str).collect();
        Coverage {
            en_total: self.en.len(),
            fr_total: self.fr.len(),
            missing_fr: en_keys
                .difference(&fr_keys)
                .map(|key| (*key).to_string())
                .collect(),
            orphan_fr: fr_keys
                .difference(&en_keys)
                .map(|key| (*key).to_string())
                .collect(),
        }
    }

    fn rows(&self, language: Language) -> &BTreeMap<String, String> {
        match language {
            Language::En => &self.en,
            Language::Fr => &self.fr,
        }
    }
}

/// How complete the French catalogue is against the English source.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Coverage {
    /// Number of source keys.
    pub en_total: usize,
    /// Number of French rows (may exceed `en_total` only when orphan rows exist).
    pub fr_total: usize,
    /// Keys present in EN but absent in FR, sorted.
    pub missing_fr: Vec<String>,
    /// Keys present in FR but absent in EN, sorted — dead rows, deleted rather than parked.
    pub orphan_fr: Vec<String>,
}

impl Coverage {
    /// Number of EN keys that have a French row.
    pub fn translated_fr(&self) -> usize {
        self.en_total.saturating_sub(self.missing_fr.len())
    }

    /// French coverage in whole percent (`0..=100`).
    pub fn fr_percent(&self) -> usize {
        self.translated_fr()
            .saturating_mul(100)
            .checked_div(self.en_total)
            .unwrap_or(0)
    }
}

fn parse(name: CatalogueName, content: &str) -> Result<BTreeMap<String, String>, CatalogueError> {
    let content = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut rows: BTreeMap<String, String> = BTreeMap::new();

    for (index, raw) in content.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let number = index.saturating_add(1);
        let Some((key, text)) = line.split_once('\t') else {
            return Err(CatalogueError::MalformedRow {
                catalogue: name,
                line: number,
                detail: "expected `key<TAB>text`".to_string(),
            });
        };
        let key = key.trim();
        if key.is_empty() {
            return Err(CatalogueError::MalformedRow {
                catalogue: name,
                line: number,
                detail: "empty key".to_string(),
            });
        }
        if rows.insert(key.to_string(), text.to_string()).is_some() {
            return Err(CatalogueError::DuplicateKey {
                catalogue: name,
                key: key.to_string(),
            });
        }
    }

    Ok(rows)
}
