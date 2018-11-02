use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyIDValue {
    d1: u8,
    d2: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyIDValueError {
    OutOfRange,
}

impl PropertyIDValue {
    pub fn from_u8(d1: u8, d2: Option<u8>) -> Result<PropertyIDValue, PropertyIDValueError> {
        if d1 > 9 {
            return Err(PropertyIDValueError::OutOfRange);
        }
        if let Some(d2) = d2 {
            if d2 > 9 {
                return Err(PropertyIDValueError::OutOfRange);
            }
        }

        Ok(PropertyIDValue { d1, d2 })
    }
}

impl PropertyIDValue {
    pub fn get_d1(&self) -> u8 {
        self.d1
    }

    pub fn get_d2(&self) -> Option<u8> {
        self.d2
    }
}

impl Value for PropertyIDValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}", self.d1))?;

        if let Some(d2) = self.d2 {
            f.write_str(".")?;
            f.write_fmt(format_args!("{}", d2))?;
        }

        Ok(())
    }
}

impl Display for PropertyIDValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for PropertyIDValue {}

impl ValidatedWrapper for PropertyIDValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

impl Value for List<PropertyIDValue> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<PropertyIDValue> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}
