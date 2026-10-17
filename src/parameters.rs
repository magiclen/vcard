//! Property parameters defined in RFC 6350 section 5 and RFC 9554 section 4.

use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use mime::Mime;

use crate::{
    error::{InvalidValueError, ParseErrorKind},
    fold::FoldingWriter,
    syntax::{
        caret_decode, is_token, param_value_needs_quoting, parse_number, write_param_value,
        write_quoted_uri,
    },
    values::{LanguageTag, Timestamp, Token, Uri},
};

/// The value of the PREF parameter, an integer between 1 and 100 where lower means more preferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pref(u8);

impl Pref {
    /// Creates a preference value, which must be between 1 and 100.
    pub const fn new(value: u8) -> Result<Self, InvalidValueError> {
        if value >= 1 && value <= 100 {
            Ok(Self(value))
        } else {
            Err(InvalidValueError::new("preference"))
        }
    }

    /// Returns the preference as a number.
    #[inline]
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl FromStr for Pref {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_number(s).ok_or(InvalidValueError::new("preference")).and_then(Self::new)
    }
}

impl Display for Pref {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// The value of the PID parameter, e.g. `1` or `3.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pid {
    /// The number that identifies the property instance.
    pub id:     u32,
    /// The optional number after the dot, which refers to a CLIENTPIDMAP source.
    pub source: Option<u32>,
}

impl Display for Pid {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.id, f)?;

        if let Some(source) = self.source {
            write!(f, ".{source}")?;
        }

        Ok(())
    }
}

impl FromStr for Pid {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR: InvalidValueError = InvalidValueError::new("pid");

        let (id, source) = match s.split_once('.') {
            Some((id, source)) => (id, Some(parse_number(source).ok_or(ERROR)?)),
            None => (s, None),
        };

        Ok(Self {
            id: parse_number(id).ok_or(ERROR)?,
            source,
        })
    }
}

/// A value of the TYPE parameter.
///
/// This single type covers the general values, the TEL-specific values, the RELATED relation values and the RFC 9554 values.
/// [`VCard::validate`](crate::VCard::validate) rejects the TEL-specific values outside TEL and the RELATED values outside RELATED.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TypeValue {
    /// Related to the individual's work.
    Work,
    /// Related to the individual's personal life.
    Home,
    /// A telephone number for voice calls.
    Voice,
    /// A telephone number for faxes.
    Fax,
    /// A telephone number of a mobile phone.
    Cell,
    /// A telephone number for video calls.
    Video,
    /// A telephone number of a pager.
    Pager,
    /// A telephone number of a device for people with hearing or speech difficulties.
    Textphone,
    /// A telephone number that supports text messages.
    Text,
    /// A contact, someone the individual knows how to get in touch with, from the RELATED relation values.
    Contact,
    /// An acquaintance, from the RELATED relation values.
    Acquaintance,
    /// A friend, from the RELATED relation values.
    Friend,
    /// Someone the individual has met, from the RELATED relation values.
    Met,
    /// A co-worker, from the RELATED relation values.
    CoWorker,
    /// A colleague, from the RELATED relation values.
    Colleague,
    /// A co-resident, from the RELATED relation values.
    CoResident,
    /// A neighbor, from the RELATED relation values.
    Neighbor,
    /// A child, from the RELATED relation values.
    Child,
    /// A parent, from the RELATED relation values.
    Parent,
    /// A sibling, from the RELATED relation values.
    Sibling,
    /// A spouse, from the RELATED relation values.
    Spouse,
    /// A family member, from the RELATED relation values.
    Kin,
    /// A muse, from the RELATED relation values.
    Muse,
    /// A crush, from the RELATED relation values.
    Crush,
    /// Someone the individual is dating, from the RELATED relation values.
    Date,
    /// A sweetheart, from the RELATED relation values.
    Sweetheart,
    /// The individual itself, from the RELATED relation values.
    Me,
    /// An agent of the individual, from the RELATED relation values.
    Agent,
    /// An emergency contact, from the RELATED relation values.
    Emergency,
    /// An address used for billing, added by RFC 9554.
    Billing,
    /// An address used for delivery, added by RFC 9554.
    Delivery,
    /// An extension value which is an IANA token or an x-name.
    /// Known values must use their dedicated variants, which [`VCard::validate`](crate::VCard::validate) checks.
    Extension(Token),
}

