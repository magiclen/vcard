use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::{super::values::language_tag::LanguageTag, *};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Language {
    language_tag: LanguageTag,
}

impl Language {
    pub fn from_language_tag(language_tag: LanguageTag) -> Language {
        Language {
            language_tag,
        }
    }
}

impl Language {
    pub fn get_language_tag(&self) -> &LanguageTag {
        &self.language_tag
    }
}

impl Parameter for Language {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";LANGUAGE=")?;
        f.write_str(self.language_tag.as_str())?;

        Ok(())
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Language {}

impl ValidatedWrapper for Language {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
