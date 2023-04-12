use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{
        parameters::{
            alternative_id::AlternativeID, any::Any, media_type::MediaType, preference::Preference,
            property_id::PropertyID, typ::TypeWithTelType, Parameter,
        },
        values::{telephone_value::TelephoneValue, text::Text, Value},
        Set,
    },
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum Telephone {
    Text {
        typ: Option<TypeWithTelType>,

        property_id:    Option<PropertyID>,
        preference:     Option<Preference>,
        alternative_id: Option<AlternativeID>,
        any:            Option<Set<Any>>,
        value:          Text,
    },
    TelephoneValue {
        typ:        Option<TypeWithTelType>,
        media_type: Option<MediaType>,

        property_id:    Option<PropertyID>,
        preference:     Option<Preference>,
        alternative_id: Option<AlternativeID>,
        any:            Option<Set<Any>>,
        value:          TelephoneValue,
    },
}

impl Telephone {
    pub fn from_text(text: Text) -> Telephone {
        Telephone::Text {
            typ: None,

            property_id:    None,
            preference:     None,
            alternative_id: None,
            any:            None,
            value:          text,
        }
    }

    pub fn from_telephone_value(telephone_value: TelephoneValue) -> Telephone {
        Telephone::TelephoneValue {
            typ:        None,
            media_type: None,

            property_id:    None,
            preference:     None,
            alternative_id: None,
            any:            None,
            value:          telephone_value,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Telephone::Text {
            value, ..
        } = self
        {
            return value.is_empty();
        }

        false
    }
}

impl Property for Telephone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("TEL")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_gg!($c, Parameter, $p, f);
            };
        }

        match self {
            Telephone::Text {
                typ,
                property_id,
                preference,
                alternative_id,
                any,
                value,
            } => {
                fmt!(0, typ);
                fmt!(0, property_id);
                fmt!(0, preference);
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_char(':')?;

                Value::fmt(value, f)?;
            },
            Telephone::TelephoneValue {
                typ,
                media_type,
                property_id,
                preference,
                alternative_id,
                any,
                value,
            } => {
                fmt!(0, typ);
                fmt!(0, media_type);
                fmt!(0, property_id);
                fmt!(0, preference);
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_str(";VALUE=uri:")?;

                Value::fmt(value, f)?;
            },
        }

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Telephone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Telephone {}

impl ValidatedWrapper for Telephone {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