impl TypeValue {
    /// Returns the only property that can use this value, as the TEL and RELATED rules of RFC 6350 require.
    pub(crate) const fn restricted_property(&self) -> Option<&'static str> {
        match self {
            Self::Voice
            | Self::Fax
            | Self::Cell
            | Self::Video
            | Self::Pager
            | Self::Textphone
            | Self::Text => Some("TEL"),
            Self::Contact
            | Self::Acquaintance
            | Self::Friend
            | Self::Met
            | Self::CoWorker
            | Self::Colleague
            | Self::CoResident
            | Self::Neighbor
            | Self::Child
            | Self::Parent
            | Self::Sibling
            | Self::Spouse
            | Self::Kin
            | Self::Muse
            | Self::Crush
            | Self::Date
            | Self::Sweetheart
            | Self::Me
            | Self::Agent
            | Self::Emergency => Some("RELATED"),
            Self::Work | Self::Home | Self::Billing | Self::Delivery | Self::Extension(_) => None,
        }
    }

    /// Checks whether this is an extension value that spells a known value, which parsing never produces.
    pub(crate) fn is_known_extension(&self) -> bool {
        match self {
            Self::Extension(token) => {
                !matches!(Self::from_str(token.as_str()), Ok(Self::Extension(_)))
            },
            _ => false,
        }
    }
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Work => "work",
            Self::Home => "home",
            Self::Voice => "voice",
            Self::Fax => "fax",
            Self::Cell => "cell",
            Self::Video => "video",
            Self::Pager => "pager",
            Self::Textphone => "textphone",
            Self::Text => "text",
            Self::Contact => "contact",
            Self::Acquaintance => "acquaintance",
            Self::Friend => "friend",
            Self::Met => "met",
            Self::CoWorker => "co-worker",
            Self::Colleague => "colleague",
            Self::CoResident => "co-resident",
            Self::Neighbor => "neighbor",
            Self::Child => "child",
            Self::Parent => "parent",
            Self::Sibling => "sibling",
            Self::Spouse => "spouse",
            Self::Kin => "kin",
            Self::Muse => "muse",
            Self::Crush => "crush",
            Self::Date => "date",
            Self::Sweetheart => "sweetheart",
            Self::Me => "me",
            Self::Agent => "agent",
            Self::Emergency => "emergency",
            Self::Billing => "billing",
            Self::Delivery => "delivery",
            Self::Extension(token) => return Display::fmt(token, f),
        })
    }
}

impl FromStr for TypeValue {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "work" => Self::Work,
            "home" => Self::Home,
            "voice" => Self::Voice,
            "fax" => Self::Fax,
            "cell" => Self::Cell,
            "video" => Self::Video,
            "pager" => Self::Pager,
            "textphone" => Self::Textphone,
            "text" => Self::Text,
            "contact" => Self::Contact,
            "acquaintance" => Self::Acquaintance,
            "friend" => Self::Friend,
            "met" => Self::Met,
            "co-worker" => Self::CoWorker,
            "colleague" => Self::Colleague,
            "co-resident" => Self::CoResident,
            "neighbor" => Self::Neighbor,
            "child" => Self::Child,
            "parent" => Self::Parent,
            "sibling" => Self::Sibling,
            "spouse" => Self::Spouse,
            "kin" => Self::Kin,
            "muse" => Self::Muse,
            "crush" => Self::Crush,
            "date" => Self::Date,
            "sweetheart" => Self::Sweetheart,
            "me" => Self::Me,
            "agent" => Self::Agent,
            "emergency" => Self::Emergency,
            "billing" => Self::Billing,
            "delivery" => Self::Delivery,
            _ => Self::Extension(Token::from_str(s)?),
        })
    }
}

/// The value of the CALSCALE parameter, which describes the calendar system of a date.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Calscale {
    /// The Gregorian calendar, which is the only calendar system defined by RFC 6350.
    Gregorian,
    /// An extension value which is an IANA token or an x-name.
    /// Known values must use their dedicated variants, which [`VCard::validate`](crate::VCard::validate) checks.
    Extension(Token),
}

impl Calscale {
    /// Returns the calendar name as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Gregorian => "gregorian",
            Self::Extension(token) => token.as_str(),
        }
    }

    /// Checks whether this is an extension value that spells `gregorian`, which parsing never produces.
    #[inline]
    pub(crate) fn is_known_extension(&self) -> bool {
        matches!(self, Self::Extension(token) if token.as_str().eq_ignore_ascii_case("gregorian"))
    }
}

