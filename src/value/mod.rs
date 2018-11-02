use std::fmt::{self, Formatter};

pub mod text;
pub mod uri;
pub mod date_time;
pub mod boolean;
pub mod integer;
pub mod float;

validated_customized_ranged_length_vec!(pub List, 1, usize::max_value());

pub trait Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}