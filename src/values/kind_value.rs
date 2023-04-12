use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{IanaToken, XName},
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KindValue {
    Individual,
    Group,
    Organization,
    Location,
    IanaToken(IanaToken),
    XName(XName),
}

impl KindValue {
    pub fn get_str(&self) -> &str {
        match self {
            KindValue::Individual => "individual",
            KindValue::Group => "group",
            KindValue::Organization => "org",
            KindValue::Location => "location",
            KindValue::IanaToken(x) => x.as_str(),
            KindValue::XName(x) => x.as_str(),
        }
    }
}

impl Value for KindValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for KindValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for KindValue {}

impl ValidatedWrapper for KindValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
