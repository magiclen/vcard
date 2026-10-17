//! Low-level helpers for the vCard 4.0 syntax defined in RFC 6350 section 3 and RFC 6868.

use std::{
    fmt::{self, Write},
    str::FromStr,
};

/// Checks whether the string is a non-empty token made of ASCII letters, digits and hyphens.
/// This matches both the `iana-token` and the `group` rules of RFC 6350.
#[inline]
pub(crate) fn is_token(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

/// Parses a number written only with ASCII digits, so a sign such as the `+` of `+1` is rejected.
pub(crate) fn parse_number<T: FromStr>(s: &str) -> Option<T> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    s.parse().ok()
}

/// Checks text before escaping, allowing tabs and newlines.
#[inline]
pub(crate) fn is_valid_text(s: &str) -> bool {
    s.bytes().all(|b| !b.is_ascii_control() || matches!(b, b'\t' | b'\r' | b'\n'))
}

/// Checks wire text, allowing tabs but no other ASCII control characters.
#[inline]
pub(crate) fn is_valid_raw_text(s: &str) -> bool {
    s.bytes().all(|b| !b.is_ascii_control() || b == b'\t')
}

/// Writes a text value with the escaping rules of RFC 6350 section 3.4.
/// Backslashes, commas and newlines are always escaped.
/// Semicolons are escaped only when the text is a component of a compound value.
pub(crate) fn write_escaped_text(
    w: &mut impl Write,
    s: &str,
    escape_semicolon: bool,
) -> fmt::Result {
    let bytes = s.as_bytes();

    // The start of the pending chunk which contains no byte that needs escaping.
    let mut start = 0;

    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        let replacement = match b {
            b'\\' => "\\\\",
            b',' => "\\,",
            b';' if escape_semicolon => "\\;",
            b'\r' | b'\n' => "\\n",
            _ => {
                i += 1;
                continue;
            },
        };

        // The special bytes above are all ASCII, so slicing here always cuts at char boundaries.
        w.write_str(&s[start..i])?;
        w.write_str(replacement)?;

        // Treat a CRLF pair as a single newline.
        if b == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
            i += 1;
        }

        i += 1;
        start = i;
    }

    w.write_str(&s[start..])
}

/// Reverses the escaping rules of RFC 6350 section 3.4.
/// Both `\n` and `\N` become a newline, and an unknown escape sequence is kept as it is.
pub(crate) fn unescape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }

        match chars.next() {
            Some('\\') => out.push('\\'),
            Some(',') => out.push(','),
            Some(';') => out.push(';'),
            Some('n') | Some('N') => out.push('\n'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            },
            None => out.push('\\'),
        }
    }

    out
}

/// Lazily splits a compound or list value at every separator that is not escaped by a backslash.
pub(crate) fn split_unescaped(s: &str, separator: u8) -> impl Iterator<Item = &str> {
    debug_assert!(separator.is_ascii() && separator != b'\\');

    let mut rest = Some(s);

    std::iter::from_fn(move || {
        let s = rest?;
        let mut escaped = false;

        for (i, b) in s.bytes().enumerate() {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == separator {
                rest = Some(&s[i + 1..]);

                return Some(&s[..i]);
            }
        }

        rest = None;

        Some(s)
    })
}

/// Writes a parameter value with the caret escaping of RFC 6868.
/// Carets, double quotes and newlines are encoded so that any text can be carried safely.
pub(crate) fn write_caret_encoded(w: &mut impl Write, s: &str) -> fmt::Result {
    let bytes = s.as_bytes();

    let mut start = 0;

    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        let replacement = match b {
            b'^' => "^^",
            b'"' => "^'",
            b'\r' | b'\n' => "^n",
            _ => {
                i += 1;
                continue;
            },
        };

        w.write_str(&s[start..i])?;
        w.write_str(replacement)?;

        if b == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
            i += 1;
        }

        i += 1;
        start = i;
    }

    w.write_str(&s[start..])
}

/// Reverses the caret escaping of RFC 6868.
/// A caret followed by an unknown character is kept unchanged, as the RFC requires.
pub(crate) fn caret_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c != '^' {
            out.push(c);
            continue;
        }

        match chars.next() {
            Some('^') => out.push('^'),
            Some('\'') => out.push('"'),
            Some('n') => out.push('\n'),
            Some(other) => {
                out.push('^');
                out.push(other);
            },
            None => out.push('^'),
        }
    }

    out
}

/// Checks whether a caret-encoded parameter value must be wrapped in double quotes.
/// Quotes are needed when the value contains a character outside the `SAFE-CHAR` rule.
#[inline]
pub(crate) fn param_value_needs_quoting(s: &str) -> bool {
    s.bytes().any(|b| matches!(b, b',' | b';' | b':'))
}

/// Writes a parameter value, applying caret encoding and double quotes when necessary.
pub(crate) fn write_param_value(w: &mut impl Write, s: &str) -> fmt::Result {
    if param_value_needs_quoting(s) {
        w.write_char('"')?;
        write_caret_encoded(w, s)?;
        w.write_char('"')
    } else {
        write_caret_encoded(w, s)
    }
}

/// Writes a URI parameter value inside double quotes.
/// A double quote cannot appear inside a quoted parameter value, so it is percent-encoded as `%22`.
pub(crate) fn write_quoted_uri(w: &mut impl Write, uri: &str) -> fmt::Result {
    w.write_char('"')?;

    for (i, part) in uri.split('"').enumerate() {
        if i > 0 {
            w.write_str("%22")?;
        }

        w.write_str(part)?;
    }

    w.write_char('"')
}
