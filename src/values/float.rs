use super::*;

validated_customized_primitive_number!(pub Float, f64);

impl Value for Float {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{:.7}", self.get_number()))?;

        Ok(())
    }
}

impl Value for List<Float> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<Float> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}
