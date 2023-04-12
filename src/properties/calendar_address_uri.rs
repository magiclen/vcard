use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{
        parameters::{
            alternative_id::AlternativeID, any::Any, media_type::MediaType, preference::Preference,
            property_id::PropertyID, typ::Type, Parameter,
        },
        values::{uri::URI, Value},
        Set,
    },
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CalendarAddressURI {
    pub typ:            Option<Type>,
    pub media_type:     Option<MediaType>,
    pub property_id:    Option<PropertyID>,
    pub preference:     Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any:            Option<Set<Any>>,
    pub value:          URI,
}

impl CalendarAddressURI {
    pub fn from_uri(uri: URI) -> CalendarAddressURI {
        CalendarAddressURI {
            typ:        None,
            media_type: None,

            property_id:    None,
            preference:     None,
            alternative_id: None,
            any:            None,
            value:          uri,
        }
    }
}

impl Property for CalendarAddressURI {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("CALADRURI")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(0, typ);
        fmt!(0, media_type);
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

impl Display for CalendarAddressURI {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for CalendarAddressURI {}

impl ValidatedWrapper for CalendarAddressURI {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
