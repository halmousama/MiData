//! The value-encoding contract (`REBUILD_PLAN.md` §5.3).
//!
//! An observation stores **exactly one** of `value_num` / `value_text`, and the column must match the
//! field's data type. That rule is made structural here: [`EncodedValue`] *is* one of the two
//! columns, so "both set" and "neither set" are not representable after construction — only
//! [`EncodedValue::from_columns`], which validates the raw row, can see that state.
//!
//! Every failure is a typed [`CommandError`] with params for an agent; nothing here panics on a
//! hostile payload (imports are the hostile-payload surface).

use crate::command_error::CommandError;
use crate::data_type::DataType;
use crate::json_array;
use chrono::NaiveDate;
use std::collections::BTreeSet;

/// The maximum number of choices in one `multi_select` value.
///
/// A cap exists because the value arrives from a form, an import file or an agent: an unbounded list
/// is an unbounded allocation.
pub const MAX_MULTI_SELECT_ITEMS: usize = 256;

/// i18n key: the value's type does not match the field's data type.
pub const ERR_TYPE_MISMATCH: &str = "err.value.type_mismatch";
/// i18n key: `value_num` and `value_text` are not exactly-one.
pub const ERR_MISSING_COLUMN: &str = "err.value.missing_column";
/// i18n key: an empty choice (select or multi-select member).
pub const ERR_EMPTY_CHOICE: &str = "err.value.empty_choice";
/// i18n key: the same choice is listed twice.
pub const ERR_DUPLICATE_CHOICE: &str = "err.value.duplicate_choice";
/// i18n key: more choices than [`MAX_MULTI_SELECT_ITEMS`].
pub const ERR_TOO_MANY_CHOICES: &str = "err.value.too_many_choices";
/// i18n key: the stored JSON array is unreadable.
pub const ERR_MULTI_SELECT_MALFORMED: &str = "err.value.multi_select_malformed";
/// i18n key: a date that is not `YYYY-MM-DD`.
pub const ERR_DATE_MALFORMED: &str = "err.value.date_malformed";
/// i18n key: NaN or an infinity where a number is stored.
pub const ERR_NUMBER_NOT_FINITE: &str = "err.value.number_not_finite";

/// A typed field value, before encoding.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Value {
    /// A finite real number.
    Number(f64),
    /// Free text.
    Text(String),
    /// One option of a `select` field.
    Select(String),
    /// A truth value.
    Boolean(bool),
    /// Several options of a `multi_select` field, in user order.
    MultiSelect(Vec<String>),
    /// A calendar date.
    Date(NaiveDate),
}

/// A value as it is stored: exactly one observation column.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodedValue {
    /// `observations.value_num`.
    Num(f64),
    /// `observations.value_text`.
    Text(String),
}

impl EncodedValue {
    /// Read a raw row (both columns nullable) and enforce the exactly-one rule.
    pub fn from_columns(num: Option<f64>, text: Option<&str>) -> Result<Self, CommandError> {
        match (num, text) {
            (Some(number), None) => Ok(Self::Num(number)),
            (None, Some(value)) => Ok(Self::Text(value.to_string())),
            (Some(_), Some(_)) => Err(CommandError::invalid_param(ERR_MISSING_COLUMN)
                .with_param("detail", "both_columns_set")),
            (None, None) => Err(CommandError::invalid_param(ERR_MISSING_COLUMN)
                .with_param("detail", "no_column_set")),
        }
    }

    /// The numeric column, when this value occupies it.
    pub const fn num(&self) -> Option<f64> {
        match self {
            Self::Num(value) => Some(*value),
            Self::Text(_) => None,
        }
    }

    /// The text column, when this value occupies it.
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            Self::Num(_) => None,
        }
    }

    /// Which column this value occupies, as the matching `DataType` shorthand.
    const fn column_type(&self) -> DataType {
        match self {
            Self::Num(_) => DataType::Number,
            Self::Text(_) => DataType::Text,
        }
    }
}

impl Value {
    /// The data type this value belongs to.
    pub const fn data_type(&self) -> DataType {
        match self {
            Self::Number(_) => DataType::Number,
            Self::Text(_) => DataType::Text,
            Self::Select(_) => DataType::Select,
            Self::Boolean(_) => DataType::Boolean,
            Self::MultiSelect(_) => DataType::MultiSelect,
            Self::Date(_) => DataType::Date,
        }
    }

    /// Encode for storage, refusing a mismatch with the field's declared type.
    pub fn encode(&self, data_type: DataType) -> Result<EncodedValue, CommandError> {
        match (self, data_type) {
            (Self::Number(number), DataType::Number) => {
                if !number.is_finite() {
                    return Err(CommandError::invalid_param(ERR_NUMBER_NOT_FINITE));
                }
                Ok(EncodedValue::Num(*number))
            }
            (Self::Boolean(flag), DataType::Boolean) => {
                Ok(EncodedValue::Num(if *flag { 1.0 } else { 0.0 }))
            }
            (Self::Text(text), DataType::Text) => Ok(EncodedValue::Text(text.clone())),
            (Self::Select(choice), DataType::Select) => {
                if choice.is_empty() {
                    return Err(CommandError::invalid_param(ERR_EMPTY_CHOICE));
                }
                Ok(EncodedValue::Text(choice.clone()))
            }
            (Self::MultiSelect(items), DataType::MultiSelect) => {
                Ok(EncodedValue::Text(encode_choices(items)?))
            }
            (Self::Date(date), DataType::Date) => {
                Ok(EncodedValue::Text(date.format("%Y-%m-%d").to_string()))
            }
            (value, expected) => Err(type_mismatch(value.data_type(), expected)),
        }
    }

