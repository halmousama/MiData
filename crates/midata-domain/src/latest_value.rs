//! Latest-value ordering (`REBUILD_PLAN.md` §5.4) as a pure rule.
//!
//! "Current value" of a field for an entity is the observation with **max `recorded_on`**, tie-break
//! **max `id`**. The production path is one window-function query in `midata-store`; this module holds
//! the same rule as code so the store's brute-force cross-check test and the analytics layer cannot
//! drift into a second, subtly different definition.

use chrono::NaiveDate;

/// The ordering key of an observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ObservationStamp {
    /// The calendar day the value was recorded on (the tie-break is the row id, not the clock time).
    pub recorded_on: NaiveDate,
    /// The observation row id.
    pub id: i64,
}

impl ObservationStamp {
    /// Build a stamp from the two ordering columns.
    pub const fn new(recorded_on: NaiveDate, id: i64) -> Self {
        Self { recorded_on, id }
    }

    /// True when this observation is the later write: max `recorded_on`, then max `id`.
    pub fn supersedes(&self, other: &Self) -> bool {
        (self.recorded_on, self.id) > (other.recorded_on, other.id)
    }
}

/// The latest item by [`ObservationStamp`], or `None` for an empty slice.
///
/// Deliberately a fold over the items: the caller decides what to materialise, and no ordering is
/// assumed about the input (the database returns rows in whatever order it likes).
pub fn latest_by_stamp<'a, T, F>(items: &'a [T], stamp: F) -> Option<&'a T>
where
    F: Fn(&T) -> ObservationStamp,
{
    let mut best: Option<(&'a T, ObservationStamp)> = None;
    for item in items {
        let current = stamp(item);
        match best {
            Some((_, best_stamp)) if !current.supersedes(&best_stamp) => {}
            _ => best = Some((item, current)),
        }
    }
    best.map(|(item, _)| item)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, day).expect("valid day")
    }

    #[test]
    fn a_later_day_wins_over_a_higher_id() {
        let rows = [
            ObservationStamp::new(day(1), 100),
            ObservationStamp::new(day(2), 1),
        ];
        let latest = latest_by_stamp(&rows, |row| *row).expect("latest");
        assert_eq!(latest.id, 1);
    }

    #[test]
    fn the_same_day_is_broken_by_the_highest_id() {
        let rows = [
            ObservationStamp::new(day(2), 3),
            ObservationStamp::new(day(2), 9),
            ObservationStamp::new(day(2), 4),
        ];
        assert_eq!(latest_by_stamp(&rows, |row| *row).expect("latest").id, 9);
    }

    #[test]
    fn an_empty_set_has_no_latest() {
        assert!(latest_by_stamp(&[] as &[ObservationStamp], |row| *row).is_none());
    }
}
