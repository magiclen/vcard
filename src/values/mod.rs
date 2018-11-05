use super::Set;

use std::fmt::{self, Formatter};
use std::collections::HashSet;
use std::hash::Hash;
use validators::ValidatedWrapper;

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
pub mod attribute_value;
pub mod calscale_value;
pub mod value_type;
pub mod telephone_type;
pub mod telephone_value;
pub mod related_type;
pub mod geo_value;
pub mod kind_value;
pub mod preference_value;
pub mod version_value;
pub mod name_value;
pub mod image_value;
pub mod gender_value;
pub mod address_value;
pub mod email_value;
pub mod time_zone_value;

pub trait Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}

impl<V: Value + ValidatedWrapper + Eq + Hash> Value for Set<V> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &HashSet<V> = self.as_hash_set();

        for e in v.iter().take(1) {
            Value::fmt(e, f)?;
        }

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}