use super::*;

validated_customized_primitive_number!(pub Float, f64);

impl Value for Float {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{:.7}", self.get_number()))?;

        Ok(())
    }
}
