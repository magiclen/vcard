use super::super::values::{parameter_value::ParameterValues, Value};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SortAs {
    parameter_values: ParameterValues,
}

impl SortAs {
    pub fn from_parameter_values(parameter_values: ParameterValues) -> SortAs {
        SortAs { parameter_values }
    }

    pub fn is_empty(&self) -> bool {
        self.parameter_values.is_empty()
    }
}

impl SortAs {
    pub fn get_parameter_values(&self) -> &ParameterValues {
        &self.parameter_values
    }
}

impl Parameter for SortAs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";SORT-AS=")?;

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
