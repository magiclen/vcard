use super::text::Text;
use super::uri::URI;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyValue {
    URI(URI),
    Text(Text),
}

impl Value for KeyValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            KeyValue::URI(uri) => {
                Value::fmt(uri, f)?;
            }
            KeyValue::Text(text) => {
                Value::fmt(text, f)?;
            }
        }

        Ok(())
    }
}

impl Display for KeyValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for KeyValue {}

impl ValidatedWrapper for KeyValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
