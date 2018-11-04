use super::*;

validated_customized_ranged_number!(pub PreferenceValue, u8, 1, 100);

impl Value for PreferenceValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}", self.get_number()))?;

        Ok(())
    }
}