use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{
        parameters::{
            alternative_id::AlternativeID, any::Any, language::Language, property_id::PropertyID,
            sort_as::SortAs, Parameter,
        },
        values::{name_value::NameValue, Value},
        Set,
    },
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    pub sort_as:        Option<SortAs>,
    pub language:       Option<Language>,
    pub property_id:    Option<PropertyID>,
    pub alternative_id: Option<AlternativeID>,
    pub any:            Option<Set<Any>>,
    pub value:          NameValue,
}

impl Name {
    pub fn from_name_value(name_value: NameValue) -> Name {
        Name {
            sort_as:  None,
            language: None,

            property_id:    None,
            alternative_id: None,
            any:            None,
            value:          name_value,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl Property for Name {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_char('N')?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(0, sort_as);
        fmt!(0, language);
        fmt!(0, property_id);
        fmt!(0, alternative_id);
        fmt!(2, any);

        f.write_char(':')?;

        Value::fmt(&self.value, f)?;

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Name {}

impl ValidatedWrapper for Name {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
