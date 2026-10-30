//! The clock seam (`docs/architecture.md` §9).
//!
//! Dates drive age, latest-value ordering, pregnancy alerts and every analytics window, so "what day is
//! it?" must be an **injected dependency**, never a global read: a test that needs a 2024 date must not
//! depend on the wall clock, and an analytics result must be reproducible.
//!
//! The rule is one line long and enforced by a test in this crate: **`SystemClock` is the only place
//! in the workspace that may call `Utc::now()`** — every other crate takes a `&dyn Clock` (or a
//! generic `C: Clock`) from its caller. The composition roots (`apps/midata`, `apps/midata-cli`) decide
//! which implementation is real; `midata-testkit` supplies a fixed one.

use chrono::{DateTime, NaiveDate, Utc};

/// The only way the application learns the current time.
pub trait Clock: Send + Sync {
    /// The current instant in UTC.
    fn now_utc(&self) -> DateTime<Utc>;

    /// The current calendar day in UTC.
    ///
    /// Derived by default so an implementation cannot disagree with itself; override only when a
    /// test needs the two answers to differ.
    fn today(&self) -> NaiveDate {
        self.now_utc().date_naive()
    }
}

/// The real clock: the single wall-time read in the workspace.
///
/// Constructing it is a deliberate act by a composition root (or a test asserting the seam works);
/// nothing inside the engine reaches for it implicitly.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_utc(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
