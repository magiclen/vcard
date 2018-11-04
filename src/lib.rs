#[macro_use]
pub extern crate validators;
pub extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate base64_stream;
extern crate mime_guess;
extern crate percent_encoding;
extern crate regex;

macro_rules! fmt_g {
    (0, $t:ident, $o:ident, $p:ident, $f:ident) => { // *1
        if let Some(p) = &$o.$p {
            $t::fmt(p, $f)?;
        }
    };
    (1, $t:ident, $o:ident, $p:ident, $f:ident) => { // 1
        $t::fmt(&$o.$p, $f)?;
    };
    (2, $t:ident, $o:ident, $p:ident, $f:ident) => { // *
        if let Some(p) = &$o.$p {
            for e in p.as_hash_set() {
                $t::fmt(e, $f)?;
            }
        }
    };
    (3, $t:ident, $o:ident, $p:ident, $f:ident) => { // 1*
        for e in $o.$p.as_hash_set() {
            $t::fmt(e, $f)?;
        }
    };
}

pub mod escaping;
pub mod parameters;
pub mod values;
pub mod properties;

use std::fmt::{self, Display, Formatter};
use std::collections::HashSet;

use regex::Regex;
use validators::ValidatedCustomizedStringError;

use self::properties::*;

lazy_static! {
    static ref TEXT_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x7F]*$").unwrap() };
    static ref SAFE_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x22\x3A\x3B\x7F]*$").unwrap() };
    static ref QSAFE_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x22\x7F]*$").unwrap() };
    static ref IANA_TOKEN_RE: Regex = { Regex::new(r"^[^a-zA-Z0-9\-]+$").unwrap() };
    static ref X_NAME_RE: Regex = { Regex::new(r"^[xX]-[^a-zA-Z0-9\-]+$").unwrap() };
}

validated_customized_ranged_length_hash_set!(pub Set, 1, usize::max_value());
validated_customized_regex_string!(pub IanaToken, ref IANA_TOKEN_RE);
validated_customized_regex_string!(pub XName, ref X_NAME_RE);

#[derive(Clone, Debug, PartialEq)]
pub struct VCard {
    pub begin: Begin,
    pub version: Version,
    pub formatted_names: Set<FormattedName>,
    pub source: Option<Set<Source>>,
    pub end: End,
}

impl VCard {
    pub fn from_formatted_names(formatted_names: Set<FormattedName>) -> VCard {
        VCard {
            begin: properties::Begin,
            version: properties::Version::from_version_value(values::version_value::VersionValue::V4P0),
            formatted_names,
            source: None,
            end: properties::End,
        }
    }

    pub fn from_formatted_name(formatted_name: FormattedName) -> VCard {
        let mut formatted_names = HashSet::new();
        formatted_names.insert(formatted_name);

        Self::from_formatted_names(Set::from_hash_set(formatted_names).unwrap())
    }

    pub fn from_formatted_name_string(formatted_name: String) -> Result<VCard, ValidatedCustomizedStringError> {
        let text = values::text::Text::from_string(formatted_name)?;
        let formatted_name = FormattedName::from_text(text);

        Ok(Self::from_formatted_name(formatted_name))
    }

    pub fn from_formatted_name_str(formatted_name: &str) -> Result<VCard, ValidatedCustomizedStringError> {
        let text = values::text::Text::from_str(formatted_name)?;
        let formatted_name = FormattedName::from_text(text);

        Ok(Self::from_formatted_name(formatted_name))
    }
}

impl Display for VCard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        macro_rules! fmt {
            ($c:tt, $p:ident) => {
                fmt_g!($c, Property, self, $p, f);
            };
        }

        fmt!(1, begin);
        fmt!(1, version);
        fmt!(3, formatted_names);
        fmt!(2, source);
        fmt!(1, end);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vcard = VCard::from_formatted_name_str("Magic Len").unwrap();

        let mut source = HashSet::new();

        source.insert(properties::Source::from_uri(values::uri::URI::from_str("https://magiclen.org").unwrap()));

        vcard.source = Some(Set::from_hash_set(source).unwrap());

        println!("{}", vcard.to_string());
    }
}
