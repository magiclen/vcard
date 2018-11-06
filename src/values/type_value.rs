use super::super::{IanaToken, XName};
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeValue {
    Work,
    Home,
    IanaToken(IanaToken),
    XName(XName),
}

impl TypeValue {
    pub fn get_str(&self) -> &str {
        match self {
            TypeValue::Work => "work",
            TypeValue::Home => "home",
            TypeValue::IanaToken(x) => x.as_str(),
            TypeValue::XName(x) => x.as_str(),
        }
    }
}

impl Value for TypeValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TypeValue {}

impl ValidatedWrapper for TypeValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeValueWithTelephoneType {
    Work,
    Home,
    Text,
    Voice,
    Fax,
    Cell,
    Video,
    Pager,
    TextPhone,
    IanaToken(IanaToken),
    XName(XName),
}

impl TypeValueWithTelephoneType {
    pub fn get_str(&self) -> &str {
        match self {
            TypeValueWithTelephoneType::Work => "work",
            TypeValueWithTelephoneType::Home => "home",
            TypeValueWithTelephoneType::Text => "text",
            TypeValueWithTelephoneType::Voice => "voice",
            TypeValueWithTelephoneType::Fax => "fax",
            TypeValueWithTelephoneType::Cell => "cell",
            TypeValueWithTelephoneType::Video => "video",
            TypeValueWithTelephoneType::Pager => "pager",
            TypeValueWithTelephoneType::TextPhone => "textphone",
            TypeValueWithTelephoneType::IanaToken(x) => x.as_str(),
            TypeValueWithTelephoneType::XName(x) => x.as_str(),
        }
    }
}

impl Value for TypeValueWithTelephoneType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TypeValueWithTelephoneType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TypeValueWithTelephoneType {}

impl ValidatedWrapper for TypeValueWithTelephoneType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeValueWithRelatedType {
    Work,
    Home,
    Contact,
    Acquaintance,
    Friend,
    Met,
    CoWorker,
    Colleague,
    CoResident,
    Neighbor,
    Child,
    Parent,
    Spouse,
    Kin,
    Muse,
    Crush,
    Date,
    Sweetheart,
    Me,
    Agent,
    Emergency,
    IanaToken(IanaToken),
    XName(XName),
}

impl TypeValueWithRelatedType {
    pub fn get_str(&self) -> &str {
        match self {
            TypeValueWithRelatedType::Work => "work",
            TypeValueWithRelatedType::Home => "home",
            TypeValueWithRelatedType::Contact => "contact",
            TypeValueWithRelatedType::Acquaintance => "acquaintance",
            TypeValueWithRelatedType::Friend => "friend",
            TypeValueWithRelatedType::Met => "met",
            TypeValueWithRelatedType::CoWorker => "co-worker",
            TypeValueWithRelatedType::Colleague => "colleague",
            TypeValueWithRelatedType::CoResident => "co-resident",
            TypeValueWithRelatedType::Neighbor => "neighbor",
            TypeValueWithRelatedType::Child => "child",
            TypeValueWithRelatedType::Parent => "parent",
            TypeValueWithRelatedType::Spouse => "spouse",
            TypeValueWithRelatedType::Kin => "kin",
            TypeValueWithRelatedType::Muse => "muse",
            TypeValueWithRelatedType::Crush => "crush",
            TypeValueWithRelatedType::Date => "date",
            TypeValueWithRelatedType::Sweetheart => "sweetheart",
            TypeValueWithRelatedType::Me => "me",
            TypeValueWithRelatedType::Agent => "agent",
            TypeValueWithRelatedType::Emergency => "emergency",
            TypeValueWithRelatedType::IanaToken(x) => x.as_str(),
            TypeValueWithRelatedType::XName(x) => x.as_str(),
        }
    }
}

impl Value for TypeValueWithRelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for TypeValueWithRelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for TypeValueWithRelatedType {}

impl ValidatedWrapper for TypeValueWithRelatedType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}