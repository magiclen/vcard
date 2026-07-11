//! Value types used by vCard properties, defined in RFC 6350 section 4 and extended by RFC 9554.

mod date_time;
mod language_tag;
mod name;
mod uri;

use std::{
    fmt::{self, Display, Formatter, Write as _},
    str::FromStr,
};

pub use date_time::{Date, DateAndOrTime, DateTime, Time, Timestamp, UtcOffset, Zone};
pub use language_tag::LanguageTag;
pub use name::{AddressValue, NameValue};
pub use uri::Uri;
use validators::prelude::*;

use crate::{
    error::InvalidValueError,
    syntax::{is_token, split_unescaped, unescape_text, write_escaped_text},
};

/// A token made of ASCII letters, digits and hyphens.
///
/// This covers both the `iana-token` and the `x-name` rules of RFC 6350 and is used for extension names and values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(String);

impl Token {
    /// Returns the token as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks whether this token is an `x-name`, which starts with `x-` or `X-`.
    #[inline]
    pub fn is_x_name(&self) -> bool {
        let bytes = self.0.as_bytes();

        bytes.len() > 2 && (bytes[0] == b'x' || bytes[0] == b'X') && bytes[1] == b'-'
    }
}

impl FromStr for Token {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_token(s) { Ok(Self(s.to_string())) } else { Err(InvalidValueError::new("token")) }
    }
}

impl Display for Token {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The value of the KIND property, which describes what the vCard represents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KindValue {
    /// A single person or entity.
    Individual,
    /// A group of persons or entities.
    Group,
    /// An organization.
    Org,
    /// A named geographical place.
    Location,
    /// An extension value which is an IANA token or an x-name.
    Extension(Token),
}

impl Display for KindValue {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Individual => f.write_str("individual"),
            Self::Group => f.write_str("group"),
            Self::Org => f.write_str("org"),
            Self::Location => f.write_str("location"),
            Self::Extension(token) => Display::fmt(token, f),
        }
    }
}

impl FromStr for KindValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Known values are compared case-insensitively as RFC 6350 requires.
        if s.eq_ignore_ascii_case("individual") {
            Ok(Self::Individual)
        } else if s.eq_ignore_ascii_case("group") {
            Ok(Self::Group)
        } else if s.eq_ignore_ascii_case("org") {
            Ok(Self::Org)
        } else if s.eq_ignore_ascii_case("location") {
            Ok(Self::Location)
        } else {
            Token::from_str(s).map(Self::Extension)
        }
    }
}

/// The sex component of the GENDER property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sex {
    /// Male, written as `M`.
    Male,
    /// Female, written as `F`.
    Female,
    /// Other, written as `O`.
    Other,
    /// None or not applicable, written as `N`.
    NoneOrNotApplicable,
    /// Unknown, written as `U`.
    Unknown,
}

impl Display for Sex {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Male => "M",
            Self::Female => "F",
            Self::Other => "O",
            Self::NoneOrNotApplicable => "N",
            Self::Unknown => "U",
        })
    }
}

impl FromStr for Sex {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" | "m" => Ok(Self::Male),
            "F" | "f" => Ok(Self::Female),
            "O" | "o" => Ok(Self::Other),
            "N" | "n" => Ok(Self::NoneOrNotApplicable),
            "U" | "u" => Ok(Self::Unknown),
            _ => Err(InvalidValueError::new("sex")),
        }
    }
}

/// The structured value of the GENDER property.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GenderValue {
    /// The sex component.
    pub sex:      Option<Sex>,
    /// The free-form gender identity component.
    pub identity: Option<String>,
}

impl Display for GenderValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(sex) = self.sex {
            Display::fmt(&sex, f)?;
        }

        if let Some(identity) = &self.identity {
            f.write_char(';')?;

            write_escaped_text(f, identity, true)?;
        }

        Ok(())
    }
}

