use super::super::values::{parameter_value::ParameterValue, Value};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label {
    parameter_value: ParameterValue,
}

impl Label {
    pub fn from_parameter_value(parameter_value: ParameterValue) -> Label {
        Label { parameter_value }
    }

    pub fn is_empty(&self) -> bool {
        self.parameter_value.is_empty()
    }
}

impl Label {
    pub fn get_parameter_value(&self) -> &ParameterValue {
        &self.parameter_value
    }
}

impl Parameter for Label {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str(";LABEL=")?;

        Value::fmt(&self.parameter_value, f)?;

        Ok(())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Label {}

impl ValidatedWrapper for Label {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
