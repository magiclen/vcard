use vcard::{
    Address, Birthday, Created, Email, ExtensionProperty, FormattedName, Gender, GramGender, Kind,
    Lang, Language, Name, Nickname, Note, Phonetic, Pref, Pronouns, Rev, SocialProfile, Tel,
    TimeZone, TypeValue, Uid, Url, VCard,
    values::{
        AddressValue, Date, GenderValue, GramGenderValue, KindValue, NameValue, Sex, TelValue,
        TextOrUri, TzValue,
    },
};

#[test]
fn basic_v4p0() {
    let vcard = VCard::new("Magic Len");

    assert_eq!("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Magic Len\r\nEND:VCARD\r\n", vcard.to_string());
}

#[test]
fn personal_card() {
    let mut vcard = VCard::new("David Wang");

    vcard.kind = Some(Kind::new(KindValue::Individual));

    let mut formatted_name = FormattedName::from("王大衛");
    formatted_name.parameters.language = Some("zh-Hant".parse().unwrap());
    vcard.formatted_names.push(formatted_name);

    vcard.names.push(Name::new(NameValue {
        family_names: vec![String::from("Wang")],
        given_names: vec![String::from("David")],
        ..NameValue::default()
    }));

    vcard.nicknames.push(Nickname::new(vec![String::from("Dave"), String::from("大衛")]));

    vcard.birthday = Some(Birthday::new(Date::from_year_month_day(1990, 1, 2).unwrap().into()));

    vcard.gender = Some(Gender::new(GenderValue {
        sex: Some(Sex::Male), identity: None
    }));

    let mut address = Address::new(AddressValue {
        street_addresses: vec![String::from("No.5 Xinyi Road")],
        localities: vec![String::from("Taipei")],
        postal_codes: vec![String::from("110")],
        countries: vec![String::from("Taiwan")],
        ..AddressValue::default()
    });
    address.parameters.types.push(TypeValue::Home);
    vcard.addresses.push(address);

    let mut telephone = Tel::new(TelValue::Uri("tel:+886-2-1234-5678".parse().unwrap()));
    telephone.parameters.pref = Some(Pref::new(1).unwrap());
    telephone.parameters.types.extend([TypeValue::Work, TypeValue::Voice]);
    vcard.telephones.push(telephone);

    let mut email = Email::new("len@magiclen.org".parse().unwrap());
    email.parameters.types.push(TypeValue::Work);
    vcard.emails.push(email);

    vcard.langs.push(Lang::new("zh-Hant".parse().unwrap()));

    vcard.time_zones.push(TimeZone::new(TzValue::from_time_zone(chrono_tz::Tz::Asia__Taipei)));

    vcard.revision = Some(Rev::new("20080124T195509Z".parse().unwrap()));

    vcard.uid =
        Some(Uid::new(TextOrUri::from_uuid_str("550e8400-e29b-41d4-a716-446655440000").unwrap()));

    vcard.urls.push(Url::new("https://magiclen.org".parse().unwrap()));

    assert_eq!(
        "BEGIN:VCARD\r\n\
         VERSION:4.0\r\n\
         KIND:individual\r\n\
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
        vcard.to_string()
    );
}

#[test]
fn escaping() {
    let mut vcard = VCard::new("Test");

    vcard.names.push(Name::new(NameValue {
        family_names: vec![String::from("Foo,Bar;Baz")],
        given_names: vec![String::from("A"), String::from("B")],
        ..NameValue::default()
    }));

    vcard.gender = Some(Gender::new(GenderValue {
        sex:      Some(Sex::Other),
        identity: Some(String::from("it's complicated")),
    }));

    vcard.notes.push(Note::from("Hello, world; A\\B\nnew line"));

    assert_eq!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nN:Foo\\,Bar\\;Baz;A,B;;;\r\nGENDER:O;it's \
         complicated\r\nNOTE:Hello\\, world; A\\\\B\\nnew line\r\nEND:VCARD\r\n",
        vcard.to_string()
    );
}

#[test]
fn group_construct() {
    let mut vcard = VCard::new("Test");

    let mut telephone = Tel::new(TelValue::Text(String::from("0912345678")));
    telephone.group = Some("item1".parse().unwrap());
    vcard.telephones.push(telephone);

    assert_eq!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nitem1.TEL:0912345678\r\nEND:VCARD\r\n",
        vcard.to_string()
    );
}

