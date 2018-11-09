//! # VCard
//!
//! A pure Rust implementation of vCard based on RFC 6350.
//!
//! ## Example
//! ```
/*!
extern crate vcard;

use std::collections::HashSet;

use vcard::{Set, VCard, XPropertyName, IanaToken};
use vcard::properties::*;
use vcard::values::text::{Text, Component};
use vcard::values::language_tag::LanguageTag;
use vcard::values::name_value::NameValue;
use vcard::values::image_value::ImageValue;
use vcard::values::date_time::{DateAndOrTime, Date, Timestamp};
use vcard::values::gender_value::{GenderValue, SexType};
use vcard::values::address_value::AddressValue;
use vcard::values::type_value::{TypeValue, TypeValueWithTelephoneType};
use vcard::values::telephone_value::TelephoneValue;
use vcard::values::email_value::EmailValue;
use vcard::values::url;

use vcard::parameters::language::Language;
use vcard::parameters::typ::{Type, TypeWithTelType};

use vcard::chrono::prelude::*;

let formatted_names = {
    let mut formatted_names = HashSet::new();

    let english_name = {
        let mut formatted_name = FormattedName::from_text(Text::from_str("David Wang").unwrap());
        formatted_name.language = Some(Language::from_language_tag(LanguageTag::from_str("en").unwrap()));

        formatted_name
    };

    formatted_names.insert(english_name);

    let chinese_name = {
        let mut formatted_name = FormattedName::from_text(Text::from_str("王大衛").unwrap());
        formatted_name.language = Some(Language::from_language_tag(LanguageTag::from_str("zh").unwrap()));

        formatted_name
    };

    formatted_names.insert(chinese_name);

    formatted_names
};

let mut vcard = VCard::from_formatted_names(Set::from_hash_set(formatted_names).unwrap()).unwrap();

let names = {
    let mut names = HashSet::new();

    let name = {
        let name_value = NameValue::from_components(
            Some(Component::from_str("Wang").unwrap()),
            Some(Component::from_str("David").unwrap()),
            None,
            Some(Component::from_str("Dr.").unwrap()),
            None,
        );

        Name::from_name_value(name_value)
    };

    names.insert(name);

    names
};

vcard.names = Some(Set::from_hash_set(names).unwrap());

let photos = {
    let mut photos = HashSet::new();

    let photo = {
        let image_value = ImageValue::from_file("tests/data/photo.png").unwrap();

        Photo::from_image_value(image_value)
    };

    photos.insert(photo);

    photos
};

vcard.photos = Some(Set::from_hash_set(photos).unwrap());

let birthdays = {
    let mut birthdays = HashSet::new();

    let birthday = {
        Birthday::from_date_and_or_time(DateAndOrTime::Date(Date::from_year_month_day(1993, 7, 7).unwrap()))
    };

    birthdays.insert(birthday);

    birthdays
};

vcard.birthdays = Some(Set::from_hash_set(birthdays).unwrap());

vcard.gender = Some(Gender::from_gender_value(GenderValue::from_sex_type(SexType::Male)));

let addresses = {
    let mut addresses = HashSet::new();

    let home_address = {
        let address_value = AddressValue::from_components(
            None,
            Some(Component::from_str("No.5").unwrap()),
            Some(Component::from_str("Section 5, Xinyi Road, Xinyi District").unwrap()),
            Some(Component::from_str("Taipei City").unwrap()),
            None,
            Some(Component::from_str("110").unwrap()),
            Some(Component::from_str("Taiwan").unwrap()),
        );

        let mut address = Address::from_address_value(address_value);

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValue::Home);

            Set::from_hash_set(type_values).unwrap()
        };

        address.typ = Some(Type::from_type_values(type_values));

        address
    };

    addresses.insert(home_address);

    let work_address = {
        let address_value = AddressValue::from_components(
            None,
            Some(Component::from_str("No.3").unwrap()),
            Some(Component::from_str("Beiping West Road, Zhongzheng District").unwrap()),
            Some(Component::from_str("Taipei City").unwrap()),
            None,
            Some(Component::from_str("100").unwrap()),
            Some(Component::from_str("Taiwan").unwrap()),
        );

        let mut address = Address::from_address_value(address_value);

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValue::Work);

            Set::from_hash_set(type_values).unwrap()
        };

        address.typ = Some(Type::from_type_values(type_values));

        address
    };

    addresses.insert(work_address);

    addresses
};

vcard.addresses = Some(Set::from_hash_set(addresses).unwrap());

let telephones = {
    let mut telephones = HashSet::new();

    let home_phone = {
        let mut telephone = Telephone::from_telephone_value(TelephoneValue::from_telephone_number_str(
            "+886 02 1234 5678",
            None::<&str>,
        ).unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValueWithTelephoneType::Home);
            type_values.insert(TypeValueWithTelephoneType::Voice);

            Set::from_hash_set(type_values).unwrap()
        };

        if let Telephone::TelephoneValue { ref mut typ, .. } = telephone {
            *typ = Some(TypeWithTelType::from_type_values(type_values));
        }

        telephone
    };

    telephones.insert(home_phone);

    let cell_phone = {
        let mut telephone = Telephone::from_telephone_value(TelephoneValue::from_telephone_number_str(
            "+886 987 654 321",
            None::<&str>,
        ).unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValueWithTelephoneType::Cell);
            type_values.insert(TypeValueWithTelephoneType::Voice);

            Set::from_hash_set(type_values).unwrap()
        };

        if let Telephone::TelephoneValue { ref mut typ, .. } = telephone {
            *typ = Some(TypeWithTelType::from_type_values(type_values));
        }

        telephone
    };

    telephones.insert(cell_phone);

    let work_phone = {
        let mut telephone = Telephone::from_telephone_value(TelephoneValue::from_telephone_number_str(
            "+886 02 8888 8888",
            Some("532"),
        ).unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValueWithTelephoneType::Work);
            type_values.insert(TypeValueWithTelephoneType::Voice);

            Set::from_hash_set(type_values).unwrap()
        };

        if let Telephone::TelephoneValue { ref mut typ, .. } = telephone {
            *typ = Some(TypeWithTelType::from_type_values(type_values));
        }

        telephone
    };

    telephones.insert(work_phone);

    telephones
};

vcard.telephones = Some(Set::from_hash_set(telephones).unwrap());

let emails = {
    let mut emails = HashSet::new();

    let personal_email = {
        let mut email = Email::from_email_value(EmailValue::from_str("david@gmail.com").unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValue::Home);

            Set::from_hash_set(type_values).unwrap()
        };

        email.typ = Some(Type::from_type_values(type_values));

        email
    };

    emails.insert(personal_email);

    let work_email = {
        let mut email = Email::from_email_value(EmailValue::from_str("david@thaumaturgiclen.com").unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValue::Work);

            Set::from_hash_set(type_values).unwrap()
        };

        email.typ = Some(Type::from_type_values(type_values));

        email
    };

    emails.insert(work_email);

    emails
};

vcard.emails = Some(Set::from_hash_set(emails).unwrap());

let urls = {
    let mut urls = HashSet::new();

    let company_site = {
        let mut url = URL::from_url(url::URL::from_str("https://職員.thaumaturgiclen.com:444/王大衛").unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValue::Work);

            Set::from_hash_set(type_values).unwrap()
        };

        url.typ = Some(Type::from_type_values(type_values));

        url
    };

    urls.insert(company_site);

    urls
};

vcard.urls = Some(Set::from_hash_set(urls).unwrap());

let x_properties = {
    let mut x_properties = HashSet::new();

    let facebook = {
        let mut x_socialprofile = XProperty::from_text(XPropertyName::from_str("X-SOCIALPROFILE").unwrap(), Text::from_str("https://www.facebook.com/david.vard.wang").unwrap());

        let type_values = {
            let mut type_values = HashSet::new();

            type_values.insert(TypeValue::IanaToken(IanaToken::from_str("facebook").unwrap()));

            Set::from_hash_set(type_values).unwrap()
        };

        x_socialprofile.typ = Some(Type::from_type_values(type_values));

        x_socialprofile
    };

    x_properties.insert(facebook);

    x_properties
};

vcard.x_properties = Some(Set::from_hash_set(x_properties).unwrap());

//    vcard.revision = Some(Revision::now()); // this is the default value.

vcard.revision = Some(Revision::from_timestamp(Timestamp::from_date_time("2018-11-06T00:00:00Z".parse::<DateTime<Utc>>().unwrap()).unwrap()));

println!("{}", vcard);

//    BEGIN:VCARD
//    VERSION:4.0
//    FN;LANGUAGE=en:David Wang
//    FN;LANGUAGE=zh:王大衛
//    N:Wang;David;;Dr.;
//    GENDER:M
//    BDAY:19930707
//    ADR;TYPE=home:;No.5;Section 5\, Xinyi Road\, Xinyi District;Taipei City;;110;Taiwan
//    ADR;TYPE=work:;No.3;Beiping West Road\, Zhongzheng District;Taipei City;;100;Taiwan
//    TEL;TYPE="voice,cell";VALUE=uri:tel:+886-987-654-321
//    TEL;TYPE="voice,home";VALUE=uri:tel:+886-02-1234-5678
//    TEL;TYPE="voice,work";VALUE=uri:tel:+886-02-8888-8888;ext=532
//    EMAIL;TYPE=work:david@thaumaturgiclen.com
//    EMAIL;TYPE=home:david@gmail.com
//    PHOTO:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAAG0OVFdAAAABmJLR0...
//    URL;TYPE=work:https://xn--gwr372h.thaumaturgiclen.com:444/%E7%8E%8B%E5%A4%A7%E8%A1%9B
//    X-SOCIALPROFILE;TYPE=facebook:https://www.facebook.com/david.vard.wang
//    REV:20181106T000000Z
//    END:VCARD
*/
//! ```

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
extern crate idna;

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
    ($c:tt, $t:ident, $o:ident, $p:ident, $f:ident) => {
        fmt_gg!($c, $t, $o.$p, $f);
    };
}

