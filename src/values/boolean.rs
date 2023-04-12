use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Boolean {
    boolean: bool,
}

impl Boolean {
    pub fn from_bool(b: bool) -> Boolean {
        Boolean {
            boolean: b
        }
    }
}

impl Boolean {
    pub fn get_bool(&self) -> bool {
        self.boolean
    }
}

impl Value for Boolean {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.boolean {
            f.write_str("TRUE")?;
        } else {
            f.write_str("FALSE")?;
        }

        Ok(())
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for Boolean {}

impl ValidatedWrapper for Boolean {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
