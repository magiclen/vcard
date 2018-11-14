use super::super::values::type_value::{TypeValue, TypeValueWithTelephoneType, TypeValueWithRelatedType};
use super::super::values::Value;
use super::super::Set;
use super::*;

use std::fmt::{Display, Write};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Type {
    type_values: Set<TypeValue>,
}

impl Type {
    pub fn from_type_values(type_values: Set<TypeValue>) -> Type {
        Type { type_values }
    }
}

impl Type {
    pub fn get_type_values(&self) -> &Set<TypeValue> {
        &self.type_values
    }
}

impl Parameter for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TYPE=")?;

        let has_double_quote = self.type_values.as_hash_set().len() > 1;

        if has_double_quote {
            f.write_char('\"')?;
        }

        Value::fmt(&self.type_values, f)?;

        if has_double_quote {
            f.write_char('\"')?;
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
    type_values: Set<TypeValueWithTelephoneType>,
}

impl TypeWithTelType {
    pub fn from_type_values(type_values: Set<TypeValueWithTelephoneType>) -> TypeWithTelType {
        TypeWithTelType { type_values }
    }
}

impl TypeWithTelType {
    pub fn get_ids(&self) -> &Set<TypeValueWithTelephoneType> {
        &self.type_values
    }
}

impl Parameter for TypeWithTelType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TYPE=")?;

        let has_double_quote = self.type_values.as_hash_set().len() > 1;

        if has_double_quote {
            f.write_char('\"')?;
        }

        Value::fmt(&self.type_values, f)?;

        if has_double_quote {
            f.write_char('\"')?;
        }

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
    type_values: Set<TypeValueWithRelatedType>,
}

impl TypeWithRelatedType {
    pub fn from_type_values(type_values: Set<TypeValueWithRelatedType>) -> TypeWithRelatedType {
        TypeWithRelatedType { type_values }
    }
}

impl TypeWithRelatedType {
    pub fn get_type_values(&self) -> &Set<TypeValueWithRelatedType> {
        &self.type_values
    }
}

impl Parameter for TypeWithRelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TYPE=")?;

        let has_double_quote = self.type_values.as_hash_set().len() > 1;

        if has_double_quote {
            f.write_char('\"')?;
        }

        Value::fmt(&self.type_values, f)?;

        if has_double_quote {
            f.write_char('\"')?;
        }

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