mod escaping;
pub mod parameters;
pub mod values;
pub mod properties;

use std::fmt::{self, Display, Formatter};
use std::collections::HashSet;
use std::io;
use std::fs;
use std::path::Path;

use regex::Regex;
use validators::ValidatedCustomizedStringError;

use self::properties::*;

pub use mime_guess::Mime;

lazy_static! {
    static ref TEXT_RE: Regex = { Regex::new(r"^([^\x00-\x08\x0A-\x1F\x7F]|\n\r|\r\n|\n)*$").unwrap() };
    static ref SAFE_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x22\x3A\x3B\x7F]*$").unwrap() };
    static ref QSAFE_RE: Regex = { Regex::new(r"^[^\x00-\x1F\x22\x7F]*$").unwrap() };
    static ref IANA_TOKEN_RE: Regex = { Regex::new(r"^[a-zA-Z0-9\-]+$").unwrap() };
    static ref X_NAME_RE: Regex = { Regex::new(r"^[xX]-[a-zA-Z0-9\-]+$").unwrap() };
    static ref X_PROPERTY_NAME_RE: Regex = { Regex::new(r"^X-[A-Z0-9\-]+$").unwrap() };
}

validated_customized_ranged_length_hash_set!(pub Set, 1, usize::max_value());
validated_customized_regex_string!(pub IanaToken, ref IANA_TOKEN_RE);
validated_customized_regex_string!(pub XName, ref X_NAME_RE);
validated_customized_regex_string!(pub XPropertyName, ref X_PROPERTY_NAME_RE);

