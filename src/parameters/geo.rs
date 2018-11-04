use super::super::values::Value;
use super::super::values::geo_value::GeoValue;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct Geo {
    geo_value: GeoValue
}

impl Geo {
    pub fn from_geo_value(geo_value: GeoValue) -> Geo {
        Geo { geo_value }
    }
}

impl Geo {
    pub fn get_geo_value(&self) -> &GeoValue {
        &self.geo_value
    }
}

impl Parameter for Geo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";GEO=\"")?;

        Value::fmt(&self.geo_value, f)?;

        f.write_str("\"")?;

        Ok(())
    }
}

impl Display for Geo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Geo {}

impl ValidatedWrapper for Geo {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
