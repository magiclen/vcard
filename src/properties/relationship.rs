use super::super::parameters::alternative_id::AlternativeID;
use super::super::parameters::any::Any;
use super::super::parameters::language::Language;
use super::super::parameters::media_type::MediaType;
use super::super::parameters::preference::Preference;
use super::super::parameters::property_id::PropertyID;
use super::super::parameters::typ::TypeWithRelatedType;
use super::super::parameters::Parameter;
use super::super::values::text::Text;
use super::super::values::uri::URI;
use super::super::values::Value;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::large_enum_variant)]
pub enum Relationship {
    URI {
        typ: Option<TypeWithRelatedType>,
        media_type: Option<MediaType>,

        property_id: Option<PropertyID>,
        preference: Option<Preference>,
        alternative_id: Option<AlternativeID>,
        any: Option<Set<Any>>,
        value: URI,
    },
    Text {
        typ: Option<TypeWithRelatedType>,
        language: Option<Language>,

        property_id: Option<PropertyID>,
        preference: Option<Preference>,
        alternative_id: Option<AlternativeID>,
        any: Option<Set<Any>>,
        value: Text,
    },
}

impl Relationship {
    pub fn from_text(text: Text) -> Relationship {
        Relationship::Text {
            typ: None,
            language: None,

            property_id: None,
            preference: None,
            alternative_id: None,
            any: None,
            value: text,
        }
    }

    pub fn from_uri(uri: URI) -> Relationship {
        Relationship::URI {
            typ: None,
            media_type: None,

            property_id: None,
            preference: None,
            alternative_id: None,
            any: None,
            value: uri,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Relationship::Text {
            value,
            ..
        } = self
        {
            return value.is_empty();
        }

        false
    }
}

impl Property for Relationship {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("RELATED")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_gg!($c, Parameter, $p, f);
            };
        }

        match self {
            Relationship::URI {
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

                f.write_char(':')?;

                Value::fmt(value, f)?;
            }
            Relationship::Text {
                typ,
                language,
                property_id,
                preference,
                alternative_id,
                any,
                value,
            } => {
                fmt!(0, typ);
                fmt!(0, language);
                fmt!(0, property_id);
                fmt!(0, preference);
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_str(";VALUE=text:")?;

                Value::fmt(value, f)?;
            }
        }

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Relationship {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Relationship {}

impl ValidatedWrapper for Relationship {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
