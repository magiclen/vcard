use super::super::values::Value;
use super::super::values::client_property_id_map_value::ClientPropertyIDMapValue;
use super::super::parameters::Parameter;
use super::super::parameters::any::Any;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientPropertyIDMap {
    pub any: Option<Set<Any>>,
    pub value: ClientPropertyIDMapValue,
}

impl ClientPropertyIDMap {
    pub fn from_client_property_id_map_value(client_property_id_map_value: ClientPropertyIDMapValue) -> ClientPropertyIDMap {
        ClientPropertyIDMap {
            any: None,
            value: client_property_id_map_value,
        }
    }
}

impl Property for ClientPropertyIDMap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("CLIENTPIDMAP")?;

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

impl Display for ClientPropertyIDMap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for ClientPropertyIDMap {}

impl ValidatedWrapper for ClientPropertyIDMap {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
