use super::uri::URI;
use super::*;

use std::fmt::Display;

use regex::Regex;

use validators::{Validated, ValidatedWrapper, ValidatedCustomizedStringError};

lazy_static! {
    static ref TEL_NUMBER_RE: Regex = { Regex::new(r"^[+\-0-9]+$").unwrap() };
    static ref SPACES_RE: Regex = { Regex::new("[ ]+").unwrap() };
}

validated_customized_regex_string!(pub TelephoneNumber, ref TEL_NUMBER_RE);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TelephoneValue {
    URI(URI),
    TelephoneNumber {
        telephone_number: TelephoneNumber,
        extension: Option<TelephoneNumber>,
    },
}

impl TelephoneValue {
    pub fn from_telephone_number_str<S: AsRef<str>, SS: AsRef<str>>(
        telephone_number: S,
        extension: Option<SS>,
    ) -> Result<TelephoneValue, ValidatedCustomizedStringError> {
        let telephone_number = SPACES_RE.replace_all(telephone_number.as_ref(), "-");
        let telephone_number = TelephoneNumber::from_str(telephone_number.as_ref())?;

        let extension = match extension {
            Some(extension) => {
                let extension = SPACES_RE.replace_all(extension.as_ref(), "-");
                Some(TelephoneNumber::from_str(extension.as_ref())?)
            }
            None => None,
        };

        Ok(TelephoneValue::TelephoneNumber {
            telephone_number,
            extension,
        })
    }

    pub fn get_uri_string(&self) -> String {
        match self {
            TelephoneValue::URI(uri) => uri.get_full_uri().to_string(),
            TelephoneValue::TelephoneNumber{telephone_number, extension} => {
                let mut s = String::from("tel:");
                s.push_str(telephone_number.as_str());

                if let Some(extension) = extension {
                    s.push_str(";ext=");
                    s.push_str(extension.as_str());
                }

                s
            }
        }
    }
}

impl Value for TelephoneValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.get_uri_string())?;

        Ok(())
    }
}

impl Display for TelephoneValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TelephoneValue {}

impl ValidatedWrapper for TelephoneValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}