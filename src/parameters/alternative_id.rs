use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::{
    super::values::{parameter_value::ParameterValue, Value},
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AlternativeID {
    parameter_value: ParameterValue,
}

impl AlternativeID {
    pub fn from_parameter_value(parameter_value: ParameterValue) -> AlternativeID {
        AlternativeID {
            parameter_value,
        }
    }
}

impl AlternativeID {
    pub fn get_parameter_value(&self) -> &ParameterValue {
        &self.parameter_value
    }
}

impl Parameter for AlternativeID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";ALTID=")?;

        Value::fmt(&self.parameter_value, f)?;

        Ok(())
    }
}

impl Display for AlternativeID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for AlternativeID {}

impl ValidatedWrapper for AlternativeID {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
