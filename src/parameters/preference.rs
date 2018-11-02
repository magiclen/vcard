use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct Preference {
    p: u8
}

#[derive(Clone, Debug, PartialEq)]
pub enum PreferenceRangeError {
    OutOfRange
}

impl Preference {
    pub fn from_u8(p: u8) -> Result<Preference, PreferenceRangeError> {
        if p < 1 || p > 100 {
            return Err(PreferenceRangeError::OutOfRange);
        }

        Ok(Preference { p })
    }
}

impl Preference {
    pub fn get_number(&self) -> u8 {
        self.p
    }
}

impl Parameter for Preference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";PREF=")?;

        f.write_fmt(format_args!("{}", self.p))?;

        Ok(())
    }
}

impl Display for Preference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Preference {}

impl ValidatedWrapper for Preference {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}