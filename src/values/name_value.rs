use super::text::Component;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameValue {
    last_name: Option<Component>,
    first_name: Option<Component>,
    middle_name: Option<Component>,
    name_prefix: Option<Component>,
    name_suffix: Option<Component>,
}

impl NameValue {
    pub fn from_components(last_name: Option<Component>, first_name: Option<Component>, middle_name: Option<Component>, name_prefix: Option<Component>, name_suffix: Option<Component>) -> NameValue {
        NameValue {
            last_name,
            first_name,
            middle_name,
            name_prefix,
            name_suffix,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(c) = &self.last_name {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.first_name {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.middle_name {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.name_prefix {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.name_suffix {
            if !c.is_empty() {
                return false;
            }
        }

        true
    }
}

impl NameValue {
    pub fn get_last_name(&self) -> Option<&Component> {
        self.last_name.as_ref()
    }

    pub fn get_first_name(&self) -> Option<&Component> {
        self.first_name.as_ref()
    }

    pub fn get_middle_name(&self) -> Option<&Component> {
        self.middle_name.as_ref()
    }

    pub fn get_name_prefix(&self) -> Option<&Component> {
        self.name_prefix.as_ref()
    }

    pub fn get_name_suffix(&self) -> Option<&Component> {
        self.name_suffix.as_ref()
    }
}

impl Value for NameValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Value, self, $p, f);
            };
        }

        fmt!(0, last_name);

        f.write_str(";")?;
        fmt!(0, first_name);

        f.write_str(";")?;
        fmt!(0, middle_name);

        f.write_str(";")?;
        fmt!(0, name_prefix);

        f.write_str(";")?;
        fmt!(0, name_suffix);

        Ok(())
    }
}

impl Display for NameValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for NameValue {}

impl ValidatedWrapper for NameValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}