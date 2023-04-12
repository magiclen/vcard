use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{
        parameters::{
            alternative_id::AlternativeID, any::Any, language::Language, preference::Preference,
            property_id::PropertyID, sort_as::SortAs, typ::Type, Parameter,
        },
        values::{text::Component, Value},
        Set,
    },
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Organization {
    pub typ:            Option<Type>,
    pub language:       Option<Language>,
    pub sort_as:        Option<SortAs>,
    pub property_id:    Option<PropertyID>,
    pub preference:     Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any:            Option<Set<Any>>,
    pub value:          Set<Component>,
}

impl Organization {
    pub fn from_component_list(component_list: Set<Component>) -> Organization {
        Organization {
            typ:      None,
            language: None,
            sort_as:  None,

            property_id:    None,
            preference:     None,
            alternative_id: None,
            any:            None,
            value:          component_list,
        }
    }

    pub fn is_empty(&self) -> bool {
        for e in self.value.as_hash_set() {
            if !e.is_empty() {
                return false;
            }
        }

        true
    }
}

impl Property for Organization {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("ORG")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(0, typ);
        fmt!(0, language);
        fmt!(0, sort_as);
        fmt!(0, property_id);
        fmt!(0, preference);
        fmt!(0, alternative_id);
        fmt!(2, any);

        f.write_char(':')?;

        let v = self.value.as_hash_set();

        for e in v.iter().take(1) {
            Value::fmt(e, f)?;
        }

        for e in v.iter().skip(1) {
            f.write_char(';')?;
            Value::fmt(e, f)?;
        }

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Organization {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Organization {}

impl ValidatedWrapper for Organization {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
