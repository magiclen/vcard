use vcard::values::Uri;

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
    }
}