impl Display for Calscale {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Calscale {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("gregorian") {
            Ok(Self::Gregorian)
        } else {
            Token::from_str(s).map(Self::Extension)
        }
    }
}

/// The value of the PHONETIC parameter defined by RFC 9554, which names a phonetic system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Phonetic {
    /// The International Phonetic Alphabet.
    Ipa,
    /// The Cantonese romanization system Jyutping.
    Jyut,
    /// The Mandarin romanization system Hanyu Pinyin.
    Piny,
    /// An unknown phonetic system, where the SCRIPT parameter tells the used script.
    Script,
    /// An extension value which is an IANA token or an x-name.
    /// Known values must use their dedicated variants, which [`VCard::validate`](crate::VCard::validate) checks.
    Extension(Token),
}

impl Phonetic {
    /// Checks whether this is an extension value that spells a known value, which parsing never produces.
    pub(crate) fn is_known_extension(&self) -> bool {
        match self {
            Self::Extension(token) => {
                !matches!(Self::from_str(token.as_str()), Ok(Self::Extension(_)))
            },
            _ => false,
        }
    }
}

impl Display for Phonetic {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Ipa => f.write_str("ipa"),
            Self::Jyut => f.write_str("jyut"),
            Self::Piny => f.write_str("piny"),
            Self::Script => f.write_str("script"),
            Self::Extension(token) => Display::fmt(token, f),
        }
    }
}

impl FromStr for Phonetic {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("ipa") {
            Ok(Self::Ipa)
        } else if s.eq_ignore_ascii_case("jyut") {
            Ok(Self::Jyut)
        } else if s.eq_ignore_ascii_case("piny") {
            Ok(Self::Piny)
        } else if s.eq_ignore_ascii_case("script") {
            Ok(Self::Script)
        } else {
            Token::from_str(s).map(Self::Extension)
        }
    }
}

/// The value of the PROP-ID parameter defined by RFC 9554, which identifies a property among its siblings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropId(String);

impl PropId {
    /// Returns the identifier as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for PropId {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The identifier allows 1 to 255 characters of ASCII letters, digits, hyphens and underscores.
        if (1..=255).contains(&s.len())
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            Ok(Self(s.to_string()))
        } else {
            Err(InvalidValueError::new("property identifier"))
        }
    }
}

impl Display for PropId {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The value of the SCRIPT parameter defined by RFC 9554, which is a script subtag like `Latn`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Script(String);

impl Script {
    /// Returns the script subtag as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Script {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A script subtag is exactly four ASCII letters, as RFC 5646 section 2.2.3 defines.
        if s.len() == 4 && s.bytes().all(|b| b.is_ascii_alphabetic()) {
            Ok(Self(s.to_string()))
        } else {
            Err(InvalidValueError::new("script subtag"))
        }
    }
}

impl Display for Script {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The value of the TZ parameter, which is a time zone text or a URI.
///
/// When parsing, a quoted value that is a valid URI becomes [`TzParam::Uri`], and any other value becomes [`TzParam::Text`].
/// A text value that needs quotes, e.g. `GMT+08:00`, can therefore be read back as a URI.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TzParam {
    /// A time zone text, which should be a name from the IANA time zone database.
    Text(String),
    /// A URI, written inside double quotes.
    Uri(Uri),
}

/// An extension parameter that this crate has no dedicated field for.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyParameter {
    /// The parameter name, which is an IANA token or an x-name.
    pub name:   Token,
    /// The parameter values.
    pub values: Vec<String>,
}

