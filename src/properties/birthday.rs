use std::fmt::{self, Display, Formatter, Write};

use validators::{Validated, ValidatedWrapper};

use super::{
    super::{
        parameters::{
            alternative_id::AlternativeID, any::Any, calscale::Calscale, language::Language,
            Parameter,
        },
        values::{date_time::*, text::Text, Value},
        Set,
    },
    *,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Birthday {
    DateOrDateTime {
        calscale:       Option<Calscale>,
        alternative_id: Option<AlternativeID>,
        any:            Option<Set<Any>>,
        value:          DateOrDateTime,
    },
    DateAndOrTime {
        alternative_id: Option<AlternativeID>,
        any:            Option<Set<Any>>,
        value:          DateAndOrTime,
    },
    Text {
        language:       Option<Language>,
        alternative_id: Option<AlternativeID>,
        any:            Option<Set<Any>>,
        value:          Text,
    },
}

impl Birthday {
    pub fn from_date_or_date_time(date_or_date_time: DateOrDateTime) -> Birthday {
        Birthday::DateOrDateTime {
            calscale: None,

            alternative_id: None,
            any:            None,
            value:          date_or_date_time,
        }
    }

    pub fn from_date_and_or_time(date_and_or_time: DateAndOrTime) -> Birthday {
        Birthday::DateAndOrTime {
            alternative_id: None,
            any:            None,
            value:          date_and_or_time,
        }
    }

    pub fn from_text(text: Text) -> Birthday {
        Birthday::Text {
            language: None,

            alternative_id: None,
            any:            None,
            value:          text,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Birthday::Text {
            value, ..
        } = self
        {
            return value.is_empty();
        }

        false
    }
}

impl Property for Birthday {
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
            Birthday::DateOrDateTime {
                calscale,
                alternative_id,
                any,
                value,
            } => {
                fmt!(0, calscale);
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_char(':')?;

                Value::fmt(value, f)?;
            },
            Birthday::DateAndOrTime {
                alternative_id,
                any,
                value,
            } => {
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_char(':')?;

                Value::fmt(value, f)?;
            },
            Birthday::Text {
                language,
                alternative_id,
                any,
                value,
            } => {
                fmt!(0, language);
                fmt!(0, alternative_id);
                fmt!(2, any);

                f.write_str(";VALUE=text:")?;

                Value::fmt(value, f)?;
            },
        }

        f.write_str("\r\n")?;

        Ok(())
    }
}

impl Display for Birthday {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Property::fmt(self, f)
    }
}

impl Validated for Birthday {}

impl ValidatedWrapper for Birthday {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
