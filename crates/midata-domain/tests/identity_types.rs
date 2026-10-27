//! Acceptance tests for the identity vocabulary: sex, lifecycle status, data types, cardinality and
//! the species key. Species must be **data**: the crate cannot know which species exist.

// `clippy.toml` allows `expect`/`panic` in tests; the lint can only see `#[test]` functions, so the
// `date` fixture below opts in for the whole test binary.
#![allow(clippy::expect_used)]

use chrono::NaiveDate;
use midata_domain::{
    validate_exit_date, Cardinality, DataType, ErrorReason, Sex, SpeciesKey, Status, ValueColumn,
};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("valid test date")
}

#[test]
fn sex_is_male_or_female_and_serializes_lowercase() {
    assert_eq!(Sex::Male.as_str(), "male");
    assert_eq!(Sex::Female.as_str(), "female");
    assert_eq!(
        serde_json::to_string(&Sex::Male).expect("sex serializes"),
        "\"male\""
    );
    assert_eq!(
        serde_json::from_str::<Sex>("\"female\"").expect("sex deserializes"),
        Sex::Female
    );

    let error = "unknown"
        .parse::<Sex>()
        .expect_err("an unknown sex is refused");
    assert_eq!(error.reason(), ErrorReason::InvalidParam);
    assert_eq!(error.i18n_key(), "err.sex.unknown");
    assert_eq!(error.param("value"), Some("unknown"));
}

#[test]
fn status_serializes_lowercase_and_has_three_states() {
    for (status, text) in [
        (Status::Alive, "alive"),
        (Status::Sold, "sold"),
        (Status::Dead, "dead"),
    ] {
        assert_eq!(status.as_str(), text);
        assert_eq!(
            serde_json::to_string(&status).expect("status serializes"),
            format!("\"{text}\"")
        );
        assert_eq!(
            serde_json::from_str::<Status>(&format!("\"{text}\"")).expect("status deserializes"),
            status
        );
    }
    assert!("missing".parse::<Status>().is_err());
}

#[test]
fn alive_animals_have_no_exit_date_and_exited_animals_need_one() {
    validate_exit_date(Status::Alive, None).expect("alive without an exit date is valid");

    let missing = validate_exit_date(Status::Sold, None).expect_err("sold needs an exit date");
    assert_eq!(missing.i18n_key(), "err.status.exit_date_required");
    assert_eq!(missing.reason(), ErrorReason::InvalidParam);
    assert_eq!(
        validate_exit_date(Status::Dead, None)
            .expect_err("dead needs an exit date")
            .i18n_key(),
        "err.status.exit_date_required"
    );

    validate_exit_date(Status::Sold, Some(date(2024, 5, 1))).expect("sold with a date is valid");
    validate_exit_date(Status::Dead, Some(date(2024, 5, 1))).expect("dead with a date is valid");

    let forbidden = validate_exit_date(Status::Alive, Some(date(2024, 5, 1)))
        .expect_err("an alive animal cannot have an exit date");
    assert_eq!(forbidden.i18n_key(), "err.status.exit_date_not_allowed");
}

#[test]
fn data_type_maps_to_exactly_one_storage_column() {
    for (data_type, text, column) in [
        (DataType::Number, "number", ValueColumn::Num),
        (DataType::Boolean, "boolean", ValueColumn::Num),
        (DataType::Text, "text", ValueColumn::Text),
        (DataType::Select, "select", ValueColumn::Text),
        (DataType::MultiSelect, "multi_select", ValueColumn::Text),
        (DataType::Date, "date", ValueColumn::Text),
    ] {
        assert_eq!(data_type.as_str(), text);
        assert_eq!(data_type.column(), column);
        assert_eq!(
            serde_json::to_string(&data_type).expect("data type serializes"),
            format!("\"{text}\"")
        );
        assert_eq!(
            serde_json::from_str::<DataType>(&format!("\"{text}\"")).expect("deserializes"),
            data_type
        );
    }
    assert!("decimal".parse::<DataType>().is_err());
}

#[test]
fn cardinality_is_single_or_series() {
    assert_eq!(Cardinality::Single.as_str(), "single");
    assert_eq!(Cardinality::Series.as_str(), "series");
    assert_eq!(
        serde_json::to_string(&Cardinality::Series).expect("cardinality serializes"),
        "\"series\""
    );
    assert!("many".parse::<Cardinality>().is_err());
}

#[test]
fn any_species_key_is_accepted_as_data() {
    // The keys are data, not variants: adding a species needs no code change (D8), so an arbitrary
    // user-typed key must behave exactly like a preset one.
    for key in ["cow", "sheep", "goat", "camel", "horse", "alpaca", "yak-2"] {
        let parsed = SpeciesKey::parse(key).expect("a valid species key");
        assert_eq!(parsed.as_str(), key);
        assert_eq!(parsed.to_string(), key);
    }
}

#[test]
fn invalid_species_keys_are_rejected_with_a_typed_error() {
    for key in ["", "Cow", "cow ", "cow!", "-cow", "cow\n", "1cow"] {
        let error = SpeciesKey::parse(key).expect_err("invalid species keys are refused");
        assert_eq!(error.reason(), ErrorReason::InvalidParam);
        assert_eq!(error.i18n_key(), "err.species.invalid_key");
    }

    let too_long = "c".repeat(33);
    assert!(SpeciesKey::parse(&too_long).is_err());
    assert!(SpeciesKey::parse("camel_2").is_ok());
}
