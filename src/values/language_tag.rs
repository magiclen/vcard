use super::*;
use super::super::escaping::*;

// TODO: not implement yet, refer to [RFC5646]
validated_customized_string!(pub LanguageTag,
    from_string s {
        Ok(s)
    },
    from_str s {
        Ok(s.to_string())
    }
);

impl LanguageTag {
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}

impl Value for LanguageTag {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let c = escape_backslash(self.as_str());
        let c = escape_new_line(c.as_ref());
        let c = escape_comma(c.as_ref());
        let c = escape_semicolon(c.as_ref());
        let c = escape_colon(c.as_ref());

        f.write_str(c.as_ref())?;

        Ok(())
    }
}