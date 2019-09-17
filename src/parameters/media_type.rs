use super::super::values::attribute_value::AttributeValue;
use super::super::values::Value;
use super::super::Set;
use super::*;
use crate::PATH_PERCENT_ENCODE_SET;

use std::fmt::{Display, Write};

use validators::{Validated, ValidatedCustomizedStringError, ValidatedWrapper};

use regex::Regex;

// TODO not implement yet, refer to [RFC4288]

lazy_static! {
    static ref MEDIA_TYPE_SEGMENT_RE: Regex =
        { Regex::new(r"^[^\x00-\x1F\x22\x3A\x3B\x7F]+$").unwrap() };
}

validated_customized_regex_string!(pub MediaTypeSegment, ref MEDIA_TYPE_SEGMENT_RE);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaType {
    type_name: MediaTypeSegment,
    subtype_name: MediaTypeSegment,
    attribute_values: Option<Set<AttributeValue>>,
}

impl MediaType {
    pub fn from_str(
        type_name: &str,
        subtype_name: &str,
        attribute_values: Option<Set<AttributeValue>>,
    ) -> Result<MediaType, ValidatedCustomizedStringError> {
        Ok(Self::from_media_type_segments(
            MediaTypeSegment::from_str(type_name)?,
            MediaTypeSegment::from_str(subtype_name)?,
            attribute_values,
        ))
    }

    pub fn from_string(
        type_name: String,
        subtype_name: String,
        attribute_values: Option<Set<AttributeValue>>,
    ) -> Result<MediaType, ValidatedCustomizedStringError> {
        Ok(Self::from_media_type_segments(
            MediaTypeSegment::from_string(type_name)?,
            MediaTypeSegment::from_string(subtype_name)?,
            attribute_values,
        ))
    }

    pub fn from_media_type_segments(
        type_name: MediaTypeSegment,
        subtype_name: MediaTypeSegment,
        attribute_values: Option<Set<AttributeValue>>,
    ) -> MediaType {
        MediaType {
            type_name,
            subtype_name,
            attribute_values,
        }
    }
}

impl MediaType {
    pub fn get_type_name(&self) -> &MediaTypeSegment {
        &self.type_name
    }

    pub fn get_subtype_name(&self) -> &MediaTypeSegment {
        &self.subtype_name
    }

    pub fn get_attribute_values(&self) -> Option<&Set<AttributeValue>> {
        self.attribute_values.as_ref()
    }
}

impl Parameter for MediaType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";MEDIATYPE=")?;

        f.write_str(
            &percent_encoding::utf8_percent_encode(
                self.type_name.as_str(),
                PATH_PERCENT_ENCODE_SET,
            )
            .to_string(),
        )?;

        f.write_char('/')?;

        f.write_str(
            &percent_encoding::utf8_percent_encode(
                self.subtype_name.as_str(),
                PATH_PERCENT_ENCODE_SET,
            )
            .to_string(),
        )?;

        if let Some(attribute_values) = &self.attribute_values {
            Value::fmt(attribute_values, f)?;
        }

        Ok(())
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for MediaType {}

impl ValidatedWrapper for MediaType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
