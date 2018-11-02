#[macro_use]
pub extern crate validators;
pub extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate base64_stream;
extern crate mime_guess;
extern crate percent_encoding;
extern crate regex;

pub mod escaping;
pub mod parameters;
pub mod values;

use regex::Regex;

lazy_static! {
    static ref TEXT_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x7F]*$").unwrap() };
    static ref SAFE_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x22\x3A\x3B\x7F]*$").unwrap() };
    static ref QSAFE_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x22\x7F]*$").unwrap() };
    static ref IANA_TOKEN_RE: Regex = { Regex::new(r"^[^a-zA-Z0-9\-]+$").unwrap() };
    static ref X_NAME_RE: Regex = { Regex::new(r"^[xX]-[^a-zA-Z0-9\-]+$").unwrap() };
}

validated_customized_regex_string!(pub IanaToken, ref IANA_TOKEN_RE);
validated_customized_regex_string!(pub XName, ref X_NAME_RE);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
