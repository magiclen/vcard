use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Begin;

impl Property for Begin {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("BEGIN:VCARD\r\n")?;

        Ok(())
    }
}

impl Display for Begin {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Begin {}

impl ValidatedWrapper for Begin {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
