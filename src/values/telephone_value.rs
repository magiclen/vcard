use super::uri::URI;
use super::*;

use std::fmt::Display;

use regex::Regex;

use validators::{Validated, ValidatedWrapper, ValidatedCustomizedPhoneNumberError};

lazy_static! {
    static ref TEL_EXTENSION_RE: Regex = { Regex::new(r"^[\-0-9]+$").unwrap() };
    static ref SPACES_RE: Regex = { Regex::new("[ ]+").unwrap() };
    static ref PLUS_RE: Regex = { Regex::new(r"\+").unwrap() };
}

validated_customized_phone_number!(pub TelephoneNumber);
validated_customized_regex_string!(pub TelephoneExtension, ref TEL_EXTENSION_RE);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TelephoneValue {
    URI(URI),
    TelephoneNumber {
        telephone_number: TelephoneNumber,
        extension: Option<TelephoneExtension>,
    },
}

impl TelephoneValue {
    pub fn from_telephone_number_str<S: AsRef<str>, SS: AsRef<str>>(
        telephone_number: S,
        extension: Option<SS>,
    ) -> Result<TelephoneValue, ValidatedCustomizedPhoneNumberError> {
        let telephone_number = SPACES_RE.replace_all(telephone_number.as_ref(), "-");
        let telephone_number_no_plus = PLUS_RE.replace_all(telephone_number.as_ref(), "");
        let telephone_number = TelephoneNumber::from_str(telephone_number_no_plus.as_ref())?;

        let extension = match extension {
            Some(extension) => {
                let extension = SPACES_RE.replace_all(extension.as_ref(), "-");
                Some(TelephoneExtension::from_str(extension.as_ref()).map_err(|_| ValidatedCustomizedPhoneNumberError::IncorrectFormat)?)
            }
            None => None,
        };

        Ok(TelephoneValue::TelephoneNumber {
            telephone_number,
            extension,
        })
    }
}

impl Value for TelephoneValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            TelephoneValue::URI(uri) => {
                Value::fmt(uri, f)?;
            }
            TelephoneValue::TelephoneNumber { telephone_number, extension } => {
                f.write_fmt(format_args!("tel:{}", telephone_number))?;
                if let Some(extension) = extension {
                    f.write_str(";ext=")?;
                    f.write_str(extension.as_str())?;
                }
            }
        }

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