use super::super::values::type_value::TypeValue;
use super::super::values::{Set, Value};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

// Used in FN, NICKNAME, PHOTO, ADR, TEL, EMAIL, IMPP, LANG, TZ, GEO, TITLE, ROLE, LOGO, ORG, RELATED, CATEGORIES, NOTE, SOUND, URL, KEY, FBURL, CALADRURI, and CALURI
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    types: Set<TypeValue>,
}

impl Type {
    pub fn with_ids(types: Set<TypeValue>) -> Type {
        Type { types }
    }
}

impl Type {
    pub fn get_ids(&self) -> &Set<TypeValue> {
        &self.types
    }
}

impl Parameter for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";VALUE=")?;

        Value::fmt(&self.types, f)?;

        Ok(())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Type {}

impl ValidatedWrapper for Type {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
