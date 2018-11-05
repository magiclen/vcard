use super::text::Text;
use super::uri::URI;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};
use validators::uuid::UUID;

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
                f.write_str("urn:uuid:")?;
                if uuid.has_both_case() || uuid.has_uppercase() {
                    f.write_str(&uuid.get_full_uuid().to_lowercase())?;
                } else {
                    f.write_str(uuid.get_full_uuid())?;
                }
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