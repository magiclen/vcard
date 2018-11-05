#[macro_use]
pub extern crate validators;
pub extern crate chrono;
pub extern crate chrono_tz;
#[macro_use]
extern crate lazy_static;
extern crate base64_stream;
extern crate mime_guess;
extern crate percent_encoding;
extern crate regex;

macro_rules! fmt_gg {
    (0, $t:ident, $p:expr, $f:ident) => { // *1
        if let Some(p) = &$p {
            $t::fmt(p, $f)?;
        }
    };
    (1, $t:ident, $p:expr, $f:ident) => { // 1
        $t::fmt(&$p, $f)?;
    };
    (2, $t:ident, $p:expr, $f:ident) => { // *
        if let Some(p) = &$p {
            for e in p.as_hash_set() {
                $t::fmt(e, $f)?;
            }
        }
    };
    (3, $t:ident, $p:expr, $f:ident) => { // 1*
        for e in $p.as_hash_set() {
            $t::fmt(e, $f)?;
        }
    };
}

macro_rules! fmt_g {
    ($c:tt, $t:ident, $o:ident, $p:ident, $f:ident) => { // *1
        fmt_gg!($c, $t, $o.$p, $f);
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

pub use mime_guess::Mime;

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
    pub names: Option<Set<Name>>,
    pub nicknames: Option<Set<NickName>>,
    pub uid: Option<UID>,
    pub gender: Option<Gender>,
    pub birthdays: Option<Set<Birthday>>,
    pub anniversaries: Option<Set<Anniversary>>,
    pub addresses: Option<Set<Address>>,
    pub telephones: Option<Set<Address>>,
    pub emails: Option<Set<Email>>,
    pub titles: Option<Set<Title>>,
    pub roles: Option<Set<Role>>,
    pub photos: Option<Set<Photo>>,
    pub logos: Option<Set<Logo>>,
    pub sounds: Option<Set<Sound>>,
    pub organizations: Option<Set<Organization>>,
    pub members: Option<Set<Member>>,
    pub relationships: Option<Set<Relationship>>,
    pub categories: Option<Set<Category>>,
    pub notes: Option<Set<Note>>,
    pub languages: Option<Set<Language>>,
    pub time_zones: Option<Set<TimeZone>>,
    pub geos: Option<Set<TimeZone>>,
    pub impps: Option<Set<IMPP>>,
    pub sources: Option<Set<Source>>,
    pub product_id: Option<ProductID>,
    pub revision: Option<Revision>,
    pub end: End,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VCardError {
    FormatError(ValidatedCustomizedStringError),
    EmptyFormatName,
}

impl VCard {
    pub fn from_formatted_names(formatted_names: Set<FormattedName>) -> Result<VCard, VCardError> {
        let mut has_formatted_names = false;

        for e in formatted_names.as_hash_set() {
            if !e.is_empty() {
                has_formatted_names = true;
                break;
            }
        }

        if !has_formatted_names {
            return Err(VCardError::EmptyFormatName);
        }

        let revision = Revision::now();

        Ok(VCard {
            begin: properties::Begin,
            version: properties::Version::from_version_value(values::version_value::VersionValue::V4P0),
            formatted_names,
            names: None,
            nicknames: None,
            uid: None,
            gender: None,
            birthdays: None,
            anniversaries: None,
            addresses: None,
            telephones: None,
            emails: None,
            titles: None,
            roles: None,
            photos: None,
            logos: None,
            sounds: None,
            organizations: None,
            members: None,
            relationships: None,
            categories: None,
            notes: None,
            languages: None,
            time_zones: None,
            geos: None,
            impps: None,
            sources: None,
            product_id: None,
            revision: Some(revision),
            end: properties::End,
        })
    }

    pub fn from_formatted_name(formatted_name: FormattedName) -> Result<VCard, VCardError> {
        let mut formatted_names = HashSet::new();
        formatted_names.insert(formatted_name);

        Self::from_formatted_names(Set::from_hash_set(formatted_names).unwrap())
    }

    pub fn from_formatted_name_string(formatted_name: String) -> Result<VCard, VCardError> {
        let text = values::text::Text::from_string(formatted_name).map_err(|err| VCardError::FormatError(err))?;
        let formatted_name = FormattedName::from_text(text);

        Self::from_formatted_name(formatted_name)
    }

    pub fn from_formatted_name_str(formatted_name: &str) -> Result<VCard, VCardError> {
        let text = values::text::Text::from_str(formatted_name).map_err(|err| VCardError::FormatError(err))?;
        let formatted_name = FormattedName::from_text(text);

        Self::from_formatted_name(formatted_name)
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
        fmt!(2, names);
        fmt!(2, nicknames);
        fmt!(0, uid);
        fmt!(0, gender);
        fmt!(2, birthdays);
        fmt!(2, anniversaries);
        fmt!(2, addresses);
        fmt!(2, telephones);
        fmt!(2, emails);
        fmt!(2, titles);
        fmt!(2, roles);
        fmt!(2, photos);
        fmt!(2, logos);
        fmt!(2, sounds);
        fmt!(2, organizations);
        fmt!(2, members);
        fmt!(2, relationships);
        fmt!(2, categories);
        fmt!(2, notes);
        fmt!(2, languages);
        fmt!(2, time_zones);
        fmt!(2, geos);
        fmt!(2, impps);
        fmt!(2, sources);
        fmt!(0, product_id);
        fmt!(0, revision);
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

        let mut sources = HashSet::new();

        {
            let mut source = Source::from_uri(values::uri::URI::from_str("https://magiclen.org").unwrap());

            let mut property_ids = HashSet::new();

            property_ids.insert(values::property_id_value::PropertyIDValue::from_u8(1, None).unwrap());
            property_ids.insert(values::property_id_value::PropertyIDValue::from_u8(2, Some(5)).unwrap());

            let source_property_id = parameters::property_id::PropertyID::from_ids(Set::from_hash_set(property_ids).unwrap());

            source.property_id = Some(source_property_id);

            sources.insert(source);
        }

        vcard.sources = Some(Set::from_hash_set(sources).unwrap());

        println!("{}", vcard.to_string());
    }
}
