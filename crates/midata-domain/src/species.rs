//! The species key — a **data** identifier, never an enum.
//!
//! The registry presets are Cow, Sheep and Goat, but the product accepts any species the user adds
//! (a camel, a horse, an alpaca…). Encoding that as an enum would make "add a species" a code change,
//! so the only thing the domain knows about species is the *shape* of the key; everything else
//! (labels, gestation days, field packs) lives in the `species` table (D8, `REBUILD_PLAN.md` §5.6).

use crate::command_error::CommandError;
use std::fmt;
use std::str::FromStr;

/// i18n key for a species key that does not match the documented shape.
pub const ERR_INVALID_SPECIES_KEY: &str = "err.species.invalid_key";

/// Maximum length of a species key, in bytes (it is ASCII by construction).
pub const MAX_SPECIES_KEY_LEN: usize = 32;

/// A validated machine key for a species (`cow`, `camel`, `alpaca_2`, …).
///
/// Validation is structural only: lowercase ASCII start, then lowercase letters, digits, `_` or `-`.
/// Nothing in this type enumerates which species exist — that is data.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct SpeciesKey(String);

impl SpeciesKey {
    /// Validate a raw key from a command param, an import cell or the settings screen.
    pub fn parse(raw: &str) -> Result<Self, CommandError> {
        if !is_valid(raw) {
            return Err(CommandError::invalid_param(ERR_INVALID_SPECIES_KEY).with_param("key", raw));
        }
        Ok(Self(raw.to_string()))
    }

    /// The key as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Byte length (ASCII by construction, so this is also the character count).
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always false: a valid key is never empty. Present because clippy pairs it with `len`.
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl fmt::Display for SpeciesKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for SpeciesKey {
    type Err = CommandError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::parse(raw)
    }
}

/// The shape rule, in one place.
fn is_valid(raw: &str) -> bool {
    if raw.is_empty() || raw.len() > MAX_SPECIES_KEY_LEN {
        return false;
    }
    let mut characters = raw.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && characters.all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '_' | '-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shape_rule_is_the_only_rule() {
        for valid in [
            "cow",
            "camel",
            "a",
            "alpaca-2",
            "x_9",
            "z".repeat(MAX_SPECIES_KEY_LEN).as_str(),
        ] {
            assert!(SpeciesKey::parse(valid).is_ok(), "{valid} is a valid key");
        }
        for invalid in ["", "Cow", "cow!", " cow", "cow ", "9cow", "cow\n", "éwe"] {
            assert!(
                SpeciesKey::parse(invalid).is_err(),
                "{invalid:?} is not a valid key"
            );
        }
        assert!(SpeciesKey::parse(&"c".repeat(MAX_SPECIES_KEY_LEN + 1)).is_err());
    }
}
