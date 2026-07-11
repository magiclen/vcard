use vcard::{
    ParseErrorKind, Pref, TypeValue, VCard,
    values::{
        Date, DateAndOrTime, DateAndOrTimeOrText, DateTime, KindValue, Sex, TelValue, Time,
        TzValue, UtcOffset, Zone,
    },
};

/// The example vCard from RFC 6350 section 8, with its original folded lines.
const RFC6350_AUTHOR_CARD: &str = "BEGIN:VCARD\r\n\
                                   VERSION:4.0\r\n\
                                   FN:Simon Perreault\r\n\
                                   N:Perreault;Simon;;;ing. jr,M.Sc.\r\n\
                                   BDAY:--0203\r\n\
                                   ANNIVERSARY:20090808T1430-0500\r\n\
                                   GENDER:M\r\n\
                                   LANG;PREF=1:fr\r\n\
                                   LANG;PREF=2:en\r\n\
                                   ORG;TYPE=work:Viagenie\r\n\
                                   ADR;TYPE=work:;Suite D2-630;2875 Laurier;\r\n\
                                   \x20Quebec;QC;G1V 2M2;Canada\r\n\
                                   TEL;VALUE=uri;TYPE=\"work,voice\";PREF=1:tel:+1-418-656-9254;ext=102\r\n\
                                   TEL;VALUE=uri;TYPE=\"work,cell,voice,video,text\":tel:+1-418-262-6501\r\n\
                                   EMAIL;TYPE=work:simon.perreault@viagenie.ca\r\n\
                                   GEO;TYPE=work:geo:46.772673,-71.282945\r\n\
                                   KEY;TYPE=work;VALUE=uri:\r\n\
                                   \x20http://www.viagenie.ca/simon.perreault/simon.asc\r\n\
                                   TZ:-0500\r\n\
                                   URL;TYPE=home:http://nomis80.org\r\n\
                                   END:VCARD\r\n";

#[test]
fn rfc6350_author_card() {
    let vcard: VCard = RFC6350_AUTHOR_CARD.parse().unwrap();

    assert_eq!("Simon Perreault", vcard.formatted_names[0].value);

    let name = &vcard.names[0].value;
    assert_eq!(name.family_names, ["Perreault"]);
    assert_eq!(name.given_names, ["Simon"]);
    assert_eq!(name.honorific_suffixes, ["ing. jr", "M.Sc."]);

    assert_eq!(
        DateAndOrTimeOrText::from(Date::from_month_day(2, 3).unwrap()),
        vcard.birthday.as_ref().unwrap().value
    );

    assert_eq!(
        DateAndOrTimeOrText::DateAndOrTime(DateAndOrTime::DateTime(
            DateTime::new(
                Date::from_year_month_day(2009, 8, 8).unwrap(),
                Time::from_hour_minute(14, 30)
                    .unwrap()
                    .with_zone(Zone::Offset(UtcOffset::new(true, 5, 0).unwrap())),
            )
            .unwrap()
        )),
        vcard.anniversary.as_ref().unwrap().value
    );

    assert_eq!(Some(Sex::Male), vcard.gender.as_ref().unwrap().value.sex);

    assert_eq!("fr", vcard.langs[0].value.as_str());
    assert_eq!(Some(Pref::new(1).unwrap()), vcard.langs[0].parameters.pref);
    assert_eq!("en", vcard.langs[1].value.as_str());

    assert_eq!("Viagenie", vcard.organizations[0].value.name);

    // The folded ADR line must be unfolded before being split into components.
    let address = &vcard.addresses[0].value;
    assert_eq!(address.extended_addresses, ["Suite D2-630"]);
    assert_eq!(address.street_addresses, ["2875 Laurier"]);
    assert_eq!(address.localities, ["Quebec"]);
    assert_eq!(address.regions, ["QC"]);
    assert_eq!(address.postal_codes, ["G1V 2M2"]);
    assert_eq!(address.countries, ["Canada"]);

    // The quoted TYPE list form must be accepted and split into values.
    let telephone = &vcard.telephones[0];
    assert_eq!(TelValue::Uri("tel:+1-418-656-9254;ext=102".parse().unwrap()), telephone.value);
    assert_eq!(telephone.parameters.types, [TypeValue::Work, TypeValue::Voice]);
    assert_eq!(Some(Pref::new(1).unwrap()), telephone.parameters.pref);

    assert_eq!(vcard.telephones[1].parameters.types, [
        TypeValue::Work,
        TypeValue::Cell,
        TypeValue::Voice,
        TypeValue::Video,
        TypeValue::Text
    ]);

    assert_eq!("simon.perreault@viagenie.ca", vcard.emails[0].value.as_str());

    assert_eq!("geo:46.772673,-71.282945", vcard.geos[0].value.as_str());

    // The KEY value is folded right after the colon.
    assert_eq!("http://www.viagenie.ca/simon.perreault/simon.asc", match &vcard.keys[0].value {
        vcard::values::TextOrUri::Uri(uri) => uri.as_str(),
        _ => panic!("the KEY value should be a URI"),
    });

    // The TZ property uses the default text form here.
    assert_eq!(TzValue::Text(String::from("-0500")), vcard.time_zones[0].value);

    assert_eq!("http://nomis80.org/", vcard.urls[0].value.as_str());
}

