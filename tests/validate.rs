use vcard::{
    AnyParameter, Calscale, ExtensionProperty, Name, Phonetic, Pref, SocialProfile, VCard,
    ValidationError,
    values::{NameValue, TextOrUri},
};

#[test]
fn names_need_a_shared_altid() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Test\r\n",
        "N;ALTID=1:Doe;John;;;\r\n",
        "N;ALTID=1:杜;約翰;;;\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.names[1].parameters.altid = Some(String::from("2"));

    assert_eq!(Err(ValidationError::DuplicateProperty(String::from("N"))), vcard.validate());

    for name in &mut vcard.names {
        name.parameters.altid = None;
    }

    assert_eq!(Err(ValidationError::DuplicateProperty(String::from("N"))), vcard.validate());
}

#[test]
fn single_properties_need_a_shared_altid() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Test\r\n",
        "BDAY;ALTID=1:19900102\r\n",
        "BDAY;VALUE=text;ALTID=1:circa 1990\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.birthdays[1].parameters.altid = None;

    assert_eq!(Err(ValidationError::DuplicateProperty(String::from("BDAY"))), vcard.validate());
}

#[test]
fn text_social_profiles_need_a_service_type() {
    let mut vcard = VCard::new("Test");

    vcard.social_profiles.push(SocialProfile::new(TextOrUri::Text(String::from("peter94"))));

    assert_eq!(Err(ValidationError::SocialProfileWithoutServiceType), vcard.validate());

    vcard.social_profiles[0].parameters.service_type = Some(String::from("SomeSite"));
    vcard
        .social_profiles
        .push(SocialProfile::new(TextOrUri::Uri("https://example.com/peter94".parse().unwrap())));

    assert!(vcard.validate().is_ok());
}

#[test]
fn phonetic_names_follow_related_properties() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Sun Yat-sen\r\n",
        "N;ALTID=1;LANGUAGE=zh-Hant:孫;中山;文,逸仙;;\r\n",
        "N;ALTID=1;PHONETIC=script;SCRIPT=Latn;LANGUAGE=yue:syun1;zung1saan1;;;\r\n",
        "N;ALTID=1;PHONETIC=piny;LANGUAGE=zh:sun;zhongshan;;;\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.names[1].parameters.script = None;
    assert_eq!(Err(ValidationError::PhoneticWithoutScript(String::from("N"))), vcard.validate());

    vcard.names[1].parameters.phonetic = Some(Phonetic::Jyut);
    vcard.names[2].parameters.language = Some("YUE".parse().unwrap());
    assert_eq!(
        Err(ValidationError::DuplicatePhoneticLanguage(String::from("N"))),
        vcard.validate()
    );

    vcard.names[2].parameters.language = None;
    assert!(vcard.validate().is_ok());

    vcard.names[1].parameters.language = None;
    assert_eq!(
        Err(ValidationError::DuplicatePhoneticLanguage(String::from("N"))),
        vcard.validate()
    );

    vcard.names.remove(2);
    vcard.names[1].value.honorific_prefixes.push(String::from("Dr."));
    assert_eq!(
        Err(ValidationError::PhoneticWithoutRelatedProperty(String::from("N"))),
        vcard.validate()
    );

    vcard.names[0].value.honorific_prefixes.push(String::from("博士"));
    assert!(vcard.validate().is_ok());

    vcard.names.remove(0);
    assert_eq!(
        Err(ValidationError::PhoneticWithoutRelatedProperty(String::from("N"))),
        vcard.validate()
    );

    vcard.names[0].parameters.altid = None;
    assert_eq!(
        Err(ValidationError::PhoneticWithoutRelatedProperty(String::from("N"))),
        vcard.validate()
    );
}

