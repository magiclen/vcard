use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CalscaleValue {
    Gregorian,
    IanaToken(IanaToken),
    XName(XName),
}

impl CalscaleValue {
    pub fn gregorian() -> CalscaleValue {
        CalscaleValue::Gregorian
    }

    pub fn from_x_name(x_name: XName) -> CalscaleValue {
        CalscaleValue::XName(x_name)
    }

    pub fn from_iana_token(iana_token: IanaToken) -> CalscaleValue {
        CalscaleValue::IanaToken(iana_token)
    }

    pub fn get_str(&self) -> &str {
        match self {
            CalscaleValue::Gregorian => "gregorian",
            CalscaleValue::IanaToken(x) => x.as_str(),
            CalscaleValue::XName(x) => x.as_str(),
        }
    }
}

impl Value for CalscaleValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for CalscaleValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for CalscaleValue {}

impl ValidatedWrapper for CalscaleValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}