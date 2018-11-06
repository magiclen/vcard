use super::super::XPropertyName;
use super::super::values::Value;
use super::super::values::text::Text;
use super::super::parameters::Parameter;
use super::super::parameters::property_id::PropertyID;
use super::super::parameters::preference::Preference;
use super::super::parameters::alternative_id::AlternativeID;
use super::super::parameters::any::Any;
use super::super::parameters::typ::Type;
use super::super::parameters::media_type::MediaType;
use super::super::parameters::language::Language;
use super::super::parameters::sort_as::SortAs;
use super::super::parameters::calscale::Calscale;
use super::super::parameters::geo::Geo;
use super::super::parameters::label::Label;
use super::super::parameters::time_zone::TimeZone;
use super::super::parameters::value;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct XProperty {
    pub typ: Option<Type>,
    pub media_type: Option<MediaType>,
    pub language: Option<Language>,
    pub sort_as: Option<SortAs>,
    pub calscale: Option<Calscale>,
    pub geo: Option<Geo>,
    pub label: Option<Label>,
    pub time_zone: Option<TimeZone>,
    pub property_id: Option<PropertyID>,
    pub preference: Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any: Option<Set<Any>>,
    pub value_type: Option<value::Value>,
    pub name: XPropertyName,
    pub value: Text,
}

impl XProperty {
    pub fn from_text(name: XPropertyName, text: Text) -> XProperty {
        XProperty {
            typ: None,
            media_type: None,
            language: None,
            sort_as: None,
            calscale: None,
            geo: None,
            label: None,
            time_zone: None,

            property_id: None,
            preference: None,
            alternative_id: None,
            any: None,
            value_type: None,
            name,
            value: text,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl Property for XProperty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str(self.name.as_str())?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(0, typ);
        fmt!(0, media_type);
        fmt!(0, language);
        fmt!(0, sort_as);
        fmt!(0, calscale);
        fmt!(0, geo);
        fmt!(0, label);
        fmt!(0, time_zone);
        fmt!(0, property_id);
        fmt!(0, preference);
        fmt!(0, alternative_id);
        fmt!(2, any);
        fmt!(0, value_type);

        f.write_str(":")?;

        Value::fmt(&self.value, f)?;

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for XProperty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for XProperty {}

impl ValidatedWrapper for XProperty {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
