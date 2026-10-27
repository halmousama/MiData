//! The UI language, as a closed set of two (D3).
//!
//! The persisted value is the lowercase code (`en` / `fr`) so settings rows stay stable and
//! human-readable. Adding a language is a product decision (D3 says English + French only), which is
//! why this is an enum and not an open registry like species.

/// A supported UI language.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English — the source catalogue.
    #[default]
    En,
    /// French — first-class, not an afterthought.
    Fr,
}

impl Language {
    /// Every supported language, in menu order.
    pub const ALL: [Self; 2] = [Self::En, Self::Fr];

    /// The persisted code (`en` / `fr`).
    pub const fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
        }
    }

    /// Parse a settings code. `None` for anything else — an unknown language falls back to the
    /// default at the call site instead of being silently coerced here.
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "fr" => Some(Self::Fr),
            _ => None,
        }
    }

    /// The language that follows this one in [`Language::ALL`] (for a toggle button).
    pub const fn next(self) -> Self {
        match self {
            Self::En => Self::Fr,
            Self::Fr => Self::En,
        }
    }
}
