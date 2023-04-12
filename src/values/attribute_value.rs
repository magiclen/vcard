use std::fmt::{Display, Write};

use validators::{Validated, ValidatedWrapper};

use super::*;
use crate::PATH_PERCENT_ENCODE_SET;

// TODO: not implement yet, refer to [RFC2045]

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttributeValue {
    attribute: String,
    value:     String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValueError {}

impl AttributeValue {
    pub fn from_str(attribute: &str, value: &str) -> Result<AttributeValue, AttributeValueError> {
        Ok(AttributeValue {
            attribute: attribute.to_string(), value: value.to_string()
        })
    }

    pub fn from_string(
        attribute: String,
        value: String,
    ) -> Result<AttributeValue, AttributeValueError> {
        Ok(AttributeValue {
            attribute,
            value,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.attribute.is_empty() || self.value.is_empty()
    }
}

impl AttributeValue {
    pub fn get_attribute(&self) -> &str {
        &self.attribute
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Value for AttributeValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }
        f.write_char(';')?;
        f.write_str(
            &percent_encoding::utf8_percent_encode(
                self.attribute.as_str(),
                PATH_PERCENT_ENCODE_SET,
            )
            .to_string(),
        )?;
        f.write_char('=')?;
        f.write_str(
            &percent_encoding::utf8_percent_encode(self.value.as_str(), PATH_PERCENT_ENCODE_SET)
                .to_string(),
        )?;

        Ok(())
    }
}

impl Display for AttributeValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for AttributeValue {}

impl ValidatedWrapper for AttributeValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