#[test]
fn phonetic_addresses_follow_related_properties() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Test\r\n",
        "ADR;ALTID=1:;;中山路;臺北;;;;;;;1;中山路\r\n",
        "ADR;ALTID=1;PHONETIC=piny;SCRIPT=Latn:;;Zhongshan Lu;Taipei;;;;;;;1;Zhongshan Lu\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.addresses[1].parameters.altid = Some(String::from("2"));
    assert_eq!(
        Err(ValidationError::PhoneticWithoutRelatedProperty(String::from("ADR"))),
        vcard.validate()
    );

    vcard.addresses[1].parameters.altid = Some(String::from("1"));
    vcard.addresses[0].value.street_names.clear();
    assert_eq!(
        Err(ValidationError::PhoneticWithoutRelatedProperty(String::from("ADR"))),
        vcard.validate()
    );
}

#[test]
fn gram_genders_need_different_languages() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Test\r\n",
        "GRAMGENDER;LANGUAGE=de:feminine\r\n",
        "GRAMGENDER;LANGUAGE=fr:masculine\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.gram_genders[1].parameters.language = Some("DE".parse().unwrap());
    assert_eq!(Err(ValidationError::DuplicateGramGenderLanguage), vcard.validate());

    vcard.gram_genders[1].parameters.language = None;
    assert!(vcard.validate().is_ok());

    vcard.gram_genders[0].parameters.language = None;
    assert_eq!(Err(ValidationError::DuplicateGramGenderLanguage), vcard.validate());
}

#[test]
fn default_language_cannot_have_a_language_parameter() {
    let mut vcard: VCard =
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nLANGUAGE:de\r\nEND:VCARD\r\n".parse().unwrap();

    assert!(vcard.validate().is_ok());

    vcard.languages[0].parameters.language = Some("en".parse().unwrap());
    assert_eq!(Err(ValidationError::LanguageWithLanguageParameter), vcard.validate());
}

#[test]
fn usernames_need_uri_values() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Test\r\n",
        "IMPP;USERNAME=alice:xmpp:alice@example.com\r\n",
        "SOCIALPROFILE;SERVICE-TYPE=Example;USERNAME=alice:https://example.com/alice\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.social_profiles[0].value = TextOrUri::Text(String::from("alice"));
    assert_eq!(Err(ValidationError::UsernameWithoutUriValue), vcard.validate());

    vcard.social_profiles[0].parameters.username = None;
    assert!(vcard.validate().is_ok());
}

#[test]
fn author_names_must_not_be_empty() {
    let mut vcard: VCard = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "FN:Test\r\n",
        "NOTE;AUTHOR-NAME=Jane Doe:A note.\r\n",
        "END:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(vcard.validate().is_ok());

    vcard.notes[0].parameters.author_name = Some(String::new());
    assert_eq!(Err(ValidationError::EmptyAuthorName(String::from("NOTE"))), vcard.validate());

    vcard.notes[0].parameters.author_name = None;
    let mut extension = ExtensionProperty::from_text("X-NOTE".parse().unwrap(), "A note.");
    extension.parameters.author_name = Some(String::from(" "));
    vcard.extensions.push(extension);
    assert!(vcard.validate().is_ok());

    vcard.extensions[0].parameters.author_name = Some(String::new());
    assert_eq!(Err(ValidationError::EmptyAuthorName(String::from("X-NOTE"))), vcard.validate());
}

#[test]
fn pids_reference_client_sources() {
    let mut card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN;PID=1.1:Test\r\n",
        "TEL;PID=1.1,2.2,3:123\r\n",
        "CLIENTPIDMAP:1;urn:example:first\r\n",
        "CLIENTPIDMAP:2;urn:example:second\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

    card.client_pid_maps.pop();
    assert_eq!(Err(ValidationError::MissingClientPidMap(2)), card.validate());

    card.telephones[0].parameters.pids[1].source = Some(0);
    assert_eq!(Err(ValidationError::InvalidPid(String::from("TEL"))), card.validate());

    card.telephones[0].parameters.pids[1].source = None;
    card.telephones[0].parameters.pids[1].id = 0;
    assert_eq!(Err(ValidationError::InvalidPid(String::from("TEL"))), card.validate());

    card.telephones[0].parameters.pids[1].id = 2;
    card.client_pid_maps[0].value.source_id = 0;
    assert_eq!(Err(ValidationError::InvalidClientPidMap), card.validate());
}

