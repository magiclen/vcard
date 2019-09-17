use super::super::values::preference_value::PreferenceValue;
use super::super::values::Value;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Preference {
    preference_value: PreferenceValue,
}

impl Preference {
    pub fn from_preference_value(preference_value: PreferenceValue) -> Preference {
        Preference {
            preference_value,
        }
    }
}

impl Preference {
    pub fn get_preference_value(&self) -> &PreferenceValue {
        &self.preference_value
    }
}

impl Parameter for Preference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";PREF=")?;

        Value::fmt(&self.preference_value, f)?;

        Ok(())
    }
}

impl Display for Preference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for Preference {}

impl ValidatedWrapper for Preference {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
