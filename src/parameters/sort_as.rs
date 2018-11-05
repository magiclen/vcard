use super::super::values::{parameter_value::ParameterValue, Value};
use super::super::Set;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SortAs {
    parameter_values: Set<ParameterValue>,
}

impl SortAs {
    pub fn from_parameter_values(parameter_values: Set<ParameterValue>) -> SortAs {
        SortAs { parameter_values }
    }

    pub fn is_empty(&self) -> bool {
        let v = self.parameter_values.as_hash_set();

        for e in v {
            if !e.is_empty() {
                return false;
            }
        }

        true
    }
}

impl SortAs {
    pub fn get_parameter_values(&self) -> &Set<ParameterValue> {
        &self.parameter_values
    }
}

impl Parameter for SortAs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";SAFE-CHAR=")?;

        Value::fmt(&self.parameter_values, f)?;

        Ok(())
    }
}

impl Display for SortAs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for SortAs {}

impl ValidatedWrapper for SortAs {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
