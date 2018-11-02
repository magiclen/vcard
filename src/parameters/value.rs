use super::*;
use super::super::{IanaToken, XName};

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
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<ValueType> {
        let s = s.as_ref();
        let ss = s.to_uppercase();

        match ss.as_str() {
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
            _ => {
                match XName::from_str(s) {
                    Ok(x) => Some(ValueType::XName(x)),
                    Err(_) => {
                        match IanaToken::from_str(s) {
                            Ok(x) => Some(ValueType::IanaToken(x)),
                            Err(_) => {
                                None
                            }
                        }
                    }
                }
            }
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
        match ValueType::from_str(from_string_input) {
            Some(v) => Ok(v),
            None => Err("incorrect parameter: VALUE")
        }
    }

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error> {
        match ValueType::from_str(from_str_input) {
            Some(v) => Ok(v),
            None => Err("incorrect parameter: VALUE")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    value_type: ValueType
}

impl Value {
    pub fn with_value_type(value_type: ValueType) -> Value {
        Value {
            value_type
        }
    }
}

impl Value {
    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }
}

impl Parameter for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";VALUE=")?;
        f.write_str(self.value_type.get_str())?;

        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Value {}

impl ValidatedWrapper for Value {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}