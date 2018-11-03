use super::super::values::attribute_value::AttributeValue;
use super::super::values::{Value};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper, ValidatedCustomizedStringError};

use regex::Regex;

// TODO not implement yet, refer to [RFC4288]

lazy_static! {
    static ref MEDIA_TYPE_SEGMENT_RE: Regex = { Regex::new(r"^[\S]+$").unwrap() };
}

validated_customized_regex_string!(pub MediaTypeSegment, ref MEDIA_TYPE_SEGMENT_RE);

#[derive(Clone, Debug, PartialEq)]
pub struct MediaType {
    type_name: MediaTypeSegment,
    subtype_name: MediaTypeSegment,
    attribute_values: Vec<AttributeValue>,
}

impl MediaType {
    pub fn from_str(type_name: &str, subtype_name: &str, attribute_values: Vec<AttributeValue>) -> Result<MediaType, ValidatedCustomizedStringError> {
        Ok(Self::with_media_type_segments(MediaTypeSegment::from_str(type_name)?, MediaTypeSegment::from_str(subtype_name)?, attribute_values))
    }

    pub fn from_string(type_name: String, subtype_name: String, attribute_values: Vec<AttributeValue>) -> Result<MediaType, ValidatedCustomizedStringError> {
        Ok(Self::with_media_type_segments(MediaTypeSegment::from_string(type_name)?, MediaTypeSegment::from_string(subtype_name)?, attribute_values))
    }

    pub fn with_media_type_segments(type_name: MediaTypeSegment, subtype_name: MediaTypeSegment, attribute_values: Vec<AttributeValue>) -> MediaType {
        MediaType {
            type_name,
            subtype_name,
            attribute_values,
        }
    }

    pub fn is_empty(&self) -> bool {
        if !self.attribute_values.is_empty() {
            return false;
        }

        for v in &self.attribute_values {
            if !v.is_empty() {
                return false;
            }
        }

        true
    }
}

impl MediaType {
    pub fn get_type_name(&self) -> &MediaTypeSegment {
        &self.type_name
    }

    pub fn get_subtype_name(&self) -> &MediaTypeSegment {
        &self.subtype_name
    }

    pub fn get_attribute_values(&self) -> &Vec<AttributeValue> {
        &self.attribute_values
    }
}

impl Parameter for MediaType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str(";MEDIATYPE=")?;

        f.write_str(&percent_encoding::utf8_percent_encode(self.type_name.as_str(), percent_encoding::PATH_SEGMENT_ENCODE_SET).to_string())?;

        f.write_str("/")?;

        f.write_str(&percent_encoding::utf8_percent_encode(self.subtype_name.as_str(), percent_encoding::PATH_SEGMENT_ENCODE_SET).to_string())?;

        Value::fmt(&self.attribute_values, f)?;

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