#[test]
fn single_properties_cannot_have_pids() {
    for (name, value) in [("BDAY", "19900102"), ("CLIENTPIDMAP", "1;urn:example:source")] {
        let card: VCard = format!(
            "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n{name};PID=1:{value}\r\nEND:VCARD\r\n"
        )
        .parse()
        .unwrap();

        assert_eq!(Err(ValidationError::PidNotAllowed(name.to_string())), card.validate());
    }
}

#[test]
fn type_parameters_follow_property_rules() {
    let card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "ADR;TYPE=billing:;;Main Street;Town;;;\r\n",
        "TEL;TYPE=cell:123\r\n",
        "EMAIL;TYPE=work:test@example.com\r\n",
        "RELATED;TYPE=friend:urn:example:friend\r\n",
        "PRONOUNS;TYPE=home:they/them\r\n",
        "X-EXTRA;TYPE=work:raw\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());

    for (name, type_value, value) in [
        ("SOURCE", "work", "urn:example:source"),
        ("BDAY", "work", "19900102"),
        ("N", "work", "Doe;John;;;"),
        ("EMAIL", "cell", "test@example.com"),
        ("TEL", "friend", "123"),
        ("X-EXTRA", "voice", "raw"),
    ] {
        let card: VCard = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "{name};TYPE={type_value}:{value}\r\nEND:VCARD\r\n",
            ),
            name = name,
            type_value = type_value,
            value = value,
        )
        .parse()
        .unwrap();

        assert_eq!(
            Err(ValidationError::InvalidParameter {
                property:  name.to_string(),
                parameter: String::from("TYPE"),
            }),
            card.validate()
        );
    }
}

#[test]
fn calendar_scales_need_date_values() {
    for name in ["BDAY", "ANNIVERSARY"] {
        for (value_type, value, has_date) in [
            ("", "19900102", true),
            ("", "--0102", true),
            ("", "19900102T1022", true),
            ("", "T1022", false),
            (";VALUE=text", "circa 1800", false),
        ] {
            let mut card: VCard = format!(
                concat!(
                    "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                    "{name}{value_type}:{value}\r\nEND:VCARD\r\n",
                ),
                name = name,
                value_type = value_type,
                value = value,
            )
            .parse()
            .unwrap();

            assert!(card.validate().is_ok());

            let property =
                if name == "BDAY" { &mut card.birthdays[0] } else { &mut card.anniversaries[0] };
            property.parameters.calscale = Some(Calscale::Gregorian);

            let expected = if has_date {
                Ok(())
            } else {
                Err(ValidationError::InvalidParameter {
                    property:  name.to_string(),
                    parameter: String::from("CALSCALE"),
                })
            };

            assert_eq!(expected, card.validate());
        }
    }
}

#[test]
fn sort_as_follows_component_counts() {
    let mut card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "N;SORT-AS=Doe,John:Doe;John;;;\r\n",
        "ORG;SORT-AS=Example,Sales:Example;Sales\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());

    card.names[0].parameters.sort_as.resize(6, String::new());
    assert_eq!(
        Err(ValidationError::InvalidParameter {
            property:  String::from("N"),
            parameter: String::from("SORT-AS"),
        }),
        card.validate()
    );

    card.names[0].value.surname2.push(String::from("Smith"));
    assert!(card.validate().is_ok());

    card.names[0].parameters.sort_as.push(String::from("III"));
    card.names[0].value.generation.push(String::from("III"));
    assert!(card.validate().is_ok());
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

    card.organizations[0].parameters.sort_as.push(String::from("Extra"));
    assert_eq!(
        Err(ValidationError::InvalidParameter {
            property:  String::from("ORG"),
            parameter: String::from("SORT-AS"),
        }),
        card.validate()
    );
}

#[test]
fn extensions_keep_unknown_names_and_value_types() {
    let mut card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "X-EXTRA;VALUE=x-example;X-ORIGIN=one;X-ORIGIN=two:raw\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

    for name in ["end", "VERSION", "fN", "EMAIL"] {
        card.extensions[0].name = name.parse().unwrap();
        assert_eq!(Err(ValidationError::ReservedExtensionName(name.to_string())), card.validate());
    }
}

