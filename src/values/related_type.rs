use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelatedType {
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
}

impl RelatedType {
    pub fn get_str(&self) -> &str {
        match self {
            RelatedType::Contact => "contact",
            RelatedType::Acquaintance => "acquaintance",
            RelatedType::Friend => "friend",
            RelatedType::Met => "met",
            RelatedType::CoWorker => "co-worker",
            RelatedType::Colleague => "colleague",
            RelatedType::CoResident => "co-resident",
            RelatedType::Neighbor => "neighbor",
            RelatedType::Child => "child",
            RelatedType::Parent => "parent",
            RelatedType::Spouse => "spouse",
            RelatedType::Kin => "kin",
            RelatedType::Muse => "muse",
            RelatedType::Crush => "crush",
            RelatedType::Date => "date",
            RelatedType::Sweetheart => "sweetheart",
            RelatedType::Me => "me",
            RelatedType::Agent => "agent",
            RelatedType::Emergency => "emergency",
        }
    }
}

impl Value for RelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_str())?;

        Ok(())
    }
}

impl Display for RelatedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for RelatedType {}

impl ValidatedWrapper for RelatedType {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}