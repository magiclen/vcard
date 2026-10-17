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

use crate::error::InvalidValueError;

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

/// A URI value backed by [`url::Url`], which handles validation, IDNA and percent encoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri(url::Url);

impl Uri {
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
        Self(url::Url::parse(&s).unwrap())
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
        self.0.as_str()
    }

    /// Returns a reference to the underlying `url::Url`.
    #[inline]
    pub const fn as_url(&self) -> &url::Url {
        &self.0
    }

    /// Consumes this value and returns the underlying `url::Url`.
    #[inline]
    pub fn into_url(self) -> url::Url {
        self.0
    }
}

impl From<url::Url> for Uri {
    #[inline]
    fn from(url: url::Url) -> Self {
        Self(url)
    }
}

impl FromStr for Uri {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::parse(s).map(Self).map_err(|_| InvalidValueError::new("uri"))
    }
}

impl Display for Uri {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
