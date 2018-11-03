use std::fmt::{self, Formatter};
use std::collections::HashSet;
use std::hash::Hash;
use validators::ValidatedWrapper;

pub mod types;
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

pub trait Value: ValidatedWrapper {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}

validated_customized_ranged_length_vec!(pub List, 1, usize::max_value());
validated_customized_ranged_length_hash_set!(pub Set, 1, usize::max_value());

impl<V: Value> Value for List<V> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<V> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}

impl<V: Value + Eq + Hash> Value for Set<V> {
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
