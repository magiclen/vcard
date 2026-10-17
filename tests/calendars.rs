use vcard::{
    AnyParameter, Birthday, Calendar, CalendarRegistry, Calscale, Parameters, ParseErrorKind,
    VCard, ValidationError,
    values::{Date, DateAndOrTime, DateAndOrTimeOrText, DateParts},
};

struct TestCalendar;

impl Calendar for TestCalendar {
    fn name(&self) -> &'static str {
        "x-test"
    }

    fn is_valid_date(&self, date: &DateParts, parameters: &Parameters) -> bool {
        date.day.is_none_or(|day| day <= 30)
            || parameters.any.iter().any(|parameter| {
                parameter.name.as_str() == "X-EXTRA-DAY" && parameter.values == ["true"]
            })
    }
}

struct TestRegistry(TestCalendar);

impl CalendarRegistry for TestRegistry {
    fn get(&self, scale: &str) -> Option<&dyn Calendar> {
        match scale {
            "x-test" => Some(&self.0),
            _ => None,
        }
    }
}

const CUSTOM_CARD: &str = concat!(
    "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
    "BDAY;ALTID=1;CALSCALE=X-TEST:20230230\r\n",
    "BDAY;ALTID=1;CALSCALE=x-test:20230230T1022+0800\r\n",
    "ANNIVERSARY;CALSCALE=x-test:--0230\r\nEND:VCARD\r\n",
);

#[test]
fn custom_calendar_dates_round_trip() {
    let calendars = TestRegistry(TestCalendar);
    let card = VCard::parse_with_calendars(CUSTOM_CARD, &calendars).unwrap();

    assert!(card.validate().is_ok());
    assert!(card.validate_with_calendars(&calendars).is_ok());
    assert_eq!(
        Some("X-TEST"),
        card.birthdays[0].parameters.calscale.as_ref().map(Calscale::as_str)
    );

    let DateAndOrTimeOrText::DateAndOrTime(DateAndOrTime::Date(date)) = card.birthdays[0].value
    else {
        panic!("the birthday should contain a date");
    };

    assert_eq!("x-test", date.calendar_scale());
    assert_eq!(Some(2023), date.year());
    assert_eq!(Some(2), date.month());
    assert_eq!(Some(30), date.day());
    assert_eq!("20230230", date.to_string());
    assert!("20230230".parse::<Date>().is_err());
    assert_eq!(card, VCard::parse_with_calendars(&card.to_string(), &calendars).unwrap());

    let default_card: VCard = CUSTOM_CARD.parse().unwrap();
    assert!(default_card.birthdays.is_empty());
    assert!(default_card.anniversaries.is_empty());
    assert_eq!("Test", default_card.formatted_names[0].value);
}

#[test]
fn partial_dates_use_the_external_calendar() {
    let calendars = TestRegistry(TestCalendar);

    for value in ["2023", "2023-02", "--02", "---30"] {
        let input = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "BDAY;CALSCALE=x-test:{value}\r\nEND:VCARD\r\n",
            ),
            value = value,
        );
        let card = VCard::parse_with_calendars(&input, &calendars).unwrap();

        assert!(card.validate_with_calendars(&calendars).is_ok());
        assert_eq!(card, VCard::parse_with_calendars(&card.to_string(), &calendars).unwrap());
    }
}

#[test]
fn calendar_representations_share_altid() {
    let input = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "BDAY;ALTID=1:20230301\r\n",
        "BDAY;ALTID=1;CALSCALE=x-test:20230230\r\nEND:VCARD\r\n",
    );
    let calendars = TestRegistry(TestCalendar);
    let card = VCard::parse_with_calendars(input, &calendars).unwrap();

    assert_eq!(2, card.birthdays.len());
    assert!(card.validate_with_calendars(&calendars).is_ok());
    assert_eq!(card, VCard::parse_with_calendars(&card.to_string(), &calendars).unwrap());

    let default_card: VCard = input.parse().unwrap();
    assert_eq!(1, default_card.birthdays.len());
    assert_eq!(card.birthdays[0], default_card.birthdays[0]);
}

