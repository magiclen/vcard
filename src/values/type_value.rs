use super::types::{RelatedType, TelType};
use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Hash for TypeValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TypeValue::Work => {
                state.write_i8(0);
                state.write_i8(0);
            }
            TypeValue::Home => {
                state.write_i8(1);
                state.write_i8(1);
            }
            TypeValue::TelType(tt) => {
                state.write_i8(2);
                state.write(tt.get_str().as_bytes());
            }
            TypeValue::RelatedType(rt) => {
                state.write_i8(3);
                state.write(rt.get_str().as_bytes());
            }
            TypeValue::IanaToken(x) => {
                state.write_i8(4);
                state.write(x.as_str().as_bytes());
            }
            TypeValue::XName(x) => {
                state.write_i8(5);
                state.write(x.as_str().as_bytes());
            }
        }
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