pub use super::super::validators::email::Email as EmailValue;
use super::super::escaping::*;
use super::*;

impl Value for EmailValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let c = escape_backslash(self.get_full_email());
        let c = escape_comma(c.as_ref());

        f.write_str(c.as_ref())?;

        Ok(())
    }
}