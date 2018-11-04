use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl Value for TelType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TelType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TelType {}

impl ValidatedWrapper for TelType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}