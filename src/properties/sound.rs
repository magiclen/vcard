use super::super::values::Value;
use super::super::values::audio_value::AudioValue;
use super::super::parameters::Parameter;
use super::super::parameters::property_id::PropertyID;
use super::super::parameters::preference::Preference;
use super::super::parameters::alternative_id::AlternativeID;
use super::super::parameters::any::Any;
use super::super::parameters::typ::Type;
use super::super::parameters::media_type::MediaType;
use super::super::parameters::language::Language;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sound {
    pub typ: Option<Type>,
    pub media_type: Option<MediaType>,
    pub language: Option<Language>,
    pub property_id: Option<PropertyID>,
    pub preference: Option<Preference>,
    pub alternative_id: Option<AlternativeID>,
    pub any: Option<Set<Any>>,
    pub value: AudioValue,
}

impl Sound {
    pub fn from_sound_value(sound_value: AudioValue) -> Sound {
        Sound {
            typ: None,
            media_type: None,
            language: None,

            property_id: None,
            preference: None,
            alternative_id: None,
            any: None,
            value: sound_value,
        }
    }
}

impl Property for Sound {
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

        f.write_str(":")?;

        Value::fmt(&self.value, f)?;

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Sound {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Sound {}

impl ValidatedWrapper for Sound {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}