use super::super::values::Value;
use super::super::values::text::Text;
use super::super::parameters::Parameter;
use super::super::parameters::property_id::PropertyID;
use super::super::parameters::preference::Preference;
use super::super::parameters::alternative_id::AlternativeID;
use super::super::parameters::any::Any;
use super::super::parameters::typ::Type;
use super::super::parameters::language::Language;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Role {
    pub typ: Option<Type>,
    pub language: Option<Language>,
    pub property_id: Option<PropertyID>,
    pub preference: Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any: Option<Set<Any>>,
    pub value: Text,
}

impl Role {
    pub fn from_text(text: Text) -> Role {
        Role {
            typ: None,
            language: None,

            property_id: None,
            preference: None,
            alternative_id: None,
            any: None,
            value: text,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl Property for Role {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("ROLE")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(0, typ);
        fmt!(0, language);
        fmt!(0, property_id);
        fmt!(0, preference);
        fmt!(0, alternative_id);
        fmt!(2, any);

        f.write_char(':')?;

        Value::fmt(&self.value, f)?;

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Role {}

impl ValidatedWrapper for Role {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