#[test]
fn extension_values_cannot_contain_line_breaks() {
    let mut vcard = VCard::new("Test");

    vcard.extensions.push(ExtensionProperty::from_text("X-NOTE".parse().unwrap(), "first\nsecond"));
    assert!(vcard.validate().is_ok());

    vcard.extensions[0].value = String::from("first\r\nEND:VCARD");
    assert_eq!(
        Err(ValidationError::InvalidExtensionValue(String::from("X-NOTE"))),
        vcard.validate()
    );
}

#[test]
fn generic_parameters_cannot_override_typed_values() {
    let mut card = VCard::new("Test");
    let mut value_type = AnyParameter {
        name:   "VALUE".parse().unwrap(),
        values: vec![String::from("TEXT")],
    };
    card.formatted_names[0].parameters.any.push(value_type.clone());
    assert_eq!(
        Err(ValidationError::InvalidParameter {
            property:  String::from("FN"),
            parameter: String::from("VALUE"),
        }),
        card.validate()
    );

    value_type.name = "PREF".parse().unwrap();
    value_type.values[0] = String::from("2");
    card.formatted_names[0].parameters.pref = Some(Pref::new(1).unwrap());
    card.formatted_names[0].parameters.any[0] = value_type;
    assert_eq!(
        Err(ValidationError::InvalidParameter {
            property:  String::from("FN"),
            parameter: String::from("PREF"),
        }),
        card.validate()
    );
}

#[test]
fn media_types_with_nested_quotes_need_another_representation() {
    let mut card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "URL;MEDIATYPE=\"text/plain;charset=utf-8\":https://example.com\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());

    card.urls[0].parameters.media_type = Some("text/plain;name=\"a,b\"".parse().unwrap());
    assert_eq!(
        Err(ValidationError::UnrepresentableMediaType(String::from("URL"))),
        card.validate()
    );
}

#[test]
fn control_characters_cannot_be_serialized_as_text() {
    for control in ['\0', '\u{7f}'] {
        let text = format!("A{control}B");
        let mut card = VCard::new(text.clone());

        assert_eq!(Err(ValidationError::InvalidTextValue(String::from("FN"))), card.validate());

        card.formatted_names[0].value = String::from("Test");
        card.names.push(Name::new(NameValue {
            family_names: vec![text.clone()],
            ..NameValue::default()
        }));

        assert_eq!(Err(ValidationError::InvalidTextValue(String::from("N"))), card.validate());

        card.names.clear();
        card.extensions.push(ExtensionProperty::new("X-EXTRA".parse().unwrap(), text.clone()));

        assert_eq!(
            Err(ValidationError::InvalidExtensionValue(String::from("X-EXTRA"))),
            card.validate()
        );

        card.extensions.clear();
        card.formatted_names[0].parameters.altid = Some(text);

        assert_eq!(
            Err(ValidationError::InvalidParameter {
                property:  String::from("FN"),
                parameter: String::from("ALTID"),
            }),
            card.validate()
        );
    }
}

#[test]
fn tabs_and_escaped_newlines_remain_valid() {
    let mut card = VCard::new("甲\t乙\n丙");
    card.formatted_names[0].parameters.altid = Some(String::from("one\ttwo\nthree"));
    card.names.push(Name::new(NameValue {
        family_names: vec![String::from("Doe\tSmith\nJones")],
        ..NameValue::default()
    }));
    card.extensions
        .push(ExtensionProperty::from_text("X-EXTRA".parse().unwrap(), "one\ttwo\nthree"));

    assert!(card.validate().is_ok());
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

    card.formatted_names[0].value = String::from("one\r\ntwo\rthree");
    card.formatted_names[0].parameters.altid = Some(String::from("one\r\ntwo\rthree"));

    assert!(card.validate().is_ok());

    let parsed: VCard = card.to_string().parse().unwrap();

    assert_eq!("one\ntwo\nthree", parsed.formatted_names[0].value);
    assert_eq!(Some("one\ntwo\nthree"), parsed.formatted_names[0].parameters.altid.as_deref());
}
