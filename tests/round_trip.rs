use vcard::{TzParam, VCard};

/// Parses the input, serializes it, and parses it again, expecting both parses to give equal structures.
fn assert_round_trip(input: &str) {
    let first: VCard = input.parse().unwrap();

    let output = first.to_string();

    let second: VCard = output.parse().unwrap();

    assert_eq!(first, second, "the round trip changed the data of: {input}");
}

/// Serializing an already canonical vCard text must reproduce it byte by byte.
fn assert_canonical(input: &str) {
    let vcard: VCard = input.parse().unwrap();

    assert_eq!(input, vcard.to_string());
}

#[test]
fn rfc6350_author_card() {
    assert_round_trip(
        "BEGIN:VCARD\r\n\
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
         END:VCARD\r\n",
    );
}

#[test]
fn rfc9554_features() {
    assert_round_trip(
        "BEGIN:VCARD\r\n\
         VERSION:4.0\r\n\
         CREATED;VALUE=TIMESTAMP:20211022T140000-05\r\n\
         FN;DERIVED=TRUE:Mr. John Quinlan\r\n\
         N;ALTID=1;LANGUAGE=zh-Hant:孫;中山;文,逸仙;;;;\r\n\
         N;ALTID=1;PHONETIC=jyut;SCRIPT=Latn;LANGUAGE=yue:syun1;zung1saan1;man4,jat6sin1;;;;\r\n\
         GRAMGENDER;LANGUAGE=de:feminine\r\n\
         PRONOUNS;LANGUAGE=en;PREF=1:xe/xir\r\n\
         SOCIALPROFILE;SERVICE-TYPE=Mastodon:https://example.com/@foo\r\n\
         SOCIALPROFILE;SERVICE-TYPE=SomeSite;VALUE=text:peter94\r\n\
         ADR;GEO=\"geo:12.3457,78.910\":;;123 Main Street;Any Town;CA;91921-1234;U.S.A;;;;123;Main Street;;;;;;\r\n\
         NOTE;AUTHOR=\"mailto:john@example.com\":This is some note.\r\n\
         NOTE;AUTHOR-NAME=John Doe;CREATED=20221122T151823Z:Another note.\r\n\
         LANGUAGE:de-AT\r\n\
         END:VCARD\r\n",
    );
}

#[test]
fn value_type_variants() {
    assert_round_trip(
        "BEGIN:VCARD\r\n\
         VERSION:4.0\r\n\
         FN:Test\r\n\
         NICKNAME;PID=5.1,3:Jim,Jimmie\r\n\
         BDAY;VALUE=text:circa 1800\r\n\
         TZ;VALUE=utc-offset:-0500\r\n\
         TZ;VALUE=uri:https://example.com/tz\r\n\
         RELATED;TYPE=friend:urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6\r\n\
         RELATED;VALUE=text:Jane\r\n\
         UID;VALUE=text:some-identifier\r\n\
         CLIENTPIDMAP:1;urn:uuid:53e374d9-337e-4727-8803-a1e9c14e0556\r\n\
         KEY;VALUE=text:plain-key-text\r\n\
         X-UNKNOWN;VALUE=date:19960415\r\n\
         END:VCARD\r\n",
    );
}

#[test]
fn canonical_output_is_stable() {
    assert_canonical("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n");

    assert_canonical(
        "BEGIN:VCARD\r\n\
         VERSION:4.0\r\n\
         FN:David Wang\r\n\
         FN;LANGUAGE=zh-Hant:王大衛\r\n\
         N:Wang;David;;;\r\n\
         NICKNAME:Dave,大衛\r\n\
         BDAY:19900102\r\n\
         GENDER:M\r\n\
         ADR;TYPE=home:;;No.5 Xinyi Road;Taipei;;110;Taiwan\r\n\
         TEL;VALUE=uri;PREF=1;TYPE=work,voice:tel:+886-2-1234-5678\r\n\
         EMAIL;TYPE=work:len@magiclen.org\r\n\
         LANG:zh-Hant\r\n\
         TZ:Asia/Taipei\r\n\
         REV:20080124T195509Z\r\n\
         UID:urn:uuid:550e8400-e29b-41d4-a716-446655440000\r\n\
         URL:https://magiclen.org/\r\n\
         END:VCARD\r\n",
    );
}

