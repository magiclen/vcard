use vcard::{Url, VCard};

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
