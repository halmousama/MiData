//! A minimal, strict JSON **string-array** codec for `multi_select` values (internal).
//!
//! Why hand-rolled: `midata-domain` may depend only on `serde`, `chrono` and `thiserror`
//! (AGENTS.md §2, `xtask/src/layers.rs`), and `serde` alone has no format. The grammar is therefore
//! the smallest useful one — `["a","b"]` with JSON escapes — and it is **strict and bounded**:
//! anything else is an error, and decoding is capped so a hostile import cell cannot make the app
//! allocate without limit.
//!
//! The encoder only ever emits plain UTF-8 plus the standard escapes, so a value produced here always
//! round-trips; the decoder additionally rejects unpaired surrogates and raw control characters.

use std::fmt::Write as _;

/// Hard cap on the decoded payload (bytes) — well above any human choice list, far below a file.
const MAX_PAYLOAD_LEN: usize = 64 * 1024;

/// Hard cap on the number of decoded entries, before the domain-level cap applies.
const MAX_DECODED_ITEMS: usize = 4 * 1024;

/// Why a payload is not a JSON string array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The payload is larger than [`MAX_PAYLOAD_LEN`] or holds too many entries.
    #[error("payload exceeds the supported size")]
    TooLong,
    /// Not the `[` … `]` shape, or an element is not a string.
    #[error("not a JSON array of strings")]
    NotAnArray,
    /// The input ended inside a string.
    #[error("unterminated string")]
    UnterminatedString,
    /// An escape sequence that is not part of JSON.
    #[error("invalid escape")]
    InvalidEscape,
    /// A `\u` escape that is not four hex digits or encodes an unpaired surrogate.
    #[error("invalid unicode escape")]
    InvalidUnicodeEscape,
    /// A raw control character inside a string (JSON requires `\u00XX`).
    #[error("unescaped control character")]
    UnescapedControl,
    /// Content after the closing `]`.
    #[error("trailing content")]
    TrailingContent,
    /// The input ended where a token was expected.
    #[error("unexpected end of input")]
    UnexpectedEnd,
}

impl Error {
    /// Stable machine code, surfaced as a `CommandError` param for agents and diagnostics.
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::TooLong => "too_long",
            Self::NotAnArray => "not_an_array",
            Self::UnterminatedString => "unterminated_string",
            Self::InvalidEscape => "invalid_escape",
            Self::InvalidUnicodeEscape => "invalid_unicode_escape",
            Self::UnescapedControl => "unescaped_control",
            Self::TrailingContent => "trailing_content",
            Self::UnexpectedEnd => "unexpected_end",
        }
    }
}

/// Encode choices as `["a","b"]`, escaping what JSON requires.
pub(crate) fn encode(items: &[String]) -> String {
    let mut out = String::with_capacity(2 + items.len().saturating_mul(8));
    out.push('[');
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_string(&mut out, item);
    }
    out.push(']');
    out
}

