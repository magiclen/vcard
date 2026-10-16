use vcard::{SocialProfile, VCard, ValidationError, values::TextOrUri};

#[test]
fn names_need_a_shared_altid() {
    let mut vcard: VCard = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nN:Doe;John;;;\r\nN:杜;約翰;;;\
                            \r\nEND:VCARD\r\n"
        .parse()
        .unwrap();

    assert_eq!(Err(ValidationError::DuplicateProperty(String::from("N"))), vcard.validate());

    for name in &mut vcard.names {
        name.parameters.altid = Some(String::from("1"));
    }

    assert!(vcard.validate().is_ok());

    vcard.names[1].parameters.altid = Some(String::from("2"));

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