/// The parameters of a property.
///
/// Every property shares this struct, and [`VCard::validate`](crate::VCard::validate) checks the restrictions that depend on the property.
/// A parameter that is `None` or an empty list is simply not written.
/// When parsing, a repeated single-value parameter keeps its last value, unless RFC 9554 allows it only once for the property.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Parameters {
    /// The LANGUAGE parameter, which tells the language of the property value.
    pub language:     Option<LanguageTag>,
    /// The PREF parameter, which tells how preferred the property is among its siblings.
    pub pref:         Option<Pref>,
    /// The ALTID parameter, which groups properties that are different representations of the same data.
    pub altid:        Option<String>,
    /// The PID values, which identify the property instance for synchronization.
    pub pids:         Vec<Pid>,
    /// The TYPE values, which describe the context of the property.
    pub types:        Vec<TypeValue>,
    /// The MEDIATYPE parameter, which hints the media type of the resource behind a URI value.
    /// Quoted MIME parameter values are rejected by [`VCard::validate`](crate::VCard::validate) because their wire quoting is ambiguous.
    pub media_type:   Option<Mime>,
    /// The CALSCALE parameter, which tells the calendar system of a date value.
    pub calscale:     Option<Calscale>,
    /// The SORT-AS values, which give the strings that sorting should use.
    pub sort_as:      Vec<String>,
    /// The GEO parameter, which attaches geographic coordinates to an address.
    /// The URI is written inside double quotes, so a double quote in it is written as `%22` and stays percent-encoded when read back.
    pub geo:          Option<Uri>,
    /// The TZ parameter, which attaches time zone information to an address.
    /// A URI value is written inside double quotes, so a double quote in it is written as `%22` and stays percent-encoded when read back.
    pub tz:           Option<TzParam>,
    /// The LABEL parameter, which carries the formatted text of an address.
    pub label:        Option<String>,
    /// The AUTHOR parameter defined by RFC 9554, which is a URI that identifies the author of the value.
    /// The URI is written inside double quotes, so a double quote in it is written as `%22` and stays percent-encoded when read back.
    pub author:       Option<Uri>,
    /// The AUTHOR-NAME parameter defined by RFC 9554, which names the author of the value.
    pub author_name:  Option<String>,
    /// The CREATED parameter defined by RFC 9554, which tells when the property was created.
    pub created:      Option<Timestamp>,
    /// The DERIVED parameter defined by RFC 9554, which marks a value derived from other properties.
    pub derived:      Option<bool>,
    /// The PHONETIC parameter defined by RFC 9554, which marks the value as the pronunciation of another property.
    pub phonetic:     Option<Phonetic>,
    /// The PROP-ID parameter defined by RFC 9554, which identifies the property among its siblings.
    pub prop_id:      Option<PropId>,
    /// The SCRIPT parameter defined by RFC 9554, which tells the script of the value.
    pub script:       Option<Script>,
    /// The SERVICE-TYPE parameter defined by RFC 9554, which names the online service of an IMPP or SOCIALPROFILE property.
    pub service_type: Option<String>,
    /// The USERNAME parameter defined by RFC 9554, which carries the user name of an IMPP or SOCIALPROFILE property.
    pub username:     Option<String>,
    /// Any other parameters that this crate has no dedicated field for.
    /// Use the dedicated fields for known parameters where available.
    pub any:          Vec<AnyParameter>,
}

