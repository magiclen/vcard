use super::text::Text;
use super::uri::URI;
use super::uuid::UUID;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UIDValue {
    URI(URI),
    UUID(UUID),
    Text(Text),
}

impl UIDValue {
    pub fn is_empty(&self) -> bool {
        if let UIDValue::Text(t) = self {
            return t.is_empty();
        }

        false
    }
}

impl Value for UIDValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        match self {
            UIDValue::URI(uri) => {
                Value::fmt(uri, f)?;
            }
            UIDValue::UUID(uuid) => {
                Value::fmt(uuid, f)?;
            }
            UIDValue::Text(t) => {
                Value::fmt(t, f)?;
            }
        }

        Ok(())
    }
}

impl Display for UIDValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for UIDValue {}

impl ValidatedWrapper for UIDValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
