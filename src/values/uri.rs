use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    io,
    path::Path,
    str::FromStr,
};

use base64::Engine;
use mime::Mime;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

use crate::{error::InvalidValueError, syntax::write_escaped_text};

/// The bytes that need URL escaping in the media type of a `data:` URI.
/// Letters, digits and the URL characters without a special meaning there are kept as they are.
const MIME_COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'*')
    .remove(b'+')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// Writes a MIME component using the URL escaping required by RFC 2397.
#[inline]
fn write_mime_component(out: &mut String, value: &str) {
    out.extend(utf8_percent_encode(value, MIME_COMPONENT));
}

/// Removes the backslashes of quoted pairs, e.g. `a\,b` becomes `a,b`.
fn unescape_quoted_pairs(value: &str) -> Cow<'_, str> {
    if !value.contains('\\') {
        return Cow::Borrowed(value);
    }

    let mut out = String::with_capacity(value.len());

    let mut chars = value.chars();

    while let Some(c) = chars.next() {
        // A trailing backslash has nothing to escape, so it is kept.
        out.push(if c == '\\' { chars.next().unwrap_or(c) } else { c });
    }

    Cow::Owned(out)
}

/// Decodes the `\\`, `\,` and `\;` escapes of a vCard URI value, keeping other backslashes unchanged.
fn unescape_uri_value(value: &str) -> Cow<'_, str> {
    if !value.contains('\\') {
        return Cow::Borrowed(value);
    }

    let mut out = String::with_capacity(value.len());

    let mut chars = value.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }

        match chars.next() {
            Some(escaped @ ('\\' | ',' | ';')) => out.push(escaped),
            Some(other) => {
                out.push('\\');
                out.push(other);
            },
            None => out.push('\\'),
        }
    }

    Cow::Owned(out)
}

/// A URI value supporting RFC 3986 and the URL parsing behavior of [`url::Url`].
/// URLs keep their usual normalization, while other valid URIs keep their original spelling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri(UriValue);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum UriValue {
    Url(url::Url),
    Rfc3986(String),
}

impl Uri {
    /// Parses a vCard URI value, decoding the escapes of RFC 6350 section 3.4 that errata 3845 and 3846 apply to URI values.
    pub(crate) fn from_vcard_value(raw: &str) -> Result<Self, InvalidValueError> {
        Self::from_str(&unescape_uri_value(raw))
    }

    /// Writes this URI as a vCard property value, escaping backslashes and commas.
    pub(crate) fn write_vcard_value(&self, w: &mut impl fmt::Write) -> fmt::Result {
        write_escaped_text(w, self.as_str(), false)
    }

    /// Builds a `data:` URI which embeds the given bytes with base64 encoding.
    ///
    /// Parameter values of the media type are URL-escaped instead of quoted, as RFC 2397 suggests.
    pub fn from_data(mime: &Mime, data: &[u8]) -> Self {
        let mut s = String::with_capacity(13 + mime.as_ref().len() + data.len().div_ceil(3) * 4);

        s.push_str("data:");
        write_mime_component(&mut s, mime.type_().as_str());
        s.push('/');
        write_mime_component(&mut s, mime.subtype().as_str());

        if let Some(suffix) = mime.suffix() {
            s.push('+');
            write_mime_component(&mut s, suffix.as_str());
        }

        for (name, value) in mime.params() {
            s.push(';');
            write_mime_component(&mut s, name.as_str());
            s.push('=');

            // The MIME parser removes the quotes of a quoted value but keeps its quoted-pair backslashes.
            write_mime_component(&mut s, &unescape_quoted_pairs(value.as_str()));
        }

        s.push_str(";base64,");

        base64::engine::general_purpose::STANDARD.encode_string(data, &mut s);

        // A data URI built this way is always valid, so parsing never fails.
        Self::from(url::Url::parse(&s).unwrap())
    }

    /// Reads a file and builds a `data:` URI, guessing the media type from the file extension.
    ///
    /// This is a convenient way to embed a photo, a logo or a sound into a vCard.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let path = path.as_ref();

        let mime = mime_guess::from_path(path).first_or_octet_stream();

        let data = std::fs::read(path)?;

        Ok(Self::from_data(&mime, &data))
    }

    /// Returns the URI as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            UriValue::Url(url) => url.as_str(),
            UriValue::Rfc3986(uri) => uri,
        }
    }

    /// Returns the underlying URL when this URI is supported by `url::Url`.
    #[inline]
    pub const fn as_url(&self) -> Option<&url::Url> {
        match &self.0 {
            UriValue::Url(url) => Some(url),
            UriValue::Rfc3986(_) => None,
        }
    }

    /// Consumes this URI and returns its URL, failing when `url::Url` does not support it.
    #[inline]
    pub fn into_url(self) -> Result<url::Url, InvalidValueError> {
        match self.0 {
            UriValue::Url(url) => Ok(url),
            UriValue::Rfc3986(_) => Err(InvalidValueError::new("WHATWG URL")),
        }
    }
}

impl From<url::Url> for Uri {
    #[inline]
    fn from(url: url::Url) -> Self {
        Self(UriValue::Url(url))
    }
}

impl FromStr for Uri {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = url::Url::parse(s) {
            return Ok(Self::from(url));
        }

        fluent_uri::Uri::parse(s).map_err(|_| InvalidValueError::new("uri"))?;

        Ok(Self(UriValue::Rfc3986(s.to_string())))
    }
}

impl Display for Uri {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