#[test]
fn folding_long_line() {
    let mut vcard = VCard::new("Fold");

    vcard.notes.push(Note::from(
        "這是一段非常長的中文文字，用來測試序列化的折行功能是否正確運作，\
         每一個實體行的長度都不可以超過七十五個位元組，而且不可以把多位元組字元從中間切斷。",
    ));

    let output = vcard.to_string();

    // Every physical line must fit in 75 octets, not counting the line break.
    for line in output.split("\r\n") {
        assert!(line.len() <= 75, "the line is too long: {line}");
    }

    // Unfolding must give back exactly the same data.
    let parsed: VCard = output.parse().unwrap();

    assert_eq!(vcard, parsed);
}

#[test]
fn rfc9554_card() {
    let mut vcard = VCard::new("Sun Yat-sen");

    let mut derived_name = FormattedName::from("Dr. Sun Yat-sen");
    derived_name.parameters.derived = Some(true);
    vcard.formatted_names.push(derived_name);

    let mut name = Name::new(NameValue {
        family_names: vec![String::from("孫")],
        given_names: vec![String::from("中山")],
        ..NameValue::default()
    });
    name.parameters.language = Some("zh-Hant".parse().unwrap());
    name.parameters.altid = Some(String::from("1"));
    vcard.names.push(name);

    let mut phonetic_name = Name::new(NameValue {
        family_names: vec![String::from("syun1")],
        given_names: vec![String::from("zung1saan1")],
        ..NameValue::default()
    });
    phonetic_name.parameters.language = Some("yue".parse().unwrap());
    phonetic_name.parameters.altid = Some(String::from("1"));
    phonetic_name.parameters.phonetic = Some(Phonetic::Jyut);
    phonetic_name.parameters.script = Some("Latn".parse().unwrap());
    vcard.names.push(phonetic_name);

    let mut address = Address::new(AddressValue {
        street_addresses: vec![String::from("1 Main St")],
        localities: vec![String::from("Town")],
        street_numbers: vec![String::from("1")],
        street_names: vec![String::from("Main St")],
        ..AddressValue::default()
    });
    address.parameters.label = Some(String::from("1 Main St\nTown, USA"));
    vcard.addresses.push(address);

    let mut note = Note::from("A note.");
    note.parameters.author_name = Some(String::from("Jane \"JD\" Doe"));
    vcard.notes.push(note);

    vcard.created = Some(Created::new("20220705T093412Z".parse().unwrap()));

    let mut gram_gender = GramGender::new(GramGenderValue::Feminine);
    gram_gender.parameters.language = Some("de".parse().unwrap());
    vcard.gram_genders.push(gram_gender);

    vcard.language = Some(Language::new("zh-Hant".parse().unwrap()));

    let mut pronouns = Pronouns::from("xe/xir");
    pronouns.parameters.pref = Some(Pref::new(1).unwrap());
    vcard.pronouns.push(pronouns);

    let mut social_profile =
        SocialProfile::new(TextOrUri::Uri("https://example.com/@foo".parse().unwrap()));
    social_profile.parameters.service_type = Some(String::from("Mastodon"));
    social_profile.parameters.username = Some(String::from("foo"));
    vcard.social_profiles.push(social_profile);

    let mut text_profile = SocialProfile::new(TextOrUri::Text(String::from("peter94")));
    text_profile.parameters.service_type = Some(String::from("SomeSite"));
    vcard.social_profiles.push(text_profile);

    vcard.extensions.push(ExtensionProperty::from_text("X-TEST".parse().unwrap(), "hello, world"));

    assert_eq!(
        "BEGIN:VCARD\r\n\
         VERSION:4.0\r\n\
         FN:Sun Yat-sen\r\n\
         FN;DERIVED=true:Dr. Sun Yat-sen\r\n\
         N;LANGUAGE=zh-Hant;ALTID=1:孫;中山;;;\r\n\
         N;LANGUAGE=yue;ALTID=1;PHONETIC=jyut;SCRIPT=Latn:syun1;zung1saan1;;;\r\n\
         ADR;LABEL=\"1 Main St^nTown, USA\":;;1 Main St;Town;;;;;;;1;Main St\r\n\
         NOTE;AUTHOR-NAME=Jane ^'JD^' Doe:A note.\r\n\
         CREATED:20220705T093412Z\r\n\
         GRAMGENDER;LANGUAGE=de:feminine\r\n\
         LANGUAGE:zh-Hant\r\n\
         PRONOUNS;PREF=1:xe/xir\r\n\
         SOCIALPROFILE;SERVICE-TYPE=Mastodon;USERNAME=foo:https://example.com/@foo\r\n\
         SOCIALPROFILE;VALUE=text;SERVICE-TYPE=SomeSite:peter94\r\n\
         X-TEST:hello\\, world\r\n\
         END:VCARD\r\n",
        vcard.to_string()
    );
}
