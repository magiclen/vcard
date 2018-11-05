use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TelephoneType {
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

impl TelephoneType {
    pub fn get_str(&self) -> &str {
        match self {
            TelephoneType::Text => "text",
            TelephoneType::Voice => "voice",
            TelephoneType::Fax => "fax",
            TelephoneType::Cell => "cell",
            TelephoneType::Video => "video",
            TelephoneType::Pager => "pager",
            TelephoneType::Textphone => "textphone",
            TelephoneType::IanaToken(x) => x.as_str(),
            TelephoneType::XName(x) => x.as_str(),
        }
    }
}

impl Value for TelephoneType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TelephoneType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TelephoneType {}

impl ValidatedWrapper for TelephoneType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}