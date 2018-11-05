use super::related_type::RelatedType;
use super::telephone_type::TelephoneType;
use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeValue {
    Work,
    Home,
    RelatedType(RelatedType),
    IanaToken(IanaToken),
    XName(XName),
}

impl TypeValue {
    pub fn get_str(&self) -> &str {
        match self {
            TypeValue::Work => "work",
            TypeValue::Home => "home",
            TypeValue::RelatedType(rt) => rt.get_str(),
            TypeValue::IanaToken(x) => x.as_str(),
            TypeValue::XName(x) => x.as_str(),
        }
    }
}

impl Value for TypeValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TypeValue {}

impl ValidatedWrapper for TypeValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeValueWithTelephoneType {
    Work,
    Home,
    TelType(TelephoneType),
    RelatedType(RelatedType),
    IanaToken(IanaToken),
    XName(XName),
}

impl TypeValueWithTelephoneType {
    pub fn get_str(&self) -> &str {
        match self {
            TypeValueWithTelephoneType::Work => "work",
            TypeValueWithTelephoneType::Home => "home",
            TypeValueWithTelephoneType::TelType(tt) => tt.get_str(),
            TypeValueWithTelephoneType::RelatedType(rt) => rt.get_str(),
            TypeValueWithTelephoneType::IanaToken(x) => x.as_str(),
            TypeValueWithTelephoneType::XName(x) => x.as_str(),
        }
    }
}

impl Value for TypeValueWithTelephoneType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TypeValueWithTelephoneType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TypeValueWithTelephoneType {}

impl ValidatedWrapper for TypeValueWithTelephoneType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}