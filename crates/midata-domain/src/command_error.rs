//! Typed command failures.
//!
//! Every rejection is a [`CommandError`]: a machine-readable [`ErrorReason`] class (an agent branches
//! on it without parsing prose), an i18n key (a human reads it in their language) and optional
//! params. A raw English message is never the contract (`REBUILD_PLAN.md` §3 `errors.typed-reasons`).

use std::collections::BTreeMap;
use std::fmt;

/// Why a command failed, as a stable machine class.
///
/// The five classes are the graceful-failure vocabulary of every command: a caller can tell "your
/// params are wrong" from "that row is gone" without a transport concept leaking into the domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorReason {
    /// The params (or the values they carry) are wrong; nothing was written.
    InvalidParam,
    /// The row or resource does not exist.
    NotFound,
    /// The request is valid but conflicts with the current state (duplicate id, mixed currency…).
    Conflict,
    /// A real feature that this build does not implement yet.
    Unsupported,
    /// A bug or an environment failure; the app stays alive and reports it.
    Internal,
}

impl ErrorReason {
    /// Stable machine key (`invalid_param`, `not_found`, …) — part of the agent contract, never
    /// renamed.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidParam => "invalid_param",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::Unsupported => "unsupported",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for ErrorReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A failed command: reason class, i18n key and params.
///
/// The `Display` impl is **developer-facing only** (log lines, test output): it shows the key and the
/// params so a failure can be traced. User-visible text is always produced at the edge by rendering
/// the key through `midata-i18n`, which is why no message string ever lands in this type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommandError {
    reason: ErrorReason,
    key: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    params: BTreeMap<String, String>,
}

impl CommandError {
    /// Build an error from a reason class and an i18n key.
    pub fn new(reason: ErrorReason, key: impl Into<String>) -> Self {
        Self {
            reason,
            key: key.into(),
            params: BTreeMap::new(),
        }
    }

    /// The caller's params are wrong (`invalid_param`).
    pub fn invalid_param(key: impl Into<String>) -> Self {
        Self::new(ErrorReason::InvalidParam, key)
    }

    /// The requested row or resource does not exist (`not_found`).
    pub fn not_found(key: impl Into<String>) -> Self {
        Self::new(ErrorReason::NotFound, key)
    }

    /// The request conflicts with the current state (`conflict`).
    pub fn conflict(key: impl Into<String>) -> Self {
        Self::new(ErrorReason::Conflict, key)
    }

    /// The behaviour is not implemented in this build (`unsupported`).
    pub fn unsupported(key: impl Into<String>) -> Self {
        Self::new(ErrorReason::Unsupported, key)
    }

    /// A bug or environment failure (`internal`).
    pub fn internal(key: impl Into<String>) -> Self {
        Self::new(ErrorReason::Internal, key)
    }

    /// Attach a placeholder value. Params are data for the message and diagnostics for agents; they
    /// are never concatenated into prose.
    #[must_use]
    pub fn with_param(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(name.into(), value.into());
        self
    }

    /// The machine-readable failure class.
    pub const fn reason(&self) -> ErrorReason {
        self.reason
    }

    /// The i18n key a surface renders with `t!`/`format`.
    pub fn i18n_key(&self) -> &str {
        &self.key
    }

    /// All placeholder values, in deterministic order.
    pub const fn params(&self) -> &BTreeMap<String, String> {
        &self.params
    }

    /// One placeholder value, if it was supplied.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(String::as_str)
    }

    /// Developer-facing param rendering used by `Display` (never shown to a user).
    fn params_debug(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }
        let pairs: Vec<String> = self
            .params
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect();
        format!("({})", pairs.join(", "))
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.key, self.params_debug())
    }
}

impl std::error::Error for CommandError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reasons_keep_their_machine_keys() {
        assert_eq!(ErrorReason::InvalidParam.as_str(), "invalid_param");
        assert_eq!(ErrorReason::NotFound.as_str(), "not_found");
        assert_eq!(ErrorReason::Conflict.as_str(), "conflict");
        assert_eq!(ErrorReason::Unsupported.as_str(), "unsupported");
        assert_eq!(ErrorReason::Internal.as_str(), "internal");
    }

    #[test]
    fn display_is_developer_facing_and_keeps_params() {
        let error = CommandError::conflict("err.money.currency_mismatch")
            .with_param("left", "TND")
            .with_param("right", "EUR");
        let rendered = error.to_string();
        assert!(rendered.contains("err.money.currency_mismatch"));
        assert!(rendered.contains("left=TND"));
        assert_eq!(
            CommandError::not_found("err.x").to_string(),
            "err.x ",
            "an error without params keeps a readable shape"
        );
    }

    #[test]
    fn params_are_ordered_and_optional() {
        let error = CommandError::invalid_param("err.x")
            .with_param("b", "2")
            .with_param("a", "1");
        assert_eq!(
            error.params().keys().collect::<Vec<_>>(),
            vec!["a", "b"],
            "params are deterministic for snapshots and tests"
        );
        assert_eq!(error.param("c"), None);
    }
}
