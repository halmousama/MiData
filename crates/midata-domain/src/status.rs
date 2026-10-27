//! Lifecycle status and its exit invariants.
//!
//! An animal is `Alive`, `Sold` or `Dead`. The invariant that matters is a pair: an exited animal
//! carries an `exit_date`, an alive one never does. The rule lives here as one function so every
//! command (`animal.update`, the birth event, an import row) shares it instead of re-deriving it.

use crate::command_error::CommandError;
use chrono::NaiveDate;
use std::fmt;
use std::str::FromStr;

/// i18n key for a status token outside the three states.
pub const ERR_UNKNOWN_STATUS: &str = "err.status.unknown";
/// i18n key for an exited animal with no exit date.
pub const ERR_EXIT_DATE_REQUIRED: &str = "err.status.exit_date_required";
/// i18n key for an alive animal carrying an exit date.
pub const ERR_EXIT_DATE_NOT_ALLOWED: &str = "err.status.exit_date_not_allowed";

/// The lifecycle status of an animal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Still in the herd.
    Alive,
    /// Left the herd by sale.
    Sold,
    /// Left the herd by death.
    Dead,
}

impl Status {
    /// The persisted machine key (`alive` / `sold` / `dead`), matching the schema CHECK constraint.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Alive => "alive",
            Self::Sold => "sold",
            Self::Dead => "dead",
        }
    }

    /// True when the animal has left the herd and therefore carries an exit date.
    pub const fn has_left_the_herd(self) -> bool {
        !matches!(self, Self::Alive)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Status {
    type Err = CommandError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "alive" => Ok(Self::Alive),
            "sold" => Ok(Self::Sold),
            "dead" => Ok(Self::Dead),
            other => {
                Err(CommandError::invalid_param(ERR_UNKNOWN_STATUS).with_param("value", other))
            }
        }
    }
}

/// Check the status/exit-date pair before it reaches the database.
///
/// Both directions are errors, not normalisations: silently inventing an exit date (or dropping one)
/// would rewrite the farmer's record.
pub fn validate_exit_date(
    status: Status,
    exit_date: Option<NaiveDate>,
) -> Result<(), CommandError> {
    match (status.has_left_the_herd(), exit_date) {
        (true, None) => Err(CommandError::invalid_param(ERR_EXIT_DATE_REQUIRED)
            .with_param("status", status.as_str())),
        (false, Some(_)) => Err(CommandError::invalid_param(ERR_EXIT_DATE_NOT_ALLOWED)
            .with_param("status", status.as_str())),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_invariants_hold_in_both_directions() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 1).expect("valid date");
        assert!(validate_exit_date(Status::Alive, None).is_ok());
        assert!(validate_exit_date(Status::Sold, Some(date)).is_ok());
        assert!(validate_exit_date(Status::Dead, Some(date)).is_ok());
        assert_eq!(
            validate_exit_date(Status::Dead, None)
                .expect_err("dead needs a date")
                .i18n_key(),
            ERR_EXIT_DATE_REQUIRED
        );
        assert_eq!(
            validate_exit_date(Status::Alive, Some(date))
                .expect_err("alive cannot have a date")
                .i18n_key(),
            ERR_EXIT_DATE_NOT_ALLOWED
        );
    }

    #[test]
    fn parsing_is_exact_and_graceful() {
        assert_eq!(Status::from_str("sold").expect("sold"), Status::Sold);
        assert_eq!(
            Status::from_str("Sold")
                .expect_err("case matters")
                .i18n_key(),
            ERR_UNKNOWN_STATUS
        );
    }
}
