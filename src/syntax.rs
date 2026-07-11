//! Low-level helpers for the vCard 4.0 syntax defined in RFC 6350 section 3 and RFC 6868.

use std::fmt::{self, Write};

/// Checks whether the string is a non-empty token made of ASCII letters, digits and hyphens.
/// This matches both the `iana-token` and the `group` rules of RFC 6350.
#[inline]
pub(crate) fn is_token(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
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

/// Splits a compound or list value at every separator that is not escaped by a backslash.
pub(crate) fn split_unescaped(s: &str, separator: u8) -> Vec<&str> {
    debug_assert!(separator.is_ascii() && separator != b'\\');

    let bytes = s.as_bytes();

    let mut parts = Vec::new();

    let mut start = 0;
    let mut escaped = false;

    for (i, b) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
        } else if *b == b'\\' {
            escaped = true;
        } else if *b == separator {
            parts.push(&s[start..i]);
            start = i + 1;
        }
    }

    parts.push(&s[start..]);

    parts
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
            Some('n') | Some('N') => out.push('\n'),
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
