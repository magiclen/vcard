use vcard::{
    Photo, Url, VCard,
    values::{TelValue, TextOrUri, TzValue, Uri},
};

#[test]
fn data_uri_escapes_mime_components() {
    for (mime, metadata) in [
        ("text/plain", "text/plain"),
        (r#"text/plain;name="a,b""#, "text/plain;name=a%2Cb"),
        ("text/plain;name=a%2Cb", "text/plain;name=a%252Cb"),
        ("image/svg+xml;charset=utf-8", "image/svg+xml;charset=utf-8"),
        (r#"text/plain;name="a?b;c=d e""#, "text/plain;name=a%3Fb%3Bc%3Dd%20e"),
        (r#"text/plain;name="a\\b""#, "text/plain;name=a%5Cb"),
        ("application/x-a#b;p#q=x", "application/x-a%23b;p%23q=x"),
    ] {
        let uri = Uri::from_data(&mime.parse().unwrap(), b"Hi");

        assert_eq!(format!("data:{metadata};base64,SGk="), uri.as_str());

        let mut card = VCard::new("Test");
        card.photos.push(Photo::new(uri));
        let output = card.to_string();

        assert_eq!(
            format!(
                concat!(
                    "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                    "PHOTO:data:{metadata};base64\\,SGk=\r\nEND:VCARD\r\n",
                ),
                metadata = metadata,
            ),
            output
        );
        assert_eq!(card, output.parse::<VCard>().unwrap());
    }
}

#[test]
fn property_uris_accept_backslash_escapes() {
    for (raw, expected, wire) in [
        ("geo:25.0,121.0", "geo:25.0,121.0", r"geo:25.0\,121.0"),
        (r"geo:25.0\,121.0", "geo:25.0,121.0", r"geo:25.0\,121.0"),
        (
            "data:text/plain;base64,SGk=",
            "data:text/plain;base64,SGk=",
            r"data:text/plain;base64\,SGk=",
        ),
        (
            r"data:text/plain;base64\,SGk=",
            "data:text/plain;base64,SGk=",
            r"data:text/plain;base64\,SGk=",
        ),
        (
            r"data:image/png\;base64\,SGk=",
            "data:image/png;base64,SGk=",
            r"data:image/png;base64\,SGk=",
        ),
        ("urn:example:a%2Cb%5Cc", "urn:example:a%2Cb%5Cc", "urn:example:a%2Cb%5Cc"),
    ] {
        let vcard: VCard = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "GEO:{raw}\r\n",
                "TEL;VALUE=uri:{raw}\r\n",
                "RELATED:{raw}\r\n",
                "TZ;VALUE=uri:{raw}\r\n",
                "CLIENTPIDMAP:1;{raw}\r\nEND:VCARD\r\n",
            ),
            raw = raw,
        )
        .parse()
        .unwrap();
        let uri: Uri = expected.parse().unwrap();

        assert_eq!(expected, vcard.geos[0].value.as_str());
        assert_eq!(TelValue::Uri(uri.clone()), vcard.telephones[0].value);
        assert_eq!(TextOrUri::Uri(uri.clone()), vcard.relations[0].value);
        assert_eq!(TzValue::Uri(uri.clone()), vcard.time_zones[0].value);
        assert_eq!(uri, vcard.client_pid_maps[0].value.uri);

        let output = vcard.to_string();

        for prefix in ["GEO:", "TEL;VALUE=uri:", "RELATED:", "TZ;VALUE=uri:", "CLIENTPIDMAP:1;"] {
            assert!(output.contains(&format!("{prefix}{wire}\r\n")));
        }

        assert_eq!(vcard, output.parse::<VCard>().unwrap());
    }
}

#[test]
fn property_uris_keep_literal_backslashes() {
    let mut card = VCard::new("Test");
    card.urls.push(Url::new(r"http://example.com/?a\,b".parse().unwrap()));

    assert_eq!(card, card.to_string().parse::<VCard>().unwrap());
}

#[test]
fn rfc3986_uris_round_trip_without_a_url() {
    for input in ["foo://[v1.example]/", "foo://example.com:999999/a?x=%2C#part", "https://"] {
        let uri: Uri = input.parse().unwrap();

        assert_eq!(input, uri.as_str());
        assert!(uri.as_url().is_none());
        assert!(uri.clone().into_url().is_err());

        let card: VCard = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "URL:{input}\r\n",
                "NOTE;AUTHOR=\"{input}\":Test\r\nEND:VCARD\r\n",
            ),
            input = input,
        )
        .parse()
        .unwrap();

        assert!(card.validate().is_ok());
        assert_eq!(uri, card.urls[0].value);
        assert_eq!(Some(input), card.notes[0].parameters.author.as_ref().map(Uri::as_str));
        assert_eq!("Test", card.notes[0].value);
        assert_eq!(card, card.to_string().parse::<VCard>().unwrap());
    }
}

#[test]
fn urls_keep_their_normalization_and_conversions() {
    let uri: Uri = "https://EXAMPLE.com".parse().unwrap();

    assert_eq!("https://example.com/", uri.as_str());
    assert_eq!(Some("example.com"), uri.as_url().unwrap().host_str());
    assert_eq!(uri, Uri::from(uri.clone().into_url().unwrap()));
}
