use super::*;

validated_customized_ranged_number!(pub Integer, i64, -9223372036854775808i64, 9223372036854775807i64);

impl Value for Integer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}", self.get_number()))?;

        Ok(())
    }
}

impl Value for List<Integer> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<Integer> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}