impl FromStr for GenderValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_unescaped(s, b';');

        let sex = match parts[0] {
            "" => None,
            sex => Some(Sex::from_str(sex)?),
        };

        let identity = match parts.get(1) {
            Some(identity) if !identity.is_empty() => Some(unescape_text(identity)),
            _ => None,
        };

        Ok(Self {
            sex,
            identity,
        })
    }
}

/// The value of the GRAMGENDER property defined by RFC 9554.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GramGenderValue {
    /// The animate grammatical gender.
    Animate,
    /// The common grammatical gender.
    Common,
    /// The feminine grammatical gender.
    Feminine,
    /// The inanimate grammatical gender.
    Inanimate,
    /// The masculine grammatical gender.
    Masculine,
    /// The neuter grammatical gender.
    Neuter,
    /// An extension value which is an IANA token or an x-name.
    Extension(Token),
}

impl Display for GramGenderValue {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Animate => f.write_str("animate"),
            Self::Common => f.write_str("common"),
            Self::Feminine => f.write_str("feminine"),
            Self::Inanimate => f.write_str("inanimate"),
            Self::Masculine => f.write_str("masculine"),
            Self::Neuter => f.write_str("neuter"),
            Self::Extension(token) => Display::fmt(token, f),
        }
    }
}

impl FromStr for GramGenderValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("animate") {
            Ok(Self::Animate)
        } else if s.eq_ignore_ascii_case("common") {
            Ok(Self::Common)
        } else if s.eq_ignore_ascii_case("feminine") {
            Ok(Self::Feminine)
        } else if s.eq_ignore_ascii_case("inanimate") {
            Ok(Self::Inanimate)
        } else if s.eq_ignore_ascii_case("masculine") {
            Ok(Self::Masculine)
        } else if s.eq_ignore_ascii_case("neuter") {
            Ok(Self::Neuter)
        } else {
            Token::from_str(s).map(Self::Extension)
        }
    }
}

/// The structured value of the ORG property, which is an organization name followed by organizational unit names.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OrgValue {
    /// The organization name.
    pub name:  String,
    /// The names of the organizational units, from the largest to the smallest.
    pub units: Vec<String>,
}

impl Display for OrgValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write_escaped_text(f, &self.name, true)?;

        for unit in &self.units {
            f.write_char(';')?;

            write_escaped_text(f, unit, true)?;
        }

        Ok(())
    }
}

impl FromStr for OrgValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = split_unescaped(s, b';');

        Ok(Self {
            name:  unescape_text(parts[0]),
            units: parts[1..].iter().map(|unit| unescape_text(unit)).collect(),
        })
    }
}

/// The structured value of the CLIENTPIDMAP property, which maps a PID source number to a globally unique URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientPidMapValue {
    /// The small integer that PID parameters refer to after their dot.
    pub source_id: u32,
    /// The URI that identifies the product which created the vCard.
    pub uri:       Uri,
}

impl Display for ClientPidMapValue {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{};{}", self.source_id, self.uri)
    }
}

impl FromStr for ClientPidMapValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR: InvalidValueError = InvalidValueError::new("client pid map");

        // The URI part may also contain semicolons, so only the first one splits.
        let (source_id, uri) = s.split_once(';').ok_or(ERROR)?;

        Ok(Self {
            source_id: source_id.parse().map_err(|_| ERROR)?,
            uri:       Uri::from_str(uri).map_err(|_| ERROR)?,
        })
    }
}

/// A validator for email addresses, following the common rules of RFC 5321.
#[derive(Validator)]
#[validator(email(
    comment(Disallow),
    ip(Allow),
    local(Allow),
    at_least_two_labels(Allow),
    non_ascii(Allow)
))]
#[allow(dead_code)]
struct EmailValidator {
    local_part:  String,
    need_quoted: bool,
    domain_part: validators::models::Host,
}

/// The value of the EMAIL property.
///
/// RFC 6350 defines it as free-form text, but this type validates it as an email address for correctness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailValue(String);

