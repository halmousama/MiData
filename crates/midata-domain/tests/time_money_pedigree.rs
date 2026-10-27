//! Acceptance tests for derived facts and money: age is computed from `birth_date` (never stored),
//! money never becomes a float, and pedigree traversal is bounded before it starts.

// `clippy.toml` allows `expect`/`panic` in tests; the lint can only see `#[test]` functions, so the
// `date` fixture below opts in for the whole test binary.
#![allow(clippy::expect_used)]

use chrono::NaiveDate;
use midata_domain::{
    Age, Currency, ErrorReason, Money, PedigreeBounds, DEFAULT_GENERATIONS, MAX_GENERATIONS,
    MAX_NODES, MIN_GENERATIONS,
};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("valid test date")
}

#[test]
fn age_on_the_birth_date_is_zero() {
    let age = Age::between(date(2024, 3, 9), date(2024, 3, 9)).expect("an age exists");
    assert_eq!((age.years(), age.months(), age.days()), (0, 0, 0));
}

#[test]
fn age_before_the_birth_date_is_absent() {
    assert!(Age::between(date(2024, 3, 9), date(2024, 3, 8)).is_none());
}

#[test]
fn age_counts_calendar_months_across_a_leap_day() {
    let birth = date(2020, 2, 29);
    for (on, expected) in [
        (date(2020, 2, 29), (0, 0, 0)),
        (date(2020, 3, 28), (0, 0, 28)),
        (date(2020, 3, 29), (0, 1, 0)),
        (date(2021, 2, 27), (0, 11, 29)),
        // A non-leap anniversary is the 28th: the calendar clamps, we never invent a 29th of February.
        (date(2021, 2, 28), (1, 0, 0)),
        (date(2021, 3, 1), (1, 0, 1)),
        (date(2024, 2, 29), (4, 0, 0)),
        (date(2024, 3, 1), (4, 0, 1)),
    ] {
        let age = Age::between(birth, on).expect("an age exists");
        assert_eq!(
            (age.years(), age.months(), age.days()),
            expected,
            "age of {birth} on {on}"
        );
    }
}

#[test]
fn age_handles_a_century_without_drifting() {
    let age = Age::between(date(1900, 1, 1), date(2024, 1, 1)).expect("an age exists");
    assert_eq!((age.years(), age.months(), age.days()), (124, 0, 0));
}

#[test]
fn money_is_integer_minor_units() {
    let price = Money::from_minor(125_050, Currency::Tnd);
    assert_eq!(price.minor(), 125_050);
    assert_eq!(price.currency(), Currency::Tnd);
    assert_eq!(Currency::Tnd.code(), "TND");
    assert_eq!(Currency::Usd.code(), "USD");
    assert_eq!(Currency::Eur.code(), "EUR");

    let total = price
        .checked_add(Money::from_minor(4_950, Currency::Tnd))
        .expect("same currency adds");
    assert_eq!(total.minor(), 130_000);

    let difference = total
        .checked_sub(Money::from_minor(30_000, Currency::Tnd))
        .expect("same currency subtracts");
    assert_eq!(difference.minor(), 100_000);
    assert!(!difference.is_zero());
    assert!(Money::from_minor(0, Currency::Tnd).is_zero());
}

#[test]
fn money_rejects_mixed_currencies_and_overflow() {
    let mismatch = Money::from_minor(100, Currency::Tnd)
        .checked_add(Money::from_minor(100, Currency::Eur))
        .expect_err("currencies cannot be mixed");
    assert_eq!(mismatch.reason(), ErrorReason::Conflict);
    assert_eq!(mismatch.i18n_key(), "err.money.currency_mismatch");

    let overflow = Money::from_minor(i64::MAX, Currency::Tnd)
        .checked_add(Money::from_minor(1, Currency::Tnd))
        .expect_err("overflow must be an error");
    assert_eq!(overflow.reason(), ErrorReason::InvalidParam);
    assert_eq!(overflow.i18n_key(), "err.money.overflow");

    let underflow = Money::from_minor(i64::MIN, Currency::Tnd)
        .checked_sub(Money::from_minor(1, Currency::Tnd))
        .expect_err("underflow must be an error");
    assert_eq!(underflow.i18n_key(), "err.money.overflow");
}

#[test]
fn currency_parses_from_its_iso_code() {
    assert_eq!(
        "TND".parse::<Currency>().expect("TND is a currency"),
        Currency::Tnd
    );
    assert_eq!(
        serde_json::to_string(&Currency::Eur).expect("currency serializes"),
        "\"EUR\""
    );
    assert_eq!(
        "XYZ"
            .parse::<Currency>()
            .expect_err("unknown currency is refused")
            .i18n_key(),
        "err.currency.unknown"
    );
}

#[test]
fn pedigree_traversal_bounds_are_enforced_before_traversal() {
    assert_eq!(MIN_GENERATIONS, 1);
    assert_eq!(MAX_GENERATIONS, 10);
    assert_eq!(DEFAULT_GENERATIONS, 3);
    assert_eq!(MAX_NODES, 500);

    assert_eq!(
        PedigreeBounds::default().generations(),
        DEFAULT_GENERATIONS,
        "the default depth is the documented one"
    );
    for generations in MIN_GENERATIONS..=MAX_GENERATIONS {
        assert_eq!(
            PedigreeBounds::new(generations)
                .expect("in range is accepted")
                .generations(),
            generations
        );
    }
    for generations in [0, MAX_GENERATIONS + 1, u8::MAX] {
        let error = PedigreeBounds::new(generations).expect_err("out of range is refused");
        assert_eq!(error.reason(), ErrorReason::InvalidParam);
        assert_eq!(error.i18n_key(), "err.pedigree.generations_out_of_range");
        assert_eq!(error.param("min"), Some("1"));
        assert_eq!(error.param("max"), Some("10"));
    }
}
