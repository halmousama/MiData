//! Dynamic-field data types and the column each one is stored in.
//!
//! `REBUILD_PLAN.md` §5.3 fixes the encoding: `number`/`boolean` live in `value_num`;
//! `text`/`select`/`multi_select`/`date` live in `value_text`. The mapping is a method here so no
//! caller re-invents it, and the layer is checkable in one test.

use crate::command_error::CommandError;
use std::fmt;
use std::str::FromStr;

/// i18n key for a data-type token outside the six supported types.
pub const ERR_UNKNOWN_DATA_TYPE: &str = "err.data_type.unknown";

/// Which observation column holds a value of this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueColumn {
    /// `observations.value_num` (REAL).
    Num,
    /// `observations.value_text` (TEXT).
    Text,
}

/// The data type of a dynamic field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    /// A real number (`value_num`).
    Number,
    /// Free text (`value_text`).
    Text,
    /// One choice from `options_json.choices` (`value_text`).
    Select,
    /// A truth value, stored as `0.0`/`1.0` (`value_num`).
    Boolean,
    /// Several choices, stored as a JSON array string (`value_text`).
    MultiSelect,
    /// A calendar date, stored as ISO-8601 `YYYY-MM-DD` (`value_text`).
    Date,
}

impl DataType {
    /// The persisted machine key (`multi_select`, …), matching the schema CHECK constraint.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Number => "number",
            Self::Text => "text",
            Self::Select => "select",
            Self::Boolean => "boolean",
            Self::MultiSelect => "multi_select",
            Self::Date => "date",
        }
    }

    /// The single column a value of this type may occupy (§5.3).
    pub const fn column(self) -> ValueColumn {
        match self {
            Self::Number | Self::Boolean => ValueColumn::Num,
            Self::Text | Self::Select | Self::MultiSelect | Self::Date => ValueColumn::Text,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DataType {
    type Err = CommandError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "number" => Ok(Self::Number),
            "text" => Ok(Self::Text),
            "select" => Ok(Self::Select),
            "boolean" => Ok(Self::Boolean),
            "multi_select" => Ok(Self::MultiSelect),
            "date" => Ok(Self::Date),
            other => {
                Err(CommandError::invalid_param(ERR_UNKNOWN_DATA_TYPE).with_param("value", other))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_type_names_one_column_and_round_trips() {
        for (data_type, column) in [
            (DataType::Number, ValueColumn::Num),
            (DataType::Boolean, ValueColumn::Num),
            (DataType::Text, ValueColumn::Text),
            (DataType::Select, ValueColumn::Text),
            (DataType::MultiSelect, ValueColumn::Text),
            (DataType::Date, ValueColumn::Text),
        ] {
            assert_eq!(data_type.column(), column);
            assert_eq!(
                DataType::from_str(data_type.as_str()).expect("round trip"),
                data_type
            );
        }
        assert_eq!(
            DataType::from_str("decimal")
                .expect_err("unknown type")
                .i18n_key(),
            ERR_UNKNOWN_DATA_TYPE
        );
    }
}
