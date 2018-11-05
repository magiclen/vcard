use super::super::{IanaToken, XName};
use super::super::values::Value;
use super::super::values::parameter_value::ParameterValues;
use super::super::Set;
use super::Parameter;

use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Any {
    IanaToken(IanaToken, ParameterValues),
    XName(XName, ParameterValues),
}

impl Any {
    pub fn is_empty(&self) -> bool {
        match self {
            Any::IanaToken(_, v) => {
                v.is_empty()
            }
            Any::XName(_, v) => {
                v.is_empty()
            }
        }
    }
}

impl Parameter for Any {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str(";")?;

        let set = match self {
            Any::IanaToken(a, b) => {
                f.write_str(a.as_str())?;
                b
            }
            Any::XName(a, b) => {
                f.write_str(a.as_str())?;
                b
            }
        };

        f.write_str("=")?;

        Value::fmt(set, f)?;

        Ok(())
    }
}

impl Parameter for Set<Any> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for e in self.as_hash_set() {
            Parameter::fmt(e, f)?;
        }

        Ok(())
    }
}

impl Value for Any {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Display for Any {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Hash for Any {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Any::XName(a, b) => {
                state.write(a.as_str().as_bytes());
                b.hash(state);
            }
            Any::IanaToken(a, b) => {
                state.write(a.as_str().as_bytes());
                b.hash(state);
            }
        }
    }
}

impl Validated for Any {}

impl ValidatedWrapper for Any {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
