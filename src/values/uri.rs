pub use super::super::validators::uri::URI;
use super::*;

impl Value for URI {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.get_full_uri())?;

        Ok(())
    }
}
