use super::*;
use super::super::{SAFE_RE, QSAFE_RE};

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterValue {
    qsafe: bool,
    text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterValueError {
    IncorrectFormat
}

impl ParameterValue {
    pub fn from_str(text: &str) -> Result<ParameterValue, ParameterValueError> {
        let qsafe = if !SAFE_RE.is_match(text) {
            if QSAFE_RE.is_match(text) {
                true
            } else {
                return Err(ParameterValueError::IncorrectFormat);
            }
        } else {
            false
        };

        Ok(ParameterValue {
            qsafe,
            text: text.to_string(),
        })
    }

    pub fn from_string(text: String) -> Result<ParameterValue, ParameterValueError> {
        let qsafe = if !SAFE_RE.is_match(&text) {
            if QSAFE_RE.is_match(&text) {
                true
            } else {
                return Err(ParameterValueError::IncorrectFormat);
            }
        } else {
            false
        };

        Ok(ParameterValue {
            qsafe,
            text,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl ParameterValue {
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl Value for ParameterValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.qsafe {
            f.write_str("\"")?;
            f.write_str(&self.text)?;
            f.write_str("\"")?;
        } else {
            f.write_str(&self.text)?;
        }

        Ok(())
    }
}

impl Display for ParameterValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for ParameterValue {}

impl ValidatedWrapper for ParameterValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}