impl Parameters {
    /// Creates an empty parameter set.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Writes all parameters, where every parameter starts with a semicolon.
    pub(crate) fn write(&self, w: &mut FoldingWriter) -> fmt::Result {
        if let Some(language) = &self.language {
            write!(w, ";LANGUAGE={language}")?;
        }

        if let Some(pref) = &self.pref {
            write!(w, ";PREF={pref}")?;
        }

        if let Some(altid) = &self.altid {
            w.write_str(";ALTID=")?;
            write_param_value(w, altid)?;
        }

        if !self.pids.is_empty() {
            w.write_str(";PID=")?;

            for (i, pid) in self.pids.iter().enumerate() {
                if i > 0 {
                    w.write_char(',')?;
                }

                write!(w, "{pid}")?;
            }
        }

        if !self.types.is_empty() {
            w.write_str(";TYPE=")?;

            for (i, type_value) in self.types.iter().enumerate() {
                if i > 0 {
                    w.write_char(',')?;
                }

                write!(w, "{type_value}")?;
            }
        }

        if let Some(media_type) = &self.media_type {
            w.write_str(";MEDIATYPE=")?;

            // MEDIATYPE uses MIME syntax, so carets are literal characters.
            if param_value_needs_quoting(media_type.as_ref()) {
                write!(w, "\"{media_type}\"")?;
            } else {
                write!(w, "{media_type}")?;
            }
        }

        if let Some(calscale) = &self.calscale {
            write!(w, ";CALSCALE={calscale}")?;
        }

        if !self.sort_as.is_empty() {
            w.write_str(";SORT-AS=")?;

            for (i, sort_as) in self.sort_as.iter().enumerate() {
                if i > 0 {
                    w.write_char(',')?;
                }

                write_param_value(w, sort_as)?;
            }
        }

        if let Some(geo) = &self.geo {
            // The GEO parameter is always written inside double quotes, as its ABNF requires.
            w.write_str(";GEO=")?;
            write_quoted_uri(w, geo.as_str())?;
        }

        if let Some(tz) = &self.tz {
            match tz {
                TzParam::Text(text) => {
                    w.write_str(";TZ=")?;
                    write_param_value(w, text)?;
                },
                TzParam::Uri(uri) => {
                    w.write_str(";TZ=")?;
                    write_quoted_uri(w, uri.as_str())?;
                },
            }
        }

        if let Some(label) = &self.label {
            w.write_str(";LABEL=")?;
            write_param_value(w, label)?;
        }

        if let Some(author) = &self.author {
            // The AUTHOR parameter is always written inside double quotes, as its ABNF requires.
            w.write_str(";AUTHOR=")?;
            write_quoted_uri(w, author.as_str())?;
        }

        if let Some(author_name) = &self.author_name {
            w.write_str(";AUTHOR-NAME=")?;
            write_param_value(w, author_name)?;
        }

        if let Some(created) = &self.created {
            write!(w, ";CREATED={created}")?;
        }

        if let Some(derived) = self.derived {
            write!(w, ";DERIVED={}", if derived { "true" } else { "false" })?;
        }

        if let Some(phonetic) = &self.phonetic {
            write!(w, ";PHONETIC={phonetic}")?;
        }

        if let Some(prop_id) = &self.prop_id {
            write!(w, ";PROP-ID={prop_id}")?;
        }

        if let Some(script) = &self.script {
            write!(w, ";SCRIPT={script}")?;
        }

        if let Some(service_type) = &self.service_type {
            w.write_str(";SERVICE-TYPE=")?;
            write_param_value(w, service_type)?;
        }

        if let Some(username) = &self.username {
            w.write_str(";USERNAME=")?;
            write_param_value(w, username)?;
        }

        for any in &self.any {
            if any.values.is_empty() {
                continue;
            }

            write!(w, ";{}=", any.name)?;

            for (i, value) in any.values.iter().enumerate() {
                if i > 0 {
                    w.write_char(',')?;
                }

                write_param_value(w, value)?;
            }
        }

        Ok(())
    }

