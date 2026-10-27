//! Acceptance tests for the value-encoding contract (`REBUILD_PLAN.md` §5.3).
//!
//! Each test names the boundary it exercises: the *only* thing these tests care about is that a
//! value can be stored in exactly one column, matching its data type, and that a hostile payload
//! comes back as a typed `CommandError` rather than a panic.

// `clippy.toml` allows `expect`/`panic` in tests; the lint can only see `#[test]` functions, so the
// helpers below (fixtures, round-trip helpers) opt in for the whole test binary.
#![allow(clippy::expect_used)]

use chrono::NaiveDate;
use midata_domain::{DataType, EncodedValue, ErrorReason, Value, MAX_MULTI_SELECT_ITEMS};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("valid test date")
}

fn choices(items: &[&str]) -> Value {
    Value::MultiSelect(items.iter().map(|item| (*item).to_string()).collect())
}

fn round_trip(value: Value, data_type: DataType) -> Value {
    let encoded = value.encode(data_type).expect("value encodes");
    Value::decode(data_type, &encoded).expect("encoded value decodes")
}

#[test]
fn number_round_trips_through_value_num() {
    let encoded = Value::Number(412.5)
        .encode(DataType::Number)
        .expect("number encodes");
    assert_eq!(encoded.num(), Some(412.5));
    assert_eq!(encoded.text(), None);
    assert_eq!(
        round_trip(Value::Number(412.5), DataType::Number),
        Value::Number(412.5)
    );
}

#[test]
fn boolean_encodes_as_zero_or_one() {
    let true_value = Value::Boolean(true)
        .encode(DataType::Boolean)
        .expect("true encodes");
    let false_value = Value::Boolean(false)
        .encode(DataType::Boolean)
        .expect("false encodes");
    assert_eq!(true_value.num(), Some(1.0));
    assert_eq!(false_value.num(), Some(0.0));
    assert_eq!(true_value.text(), None);
    assert_eq!(
        round_trip(Value::Boolean(true), DataType::Boolean),
        Value::Boolean(true)
    );
    assert_eq!(
        round_trip(Value::Boolean(false), DataType::Boolean),
        Value::Boolean(false)
    );
}

#[test]
fn text_and_select_use_value_text() {
    let text = Value::Text("Friesian cross".to_string())
        .encode(DataType::Text)
        .expect("text encodes");
    assert_eq!(text.text(), Some("Friesian cross"));
    assert_eq!(text.num(), None);

    let select = Value::Select("pregnant".to_string())
        .encode(DataType::Select)
        .expect("select encodes");
    assert_eq!(select.text(), Some("pregnant"));
    assert_eq!(select.num(), None);
}

