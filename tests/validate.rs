use vcard::{
    ExtensionProperty, Phonetic, SocialProfile, VCard, ValidationError, values::TextOrUri,
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