#[test]
fn group_card() {
    let vcard: VCard = "BEGIN:VCARD\r\nVERSION:4.0\r\nKIND:group\r\nFN:The Doe \
                        family\r\nMEMBER:urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af\r\nMEMBER:\
                        urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519\r\nEND:VCARD\r\n"
        .parse()
        .unwrap();

    assert_eq!(KindValue::Group, vcard.kind.as_ref().unwrap().value);
    assert_eq!(2, vcard.members.len());
    assert_eq!("urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af", vcard.members[0].value.as_str());

    assert!(vcard.validate().is_ok());
}

#[test]
fn multiple_cards() {
    let vcards = VCard::parse_multiple(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nKIND:group\r\nFN:The Doe \
         family\r\nMEMBER:urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af\r\nMEMBER:urn:uuid:\
         b8767877-b4a1-4c70-9acc-505d3819e519\r\nEND:VCARD\r\nBEGIN:VCARD\r\nVERSION:4.0\r\nFN:\
         John Doe\r\nUID:urn:uuid:03a0e51f-d1aa-4385-8a53-e29025acd8af\r\nEND:VCARD\r\nBEGIN:\
         VCARD\r\nVERSION:4.0\r\nFN:Jane \
         Doe\r\nUID:urn:uuid:b8767877-b4a1-4c70-9acc-505d3819e519\r\nEND:VCARD\r\n",
    )
    .unwrap();

    assert_eq!(3, vcards.len());
    assert_eq!("John Doe", vcards[1].formatted_names[0].value);
    assert_eq!("Jane Doe", vcards[2].formatted_names[0].value);
}

#[test]
fn extension_property() {
    let vcard: VCard = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nitem1.X-FOO;X-BAR=1:hello\\, \
                        world\r\nEND:VCARD\r\n"
        .parse()
        .unwrap();

    let extension = &vcard.extensions[0];
    assert_eq!("item1", extension.group.as_ref().unwrap().as_str());
    assert_eq!("X-FOO", extension.name.as_str());
    assert_eq!("X-BAR", extension.parameters.any[0].name.as_str());
    assert_eq!(extension.parameters.any[0].values, ["1"]);
    assert_eq!("hello\\, world", extension.value);
}

#[test]
fn lf_only_line_breaks() {
    let vcard: VCard = "BEGIN:VCARD\nVERSION:4.0\nFN:Magic Len\nEND:VCARD\n".parse().unwrap();

    assert_eq!("Magic Len", vcard.formatted_names[0].value);
}

#[test]
fn unsupported_version() {
    let error =
        "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Test\r\nEND:VCARD\r\n".parse::<VCard>().unwrap_err();

    assert_eq!(2, error.line);
    assert_eq!(ParseErrorKind::UnsupportedVersion(String::from("3.0")), error.kind);
}

#[test]
fn missing_formatted_name() {
    let error = "BEGIN:VCARD\r\nVERSION:4.0\r\nEND:VCARD\r\n".parse::<VCard>().unwrap_err();

    assert_eq!(ParseErrorKind::MissingFormattedName, error.kind);
}

#[test]
fn duplicate_single_property() {
    let error = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nBDAY:19900102\r\nBDAY:19900103\r\nEND:\
                 VCARD\r\n"
        .parse::<VCard>()
        .unwrap_err();

    assert_eq!(5, error.line);
    assert_eq!(ParseErrorKind::DuplicateProperty(String::from("BDAY")), error.kind);
}
