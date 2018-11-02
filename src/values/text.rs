use super::*;
use super::super::TEXT_RE;
use super::super::escaping::*;

use std::fmt::{self, Formatter};

validated_customized_regex_string!(pub Text, ref TEXT_RE);

validated_customized_regex_string!(pub Component, ref TEXT_RE);

impl Text {
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}

impl Value for Text {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let c = escape_backslash(self.as_str());
        let c = escape_new_line(c.as_ref());
        let c = escape_comma(c.as_ref());

        f.write_str(c.as_ref())?;

        Ok(())
    }
}

impl List<Text> {
    pub fn is_empty(&self) -> bool {
        for e in self.as_vec() {
            if !e.is_empty() {
                return false;
            }
        }

        true
    }
}

impl Value for List<Text> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let v: &Vec<Text> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}

impl Component {
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}

impl Value for Component {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let c = escape_backslash(self.as_str());
        let c = escape_new_line(c.as_ref());
        let c = escape_comma(c.as_ref());
        let c = escape_semicolon(c.as_ref());

        f.write_str(c.as_ref())?;

        Ok(())
    }
}

impl List<Component> {
    pub fn is_empty(&self) -> bool {
        for e in self.as_vec() {
            if !e.is_empty() {
                return false;
            }
        }

        true
    }
}

impl Value for List<Component> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let v: &Vec<Component> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}