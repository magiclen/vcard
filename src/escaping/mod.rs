use std::borrow::Cow;
use regex::Regex;

lazy_static! {
    static ref NEW_LINE_RE: Regex = { Regex::new(r"\n").unwrap() };
    static ref COMMA_RE: Regex = { Regex::new(r",").unwrap() };
    static ref COLON_RE: Regex = { Regex::new(r":").unwrap() };
    static ref SEMICOLON_RE: Regex = { Regex::new(r";").unwrap() };
    static ref BACKSLASH_RE: Regex = { Regex::new("\\").unwrap() };
    static ref DOUBLE_QUOTE_RE: Regex = { Regex::new("\"").unwrap() };
}

pub fn escape_new_line(s: &str) -> Cow<'_, str> {
    NEW_LINE_RE.replace_all(s, "\\n")
}

pub fn escape_comma(s: &str) -> Cow<'_, str> {
    COMMA_RE.replace_all(s, "\\,")
}

pub fn escape_colon(s: &str) -> Cow<'_, str> {
    COLON_RE.replace_all(s, "\\:")
}

pub fn escape_semicolon(s: &str) -> Cow<'_, str> {
    SEMICOLON_RE.replace_all(s, "\\;")
}

pub fn escape_backslash(s: &str) -> Cow<'_, str> {
    BACKSLASH_RE.replace_all(s, "\\\\")
}

pub fn escape_double_quote(s: &str) -> Cow<'_, str> {
    DOUBLE_QUOTE_RE.replace_all(s, "\\\"")
}