#[test]
fn custom_dates_keep_parameter_context() {
    let calendars = TestRegistry(TestCalendar);
    let parameters = Parameters {
        calscale: Some("x-test".parse().unwrap()),
        any: vec![AnyParameter {
            name:   "X-EXTRA-DAY".parse().unwrap(),
            values: vec![String::from("true")],
        }],
        ..Parameters::default()
    };
    let parts = DateParts {
        year: Some(2023), month: Some(2), day: Some(31)
    };
    let date = Date::from_calendar_parts(parts, &parameters, &calendars.0).unwrap();
    let mut birthday = Birthday::new(date.into());
    birthday.parameters = parameters;

    let mut card = VCard::new("Test");
    card.birthdays.push(birthday);

    assert!(card.validate_with_calendars(&calendars).is_ok());
    assert_eq!(card, VCard::parse_with_calendars(&card.to_string(), &calendars).unwrap());

    card.birthdays[0].parameters.any.clear();
    assert!(card.validate().is_ok());
    assert_eq!(
        Err(ValidationError::InvalidParameter {
            property:  String::from("BDAY"),
            parameter: String::from("CALSCALE"),
        }),
        card.validate_with_calendars(&calendars)
    );

    for scale in [None, Some(Calscale::Gregorian)] {
        card.birthdays[0].parameters.calscale = scale;
        assert_eq!(
            Err(ValidationError::InvalidParameter {
                property:  String::from("BDAY"),
                parameter: String::from("CALSCALE"),
            }),
            card.validate()
        );
    }
}

#[test]
fn unknown_calendars_do_not_affect_dates_or_cardinality() {
    let input = concat!(
        "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
        "BDAY;CALSCALE=x-unknown:unrecognized date syntax\r\n",
        "BDAY;CALSCALE=GREGORIAN:19900102\r\n",
        "BDAY;CALSCALE=x-unknown;CALSCALE=gregorian:19900103\r\n",
        "BDAY;CALSCALE=x-unknown:another date\r\n",
        "ANNIVERSARY;CALSCALE=x-unknown:unrecognized date syntax\r\nEND:VCARD\r\n",
    );
    let card: VCard = input.parse().unwrap();
    let calendars = TestRegistry(TestCalendar);

    assert_eq!(1, card.birthdays.len());
    assert_eq!(
        DateAndOrTimeOrText::from(Date::from_year_month_day(1990, 1, 2).unwrap()),
        card.birthdays[0].value
    );
    assert!(card.anniversaries.is_empty());
    assert!(card.validate_with_calendars(&calendars).is_ok());
    assert_eq!(card, VCard::parse_with_calendars(input, &calendars).unwrap());
    assert!(!card.to_string().contains("x-unknown"));
}

#[test]
fn multiple_cards_share_the_calendar_registry() {
    let input = [CUSTOM_CARD, CUSTOM_CARD].concat();
    let calendars = TestRegistry(TestCalendar);
    let expected = VCard::parse_with_calendars(CUSTOM_CARD, &calendars).unwrap();
    let cards = VCard::parse_multiple_with_calendars(&input, &calendars).unwrap();

    assert_eq!(vec![expected.clone(), expected], cards);

    let default_cards = VCard::parse_multiple(&input).unwrap();
    assert_eq!(2, default_cards.len());
    assert!(
        default_cards.iter().all(|card| card.birthdays.is_empty() && card.anniversaries.is_empty())
    );
}

#[test]
fn custom_calendars_keep_rfc_date_bounds() {
    let calendar = TestCalendar;
    let parameters = Parameters {
        calscale: Some("x-test".parse().unwrap()),
        ..Parameters::default()
    };

    for parts in [
        DateParts {
            year: Some(10000), month: Some(1), day: Some(1)
        },
        DateParts {
            year: Some(2023), month: Some(13), day: Some(1)
        },
        DateParts {
            year: Some(2023), month: None, day: Some(1)
        },
    ] {
        assert!(Date::from_calendar_parts(parts, &parameters, &calendar).is_err());
    }

    let calendars = TestRegistry(calendar);

    for value in ["20231301", "100000101"] {
        let input = format!(
            concat!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\n",
                "BDAY;CALSCALE=x-test:{value}\r\nEND:VCARD\r\n",
            ),
            value = value,
        );
        let error = VCard::parse_with_calendars(&input, &calendars).unwrap_err();

        assert_eq!(4, error.line);
        assert_eq!(
            ParseErrorKind::InvalidValue {
                property: String::from("BDAY")
            },
            error.kind
        );
    }
}
