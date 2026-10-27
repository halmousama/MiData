//! Age, computed from the birth date — never stored (AGENTS.md rule 2).
//!
//! Ages are calendar ages: a birth on 29 February turns one year older on 28 February in a non-leap
//! year (the anniversary clamps, exactly as `chrono`'s own month arithmetic does for the day itself).
//! The arithmetic is bounded — no recursion, no unbounded loop, no hand-rolled leap-year table
//! (AGENTS.md rule 10: dates go through `chrono`).

use chrono::{Datelike, Months, NaiveDate};

/// A calendar age in whole years, whole months and remaining days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Age {
    years: u32,
    months: u32,
    days: u32,
}

impl Age {
    /// The age of an animal born on `birth`, on the calendar day `on`.
    ///
    /// Returns `None` when `on` is before `birth` — a future birth date is a data-entry error the
    /// caller reports, not an age of zero.
    pub fn between(birth: NaiveDate, on: NaiveDate) -> Option<Self> {
        if on < birth {
            return None;
        }

        let year_gap = on.year().checked_sub(birth.year())?;
        let mut years = u32::try_from(year_gap).ok()?;
        let mut cursor = anniversary(birth, years)?;
        if cursor > on {
            years = years.checked_sub(1)?;
            cursor = anniversary(birth, years)?;
        }

        let mut months = 0u32;
        while months < 12 {
            let Some(next) = cursor.checked_add_months(Months::new(1)) else {
                break;
            };
            if next > on {
                break;
            }
            cursor = next;
            months = months.saturating_add(1);
        }

        let days = u32::try_from((on - cursor).num_days()).ok()?;
        Some(Self {
            years,
            months,
            days,
        })
    }

    /// Whole years.
    pub const fn years(&self) -> u32 {
        self.years
    }

    /// Whole months since the last whole year (`0..=11`).
    pub const fn months(&self) -> u32 {
        self.months
    }

    /// Remaining days since the last whole month.
    pub const fn days(&self) -> u32 {
        self.days
    }
}

/// The birthday `years` after `birth`, clamping the day to the length of the target month.
///
/// `chrono`'s month arithmetic clamps 29 February to 28 February even in a leap year, which would
/// lose a day of age every four years; this loop walks the day down (`31 → 30 → 29 → 28`) instead, at
/// most three times.
fn anniversary(birth: NaiveDate, years: u32) -> Option<NaiveDate> {
    let year = birth.year().checked_add(i32::try_from(years).ok()?)?;
    let month = birth.month();
    let mut day = birth.day();
    for _ in 0..4 {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            return Some(date);
        }
        day = day.checked_sub(1)?;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    #[test]
    fn the_anniversary_clamps_but_keeps_the_leap_day() {
        assert_eq!(
            anniversary(date(2020, 2, 29), 4),
            Some(date(2024, 2, 29)),
            "a leap day survives a leap year"
        );
        assert_eq!(
            anniversary(date(2020, 2, 29), 1),
            Some(date(2021, 2, 28)),
            "a non-leap year clamps to the 28th"
        );
        assert_eq!(
            anniversary(date(2023, 1, 31), 1),
            Some(date(2024, 1, 31)),
            "a 31st is a 31st when the month has one"
        );
    }

    #[test]
    fn a_future_birth_date_has_no_age() {
        assert!(Age::between(date(2024, 3, 9), date(2024, 3, 8)).is_none());
    }
}
