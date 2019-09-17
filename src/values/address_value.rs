use super::text::Component;
use super::*;

use std::fmt::{Display, Write};

use validators::{Validated, ValidatedWrapper};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AddressValue {
    post_office_box: Option<Component>,
    extension: Option<Component>,
    street: Option<Component>,
    locality: Option<Component>,
    region: Option<Component>,
    code: Option<Component>,
    country: Option<Component>,
}

impl AddressValue {
    pub fn from_components(
        post_office_box: Option<Component>,
        extension: Option<Component>,
        street: Option<Component>,
        locality: Option<Component>,
        region: Option<Component>,
        code: Option<Component>,
        country: Option<Component>,
    ) -> AddressValue {
        AddressValue {
            post_office_box,
            extension,
            street,
            locality,
            region,
            code,
            country,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(c) = &self.post_office_box {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.extension {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.street {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.locality {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.region {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.code {
            if !c.is_empty() {
                return false;
            }
        }
        if let Some(c) = &self.country {
            if !c.is_empty() {
                return false;
            }
        }

        true
    }
}

impl AddressValue {
    pub fn get_post_office_box(&self) -> Option<&Component> {
        self.post_office_box.as_ref()
    }

    pub fn get_extension(&self) -> Option<&Component> {
        self.extension.as_ref()
    }

    pub fn get_street(&self) -> Option<&Component> {
        self.street.as_ref()
    }

    pub fn get_locality(&self) -> Option<&Component> {
        self.locality.as_ref()
    }

    pub fn get_region(&self) -> Option<&Component> {
        self.region.as_ref()
    }

    pub fn get_code(&self) -> Option<&Component> {
        self.code.as_ref()
    }

    pub fn get_country(&self) -> Option<&Component> {
        self.country.as_ref()
    }
}

impl Value for AddressValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Value, self, $p, f);
            };
        }

        fmt!(0, post_office_box);

        f.write_char(';')?;
        fmt!(0, extension);

        f.write_char(';')?;
        fmt!(0, street);

        f.write_char(';')?;
        fmt!(0, locality);

        f.write_char(';')?;
        fmt!(0, region);

        f.write_char(';')?;
        fmt!(0, code);

        f.write_char(';')?;
        fmt!(0, country);

        Ok(())
    }
}

impl Display for AddressValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for AddressValue {}

impl ValidatedWrapper for AddressValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
