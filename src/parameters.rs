//! Property parameters defined in RFC 6350 section 5 and RFC 9554 section 4.

use std::{
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

use mime::Mime;

use crate::{
    error::{InvalidValueError, ParseErrorKind},
    fold::FoldingWriter,
    syntax::{caret_decode, write_param_value},
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
        s.parse::<u8>().map_err(|_| InvalidValueError::new("preference")).and_then(Self::new)
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
            Some((id, source)) => (id, Some(source.parse().map_err(|_| ERROR)?)),
            None => (s, None),
        };

        Ok(Self {
            id: id.parse().map_err(|_| ERROR)?,
            source,
        })
    }
}

/// A value of the TYPE parameter.
///
/// This single type covers the general values, the TEL-specific values, the RELATED relation values and the RFC 9554 values.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// An emergency contact, from the RELATED relation values.
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
    /// A date, from the RELATED relation values.
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
    Extension(Token),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Calscale {
    /// The Gregorian calendar, which is the only calendar system defined by RFC 6350.
    Gregorian,
    /// An extension value which is an IANA token or an x-name.
    Extension(Token),
}

impl Display for Calscale {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Gregorian => f.write_str("gregorian"),
            Self::Extension(token) => Display::fmt(token, f),
        }
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Extension(Token),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TzParam {
    /// A time zone text, which should be a name from the IANA time zone database.
    Text(String),
    /// A URI, written inside double quotes.
    Uri(Uri),
}

/// An extension parameter that this crate has no dedicated field for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyParameter {
    /// The parameter name, which is an IANA token or an x-name.
    pub name:   Token,
    /// The parameter values.
    pub values: Vec<String>,
}

/// The parameters of a property.
///
/// RFC 9554 section 3 allows most parameters on any property, so every property shares this struct.
/// A parameter that is `None` or an empty list is simply not written.
#[derive(Debug, Clone, PartialEq, Default)]
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
    pub media_type:   Option<Mime>,
    /// The CALSCALE parameter, which tells the calendar system of a date value.
    pub calscale:     Option<Calscale>,
    /// The SORT-AS values, which give the strings that sorting should use.
    pub sort_as:      Vec<String>,
    /// The GEO parameter, which attaches geographic coordinates to an address.
    pub geo:          Option<Uri>,
    /// The TZ parameter, which attaches time zone information to an address.
    pub tz:           Option<TzParam>,
    /// The LABEL parameter, which carries the formatted text of an address.
    pub label:        Option<String>,
    /// The AUTHOR parameter defined by RFC 9554, which is a URI that identifies the author of the value.
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
            write_param_value(w, media_type.as_ref())?;
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
            write!(w, ";GEO=\"{geo}\"")?;
        }

        if let Some(tz) = &self.tz {
            match tz {
                TzParam::Text(text) => {
                    w.write_str(";TZ=")?;
                    write_param_value(w, text)?;
                },
                TzParam::Uri(uri) => write!(w, ";TZ=\"{uri}\"")?,
            }
        }

        if let Some(label) = &self.label {
            w.write_str(";LABEL=")?;
            write_param_value(w, label)?;
        }

        if let Some(author) = &self.author {
            // The AUTHOR parameter is always written inside double quotes, as its ABNF requires.
            write!(w, ";AUTHOR=\"{author}\"")?;
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
    pub(crate) fn parse(
        raw_params: Vec<(&str, &str)>,
    ) -> Result<(Self, Option<String>), ParseErrorKind> {
        let mut parameters = Self::default();
        let mut value_type = None;

        for (name, raw) in raw_params {
            let invalid = || ParseErrorKind::InvalidParameter(name.to_string());

            let upper_name = name.to_ascii_uppercase();

            match upper_name.as_str() {
                "LANGUAGE" => {
                    parameters.language = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "VALUE" => value_type = Some(single_value(raw).to_ascii_lowercase()),
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
                    parameters.media_type = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "CALSCALE" => {
                    parameters.calscale = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "SORT-AS" => parameters.sort_as.extend(list_values(raw, false)),
                "GEO" => {
                    parameters.geo = Some(single_value(raw).parse().map_err(|_| invalid())?);
                },
                "TZ" => {
                    // A quoted TZ parameter carries a URI and a plain one carries text.
                    parameters.tz = Some(if raw.starts_with('"') {
                        TzParam::Uri(single_value(raw).parse().map_err(|_| invalid())?)
                    } else {
                        TzParam::Text(single_value(raw))
                    });
                },
                "LABEL" => parameters.label = Some(single_value(raw)),
                "AUTHOR" => {
                    parameters.author = Some(single_value(raw).parse().map_err(|_| invalid())?);
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
                    values: list_values(raw, false),
                }),
            }
        }

        Ok((parameters, value_type))
    }
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
fn list_values(raw: &str, split_inside_quotes: bool) -> Vec<String> {
    let mut items = Vec::new();

    let bytes = raw.as_bytes();

    let mut start = 0;
    let mut in_quotes = false;

    for (i, b) in bytes.iter().enumerate() {
        match b {
            b'"' => in_quotes = !in_quotes,
            b',' if !in_quotes => {
                items.push(&raw[start..i]);
                start = i + 1;
            },
            _ => (),
        }
    }

    items.push(&raw[start..]);

    let mut values = Vec::with_capacity(items.len());

    for item in items {
        let quoted = item.starts_with('"');

        let inner = unquote(item);

        if quoted && split_inside_quotes && inner.contains(',') {
            values.extend(inner.split(',').map(caret_decode));
        } else {
            values.push(caret_decode(inner));
        }
    }

    values
}
