use super::super::escaping::*;
use super::super::TEXT_RE;
use super::*;

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
        let c = escape_tab(c.as_ref());
        let c = escape_comma(c.as_ref());

        f.write_str(c.as_ref())?;

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
        let c = escape_tab(c.as_ref());
        let c = escape_comma(c.as_ref());
        let c = escape_semicolon(c.as_ref());

        f.write_str(c.as_ref())?;

        Ok(())
    }
}
