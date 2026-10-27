//! Biological sex of an animal.
//!
//! `Sex` is a closed set by biology, not by policy: it has exactly two variants and is *nullable at
//! the schema level* (`sex TEXT CHECK (sex IN ('male','female'))` — a row may carry no sex yet), so
//! callers hold `Option<Sex>` rather than a third "unknown" variant.

use crate::command_error::CommandError;
use std::fmt;
use std::str::FromStr;

/// i18n key for a sex token that is neither `male` nor `female`.
pub const ERR_UNKNOWN_SEX: &str = "err.sex.unknown";

/// The sex of an animal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    /// Male — the only legal sex for a sire.
    Male,
    /// Female — the only legal sex for a dam.
    Female,
}

impl Sex {
    /// The persisted machine key (`male` / `female`), matching the schema CHECK constraint.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
        }
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Sex {
    type Err = CommandError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            other => Err(CommandError::invalid_param(ERR_UNKNOWN_SEX).with_param("value", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_is_exact_and_graceful() {
        assert_eq!(Sex::from_str("male").expect("male"), Sex::Male);
        assert_eq!(Sex::from_str("female").expect("female"), Sex::Female);
        for invalid in ["Male", "MALE", "", "m", "unknown"] {
            let error = Sex::from_str(invalid).expect_err("invalid sex");
            assert_eq!(error.i18n_key(), ERR_UNKNOWN_SEX);
        }
    }
}
