use vcard::{ParseErrorKind, Pid, TypeValue, Url, VCard, ValidationError};

#[test]
fn extension_value_types_use_tokens() {
    let mut card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "X-EXTRA;VALUE=x-example:raw\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

    for value in ["", "bad value"] {
        let input = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "X-EXTRA;VALUE=\"{value}\":raw\r\nEND:VCARD\r\n",
            ),
            value = value,
        );
        let error = input.parse::<VCard>().unwrap_err();

        assert_eq!(4, error.line);
        assert_eq!(ParseErrorKind::InvalidParameter(String::from("VALUE")), error.kind);

        card.extensions[0].parameters.any[0].values[0] = value.to_string();

        assert_eq!(
            Err(ValidationError::InvalidParameter {
                property:  String::from("X-EXTRA"),
                parameter: String::from("VALUE"),
            }),
            card.validate()
        );
    }
}

#[test]
fn media_type_preserves_literal_carets() {
    for (mime, wire) in [
        ("application/x^test", "application/x^test"),
        ("application/x^news", "application/x^news"),
        ("application/x^^test", "application/x^^test"),
        ("text/plain;name=^'", "\"text/plain;name=^'\""),
        ("text/plain;name=^'^'", "\"text/plain;name=^'^'\""),
        ("text/plain;name=^'abc^'", "\"text/plain;name=^'abc^'\""),
        ("text/plain;name=^n", "\"text/plain;name=^n\""),
        ("text/plain;name=^^", "\"text/plain;name=^^\""),
        ("text/plain;charset=utf-8;name=^n", "\"text/plain;charset=utf-8;name=^n\""),
    ] {
        let media_type: mime::Mime = mime.parse().unwrap();
        let input = format!(
            "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nURL;MEDIATYPE={wire}:https://example.com/a\r\nEND:VCARD\r\n"
        );
        let parsed: VCard = input.parse().unwrap();

        assert_eq!(Some(&media_type), parsed.urls[0].parameters.media_type.as_ref());

        let mut card = VCard::new("Test");
        let mut url = Url::new("https://example.com/a".parse().unwrap());
        url.parameters.media_type = Some(media_type);
        card.urls.push(url);

        assert!(card.validate().is_ok());
        assert_eq!(input, card.to_string());
        assert_eq!(card, parsed);
    }
}

#[test]
fn media_type_accepts_plain_and_quoted_types() {
    for wire in ["text/plain", "\"text/plain\"", "\"text/plain;charset=utf-8\""] {
        let input = format!(
            "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nURL;MEDIATYPE={wire}:https://example.com/a\r\nEND:VCARD\r\n"
        );
        let card: VCard = input.parse().unwrap();
        let expected: mime::Mime = wire.trim_matches('"').parse().unwrap();

        assert_eq!(Some(&expected), card.urls[0].parameters.media_type.as_ref());
        assert_eq!(card, card.to_string().parse::<VCard>().unwrap());
    }
}

#[test]
fn parameter_lists_keep_quotes_and_repeated_values() {
    let card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "N;SORT-AS=\"Doe, family\",John:Doe;John;;;\r\n",
        "TEL;TYPE=work;TYPE=\"voice,cell\";PID=1;PID=2.1:123\r\n",
        "CLIENTPIDMAP:1;urn:example:source\r\n",
        "X-EXTRA;X-TAGS=\"a,b\",c^'d,\"e^nf\":raw\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());
    assert_eq!(vec!["Doe, family", "John"], card.names[0].parameters.sort_as);
    assert_eq!(
        vec![TypeValue::Work, TypeValue::Voice, TypeValue::Cell],
        card.telephones[0].parameters.types
    );
    assert_eq!(
        vec![
            Pid {
                id: 1, source: None
            },
            Pid {
                id: 2, source: Some(1)
            }
        ],
        card.telephones[0].parameters.pids
    );
    assert_eq!(vec!["a,b", "c\"d", "e\nf"], card.extensions[0].parameters.any[0].values);
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());
}

#[test]
fn rfc9554_parameters_follow_occurrence_limits() {
    let card: VCard = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "GRAMGENDER;LANGUAGE=de:feminine\r\n",
        "PRONOUNS;LANGUAGE=en;PREF=1;TYPE=home,work;ALTID=1:they/them\r\n",
        "SOCIALPROFILE;SERVICE-TYPE=Example;VALUE=text:alice\r\nEND:VCARD\r\n",
    )
    .parse()
    .unwrap();

    assert!(card.validate().is_ok());
    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

    for (name, first, repeated, value) in [
        ("GRAMGENDER", "LANGUAGE=de", "language=fr", "feminine"),
        ("PRONOUNS", "LANGUAGE=en", "language=de", "they/them"),
        ("PRONOUNS", "PREF=1", "pref=2", "they/them"),
        ("PRONOUNS", "TYPE=home", "type=work", "they/them"),
        ("PRONOUNS", "ALTID=1", "altid=2", "they/them"),
        (
            "SOCIALPROFILE",
            "SERVICE-TYPE=Example",
            "service-type=Other",
            "https://example.com/alice",
        ),
    ] {
        let error = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "{name};{first};{repeated}:{value}\r\nEND:VCARD\r\n",
            ),
            name = name,
            first = first,
            repeated = repeated,
            value = value,
        )
        .parse::<VCard>()
        .unwrap_err();

        let parameter = repeated.split_once('=').unwrap().0;

        assert_eq!(4, error.line);
        assert_eq!(ParseErrorKind::InvalidParameter(parameter.to_string()), error.kind);
    }
}
