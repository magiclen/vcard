use super::super::{QSAFE_RE, SAFE_RE};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParameterValue {
    qsafe: bool,
    comma: bool,
    text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterValueError {
    IncorrectFormat,
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

        let comma = text.contains(",");

        Ok(ParameterValue {
            qsafe,
            comma,
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

        let comma = text.contains(",");

        Ok(ParameterValue { qsafe, comma, text })
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
        if self.is_empty() {
            return Ok(());
        }

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParameterValues {
    parameter_values: Set<ParameterValue>
}

impl ParameterValues {
    pub fn from_parameter_values(parameter_values: Set<ParameterValue>) -> ParameterValues {
        ParameterValues {
            parameter_values,
        }
    }

    pub fn is_empty(&self) -> bool {
        let v = self.parameter_values.as_hash_set();

        for e in v {
            if !e.is_empty() {
                return false;
            }
        }

        true
    }

    fn has_multiple_non_empty_values(&self) -> bool {
        let mut flag = false;

        let v = self.parameter_values.as_hash_set();

        if v.len() < 2 {
            return false;
        }

        for e in v {
            if !e.is_empty() {
                if flag {
                    return true;
                } else {
                    flag = true;
                }
            }
        }

        false
    }
}

impl ParameterValues {
    pub fn get_parameter_values(&self) -> &Set<ParameterValue> {
        &self.parameter_values
    }
}

impl Value for ParameterValues {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let v = self.parameter_values.as_hash_set();


        if self.has_multiple_non_empty_values() {
            f.write_str("\"")?;

            let mut it = v.iter();

            loop {
                let e = it.next().unwrap();

                if !e.is_empty() {
                    f.write_str(&e.text)?;
                    break;
                }
            }

            loop {
                let e = it.next();

                match e {
                    Some(e) => {
                        if !e.is_empty() {
                            f.write_str(",")?;
                            f.write_str(&e.text)?;
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            f.write_str("\"")?;
        } else {
            for e in v {
                if !e.is_empty() {
                    Value::fmt(e, f)?;
                    break;
                }
            }
        }

        Ok(())
    }
}

impl Display for ParameterValues {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for ParameterValues {}

impl ValidatedWrapper for ParameterValues {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}