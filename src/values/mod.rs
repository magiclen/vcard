use std::{
    collections::HashSet,
    fmt::{self, Formatter, Write},
    hash::Hash,
};

use validators::ValidatedWrapper;

use super::Set;

pub mod address_value;
pub mod attribute_value;
pub mod audio_value;
pub mod boolean;
pub mod calscale_value;
pub mod client_property_id_map_value;
pub mod date_time;
pub mod email_value;
pub mod float;
pub mod gender_value;
pub mod geo_value;
pub mod image_value;
pub mod integer;
pub mod key_value;
pub mod kind_value;
pub mod language_tag;
pub mod name_value;
pub mod parameter_value;
pub mod preference_value;
pub mod product_id_value;
pub mod property_id_value;
pub mod telephone_value;
pub mod text;
pub mod time_zone_value;
pub mod type_value;
pub mod uid_value;
pub mod uri;
pub mod url;
pub mod uuid;
pub mod value_type;
pub mod version_value;

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
            f.write_char(',')?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}
