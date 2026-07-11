//! The splitter that breaks a logical line into its group, name, parameters and value.

use crate::{error::ParseErrorKind, syntax::is_token};

/// A content line split into its sections, where parameter values still carry their double quotes.
pub(crate) struct RawContentLine<'a> {
    pub(crate) group:  Option<&'a str>,
    pub(crate) name:   &'a str,
    pub(crate) params: Vec<(&'a str, &'a str)>,
    pub(crate) value:  &'a str,
}

/// Splits a logical line following the `contentline` rule of RFC 6350 section 3.3.
pub(crate) fn parse_content_line(line: &str) -> Result<RawContentLine<'_>, ParseErrorKind> {
    let bytes = line.as_bytes();

    // The name section ends at the first semicolon or colon, which cannot be quoted there.
    let mut i = 0;

    while i < bytes.len() && bytes[i] != b';' && bytes[i] != b':' {
        i += 1;
    }

    if i == bytes.len() {
        return Err(ParseErrorKind::InvalidLine);
    }

    let (group, name) = match line[..i].split_once('.') {
        Some((group, name)) => (Some(group), name),
        None => (None, &line[..i]),
    };

    if let Some(group) = group
        && !is_token(group)
    {
        return Err(ParseErrorKind::InvalidGroupName);
    }

    if !is_token(name) {
        return Err(ParseErrorKind::InvalidLine);
    }

    let mut params = Vec::new();

    while bytes[i] == b';' {
        i += 1;

        // A parameter is always a name followed by an equals sign and a value.
        let name_start = i;

        while i < bytes.len() && !matches!(bytes[i], b'=' | b';' | b':' | b'"') {
            i += 1;
        }

        if i == bytes.len() || bytes[i] != b'=' {
            return Err(ParseErrorKind::InvalidLine);
        }

        let param_name = &line[name_start..i];

        if !is_token(param_name) {
            return Err(ParseErrorKind::InvalidLine);
        }

        i += 1;

        // The parameter value ends at the next semicolon or colon outside double quotes.
        let value_start = i;

        let mut in_quotes = false;

        while i < bytes.len() {
            match bytes[i] {
                b'"' => in_quotes = !in_quotes,
                b';' | b':' if !in_quotes => break,
                _ => (),
            }

            i += 1;
        }

        if i == bytes.len() {
            return Err(ParseErrorKind::InvalidLine);
        }

        params.push((param_name, &line[value_start..i]));
    }

    Ok(RawContentLine {
        group,
        name,
        params,
        value: &line[i + 1..],
    })
}
