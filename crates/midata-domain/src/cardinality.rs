//! Field cardinality: one current value, or a time series.
//!
//! `single` fields keep at most one observation per (entity, field) through an upsert; `series`
//! fields keep a dated history. The invariant is enforced in `midata-store`, but the vocabulary — and
//! with it every branch that needs to know which rule applies — belongs here.

use crate::command_error::CommandError;
use std::fmt;
use std::str::FromStr;

/// i18n key for a cardinality token that is neither `single` nor `series`.
pub const ERR_UNKNOWN_CARDINALITY: &str = "err.cardinality.unknown";

/// How many observations a field keeps per entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    /// One current value per entity (upsert semantics).
    Single,
    /// A dated series, unique per `(entity, field, recorded_on)`.
    Series,
}

impl Cardinality {
    /// The persisted machine key (`single` / `series`), matching the schema CHECK constraint.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Series => "series",
        }
    }

    /// True when the field keeps a history instead of a current value.
    pub const fn is_series(self) -> bool {
        matches!(self, Self::Series)
    }
}

impl fmt::Display for Cardinality {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Cardinality {
    type Err = CommandError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "single" => Ok(Self::Single),
            "series" => Ok(Self::Series),
            other => {
                Err(CommandError::invalid_param(ERR_UNKNOWN_CARDINALITY).with_param("value", other))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinality_round_trips_and_refuses_unknown_tokens() {
        assert!(Cardinality::Series.is_series());
        assert!(!Cardinality::Single.is_series());
        assert_eq!(
            Cardinality::from_str("series").expect("series"),
            Cardinality::Series
        );
        assert_eq!(
            Cardinality::from_str("many")
                .expect_err("unknown cardinality")
                .i18n_key(),
            ERR_UNKNOWN_CARDINALITY
        );
    }
}
