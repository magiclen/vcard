use std::fmt::{self, Formatter};

pub mod boolean;
pub mod date_time;
pub mod float;
pub mod integer;
pub mod language_tag;
pub mod parameter_value;
pub mod property_id_value;
pub mod text;
pub mod type_value;
pub mod uri;

validated_customized_ranged_length_vec!(pub List, 1, usize::max_value());

pub trait Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}
