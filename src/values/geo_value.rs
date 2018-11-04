use super::uri::URI;
use super::*;

use std::fmt::Display;

use validators::{Validated, ValidatedWrapper};

validated_customized_ranged_number!(pub Latitude, f64, -90.0, 90.0);
validated_customized_ranged_number!(pub Longitude, f64, -180.0, 180.0);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GeoValue {
    URI(URI),
    LatLng(Latitude, Longitude),
}

impl GeoValue {
    pub fn get_uri_string(&self) -> String {
        match self {
            GeoValue::URI(uri) => uri.get_full_uri().to_string(),
            GeoValue::LatLng(lat, lng) => format!("GEO:geo:{:.6}:{:.6}", lat.get_number(), lng.get_number())
        }
    }
}

impl Value for GeoValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.get_uri_string())?;

        Ok(())
    }
}

impl Display for GeoValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for GeoValue {}

impl ValidatedWrapper for GeoValue {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}