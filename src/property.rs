//! The generic property model and the concrete property types of RFC 6350 and RFC 9554.

use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use crate::{
    error::InvalidValueError,
    fold::FoldingWriter,
    parameters::Parameters,
    syntax::{is_token, split_unescaped, unescape_text, write_escaped_text},
    values::{
        AddressValue, ClientPidMapValue, DateAndOrTime, DateAndOrTimeOrText, EmailValue,
        GenderValue, GramGenderValue, KindValue, LanguageTag, NameValue, OrgValue, TelValue,
        TextOrUri, Timestamp, Token, TzValue, Uri, UtcOffset,
    },
};

/// The group name that can prefix a property, e.g. the `item1` of `item1.TEL:...`.
///
/// RFC 6350 uses it only to mark related properties as belonging together.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupName(String);

impl GroupName {
    /// Returns the group name as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for GroupName {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_token(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(InvalidValueError::new("group name"))
        }
    }
}

impl Display for GroupName {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The behavior that a type must provide to work as the value of a property.
pub trait PropertyValue: Sized {
    /// Writes the value part of the content line, applying any escaping the value type needs.
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result;

    /// Returns the VALUE parameter to write when this value is not the default type of the property.
    #[inline]
    fn explicit_value_type(&self) -> Option<&'static str> {
        None
    }

    /// Parses the raw value text of a content line, guided by the lowercase VALUE parameter if one exists.
    fn parse_value(raw: &str, value_type: Option<&str>) -> Result<Self, InvalidValueError>;
}

/// A single vCard property with an optional group, parameters and a typed value.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Property<V> {
    /// The optional group name that prefixes the property.
    pub group:      Option<GroupName>,
    /// The parameters of the property.
    pub parameters: Parameters,
    /// The value of the property.
    pub value:      V,
}

impl<V> Property<V> {
    /// Creates a property with the given value, no group and no parameters.
    #[inline]
    pub fn new(value: V) -> Self {
        Self {
            group: None,
            parameters: Parameters::new(),
            value,
        }
    }
}

impl<V> From<V> for Property<V> {
    #[inline]
    fn from(value: V) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Property<String> {
    #[inline]
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

/// Writes a whole content line for a property, including the ending CRLF.
pub(crate) fn write_property<V: PropertyValue>(
    w: &mut FoldingWriter,
    name: &str,
    property: &Property<V>,
) -> fmt::Result {
    if let Some(group) = &property.group {
        write!(w, "{group}.")?;
    }

    w.write_str(name)?;

    if let Some(value_type) = property.value.explicit_value_type() {
        write!(w, ";VALUE={value_type}")?;
    }

    property.parameters.write(w)?;

    w.write_char(':')?;

    property.value.write_value(w)?;

    w.end_line()
}

/// A property that this crate has no dedicated field for, which is an x-name or IANA extension property.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionProperty {
    /// The optional group name that prefixes the property.
    pub group:      Option<GroupName>,
    /// The property name, which is an IANA token or an x-name.
    pub name:       Token,
    /// The parameters of the property.
    pub parameters: Parameters,
    /// The raw value in its wire format, kept as it is so that nothing is lost.
    pub value:      String,
}

impl ExtensionProperty {
    /// Creates an extension property whose raw value is already in the wire format.
    #[inline]
    pub fn new(name: Token, value: String) -> Self {
        Self {
            group: None,
            name,
            parameters: Parameters::new(),
            value,
        }
    }

    /// Creates an extension property from plain text, applying the text escaping rules.
    pub fn from_text(name: Token, text: &str) -> Self {
        let mut value = String::with_capacity(text.len());

        // Writing into a string never fails.
        write_escaped_text(&mut value, text, false).unwrap();

        Self::new(name, value)
    }

    /// Writes the whole content line of this property, including the ending CRLF.
    pub(crate) fn write(&self, w: &mut FoldingWriter) -> fmt::Result {
        if let Some(group) = &self.group {
            write!(w, "{group}.")?;
        }

        write!(w, "{}", self.name)?;

        self.parameters.write(w)?;

        w.write_char(':')?;

        w.write_str(&self.value)?;

        w.end_line()
    }
}

impl PropertyValue for String {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        write_escaped_text(w, self, false)
    }

    #[inline]
    fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        Ok(unescape_text(raw))
    }
}

impl PropertyValue for Vec<String> {
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                w.write_char(',')?;
            }

            write_escaped_text(w, item, false)?;
        }

        Ok(())
    }

    fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        if raw.is_empty() {
            return Ok(Vec::new());
        }

        Ok(split_unescaped(raw, b',').into_iter().map(unescape_text).collect())
    }
}

