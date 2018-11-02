use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    value_type: types::ValueType,
}

impl Value {
    pub fn with_value_type(value_type: types::ValueType) -> Value {
        Value { value_type }
    }
}

impl Value {
    pub fn get_value_type(&self) -> &types::ValueType {
        &self.value_type
    }
}

impl Parameter for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";VALUE=")?;
        f.write_str(self.value_type.get_str())?;

        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Value {}

impl ValidatedWrapper for Value {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
