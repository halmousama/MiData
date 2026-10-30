//! Money: integer minor units plus a currency code (AGENTS.md rule 11).
//!
//! There is no `f64` anywhere in this module. A price is `12 505` minor units of TND, not `125.05`;
//! conversion and formatting happen only at the UI edge. Combining two currencies is a typed
//! conflict, and overflow is an error rather than a wrapped total — a herd's books are not a place
//! for silent arithmetic.

use crate::command_error::CommandError;
use std::fmt;
use std::str::FromStr;

/// i18n key for a currency code the product does not support.
pub const ERR_UNKNOWN_CURRENCY: &str = "err.currency.unknown";
/// i18n key for combining amounts in two currencies.
pub const ERR_CURRENCY_MISMATCH: &str = "err.money.currency_mismatch";
/// i18n key for an amount that does not fit in `i64` minor units.
pub const ERR_MONEY_OVERFLOW: &str = "err.money.overflow";

/// A supported currency. Default is TND (owner policy); the code is the persisted value.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    /// Tunisian dinar (the default).
    #[default]
    Tnd,
    /// United States dollar.
    Usd,
    /// Euro.
    Eur,
}

impl Currency {
    /// The ISO-4217 code stored in settings and used for display.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Tnd => "TND",
            Self::Usd => "USD",
            Self::Eur => "EUR",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl FromStr for Currency {
    type Err = CommandError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "TND" => Ok(Self::Tnd),
            "USD" => Ok(Self::Usd),
            "EUR" => Ok(Self::Eur),
            other => {
                Err(CommandError::invalid_param(ERR_UNKNOWN_CURRENCY).with_param("value", other))
            }
        }
    }
}

/// An amount in minor units of one currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Money {
    minor: i64,
    currency: Currency,
}

impl Money {
    /// Build an amount from minor units.
    pub const fn from_minor(minor: i64, currency: Currency) -> Self {
        Self { minor, currency }
    }

    /// The amount in minor units (cents/millimes).
    pub const fn minor(self) -> i64 {
        self.minor
    }

    /// The currency of this amount.
    pub const fn currency(self) -> Currency {
        self.currency
    }

    /// True for exactly zero — the only float-free "empty" test.
    pub const fn is_zero(self) -> bool {
        self.minor == 0
    }

    /// Add two amounts of the same currency, failing on a currency mix or overflow.
    pub fn checked_add(self, other: Self) -> Result<Self, CommandError> {
        self.ensure_same_currency(other)?;
        let minor = self.minor.checked_add(other.minor).ok_or_else(overflow)?;
        Ok(Self {
            minor,
            currency: self.currency,
        })
    }

    /// Subtract two amounts of the same currency, failing on a currency mix or underflow.
    pub fn checked_sub(self, other: Self) -> Result<Self, CommandError> {
        self.ensure_same_currency(other)?;
        let minor = self.minor.checked_sub(other.minor).ok_or_else(overflow)?;
        Ok(Self {
            minor,
            currency: self.currency,
        })
    }

    fn ensure_same_currency(self, other: Self) -> Result<(), CommandError> {
        if self.currency == other.currency {
            return Ok(());
        }
        Err(CommandError::conflict(ERR_CURRENCY_MISMATCH)
            .with_param("left", self.currency.code())
            .with_param("right", other.currency.code()))
    }
}

fn overflow() -> CommandError {
    CommandError::invalid_param(ERR_MONEY_OVERFLOW)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_currency_is_tnd() {
        assert_eq!(Currency::default(), Currency::Tnd);
        assert_eq!(Currency::default().code(), "TND");
    }

    #[test]
    fn combining_currencies_is_a_conflict_not_a_conversion() {
        let error = Money::from_minor(1, Currency::Tnd)
            .checked_sub(Money::from_minor(1, Currency::Usd))
            .expect_err("no implicit conversion");
        assert_eq!(error.reason(), crate::command_error::ErrorReason::Conflict);
        assert_eq!(error.i18n_key(), ERR_CURRENCY_MISMATCH);
    }
}
