use std::fmt::{Display, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::values::{time_zone_value::TimeZoneValue, Value},
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TimeZone {
    time_zone_value: TimeZoneValue,
}

impl TimeZone {
    pub fn from_time_zone_value(time_zone_value: TimeZoneValue) -> TimeZone {
        TimeZone {
            time_zone_value,
        }
    }
}

impl Parameter for TimeZone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(";TZ=")?;

        match &self.time_zone_value {
            TimeZoneValue::Tz(_) => {
                Value::fmt(&self.time_zone_value, f)?;
            },
            TimeZoneValue::URI(_) => {
                f.write_char('\"')?;
                Value::fmt(&self.time_zone_value, f)?;
                f.write_char('\"')?;
            },
        }

        Ok(())
    }
}

impl Display for TimeZone {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Parameter::fmt(self, f)
    }
}

impl Validated for TimeZone {}

impl ValidatedWrapper for TimeZone {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
