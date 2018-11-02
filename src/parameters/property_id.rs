use super::super::values::property_id_value::PropertyIDValue;
use super::super::values::{List, Value};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyID {
    ids: List<PropertyIDValue>,
}

impl PropertyID {
    pub fn with_ids(ids: List<PropertyIDValue>) -> PropertyID {
        PropertyID { ids }
    }
}

impl PropertyID {
    pub fn get_ids(&self) -> &List<PropertyIDValue> {
        &self.ids
    }
}

impl Parameter for PropertyID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";PID=")?;

        Value::fmt(&self.ids, f)?;

        Ok(())
    }
}

impl Display for PropertyID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for PropertyID {}

impl ValidatedWrapper for PropertyID {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}