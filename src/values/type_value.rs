use super::super::parameters::types::{RelatedType, TelType};
use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq)]
pub enum TypeValue {
    Work,
    Home,
    TelType(TelType),
    RelatedType(RelatedType),
    IanaToken(IanaToken),
    XName(XName),
}

impl TypeValue {
    pub fn new_work() -> TypeValue {
        TypeValue::Work
    }

    pub fn new_home() -> TypeValue {
        TypeValue::Home
    }

    pub fn with_tel_type(tel_type: TelType) -> TypeValue {
        TypeValue::TelType(tel_type)
    }

    pub fn with_related_type(related_type: RelatedType) -> TypeValue {
        TypeValue::RelatedType(related_type)
    }

    pub fn with_x_name(x_name: XName) -> TypeValue {
        TypeValue::XName(x_name)
    }

    pub fn with_iana_token(iana_token: IanaToken) -> TypeValue {
        TypeValue::IanaToken(iana_token)
    }

    pub fn get_str(&self) -> &str {
        match self {
            TypeValue::Work => "work",
            TypeValue::Home => "home",
            TypeValue::TelType(tt) => tt.get_str(),
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

impl Value for List<TypeValue> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<TypeValue> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}
