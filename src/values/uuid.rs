pub use super::super::validators::uuid::UUIDAllowAnyCase as UUID;
use super::*;

impl Value for UUID {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("urn:uuid:")?;

        if self.has_uppercase() {
            f.write_str(&self.get_full_uuid().to_lowercase())?;
        } else {
            f.write_str(self.get_full_uuid())?;
        }

        Ok(())
    }
}
