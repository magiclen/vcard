use super::super::values::Value;
use super::super::values::parameter_value::ParameterValue;
use super::super::values::uri::URI;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tz {
    URI(URI),
    ParameterValue(ParameterValue),
}

impl Tz {
    pub fn from_uri(uri: URI) -> Tz {
        Tz::URI(uri)
    }

    pub fn from_parameter_value(parameter_value: ParameterValue) -> Tz {
        Tz::ParameterValue(parameter_value)
    }
}

impl Parameter for Tz {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TZ=")?;

        match self {
            Tz::URI(uri) => {
                f.write_str("\"")?;
                Value::fmt(uri, f)?;
                f.write_str("\"")?;
            }
            Tz::ParameterValue(parameter_value) => {
                Value::fmt(parameter_value, f)?;
            }
        }

        Ok(())
    }
}

impl Display for Tz {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Tz {}

impl ValidatedWrapper for Tz {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
