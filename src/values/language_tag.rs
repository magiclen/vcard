use regex::Regex;

use super::*;
use crate::PATH_PERCENT_ENCODE_SET;

// TODO: not implement yet, refer to [RFC5646]

lazy_static! {
    static ref LANGUAGE_TAG_RE: Regex = Regex::new(r"^[\S]+$").unwrap();
}

validated_customized_regex_string!(pub LanguageTag, ref LANGUAGE_TAG_RE);

impl Value for LanguageTag {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(
            &percent_encoding::utf8_percent_encode(self.as_str(), PATH_PERCENT_ENCODE_SET)
                .to_string(),
        )?;

        Ok(())
    }
}
