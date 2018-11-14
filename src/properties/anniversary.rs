use super::super::values::Value;
use super::super::values::date_time::*;
use super::super::values::text::Text;
use super::super::parameters::Parameter;
use super::super::parameters::calscale::Calscale;
use super::super::parameters::alternative_id::AlternativeID;
use super::super::parameters::any::Any;
use super::super::Set;
use super::*;

use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Anniversary {
    DateOrDateTime {
        calscale: Option<Calscale>,
        alternative_id: Option<AlternativeID>,
        any: Option<Set<Any>>,
        value: DateOrDateTime,
    },
    DateAndOrTime {
        alternative_id: Option<AlternativeID>,
        any: Option<Set<Any>>,
        value: DateAndOrTime,
    },
    Text {
        alternative_id: Option<AlternativeID>,
        any: Option<Set<Any>>,
        value: Text,
    },
}

impl Anniversary {
    pub fn from_date_or_date_time(date_or_date_time: DateOrDateTime) -> Anniversary {
        Anniversary::DateOrDateTime {
            calscale: None,

            alternative_id: None,
            any: None,
            value: date_or_date_time,
        }
    }

    pub fn from_date_and_or_time(date_and_or_time: DateAndOrTime) -> Anniversary {
        Anniversary::DateAndOrTime {
            alternative_id: None,
            any: None,
            value: date_and_or_time,
        }
    }

    pub fn from_text(text: Text) -> Anniversary {
        Anniversary::Text {
            alternative_id: None,
            any: None,
            value: text,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Anniversary::Text { value, .. } = self {
            return value.is_empty();
        }

        false
    }
}

impl Property for Anniversary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("BDAY")?;

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_gg!($c, Parameter, $p, f);
            };
        }

        match self {
            Anniversary::DateOrDateTime { calscale, alternative_id, any, value } => {
                fmt!(0, calscale);
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_char(':')?;

                Value::fmt(value, f)?;
            }
            Anniversary::DateAndOrTime { alternative_id, any, value } => {
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_char(':')?;

                Value::fmt(value, f)?;
            }
            Anniversary::Text { alternative_id, any, value } => {
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_str(";VALUE=text:")?;

                Value::fmt(value, f)?;
            }
        }

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Anniversary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Anniversary {}

impl ValidatedWrapper for Anniversary {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
