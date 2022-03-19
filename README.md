vCard
====================

[![CI](https://github.com/magiclen/vcard/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/vcard/actions/workflows/ci.yml)

A pure Rust implementation of vCard based on RFC 6350.

## Example

```rust
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
        let image_value = ImageValue::from_file("photo.png").unwrap();

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
//    TEL;TYPE="voice,cell";VALUE=uri:tel:886-987-654-321
//    TEL;TYPE="voice,home";VALUE=uri:tel:886-02-1234-5678
//    TEL;TYPE="voice,work";VALUE=uri:tel:886-02-8888-8888;ext=532
//    EMAIL;TYPE=work:david@thaumaturgiclen.com
//    EMAIL;TYPE=home:david@gmail.com
//    PHOTO:data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAAG0OVFdAAAABmJLR0...
//    URL;TYPE=work:https://xn--gwr372h.thaumaturgiclen.com:444/%E7%8E%8B%E5%A4%A7%E8%A1%9B
//    X-SOCIALPROFILE;TYPE=facebook:https://www.facebook.com/david.vard.wang
//    REV:20181106T000000Z
//    END:VCARD
```

## TODO

1. Attribute Value (RFC2045)
1. Language Tag (RFC5646)
1. Product ID Value (ISO9070 and RFC3406)
1. Media Type (RFC4288)
1. VCard Parser
1. VCard Validator
1. All versions of VCard (extra 2.1 and 3.0)

## Crates.io

https://crates.io/crates/vcard

## Documentation

https://docs.rs/vcard

## License

[MIT](LICENSE)