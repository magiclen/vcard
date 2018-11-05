use super::super::values::Value;
use super::super::values::date_time::Timestamp;
use super::super::parameters::Parameter;
use super::super::parameters::any::Any;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Revision {
    pub any: Option<Set<Any>>,
    pub value: Timestamp,
}

impl Revision {
    pub fn from_timestamp(timestamp: Timestamp) -> Revision {
        Revision {
            any: None,
            value: timestamp,
        }
    }

    pub fn now() -> Revision {
        Self::from_timestamp(Timestamp::now())
    }
}

impl Property for Revision {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("REV")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Parameter, self, $p, f);
            };
        }

        fmt!(2, any);

        f.write_str(":")?;

        Value::fmt(&self.value, f)?;

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Revision {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Revision {}

impl ValidatedWrapper for Revision {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
