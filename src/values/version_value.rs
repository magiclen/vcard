use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VersionValue {
    V4P0,
}

impl VersionValue {
    pub fn get_str(&self) -> &str {
        match self {
            VersionValue::V4P0 => "4.0",
        }
    }
}

impl Value for VersionValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for VersionValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for VersionValue {}

impl ValidatedWrapper for VersionValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
