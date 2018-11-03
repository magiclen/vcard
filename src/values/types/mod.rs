use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Text,
    Uri,
    Date,
    Time,
    DateTime,
    DateAndOrTime,
    Timestamp,
    Boolean,
    Integer,
    Float,
    UtcOffset,
    LanguageTag,
    IanaToken(IanaToken),
    XName(XName),
}

impl ValueType {
    pub fn from_str(s: &str) -> Option<ValueType> {
        match s.to_uppercase().as_str() {
            "TEXT" => Some(ValueType::Text),
            "URI" => Some(ValueType::Uri),
            "DATE" => Some(ValueType::Date),
            "TIME" => Some(ValueType::Time),
            "DATETIME" | "DATE-TIME" => Some(ValueType::DateTime),
            "DATEANDORTIME" | "DATE-AND-OR-TIME" => Some(ValueType::DateAndOrTime),
            "TIMESTAMP" => Some(ValueType::Timestamp),
            "BOOLEAN" => Some(ValueType::Boolean),
            "INTEGER" => Some(ValueType::Integer),
            "FLOAT" => Some(ValueType::Float),
            "UTCOFFSET" | "UTC-OFFSET" => Some(ValueType::UtcOffset),
            "LANGUAGETAG" | "LANGUAGE-TAG" => Some(ValueType::LanguageTag),
            _ => match XName::from_str(s) {
                Ok(x) => Some(ValueType::XName(x)),
                Err(_) => match IanaToken::from_str(s) {
                    Ok(x) => Some(ValueType::IanaToken(x)),
                    Err(_) => None,
                },
            },
        }
    }

    pub fn from_string(s: String) -> Option<ValueType> {
        match s.to_uppercase().as_str() {
            "TEXT" => Some(ValueType::Text),
            "URI" => Some(ValueType::Uri),
            "DATE" => Some(ValueType::Date),
            "TIME" => Some(ValueType::Time),
            "DATETIME" | "DATE-TIME" => Some(ValueType::DateTime),
            "DATEANDORTIME" | "DATE-AND-OR-TIME" => Some(ValueType::DateAndOrTime),
            "TIMESTAMP" => Some(ValueType::Timestamp),
            "BOOLEAN" => Some(ValueType::Boolean),
            "INTEGER" => Some(ValueType::Integer),
            "FLOAT" => Some(ValueType::Float),
            "UTCOFFSET" | "UTC-OFFSET" => Some(ValueType::UtcOffset),
            "LANGUAGETAG" | "LANGUAGE-TAG" => Some(ValueType::LanguageTag),
            _ => match XName::from_str(&s) {
                Ok(x) => Some(ValueType::XName(x)),
                Err(_) => match IanaToken::from_string(s) {
                    Ok(x) => Some(ValueType::IanaToken(x)),
                    Err(_) => None,
                },
            },
        }
    }

    pub fn with_x_name(x_name: XName) -> ValueType {
        ValueType::XName(x_name)
    }

    pub fn with_iana_token(iana_token: IanaToken) -> ValueType {
        ValueType::IanaToken(iana_token)
    }

    pub fn get_str(&self) -> &str {
        match self {
            ValueType::Text => "text",
            ValueType::Uri => "uri",
            ValueType::Date => "date",
            ValueType::Time => "time",
            ValueType::DateTime => "date-time",
            ValueType::DateAndOrTime => "date-and-or-time",
            ValueType::Timestamp => "timestamp",
            ValueType::Boolean => "boolean",
            ValueType::Integer => "integer",
            ValueType::Float => "float",
            ValueType::UtcOffset => "utf-offset",
            ValueType::LanguageTag => "language-tag",
            ValueType::IanaToken(x) => x.as_str(),
            ValueType::XName(x) => x.as_str(),
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())
    }
}

impl Validated for ValueType {}

impl ValidatedWrapper for ValueType {
    type Error = &'static str;

