use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{
        parameters::{
            alternative_id::AlternativeID, any::Any, language::Language, media_type::MediaType,
            preference::Preference, property_id::PropertyID, typ::Type, Parameter,
        },
        values::{image_value::ImageValue, Value},
        Set,
    },
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Logo {
    pub typ:            Option<Type>,
    pub media_type:     Option<MediaType>,
    pub language:       Option<Language>,
    pub property_id:    Option<PropertyID>,
    pub preference:     Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any:            Option<Set<Any>>,
    pub value:          ImageValue,
}

impl Logo {
    pub fn from_image_value(image_value: ImageValue) -> Logo {
        Logo {
            typ:        None,
            media_type: None,
            language:   None,

            property_id:    None,
            preference:     None,
            alternative_id: None,
            any:            None,
            value:          image_value,
        }
    }
}

impl Property for Logo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("LOGO")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(0, typ);
        fmt!(0, media_type);
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

impl Display for Logo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Logo {}

impl ValidatedWrapper for Logo {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