    /// Decode a stored value, refusing anything that does not match the declared type.
    pub fn decode(data_type: DataType, encoded: &EncodedValue) -> Result<Self, CommandError> {
        match (data_type, encoded) {
            (DataType::Number, EncodedValue::Num(number)) => {
                if !number.is_finite() {
                    return Err(CommandError::invalid_param(ERR_NUMBER_NOT_FINITE));
                }
                Ok(Self::Number(*number))
            }
            (DataType::Boolean, EncodedValue::Num(number)) => {
                if *number == 1.0 {
                    Ok(Self::Boolean(true))
                } else if *number == 0.0 {
                    Ok(Self::Boolean(false))
                } else {
                    Err(type_mismatch(DataType::Number, DataType::Boolean)
                        .with_param("detail", "boolean_not_zero_or_one"))
                }
            }
            (DataType::Text, EncodedValue::Text(text)) => Ok(Self::Text(text.clone())),
            (DataType::Select, EncodedValue::Text(text)) => {
                if text.is_empty() {
                    return Err(CommandError::invalid_param(ERR_EMPTY_CHOICE));
                }
                Ok(Self::Select(text.clone()))
            }
            (DataType::MultiSelect, EncodedValue::Text(text)) => {
                Ok(Self::MultiSelect(decode_choices(text)?))
            }
            (DataType::Date, EncodedValue::Text(text)) => Ok(Self::Date(parse_iso_date(text)?)),
            (expected, other) => Err(type_mismatch(other.column_type(), expected)),
        }
    }
}

fn type_mismatch(actual: DataType, expected: DataType) -> CommandError {
    CommandError::invalid_param(ERR_TYPE_MISMATCH)
        .with_param("expected", expected.as_str())
        .with_param("actual", actual.as_str())
}

/// Validate and encode a choice list as the documented JSON array string.
fn encode_choices(items: &[String]) -> Result<String, CommandError> {
    if items.len() > MAX_MULTI_SELECT_ITEMS {
        return Err(too_many_choices());
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for item in items {
        if item.is_empty() {
            return Err(CommandError::invalid_param(ERR_EMPTY_CHOICE));
        }
        if !seen.insert(item.as_str()) {
            return Err(
                CommandError::invalid_param(ERR_DUPLICATE_CHOICE).with_param("choice", item)
            );
        }
    }
    Ok(json_array::encode(items))
}

/// Decode a stored JSON array string and re-validate the list against the domain caps.
fn decode_choices(text: &str) -> Result<Vec<String>, CommandError> {
    let items = json_array::decode(text).map_err(|error| {
        CommandError::invalid_param(ERR_MULTI_SELECT_MALFORMED).with_param("detail", error.code())
    })?;
    if items.len() > MAX_MULTI_SELECT_ITEMS {
        return Err(too_many_choices());
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for item in &items {
        if item.is_empty() {
            return Err(CommandError::invalid_param(ERR_EMPTY_CHOICE));
        }
        if !seen.insert(item.as_str()) {
            return Err(
                CommandError::invalid_param(ERR_DUPLICATE_CHOICE).with_param("choice", item)
            );
        }
    }
    Ok(items)
}

fn too_many_choices() -> CommandError {
    CommandError::invalid_param(ERR_TOO_MANY_CHOICES)
        .with_param("max", MAX_MULTI_SELECT_ITEMS.to_string())
}

/// Parse an ISO-8601 calendar date, strictly (`YYYY-MM-DD`, zero-padded).
///
/// `chrono`'s parser is lenient about width; the shape check in front of it keeps `2024-3-9` out of the
/// database, because stored dates must sort as text (§5.3).
fn parse_iso_date(text: &str) -> Result<NaiveDate, CommandError> {
    let malformed =
        || CommandError::invalid_param(ERR_DATE_MALFORMED).with_param("value", text.to_string());
    let bytes = text.as_bytes();
    if bytes.len() != 10 {
        return Err(malformed());
    }
    let looks_like_a_date = bytes.iter().enumerate().all(|(index, byte)| match index {
        4 | 7 => *byte == b'-',
        _ => byte.is_ascii_digit(),
    });
    if !looks_like_a_date {
        return Err(malformed());
    }
    NaiveDate::parse_from_str(text, "%Y-%m-%d").map_err(|_| malformed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cap_is_the_documented_one() {
        assert_eq!(MAX_MULTI_SELECT_ITEMS, 256);
    }

    #[test]
    fn iso_dates_are_strict_to_the_day_width() {
        let ok = parse_iso_date("2024-03-09").expect("a valid date");
        assert_eq!(ok.format("%Y-%m-%d").to_string(), "2024-03-09");
        for malformed in [
            "2024-3-9",
            "2024-03-9",
            "24-03-09",
            "2024-02-30",
            "2024-03-09 ",
        ] {
            assert!(
                parse_iso_date(malformed).is_err(),
                "{malformed:?} is not an ISO date"
            );
        }
    }

    #[test]
    fn boolean_decoding_only_accepts_zero_or_one() {
        let encoded = EncodedValue::Num(0.5);
        let error = Value::decode(DataType::Boolean, &encoded).expect_err("0.5 is not a boolean");
        assert_eq!(error.i18n_key(), ERR_TYPE_MISMATCH);
        assert_eq!(error.param("detail"), Some("boolean_not_zero_or_one"));
    }
}