#[test]
fn birthday_date_and_time_forms() {
    // Every legal truncated or reduced form must survive a canonical round trip.
    for value in [
        "19850412",
        "1985-04",
        "1985",
        "--0412",
        "--04",
        "---12",
        "T102200",
        "T1022",
        "T10",
        "T-2200",
        "T-22",
        "T--00",
        "T102200Z",
        "T102200+0800",
        "T102200-0530",
        "19961022T140000",
        "--1022T1400",
        "---22T14",
        "19961022T140000Z",
        "19961022T140000+0800",
    ] {
        let input =
            format!("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nBDAY:{value}\r\nEND:VCARD\r\n");

        assert_canonical(&input);
    }
}

#[test]
fn parameter_text_and_uri_values() {
    let input = "BEGIN:VCARD\r\n\
                 VERSION:4.0\r\n\
                 FN:Parameters\r\n\
                 ADR;TZ=\"Europe/Paris\";LABEL=A^NB^nC^^D^'E:;;;;;;\r\n\
                 ADR;TZ=\"Europe/Paris, local\":;;;;;;\r\n\
                 ADR;TZ=\"https://example.com/^n/^^/^'\":;;;;;;\r\n\
                 NOTE;AUTHOR=\"https://example.com/^n/^^/^'\";GEO=\"https://example.com/^n/^^/^'\":Test\r\n\
                 END:VCARD\r\n";

    let vcard: VCard = input.parse().unwrap();

    assert_eq!(Some(TzParam::Text(String::from("Europe/Paris"))), vcard.addresses[0].parameters.tz);
    assert_eq!(Some("A^NB\nC^D\"E"), vcard.addresses[0].parameters.label.as_deref());
    assert_eq!(
        Some(TzParam::Text(String::from("Europe/Paris, local"))),
        vcard.addresses[1].parameters.tz
    );

    let uri = "https://example.com/^n/^^/^'";

    assert_eq!(Some(TzParam::Uri(uri.parse().unwrap())), vcard.addresses[2].parameters.tz);
    assert_eq!(uri, vcard.notes[0].parameters.author.as_ref().unwrap().as_str());
    assert_eq!(uri, vcard.notes[0].parameters.geo.as_ref().unwrap().as_str());

    assert_eq!(vcard, vcard.to_string().parse::<VCard>().unwrap());
}

#[test]
fn single_properties_keep_altid_representations() {
    let input = concat!(
        "BEGIN:VCARD\r\n",
        "VERSION:4.0\r\n",
        "KIND;ALTID=1:group\r\n",
        "KIND;ALTID=1:group\r\n",
        "FN:The Doe family\r\n",
        "N;ALTID=1;LANGUAGE=en:Doe;John;;;\r\n",
        "N;ALTID=1;LANGUAGE=zh-Hant:杜;約翰;;;\r\n",
        "BDAY;ALTID=1:19900102\r\n",
        "BDAY;VALUE=text;ALTID=1:January 2\\, 1990\r\n",
        "ANNIVERSARY;ALTID=1:20000102\r\n",
        "ANNIVERSARY;VALUE=text;ALTID=1:January 2\\, 2000\r\n",
        "GENDER;ALTID=1;LANGUAGE=en:M;Man\r\n",
        "GENDER;ALTID=1;LANGUAGE=zh-Hant:M;男性\r\n",
        "MEMBER:urn:example:member\r\n",
        "PRODID;ALTID=1;LANGUAGE=en:Address Book\r\n",
        "PRODID;ALTID=1;LANGUAGE=zh-Hant:通訊錄\r\n",
        "REV;ALTID=1:20161231T235960Z\r\n",
        "REV;ALTID=1:20170101T075960+0800\r\n",
        "UID;ALTID=1:urn:example:doe-family\r\n",
        "UID;VALUE=text;ALTID=1:doe-family\r\n",
        "CREATED;ALTID=1:20000102T120000Z\r\n",
        "CREATED;ALTID=1:20000102T200000+0800\r\n",
        "LANGUAGE;ALTID=1:en\r\n",
        "LANGUAGE;ALTID=1:EN\r\n",
        "END:VCARD\r\n",
    );

    let vcard: VCard = input.parse().unwrap();

    for (name, count) in [
        ("KIND", vcard.kinds.len()),
        ("N", vcard.names.len()),
        ("BDAY", vcard.birthdays.len()),
        ("ANNIVERSARY", vcard.anniversaries.len()),
        ("GENDER", vcard.genders.len()),
        ("PRODID", vcard.product_ids.len()),
        ("REV", vcard.revisions.len()),
        ("UID", vcard.uids.len()),
        ("CREATED", vcard.created.len()),
        ("LANGUAGE", vcard.languages.len()),
    ] {
        assert_eq!(2, count, "the {name} representations were lost");
    }

    assert!(vcard.validate().is_ok());
    assert_eq!(vcard, vcard.to_string().parse::<VCard>().unwrap());
}
