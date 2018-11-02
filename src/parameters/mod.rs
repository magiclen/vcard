use std::fmt::{self, Formatter};

pub mod language;
pub mod value;
pub mod preference;
pub mod alternative_id;
pub mod property_id;

pub trait Parameter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}