impl EmailValue {
    /// Returns the email address as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for EmailValue {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EmailValidator::validate_str(s).map_err(|_| InvalidValueError::new("email address"))?;

        Ok(Self(s.to_string()))
    }
}

impl Display for EmailValue {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A validator for phone numbers in the international format, e.g. `+886912345678`.
#[derive(Validator)]
#[validator(phone)]
#[allow(dead_code)]
struct PhoneValidator(validators::phonenumber::PhoneNumber);

/// The value of the TEL property, which is a `tel:` URI or free-form text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelValue {
    /// A URI value, usually using the `tel:` scheme, written with `VALUE=uri`.
    Uri(Uri),
    /// A free-form text value, which is the default form.
    Text(String),
}

impl TelValue {
    /// Creates a text telephone value after validating the input as a phone number in the international format.
    #[inline]
    pub fn from_phone_number_str(s: &str) -> Result<Self, InvalidValueError> {
        PhoneValidator::validate_str(s).map_err(|_| InvalidValueError::new("phone number"))?;

        Ok(Self::Text(s.to_string()))
    }
}

/// A validator for UUID strings like `550e8400-e29b-41d4-a716-446655440000`.
#[derive(Validator)]
#[validator(uuid(case(Any), separator(Allow(b'-'))))]
#[allow(dead_code)]
struct UuidValidator(u128);

/// A value that is a URI or free-form text, used by the RELATED, UID, KEY and SOCIALPROFILE properties.
///
/// The URI form is the default, and the text form is written with `VALUE=text`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextOrUri {
    /// A URI value, which is the default form.
    Uri(Uri),
    /// A free-form text value, written with `VALUE=text`.
    Text(String),
}

impl TextOrUri {
    /// Creates a `urn:uuid:` URI value from a UUID string, which is useful for the UID property.
    pub fn from_uuid_str(s: &str) -> Result<Self, InvalidValueError> {
        let uuid = UuidValidator::parse_str(s).map_err(|_| InvalidValueError::new("uuid"))?.0;

        let uri = format!(
            "urn:uuid:{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            (uuid >> 96) as u32,
            (uuid >> 80) as u16,
            (uuid >> 64) as u16,
            (uuid >> 48) as u16,
            (uuid & 0xFFFF_FFFF_FFFF) as u64
        );

        // A URN built this way is always a valid URI, so parsing never fails.
        Ok(Self::Uri(Uri::from_str(&uri).unwrap()))
    }
}

/// The value of the TZ property, which is a time zone name, a URI, or a UTC offset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TzValue {
    /// A text value, which is the default form and should be a time zone name from the IANA time zone database.
    Text(String),
    /// A URI value, written with `VALUE=uri`.
    Uri(Uri),
    /// A UTC offset value, written with `VALUE=utc-offset`.
    UtcOffset(UtcOffset),
}

impl TzValue {
    /// Creates a text time zone value from an IANA time zone provided by the `chrono-tz` crate.
    ///
    /// RFC 6350 suggests that the text form should be a name from the IANA time zone database, and this constructor guarantees that.
    #[inline]
    pub fn from_time_zone(tz: chrono_tz::Tz) -> Self {
        Self::Text(tz.name().to_string())
    }
}

/// The value of the BDAY and ANNIVERSARY properties, which is a date-and-or-time or free-form text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateAndOrTimeOrText {
    /// A date, a time, or a date with a time, which is the default form.
    DateAndOrTime(DateAndOrTime),
    /// A free-form text value like `circa 1800`, written with `VALUE=text`.
    Text(String),
}

impl From<DateAndOrTime> for DateAndOrTimeOrText {
    #[inline]
    fn from(value: DateAndOrTime) -> Self {
        Self::DateAndOrTime(value)
    }
}

impl From<Date> for DateAndOrTimeOrText {
    #[inline]
    fn from(value: Date) -> Self {
        Self::DateAndOrTime(DateAndOrTime::Date(value))
    }
}
