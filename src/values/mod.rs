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
pub mod tel_type;
pub mod related_type;

pub trait Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>;
}

validated_customized_ranged_length_vec!(pub List, 1, usize::max_value());
validated_customized_ranged_length_hash_set!(pub Set, 1, usize::max_value());

impl<V: Value + ValidatedWrapper> Value for List<V> {
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

impl<V: Value + ValidatedWrapper> Value for Vec<V> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        Value::fmt(&self[0], f)?;

        for e in self.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
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


impl<V: Value + ValidatedWrapper + Eq + Hash> Value for HashSet<V> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        for e in self.iter().take(1) {
            Value::fmt(e, f)?;
        }

        for e in self.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}