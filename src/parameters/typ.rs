use super::super::values::type_value::{TypeValue, TypeValueWithTelephoneType, TypeValueWithRelatedType};
use super::super::values::Value;
use super::super::Set;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Type {
    types: Set<TypeValue>,
}

impl Type {
    pub fn from_ids(types: Set<TypeValue>) -> Type {
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
        f.write_str(";TYPE=")?;

        let has_double_quote = self.types.as_hash_set().len() > 1;

        if has_double_quote {
            f.write_str("\"")?;
        }

        Value::fmt(&self.types, f)?;

        if has_double_quote {
            f.write_str("\"")?;
        }

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeWithTelType {
    types: Set<TypeValueWithTelephoneType>,
}

impl TypeWithTelType {
    pub fn from_ids(types: Set<TypeValueWithTelephoneType>) -> TypeWithTelType {
        TypeWithTelType { types }
    }
}

impl TypeWithTelType {
    pub fn get_ids(&self) -> &Set<TypeValueWithTelephoneType> {
        &self.types
    }
}

impl Parameter for TypeWithTelType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TYPE=")?;

        Value::fmt(&self.types, f)?;

        Ok(())
    }
}

impl Display for TypeWithTelType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for TypeWithTelType {}

impl ValidatedWrapper for TypeWithTelType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeWithRelatedType {
    types: Set<TypeValueWithRelatedType>,
}

impl TypeWithRelatedType {
    pub fn from_ids(types: Set<TypeValueWithRelatedType>) -> TypeWithRelatedType {
        TypeWithRelatedType { types }
    }
}

impl TypeWithRelatedType {
    pub fn get_ids(&self) -> &Set<TypeValueWithRelatedType> {
        &self.types
    }
}

impl Parameter for TypeWithRelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TYPE=")?;

        Value::fmt(&self.types, f)?;

        Ok(())
    }
}

impl Display for TypeWithRelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for TypeWithRelatedType {}

impl ValidatedWrapper for TypeWithRelatedType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}