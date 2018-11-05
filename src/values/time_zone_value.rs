use super::uri::URI;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimeZoneValue {
    Tz(chrono_tz::Tz),
    URI(URI),
}

impl Value for TimeZoneValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            TimeZoneValue::Tz(tz) => {
                f.write_str(&format!("{:?}", tz))?;
            }
            TimeZoneValue::URI(uri) => {
                Value::fmt(uri, f)?;
            }
        }

        Ok(())
    }
}

impl Display for TimeZoneValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TimeZoneValue {}

impl ValidatedWrapper for TimeZoneValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}