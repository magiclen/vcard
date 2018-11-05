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

use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NickName {
    pub typ: Option<Type>,
    pub language: Option<Language>,
    pub property_id: Option<PropertyID>,
    pub preference: Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any: Option<Set<Any>>,
    pub value: Option<Set<Text>>,
}

impl NickName {
    pub fn from_text_list(text_list: Option<Set<Text>>) -> NickName {
        NickName {
            typ: None,
            language: None,

            property_id: None,
            preference: None,
            alternative_id: None,
            any: None,
            value: text_list,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(v) = &self.value {
            for e in v.as_hash_set() {
                if !e.is_empty() {
                    return false;
                }
            }
        }

        true
    }
}

impl Property for NickName {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("NICKNAME")?;

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

        f.write_str(":")?;

        if let Some(v) = &self.value {
            Value::fmt(v, f)?;
        }

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for NickName {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for NickName {}

impl ValidatedWrapper for NickName {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
