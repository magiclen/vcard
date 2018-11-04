use std::fmt::{self, Formatter};

pub mod alternative_id;
pub mod language;
pub mod preference;
pub mod property_id;
pub mod typ;
pub mod value;
pub mod media_type;
pub mod calscale;
pub mod sort_as;
pub mod geo;
pub mod tz;

pub trait Parameter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}
