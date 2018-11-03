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

impl Value for ValueType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for ValueType {}

impl ValidatedWrapper for ValueType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}