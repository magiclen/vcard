use super::*;

validated_customized_ranged_number!(pub Integer, i64, -9223372036854775808i64, 9223372036854775807i64);

impl Value for Integer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}", self.get_number()))?;

        Ok(())
    }
}