fn push_string(out: &mut String, text: &str) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            control if control < ' ' => {
                // Writing to a String cannot fail; the result is a two-hex-digit escape either way.
                let _ = write!(out, "\\u{:04x}", u32::from(control));
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

/// Decode a strict `["…","…"]` payload.
pub(crate) fn decode(text: &str) -> Result<Vec<String>, Error> {
    if text.len() > MAX_PAYLOAD_LEN {
        return Err(Error::TooLong);
    }
    let characters: Vec<char> = text.chars().collect();
    let mut cursor = 0usize;

    skip_whitespace(&characters, &mut cursor);
    expect(&characters, &mut cursor, '[')?;
    skip_whitespace(&characters, &mut cursor);

    let mut items = Vec::new();
    if peek(&characters, cursor) == Some(']') {
        advance(&mut cursor);
        return Ok(items);
    }

    loop {
        items.push(parse_string(&characters, &mut cursor)?);
        if items.len() > MAX_DECODED_ITEMS {
            return Err(Error::TooLong);
        }
        skip_whitespace(&characters, &mut cursor);
        match next(&characters, &mut cursor) {
            Some(',') => skip_whitespace(&characters, &mut cursor),
            Some(']') => break,
            Some(_) => return Err(Error::NotAnArray),
            None => return Err(Error::UnexpectedEnd),
        }
    }

    skip_whitespace(&characters, &mut cursor);
    if cursor != characters.len() {
        return Err(Error::TrailingContent);
    }
    Ok(items)
}

fn parse_string(characters: &[char], cursor: &mut usize) -> Result<String, Error> {
    if next(characters, cursor) != Some('"') {
        return Err(Error::NotAnArray);
    }
    let mut out = String::new();
    loop {
        let character = next(characters, cursor).ok_or(Error::UnterminatedString)?;
        match character {
            '"' => return Ok(out),
            '\\' => out.push(parse_escape(characters, cursor)?),
            control if control < ' ' => return Err(Error::UnescapedControl),
            other => out.push(other),
        }
    }
}

fn parse_escape(characters: &[char], cursor: &mut usize) -> Result<char, Error> {
    let escape = next(characters, cursor).ok_or(Error::UnterminatedString)?;
    Ok(match escape {
        '"' => '"',
        '\\' => '\\',
        '/' => '/',
        'b' => '\u{8}',
        'f' => '\u{c}',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'u' => parse_unicode_escape(characters, cursor)?,
        _ => return Err(Error::InvalidEscape),
    })
}

fn parse_unicode_escape(characters: &[char], cursor: &mut usize) -> Result<char, Error> {
    let mut value: u32 = 0;
    for _ in 0..4 {
        let digit = next(characters, cursor).ok_or(Error::InvalidUnicodeEscape)?;
        let nibble = digit.to_digit(16).ok_or(Error::InvalidUnicodeEscape)?;
        value = value.saturating_mul(16).saturating_add(nibble);
    }
    if (0xD800..=0xDFFF).contains(&value) {
        // Surrogates are only valid as pairs; the encoder never emits them, so refusing is exact.
        return Err(Error::InvalidUnicodeEscape);
    }
    char::from_u32(value).ok_or(Error::InvalidUnicodeEscape)
}

fn expect(characters: &[char], cursor: &mut usize, expected: char) -> Result<(), Error> {
    if peek(characters, *cursor) == Some(expected) {
        advance(cursor);
        Ok(())
    } else {
        Err(Error::NotAnArray)
    }
}

fn peek(characters: &[char], cursor: usize) -> Option<char> {
    characters.get(cursor).copied()
}

fn next(characters: &[char], cursor: &mut usize) -> Option<char> {
    let character = peek(characters, *cursor);
    if character.is_some() {
        advance(cursor);
    }
    character
}

fn advance(cursor: &mut usize) {
    *cursor = cursor.saturating_add(1);
}

fn skip_whitespace(characters: &[char], cursor: &mut usize) {
    while matches!(peek(characters, *cursor), Some(' ' | '\t' | '\n' | '\r')) {
        advance(cursor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_then_decode_round_trips() {
        let items = vec!["a".to_string(), "b c".to_string(), "é".to_string()];
        let encoded = encode(&items);
        assert_eq!(encoded, r#"["a","b c","é"]"#);
        assert_eq!(decode(&encoded).expect("round trip"), items);
    }

    #[test]
    fn empty_array_is_valid_but_a_bare_value_is_not() {
        assert_eq!(decode("[]").expect("empty array"), Vec::<String>::new());
        assert_eq!(
            decode(" [ ] ").expect("whitespace is allowed"),
            Vec::<String>::new()
        );
        assert_eq!(decode("").expect_err("no array"), Error::NotAnArray);
        assert_eq!(
            decode("[\"a\"]]").expect_err("trailing"),
            Error::TrailingContent
        );
    }

    #[test]
    fn codes_are_stable_machine_tokens() {
        assert_eq!(Error::TooLong.code(), "too_long");
        assert_eq!(Error::NotAnArray.code(), "not_an_array");
        assert_eq!(Error::UnterminatedString.code(), "unterminated_string");
        assert_eq!(Error::InvalidEscape.code(), "invalid_escape");
        assert_eq!(Error::InvalidUnicodeEscape.code(), "invalid_unicode_escape");
        assert_eq!(Error::UnescapedControl.code(), "unescaped_control");
        assert_eq!(Error::TrailingContent.code(), "trailing_content");
        assert_eq!(Error::UnexpectedEnd.code(), "unexpected_end");
    }

    #[test]
    fn escapes_are_understood_and_bad_ones_refused() {
        assert_eq!(
            decode(r#"["a\"b","c\\d","e\nf","g\u00e9"]"#).expect("escapes"),
            vec!["a\"b", "c\\d", "e\nf", "g\u{e9}"]
        );
        assert_eq!(
            decode(r#"["\x"]"#).expect_err("bad escape"),
            Error::InvalidEscape
        );
        assert_eq!(
            decode(r#"["\u12"]"#).expect_err("short unicode"),
            Error::InvalidUnicodeEscape
        );
        assert_eq!(
            decode("[\"a\u{7}b\"]").expect_err("raw control"),
            Error::UnescapedControl
        );
    }
}