impl PropertyValue for Uri {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        w.write_str(self.as_str())
    }

    #[inline]
    fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        Self::from_str(raw)
    }
}

impl PropertyValue for LanguageTag {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        w.write_str(self.as_str())
    }

    #[inline]
    fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        Self::from_str(raw)
    }
}

impl PropertyValue for Timestamp {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        write!(w, "{self}")
    }

    #[inline]
    fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        Self::from_str(raw)
    }
}

impl PropertyValue for EmailValue {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        write_escaped_text(w, self.as_str(), false)
    }

    #[inline]
    fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        Self::from_str(&unescape_text(raw))
    }
}

/// Implements `PropertyValue` for value types whose `Display` and `FromStr` already use the wire format.
macro_rules! impl_property_value_by_display {
    ($($t:ty),* $(,)?) => {
        $(
            impl PropertyValue for $t {
                #[inline]
                fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
                    write!(w, "{self}")
                }

                #[inline]
                fn parse_value(raw: &str, _value_type: Option<&str>) -> Result<Self, InvalidValueError> {
                    Self::from_str(raw)
                }
            }
        )*
    };
}

impl_property_value_by_display!(
    KindValue,
    NameValue,
    AddressValue,
    GenderValue,
    GramGenderValue,
    OrgValue,
    ClientPidMapValue,
);

impl PropertyValue for DateAndOrTimeOrText {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        match self {
            Self::DateAndOrTime(value) => write!(w, "{value}"),
            Self::Text(text) => write_escaped_text(w, text, false),
        }
    }

    #[inline]
    fn explicit_value_type(&self) -> Option<&'static str> {
        match self {
            Self::DateAndOrTime(_) => None,
            Self::Text(_) => Some("text"),
        }
    }

    #[inline]
    fn parse_value(raw: &str, value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        match value_type {
            Some("text") => Ok(Self::Text(unescape_text(raw))),
            _ => DateAndOrTime::from_str(raw).map(Self::DateAndOrTime),
        }
    }
}

impl PropertyValue for TelValue {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        match self {
            Self::Uri(uri) => w.write_str(uri.as_str()),
            Self::Text(text) => write_escaped_text(w, text, false),
        }
    }

    #[inline]
    fn explicit_value_type(&self) -> Option<&'static str> {
        match self {
            Self::Uri(_) => Some("uri"),
            Self::Text(_) => None,
        }
    }

    #[inline]
    fn parse_value(raw: &str, value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        match value_type {
            Some("uri") => Uri::from_str(raw).map(Self::Uri),
            _ => Ok(Self::Text(unescape_text(raw))),
        }
    }
}

impl PropertyValue for TextOrUri {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        match self {
            Self::Uri(uri) => w.write_str(uri.as_str()),
            Self::Text(text) => write_escaped_text(w, text, false),
        }
    }

    #[inline]
    fn explicit_value_type(&self) -> Option<&'static str> {
        match self {
            Self::Uri(_) => None,
            Self::Text(_) => Some("text"),
        }
    }

    #[inline]
    fn parse_value(raw: &str, value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        match value_type {
            Some("text") => Ok(Self::Text(unescape_text(raw))),
            _ => Uri::from_str(raw).map(Self::Uri),
        }
    }
}

impl PropertyValue for TzValue {
    #[inline]
    fn write_value(&self, w: &mut FoldingWriter) -> fmt::Result {
        match self {
            Self::Text(text) => write_escaped_text(w, text, false),
            Self::Uri(uri) => w.write_str(uri.as_str()),
            Self::UtcOffset(offset) => write!(w, "{offset}"),
        }
    }

    #[inline]
    fn explicit_value_type(&self) -> Option<&'static str> {
        match self {
            Self::Text(_) => None,
            Self::Uri(_) => Some("uri"),
            Self::UtcOffset(_) => Some("utc-offset"),
        }
    }

    #[inline]
    fn parse_value(raw: &str, value_type: Option<&str>) -> Result<Self, InvalidValueError> {
        match value_type {
            Some("uri") => Uri::from_str(raw).map(Self::Uri),
            Some("utc-offset") => UtcOffset::from_str(raw).map(Self::UtcOffset),
            _ => Ok(Self::Text(unescape_text(raw))),
        }
    }
}

/// The SOURCE property, which is a URI where the newest version of this vCard can be fetched.
pub type Source = Property<Uri>;

/// The KIND property, which describes what the vCard represents.
pub type Kind = Property<KindValue>;

