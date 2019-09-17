use std::fmt::{self, Formatter};

pub mod alternative_id;
pub mod any;
pub mod calscale;
pub mod geo;
pub mod label;
pub mod language;
pub mod media_type;
pub mod preference;
pub mod property_id;
pub mod sort_as;
pub mod time_zone;
pub mod typ;
pub mod value;

pub trait Parameter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}