    /// Builds a parameter set from raw name and value pairs coming from the parser.
    ///
    /// The VALUE parameter is returned separately in lowercase because it selects how the property value is parsed.
    /// The flag tells whether CALSCALE appears again with a different value, which makes the calendar of a date ambiguous.
    /// It rejects parameters that RFC 9554 allows only once for the property, where `property_name` is uppercase.
    pub(crate) fn parse(
        raw_params: Vec<(&str, &str)>,
        property_name: &str,
    ) -> Result<(Self, Option<String>, bool), ParseErrorKind> {
        let mut parameters = Self::default();
        let mut value_type = None;
        let mut conflicting_calscale = false;
        let unique_parameters: &[&str] = match property_name {
            "GRAMGENDER" => &["LANGUAGE"],
            "PRONOUNS" => &["LANGUAGE", "PREF", "TYPE", "ALTID"],
            "SOCIALPROFILE" => &["SERVICE-TYPE"],
            _ => &[],
        };
        let mut seen = 0u8;

        for (name, raw) in raw_params {
            let invalid = || ParseErrorKind::InvalidParameter(name.to_string());

            let upper_name = name.to_ascii_uppercase();

            if let Some(index) = unique_parameters.iter().position(|name| *name == upper_name) {
                let flag = 1 << index;

                if seen & flag != 0 {
                    return Err(invalid());
                }

                seen |= flag;
            }

            match upper_name.as_str() {
                "LANGUAGE" => {
                    parameters.language = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "VALUE" => {
                    if value_type.is_some() {
                        return Err(invalid());
                    }

                    let value = single_value(raw);

                    if !is_token(&value) {
                        return Err(invalid());
                    }

                    value_type = Some(value.to_ascii_lowercase());
                },
                "PREF" => parameters.pref = Some(single_value(raw).parse().map_err(|_| invalid())?),
                "ALTID" => parameters.altid = Some(single_value(raw)),
                "PID" => {
                    for item in list_values(raw, true) {
                        parameters.pids.push(item.parse().map_err(|_| invalid())?);
                    }
                },
                "TYPE" => {
                    for item in list_values(raw, true) {
                        parameters.types.push(item.parse().map_err(|_| invalid())?);
                    }
                },
                "MEDIATYPE" => {
                    parameters.media_type = Some(unquote(raw).parse().map_err(|_| invalid())?);
                },
                "CALSCALE" => {
                    let calscale: Calscale = single_value(raw).parse().map_err(|_| invalid())?;

                    conflicting_calscale |= parameters.calscale.as_ref().is_some_and(|earlier| {
                        !earlier.as_str().eq_ignore_ascii_case(calscale.as_str())
                    });

                    parameters.calscale = Some(calscale);
                },
                "SORT-AS" => parameters.sort_as.extend(list_values(raw, false)),
                "GEO" => {
                    parameters.geo = Some(unquote(raw).parse().map_err(|_| invalid())?);
                },
                "TZ" => {
                    parameters.tz = Some(
                        if raw.starts_with('"')
                            && let Ok(uri) = unquote(raw).parse()
                        {
                            TzParam::Uri(uri)
                        } else {
                            TzParam::Text(single_value(raw))
                        },
                    );
                },
                "LABEL" => parameters.label = Some(single_value(raw)),
                "AUTHOR" => {
                    parameters.author = Some(unquote(raw).parse().map_err(|_| invalid())?);
                },
                "AUTHOR-NAME" => parameters.author_name = Some(single_value(raw)),
                "CREATED" => {
                    parameters.created = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "DERIVED" => {
                    let value = single_value(raw);

                    parameters.derived = Some(if value.eq_ignore_ascii_case("true") {
                        true
                    } else if value.eq_ignore_ascii_case("false") {
                        false
                    } else {
                        return Err(invalid());
                    });
                },
                "PHONETIC" => {
                    parameters.phonetic = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "PROP-ID" => {
                    parameters.prop_id = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "SCRIPT" => {
                    parameters.script = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "SERVICE-TYPE" => parameters.service_type = Some(single_value(raw)),
                "USERNAME" => parameters.username = Some(single_value(raw)),
                _ => parameters.any.push(AnyParameter {
                    name:   name.parse().map_err(|_| invalid())?,
                    values: list_values(raw, false).collect(),
                }),
            }
        }

        Ok((parameters, value_type, conflicting_calscale))
    }
}

/// Checks whether `Parameters::parse` handles an uppercase parameter name by itself instead of keeping it in `Parameters::any`.
/// Keep this list in sync with the names matched by `Parameters::parse`.
pub(crate) fn is_known_parameter(upper_name: &str) -> bool {
    matches!(
        upper_name,
        "LANGUAGE"
            | "VALUE"
            | "PREF"
            | "ALTID"
            | "PID"
            | "TYPE"
            | "MEDIATYPE"
            | "CALSCALE"
            | "SORT-AS"
            | "GEO"
            | "TZ"
            | "LABEL"
            | "AUTHOR"
            | "AUTHOR-NAME"
            | "CREATED"
            | "DERIVED"
            | "PHONETIC"
            | "PROP-ID"
            | "SCRIPT"
            | "SERVICE-TYPE"
            | "USERNAME"
    )
}

/// Removes the surrounding double quotes of a raw parameter value if they exist.
fn unquote(s: &str) -> &str {
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') { &s[1..s.len() - 1] } else { s }
}

/// Decodes a raw parameter value that carries a single value, where commas are literal characters.
fn single_value(raw: &str) -> String {
    caret_decode(unquote(raw))
}

/// Decodes a raw parameter value that carries a comma-separated list of values.
///
/// When `split_inside_quotes` is enabled, commas inside a quoted item also separate values.
/// That accepts the legacy `TYPE="voice,cell"` form which older versions of this crate produced.
fn list_values(raw: &str, split_inside_quotes: bool) -> impl Iterator<Item = String> {
    let mut rest = Some(raw);

    let items = std::iter::from_fn(move || {
        let raw = rest?;
        let mut in_quotes = false;

        for (i, b) in raw.bytes().enumerate() {
            match b {
                b'"' => in_quotes = !in_quotes,
                b',' if !in_quotes => {
                    rest = Some(&raw[i + 1..]);

                    return Some(&raw[..i]);
                },
                _ => (),
            }
        }

        rest = None;

        Some(raw)
    });

    items.flat_map(move |item| {
        let count = if split_inside_quotes && item.starts_with('"') { usize::MAX } else { 1 };

        unquote(item).splitn(count, ',').map(caret_decode)
    })
}
