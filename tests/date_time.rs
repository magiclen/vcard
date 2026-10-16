use chrono::{FixedOffset, NaiveDate, TimeZone, Timelike, Utc};
use vcard::values::{Date, Timestamp, Zone};

#[test]
fn leap_seconds_round_trip() {
    for input in ["20161231T235960Z", "20170101T075960+0800"] {
        let timestamp: Timestamp = input.parse().unwrap();

        assert_eq!(input, timestamp.to_string());
        assert_eq!(1_000_000_000, timestamp.date_time().nanosecond());
    }
}

#[test]
fn timestamp_drops_fraction_without_dropping_leap_second() {
    let date_time = NaiveDate::from_ymd_opt(2016, 12, 31)
        .unwrap()
        .and_hms_nano_opt(23, 59, 59, 1_500_000_000)
        .unwrap();
    let timestamp = Timestamp::new(date_time, Some(Zone::Utc)).unwrap();

    assert_eq!("20161231T235960Z", timestamp.to_string());
    assert_eq!(1_000_000_000, timestamp.date_time().nanosecond());
}

#[test]
fn offsets_with_seconds_preserve_the_instant() {
    for seconds in [30, -30] {
        let original =
            FixedOffset::east_opt(seconds).unwrap().with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let timestamp = Timestamp::from(original);

        assert_eq!(Some(Zone::Utc), timestamp.zone());
        assert_eq!(
            original.with_timezone(&Utc),
            timestamp.to_fixed_offset().unwrap().with_timezone(&Utc)
        );
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
        let date = Date::try_from_year(year).unwrap();
        assert_eq!(date, date.to_string().parse::<Date>().unwrap());

        let date = Date::from_year_month_day(year, 1, 1).unwrap();
        assert_eq!(date, date.to_string().parse::<Date>().unwrap());

        let date = Date::from_year_month(year, 1).unwrap();
        assert_eq!(date, date.to_string().parse::<Date>().unwrap());
    }

    assert!(Date::try_from_year(10000).is_err());
    assert!(Date::from_year_month_day(10000, 1, 1).is_err());
    assert!(Date::from_year_month(10000, 1).is_err());
}
