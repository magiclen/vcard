use std::fmt::{self, Formatter};

pub mod alternative_id;
pub mod language;
pub mod preference;
pub mod property_id;
pub mod typ;
pub mod value;

pub trait Parameter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}
