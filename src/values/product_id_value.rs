use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::{super::values::text::Text, *};

// TODO: not implement yet, refer to [ISO9070] and [RFC3406]

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProductIDValue {
    Text(Text),
}

impl ProductIDValue {
    pub fn is_empty(&self) -> bool {
        match self {
            ProductIDValue::Text(t) => t.is_empty(),
        }
    }
}

impl Value for ProductIDValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        match self {
            ProductIDValue::Text(text) => {
                Value::fmt(text, f)?;
            },
        }

        Ok(())
    }
}

impl Display for ProductIDValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for ProductIDValue {}

impl ValidatedWrapper for ProductIDValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