#[test]
fn multi_select_round_trips_as_a_json_array() {
    let value = choices(&["lactating", "pregnant"]);
    let encoded = value.encode(DataType::MultiSelect).expect("choices encode");
    assert_eq!(encoded.text(), Some(r#"["lactating","pregnant"]"#));
    assert_eq!(encoded.num(), None);
    assert_eq!(
        Value::decode(DataType::MultiSelect, &encoded).expect("choices decode"),
        value
    );
}

#[test]
fn multi_select_escapes_quotes_backslashes_and_control_characters() {
    let tricky = vec![
        "quote\"inside".to_string(),
        "back\\slash".to_string(),
        "line\nbreak".to_string(),
        "tab\there".to_string(),
        "control\u{1}char".to_string(),
        "accentué".to_string(),
        "clef \u{1d11e}".to_string(),
    ];
    let value = Value::MultiSelect(tricky);
    let encoded = value.encode(DataType::MultiSelect).expect("choices encode");
    let text = encoded.text().expect("choices are text");
    assert!(
        text.starts_with('['),
        "a multi-select value is a JSON array: {text}"
    );
    assert!(
        !text.contains('\n'),
        "control characters must be escaped: {text}"
    );
    assert_eq!(
        Value::decode(DataType::MultiSelect, &encoded).expect("choices decode"),
        value
    );
}

#[test]
fn date_round_trips_as_iso_8601() {
    let encoded = Value::Date(date(2024, 3, 9))
        .encode(DataType::Date)
        .expect("date encodes");
    assert_eq!(encoded.text(), Some("2024-03-09"));
    assert_eq!(encoded.num(), None);
    assert_eq!(
        Value::decode(DataType::Date, &encoded).expect("date decodes"),
        Value::Date(date(2024, 3, 9))
    );
}

#[test]
fn encoding_a_value_into_the_wrong_data_type_is_graceful() {
    let error = Value::Text("not a number".to_string())
        .encode(DataType::Number)
        .expect_err("a text value cannot be a number");
    assert_eq!(error.reason(), ErrorReason::InvalidParam);
    assert_eq!(error.i18n_key(), "err.value.type_mismatch");
    assert_eq!(error.param("expected"), Some("number"));
    assert_eq!(error.param("actual"), Some("text"));

    let date_error = Value::Date(date(2024, 3, 9))
        .encode(DataType::Boolean)
        .expect_err("a date value cannot be a boolean");
    assert_eq!(date_error.i18n_key(), "err.value.type_mismatch");
}

#[test]
fn decoding_with_both_columns_set_is_rejected() {
    let error = EncodedValue::from_columns(Some(1.0), Some("also text"))
        .expect_err("exactly one column may be set");
    assert_eq!(error.reason(), ErrorReason::InvalidParam);
    assert_eq!(error.i18n_key(), "err.value.missing_column");
}

#[test]
fn decoding_with_neither_column_set_is_rejected() {
    let error = EncodedValue::from_columns(None, None).expect_err("one column must be set");
    assert_eq!(error.i18n_key(), "err.value.missing_column");
}

#[test]
fn decoding_a_value_from_the_wrong_column_is_rejected() {
    let encoded = EncodedValue::Text("42".to_string());
    let error = Value::decode(DataType::Number, &encoded).expect_err("a number needs value_num");
    assert_eq!(error.i18n_key(), "err.value.type_mismatch");
}

#[test]
fn decoding_a_non_iso_date_is_rejected() {
    for text in [
        "09/03/2024",
        "2024-3-9",
        "2024-03-09T00:00",
        "",
        "2024-13-01",
    ] {
        let encoded = EncodedValue::Text(text.to_string());
        let error =
            Value::decode(DataType::Date, &encoded).expect_err("only ISO-8601 dates may decode");
        assert_eq!(
            error.i18n_key(),
            "err.value.date_malformed",
            "{text} must be rejected as a malformed date"
        );
    }
}

#[test]
fn non_finite_numbers_are_rejected() {
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = Value::Number(number)
            .encode(DataType::Number)
            .expect_err("a non-finite number cannot be stored");
        assert_eq!(error.reason(), ErrorReason::InvalidParam);
        assert_eq!(error.i18n_key(), "err.value.number_not_finite");

        let encoded = EncodedValue::Num(number);
        assert!(Value::decode(DataType::Number, &encoded).is_err());
    }
}

#[test]
fn empty_and_duplicate_choices_are_rejected() {
    let empty = choices(&["lactating", ""]).encode(DataType::MultiSelect);
    assert_eq!(
        empty
            .expect_err("an empty choice is not a choice")
            .i18n_key(),
        "err.value.empty_choice"
    );

    let duplicate = choices(&["lactating", "lactating"]).encode(DataType::MultiSelect);
    assert_eq!(
        duplicate
            .expect_err("duplicates would hide a data-entry mistake")
            .i18n_key(),
        "err.value.duplicate_choice"
    );
}

#[test]
fn too_many_choices_are_rejected() {
    let items: Vec<String> = (0..=MAX_MULTI_SELECT_ITEMS)
        .map(|index| format!("choice-{index}"))
        .collect();
    let error = Value::MultiSelect(items)
        .encode(DataType::MultiSelect)
        .expect_err("the choice count is capped");
    assert_eq!(error.i18n_key(), "err.value.too_many_choices");
    assert_eq!(error.param("max"), Some("256"));
}

#[test]
fn hostile_multi_select_payloads_never_panic() {
    let payloads = [
        "",
        "[",
        "]",
        "[1,2]",
        "[true]",
        r#"["a",]"#,
        r#"["a" "b"]"#,
        r#"[["a"]]"#,
        r#"["\uD800"]"#,
        r#"["\uZZZZ"]"#,
        r#"["unterminated]"#,
        "[\"nul\u{0}byte\"]",
        r#"["a"]trailing"#,
        r#"["a","a"]"#,
        "  ",
        "null",
        "{\"a\":1}",
    ];
    for payload in payloads {
        let encoded = EncodedValue::Text(payload.to_string());
        let result = Value::decode(DataType::MultiSelect, &encoded);
        assert!(
            result.is_err(),
            "payload {payload:?} is not a valid multiple-choice value"
        );
    }

    // Deeply nested / oversized payloads are capped, not recursed into.
    let deep = "[".repeat(10_000);
    let oversized = format!("[\"{}\"]", "x".repeat(100_000));
    for payload in [deep, oversized] {
        let encoded = EncodedValue::Text(payload);
        assert!(Value::decode(DataType::MultiSelect, &encoded).is_err());
    }
}