    fn from_string(from_string_input: String) -> Result<Self, Self::Error> {
        match ValueType::from_string(from_string_input) {
            Some(v) => Ok(v),
            None => Err("incorrect type"),
        }
    }

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error> {
        match ValueType::from_str(from_str_input) {
            Some(v) => Ok(v),
            None => Err("incorrect type"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TelType {
    Text,
    Voice,
    Fax,
    Cell,
    Video,
    Pager,
    Textphone,
    IanaToken(IanaToken),
    XName(XName),
}

impl TelType {
    pub fn from_str(s: &str) -> Option<TelType> {
        match s.to_uppercase().as_str() {
            "TEXT" => Some(TelType::Text),
            "VOICE" => Some(TelType::Voice),
            "FAX" => Some(TelType::Fax),
            "CELL" => Some(TelType::Cell),
            "VIDEO" => Some(TelType::Video),
            "PAGER" => Some(TelType::Pager),
            "TEXTPHONE" => Some(TelType::Textphone),
            _ => match XName::from_str(s) {
                Ok(x) => Some(TelType::XName(x)),
                Err(_) => match IanaToken::from_str(s) {
                    Ok(x) => Some(TelType::IanaToken(x)),
                    Err(_) => None,
                },
            },
        }
    }

    pub fn from_string(s: String) -> Option<TelType> {
        match s.to_uppercase().as_str() {
            "TEXT" => Some(TelType::Text),
            "VOICE" => Some(TelType::Voice),
            "FAX" => Some(TelType::Fax),
            "CELL" => Some(TelType::Cell),
            "VIDEO" => Some(TelType::Video),
            "PAGER" => Some(TelType::Pager),
            "TEXTPHONE" => Some(TelType::Textphone),
            _ => match XName::from_str(&s) {
                Ok(x) => Some(TelType::XName(x)),
                Err(_) => match IanaToken::from_string(s) {
                    Ok(x) => Some(TelType::IanaToken(x)),
                    Err(_) => None,
                },
            },
        }
    }

    pub fn with_x_name(x_name: XName) -> TelType {
        TelType::XName(x_name)
    }

    pub fn with_iana_token(iana_token: IanaToken) -> TelType {
        TelType::IanaToken(iana_token)
    }

    pub fn get_str(&self) -> &str {
        match self {
            TelType::Text => "text",
            TelType::Voice => "voice",
            TelType::Fax => "fax",
            TelType::Cell => "cell",
            TelType::Video => "video",
            TelType::Pager => "pager",
            TelType::Textphone => "textphone",
            TelType::IanaToken(x) => x.as_str(),
            TelType::XName(x) => x.as_str(),
        }
    }
}

impl Display for TelType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())
    }
}

impl Validated for TelType {}

impl ValidatedWrapper for TelType {
    type Error = &'static str;

    fn from_string(from_string_input: String) -> Result<Self, Self::Error> {
        match TelType::from_string(from_string_input) {
            Some(v) => Ok(v),
            None => Err("incorrect type"),
        }
    }

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error> {
        match TelType::from_str(from_str_input) {
            Some(v) => Ok(v),
            None => Err("incorrect type"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelatedType {
    Contact,
    Acquaintance,
    Friend,
    Met,
    CoWorker,
    Colleague,
    CoResident,
    Neighbor,
    Child,
    Parent,
    Spouse,
    Kin,
    Muse,
    Crush,
    Date,
    Sweetheart,
    Me,
    Agent,
    Emergency,
}

impl RelatedType {
    pub fn from_str(s: &str) -> Option<RelatedType> {
        match s.to_uppercase().as_str() {
            "CONTACT" => Some(RelatedType::Contact),
            "ACQUAINTANCE" => Some(RelatedType::Acquaintance),
            "FRIEND" => Some(RelatedType::Friend),
            "MET" => Some(RelatedType::Met),
            "COWORKER" | "CO-WORKER" => Some(RelatedType::CoWorker),
            "COLLEAGUE" => Some(RelatedType::Colleague),
            "CORESIDENT" | "CO-RESIDENT" => Some(RelatedType::CoResident),
            "NEIGHBOR" => Some(RelatedType::Neighbor),
            "CHILE" => Some(RelatedType::Child),
            "PARENT" => Some(RelatedType::Parent),
            "SPOUSE" => Some(RelatedType::Spouse),
            "KIN" => Some(RelatedType::Kin),
            "MUSE" => Some(RelatedType::Muse),
            "CRUSH" => Some(RelatedType::Crush),
            "DATE" => Some(RelatedType::Date),
            "SWEETHEART" => Some(RelatedType::Sweetheart),
            "ME" => Some(RelatedType::Me),
            "AGENT" => Some(RelatedType::Agent),
            "EMERGENCY" => Some(RelatedType::Emergency),
            _ => None,
        }
    }

    pub fn from_string(s: String) -> Option<RelatedType> {
        Self::from_str(&s)
    }

    pub fn get_str(&self) -> &str {
        match self {
            RelatedType::Contact => "contact",
            RelatedType::Acquaintance => "acquaintance",
            RelatedType::Friend => "friend",
            RelatedType::Met => "met",
            RelatedType::CoWorker => "co-worker",
            RelatedType::Colleague => "colleague",
            RelatedType::CoResident => "co-resident",
            RelatedType::Neighbor => "neighbor",
            RelatedType::Child => "child",
            RelatedType::Parent => "parent",
            RelatedType::Spouse => "spouse",
            RelatedType::Kin => "kin",
            RelatedType::Muse => "muse",
            RelatedType::Crush => "crush",
            RelatedType::Date => "date",
            RelatedType::Sweetheart => "sweetheart",
            RelatedType::Me => "me",
            RelatedType::Agent => "agent",
            RelatedType::Emergency => "emergency",
        }
    }
}

impl Display for RelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())
    }
}

impl Validated for RelatedType {}

impl ValidatedWrapper for RelatedType {
    type Error = &'static str;

    fn from_string(from_string_input: String) -> Result<Self, Self::Error> {
        match RelatedType::from_string(from_string_input) {
            Some(v) => Ok(v),
            None => Err("incorrect type"),
        }
    }

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error> {
        match RelatedType::from_str(from_str_input) {
            Some(v) => Ok(v),
            None => Err("incorrect type"),
        }
    }
}
