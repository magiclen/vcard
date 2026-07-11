use std::{
    fmt::{self, Display, Formatter},
    io,
    path::Path,
    str::FromStr,
};

use base64::Engine;
use mime::Mime;

use crate::error::InvalidValueError;

/// A URI value backed by [`url::Url`], which handles validation, IDNA and percent encoding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri(url::Url);

impl Uri {
    /// Builds a `data:` URI which embeds the given bytes with base64 encoding.
    pub fn from_data(mime: &Mime, data: &[u8]) -> Self {
        let mut s = String::with_capacity(13 + mime.as_ref().len() + data.len().div_ceil(3) * 4);

        s.push_str("data:");
        s.push_str(mime.as_ref());
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
