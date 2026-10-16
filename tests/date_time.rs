use chrono::{FixedOffset, TimeZone, Timelike};
use vcard::values::{Date, Timestamp};

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
    let timestamp = Timestamp::from(original);

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
