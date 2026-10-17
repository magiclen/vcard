use chrono::{FixedOffset, TimeZone, Timelike};
use vcard::{
    Birthday, VCard,
    values::{Date, DateAndOrTime, Time, Timestamp, UtcOffset, Zone},
};

#[test]
fn leap_seconds_round_trip() {
    for input in ["20161231T235960Z", "20170101T075960+0800"] {
        let timestamp: Timestamp = input.parse().unwrap();

        assert_eq!(input, timestamp.to_string());
        assert_eq!(1_000_000_000, timestamp.date_time().nanosecond());
    }
}

#[test]
fn minute_aligned_offset_is_preserved() {
    let original =
        FixedOffset::east_opt(8 * 3600).unwrap().with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    let timestamp = Timestamp::try_from(original).unwrap();

    assert_eq!("20260101T000000+0800", timestamp.to_string());
    assert_eq!(original, timestamp.to_fixed_offset().unwrap());
}

#[test]
fn date_constructors_keep_four_digit_years() {
    for year in [0, 2026, 9999] {
        let date = Date::from_year(year).unwrap();
        assert_eq!(date, date.to_string().parse::<Date>().unwrap());

        let date = Date::from_year_month_day(year, 1, 1).unwrap();
        assert_eq!(date, date.to_string().parse::<Date>().unwrap());

        let date = Date::from_year_month(year, 1).unwrap();
        assert_eq!(date, date.to_string().parse::<Date>().unwrap());
    }

    assert!(Date::from_year(10000).is_err());
    assert!(Date::from_year_month_day(10000, 1, 1).is_err());
    assert!(Date::from_year_month(10000, 1).is_err());
}

#[test]
fn times_with_hours_keep_their_zones() {
    for (time, value) in [
        (Time::from_hour(10).unwrap(), "10"),
        (Time::from_hour_minute(10, 22).unwrap(), "1022"),
        (Time::from_hour_minute_second(10, 22, 0).unwrap(), "102200"),
    ] {
        for (zone, suffix) in [
            (Zone::Utc, "Z"),
            (Zone::Offset(UtcOffset::new(false, 8, 0).unwrap()), "+0800"),
            (Zone::Offset(UtcOffset::new(true, 5, 30).unwrap()), "-0530"),
        ] {
            let time = time.with_zone(zone).unwrap();
            let expected = format!("{value}{suffix}");

            assert_eq!(expected, time.to_string());
            assert_eq!(time, expected.parse::<Time>().unwrap());

            let mut card = VCard::new("Test");
            card.birthdays.push(Birthday::new(DateAndOrTime::Time(time).into()));

            assert_eq!(card, card.to_string().parse::<VCard>().unwrap());
        }
    }
}

#[test]
fn truncated_times_remain_valid_without_zones() {
    for (time, value) in [
        (Time::from_minute(22).unwrap(), "-22"),
        (Time::from_minute_second(22, 0).unwrap(), "-2200"),
        (Time::from_second(42).unwrap(), "--42"),
    ] {
        assert_eq!(value, time.to_string());
        assert_eq!(time, value.parse::<Time>().unwrap());

        let mut card = VCard::new("Test");
        card.birthdays.push(Birthday::new(DateAndOrTime::Time(time).into()));

        assert_eq!(card, card.to_string().parse::<VCard>().unwrap());

        for (zone, suffix) in
            [(Zone::Utc, "Z"), (Zone::Offset(UtcOffset::new(false, 8, 0).unwrap()), "+0800")]
        {
            assert!(time.with_zone(zone).is_err());
            assert!(format!("{value}{suffix}").parse::<Time>().is_err());

            let input = format!(
                "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nBDAY:T{value}{suffix}\r\nEND:VCARD\r\n"
            );

            assert!(input.parse::<VCard>().is_err());
        }
    }
}
