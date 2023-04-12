use std::fmt::{Display, Write};

use validators::{Validated, ValidatedWrapper};

use super::{text::Text, *};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SexType {
    Male,
    Female,
    Other,
    None,
    Unknown,
}

impl SexType {
    pub fn get_str(self) -> &'static str {
        match self {
            SexType::Male => "M",
            SexType::Female => "F",
            SexType::Other => "O",
            SexType::None => "N",
            SexType::Unknown => "U",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenderValue {
    sex_type:        Option<SexType>,
    gender_identity: Option<Text>,
}

impl GenderValue {
    pub fn from(sex_type: Option<SexType>, gender_identity: Option<Text>) -> GenderValue {
        GenderValue {
            sex_type,
            gender_identity,
        }
    }

    pub fn from_sex_type(sex_type: SexType) -> GenderValue {
        Self::from(Some(sex_type), None)
    }

    pub fn from_gender_identity(gender_identity: Text) -> GenderValue {
        Self::from(None, Some(gender_identity))
    }

    pub fn with(sex_type: SexType, gender_identity: Text) -> GenderValue {
        Self::from(Some(sex_type), Some(gender_identity))
    }

    pub fn is_empty(&self) -> bool {
        if self.sex_type.is_some() {
            return false;
        }

        if let Some(t) = &self.gender_identity {
            return t.is_empty();
        }

        true
    }
}

impl GenderValue {
    pub fn get_sex_type(&self) -> Option<SexType> {
        self.sex_type
    }

    pub fn get_gender_identity(&self) -> Option<&Text> {
        self.gender_identity.as_ref()
    }
}

impl Value for GenderValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        if let Some(t) = &self.sex_type {
            f.write_str(t.get_str())?;
        }

        if let Some(t) = &self.gender_identity {
            if t.is_empty() {
                f.write_char(';')?;
                Value::fmt(t, f)?;
            }
        }

        Ok(())
    }
}

impl Display for GenderValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for GenderValue {}

impl ValidatedWrapper for GenderValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