#[derive(Clone, Debug, PartialEq)]
pub struct VCard {
    pub begin: Begin,
    pub version: Version,
    pub formatted_names: Set<FormattedName>,
    pub names: Option<Set<Name>>,
    pub nicknames: Option<Set<NickName>>,
    pub uid: Option<UID>,
    pub keys: Option<Set<Key>>,
    pub gender: Option<Gender>,
    pub birthdays: Option<Set<Birthday>>,
    pub anniversaries: Option<Set<Anniversary>>,
    pub addresses: Option<Set<Address>>,
    pub telephones: Option<Set<Telephone>>,
    pub emails: Option<Set<Email>>,
    pub titles: Option<Set<Title>>,
    pub roles: Option<Set<Role>>,
    pub photos: Option<Set<Photo>>,
    pub logos: Option<Set<Logo>>,
    pub urls: Option<Set<URL>>,
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
    pub client_property_id_maps: Option<Set<ClientPropertyIDMap>>,
    pub fburls: Option<Set<FBURL>>,
    pub calendar_uris: Option<Set<CalendarURI>>,
    pub calendar_address_uris: Option<Set<CalendarAddressURI>>,
    pub x_properties: Option<Set<XProperty>>,
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
            keys: None,
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
            urls: None,
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
            client_property_id_maps: None,
            fburls: None,
            calendar_uris: None,
            calendar_address_uris: None,
            x_properties: None,
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

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let path = path.as_ref();

        fs::write(path, self.to_string())
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
        fmt!(2, keys);
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
        fmt!(2, urls);
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
        fmt!(2, client_property_id_maps);
        fmt!(2, fburls);
        fmt!(2, calendar_uris);
        fmt!(2, calendar_address_uris);
        fmt!(2, x_properties);
        fmt!(0, revision);
        fmt!(1, end);

        Ok(())
    }
}