/// The XML property, which carries an extended XML-encoded element.
pub type Xml = Property<String>;

/// The FN property, which is the formatted name of the represented entity.
///
/// Every vCard must contain at least one FN property.
pub type FormattedName = Property<String>;

/// The N property, which holds the structured name components.
///
/// This crate stores N properties in a list because RFC 9554 allows several of them when they share the same ALTID, e.g. for phonetic representations.
pub type Name = Property<NameValue>;

/// The NICKNAME property, which holds one or more nicknames.
pub type Nickname = Property<Vec<String>>;

/// The PHOTO property, which is a URI pointing to or embedding an image of the represented entity.
pub type Photo = Property<Uri>;

/// The BDAY property, which is the birth date of the represented entity.
pub type Birthday = Property<DateAndOrTimeOrText>;

/// The ANNIVERSARY property, which is the anniversary date of the represented entity.
pub type Anniversary = Property<DateAndOrTimeOrText>;

/// The GENDER property, which holds the sex and the gender identity components.
pub type Gender = Property<GenderValue>;

/// The ADR property, which holds the structured delivery address components.
pub type Address = Property<AddressValue>;

/// The TEL property, which is a telephone number as a URI or text.
pub type Tel = Property<TelValue>;

/// The EMAIL property, which is an email address.
pub type Email = Property<EmailValue>;

/// The IMPP property, which is a URI for instant messaging or presence.
pub type Impp = Property<Uri>;

/// The LANG property, which is a language that the represented entity can use.
pub type Lang = Property<LanguageTag>;

/// The TZ property, which is the time zone of the represented entity.
pub type TimeZone = Property<TzValue>;

/// The GEO property, which is a URI with the geographic position of the represented entity.
pub type Geo = Property<Uri>;

/// The TITLE property, which is the position or job of the represented entity.
pub type Title = Property<String>;

/// The ROLE property, which is the function of the represented entity.
pub type Role = Property<String>;

/// The LOGO property, which is a URI pointing to or embedding a logo image.
pub type Logo = Property<Uri>;

/// The ORG property, which holds the organization name and unit names.
pub type Org = Property<OrgValue>;

/// The MEMBER property, which is a URI of a member of the group that the vCard represents.
///
/// It can only be used when the KIND property is set to `group`.
pub type Member = Property<Uri>;

/// The RELATED property, which is a URI or text describing a related entity.
pub type Related = Property<TextOrUri>;

/// The CATEGORIES property, which holds tags that can be used for filtering.
pub type Categories = Property<Vec<String>>;

/// The NOTE property, which is a free-form comment.
pub type Note = Property<String>;

/// The PRODID property, which identifies the product that created the vCard.
pub type ProdId = Property<String>;

/// The REV property, which is the timestamp of the last change of the vCard.
pub type Rev = Property<Timestamp>;

/// The SOUND property, which is a URI pointing to or embedding a sound, e.g. the name pronunciation.
pub type Sound = Property<Uri>;

/// The UID property, which is a globally unique identifier of the vCard, usually a `urn:uuid:` URI.
pub type Uid = Property<TextOrUri>;

/// The CLIENTPIDMAP property, which maps PID source numbers to globally unique URIs.
pub type ClientPidMap = Property<ClientPidMapValue>;

/// The URL property, which is a website URI associated with the represented entity.
pub type Url = Property<Uri>;

/// The KEY property, which is a public key or certificate as a URI or text.
pub type Key = Property<TextOrUri>;

/// The FBURL property, which is a URI of the free-busy time information.
pub type Fburl = Property<Uri>;

/// The CALADRURI property, which is a URI for scheduling calendar requests.
pub type CalendarAddressUri = Property<Uri>;

/// The CALURI property, which is a URI of the calendar of the represented entity.
pub type CalendarUri = Property<Uri>;

/// The CREATED property defined by RFC 9554, which is the timestamp when the vCard was created.
pub type Created = Property<Timestamp>;

/// The GRAMGENDER property defined by RFC 9554, which is the grammatical gender to use in salutations.
pub type GramGender = Property<GramGenderValue>;

/// The LANGUAGE property defined by RFC 9554, which is the default language of the text values in the vCard.
pub type Language = Property<LanguageTag>;

/// The PRONOUNS property defined by RFC 9554, which holds the pronouns of the represented entity.
pub type Pronouns = Property<String>;

/// The SOCIALPROFILE property defined by RFC 9554, which is a social media profile as a URI or a username text.
pub type SocialProfile = Property<TextOrUri>;
