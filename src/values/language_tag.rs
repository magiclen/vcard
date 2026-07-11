use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::error::InvalidValueError;

/// A language tag value defined by RFC 5646, e.g. `zh-Hant-TW`.
///
/// It is backed by [`language_tags::LanguageTag`] which checks that the tag is well-formed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageTag(language_tags::LanguageTag);

impl LanguageTag {
    /// Returns the language tag as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns a reference to the underlying `language_tags::LanguageTag`.
    #[inline]
    pub const fn as_language_tag(&self) -> &language_tags::LanguageTag {
        &self.0
    }
}

impl From<language_tags::LanguageTag> for LanguageTag {
    #[inline]
    fn from(tag: language_tags::LanguageTag) -> Self {
        Self(tag)
    }
}

impl FromStr for LanguageTag {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        language_tags::LanguageTag::parse(s)
            .map(Self)
            .map_err(|_| InvalidValueError::new("language tag"))
    }
}

impl Display for LanguageTag {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
