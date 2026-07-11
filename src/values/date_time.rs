//! Date and time value types defined in RFC 6350 section 4.3.
//!
//! vCard allows truncated date and time forms like `--0412` which cannot be represented by chrono types, so this module defines its own types for them.

use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime, TimeZone as _, Timelike, Utc};

use crate::error::InvalidValueError;

/// Parses a fixed amount of ASCII digits into a number.
fn parse_digits(s: &[u8]) -> Option<u16> {
    let mut n = 0u16;

    for b in s {
        if !b.is_ascii_digit() {
            return None;
        }

        n = n * 10 + u16::from(b - b'0');
    }

    Some(n)
}

/// A UTC offset value, e.g. `+0800` or `-0530`.
///
/// This is used by the `utc-offset` value type and by time zone information inside time values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtcOffset {
    negative: bool,
    hour:     u8,
    minute:   u8,
}

impl UtcOffset {
    /// Creates a UTC offset from a sign, an hour within `0..=23` and a minute within `0..=59`.
    pub const fn new(negative: bool, hour: u8, minute: u8) -> Result<Self, InvalidValueError> {
        if hour > 23 || minute > 59 {
            return Err(InvalidValueError::new("utc-offset"));
        }

        Ok(Self {
            negative,
            hour,
            minute,
        })
    }

    /// Returns whether this offset is on the west side of UTC.
    #[inline]
    pub const fn negative(&self) -> bool {
        self.negative
    }

    /// Returns the hour part of this offset.
    #[inline]
    pub const fn hour(&self) -> u8 {
        self.hour
    }

    /// Returns the minute part of this offset.
    #[inline]
    pub const fn minute(&self) -> u8 {
        self.minute
    }

    /// Converts this offset into a chrono `FixedOffset`.
    pub fn to_fixed_offset(&self) -> FixedOffset {
        let seconds = i32::from(self.hour) * 3600 + i32::from(self.minute) * 60;

        let seconds = if self.negative { -seconds } else { seconds };

        // The seconds value is always within one day, so this never fails.
        FixedOffset::east_opt(seconds).unwrap()
    }
}

impl From<FixedOffset> for UtcOffset {
    /// Converts a chrono `FixedOffset` into a `UtcOffset`, truncating the sub-minute part.
    fn from(offset: FixedOffset) -> Self {
        let seconds = offset.local_minus_utc();

        let negative = seconds < 0;

        let minutes = seconds.unsigned_abs() / 60;

        Self {
            negative,
            hour: (minutes / 60) as u8,
            minute: (minutes % 60) as u8,
        }
    }
}

impl Display for UtcOffset {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{:02}{:02}", if self.negative { '-' } else { '+' }, self.hour, self.minute)
    }
}

impl FromStr for UtcOffset {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR: InvalidValueError = InvalidValueError::new("utc-offset");

        let bytes = s.as_bytes();

        let negative = match bytes.first() {
            Some(b'+') => false,
            Some(b'-') => true,
            _ => return Err(ERROR),
        };

        let (hour, minute) = match bytes.len() {
            3 => (parse_digits(&bytes[1..3]).ok_or(ERROR)?, 0),
            5 => {
                (parse_digits(&bytes[1..3]).ok_or(ERROR)?, parse_digits(&bytes[3..5]).ok_or(ERROR)?)
            },
            _ => return Err(ERROR),
        };

        Self::new(negative, hour as u8, minute as u8)
    }
}

/// The time zone information carried by a time or a timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    /// The time is in UTC, written as `Z`.
    Utc,
    /// The time has a fixed offset from UTC.
    Offset(UtcOffset),
}

impl Zone {
    /// Converts this zone into a chrono `FixedOffset`.
    #[inline]
    pub fn to_fixed_offset(&self) -> FixedOffset {
        match self {
            Self::Utc => FixedOffset::east_opt(0).unwrap(),
            Self::Offset(offset) => offset.to_fixed_offset(),
        }
    }
}

impl Display for Zone {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Utc => f.write_str("Z"),
            Self::Offset(offset) => Display::fmt(offset, f),
        }
    }
}

impl FromStr for Zone {
    type Err = InvalidValueError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Z" || s == "z" { Ok(Self::Utc) } else { UtcOffset::from_str(s).map(Self::Offset) }
    }
}

/// A date value which allows the truncated and reduced forms of RFC 6350, e.g. `19850412`, `1985-04`, `1985`, `--0412` and `---12`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    year:  Option<u16>,
    month: Option<u8>,
    day:   Option<u8>,
}

/// Returns the biggest day number that the month can have in any year.
const fn max_day_in_month(month: u8) -> u8 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => 29,
        _ => 31,
    }
}

const DATE_ERROR: InvalidValueError = InvalidValueError::new("date");

impl Date {
    /// Creates a complete date, validated against the real calendar.
    pub fn from_year_month_day(year: u16, month: u8, day: u8) -> Result<Self, InvalidValueError> {
        NaiveDate::from_ymd_opt(i32::from(year), u32::from(month), u32::from(day))
            .ok_or(DATE_ERROR)?;

        Ok(Self {
            year: Some(year), month: Some(month), day: Some(day)
        })
    }

    /// Creates a date with only a year and a month, e.g. `1985-04`.
    pub const fn from_year_month(year: u16, month: u8) -> Result<Self, InvalidValueError> {
        if month < 1 || month > 12 {
            return Err(DATE_ERROR);
        }

        Ok(Self {
            year: Some(year), month: Some(month), day: None
        })
    }

    /// Creates a date with only a year, e.g. `1985`.
    pub const fn from_year(year: u16) -> Self {
        Self {
            year: Some(year), month: None, day: None
        }
    }

    /// Creates a date with only a month and a day, e.g. `--0412`.
    pub const fn from_month_day(month: u8, day: u8) -> Result<Self, InvalidValueError> {
        if month < 1 || month > 12 || day < 1 || day > max_day_in_month(month) {
            return Err(DATE_ERROR);
        }

        Ok(Self {
            year: None, month: Some(month), day: Some(day)
        })
    }

    /// Creates a date with only a month, e.g. `--04`.
    pub const fn from_month(month: u8) -> Result<Self, InvalidValueError> {
        if month < 1 || month > 12 {
            return Err(DATE_ERROR);
        }

        Ok(Self {
            year: None, month: Some(month), day: None
        })
    }

    /// Creates a date with only a day, e.g. `---12`.
    pub const fn from_day(day: u8) -> Result<Self, InvalidValueError> {
        if day < 1 || day > 31 {
            return Err(DATE_ERROR);
        }

        Ok(Self {
            year: None, month: None, day: Some(day)
        })
    }

    /// Returns the year part if it exists.
    #[inline]
    pub const fn year(&self) -> Option<u16> {
        self.year
    }

    /// Returns the month part if it exists.
    #[inline]
    pub const fn month(&self) -> Option<u8> {
        self.month
    }

    /// Returns the day part if it exists.
    #[inline]
    pub const fn day(&self) -> Option<u8> {
        self.day
    }
}

impl TryFrom<NaiveDate> for Date {
    type Error = InvalidValueError;

    /// Converts a chrono `NaiveDate` into a complete `Date`, failing when the year is out of the `0..=9999` range.
    #[inline]
    fn try_from(date: NaiveDate) -> Result<Self, Self::Error> {
        let year = u16::try_from(date.year()).map_err(|_| DATE_ERROR)?;

        if year > 9999 {
            return Err(DATE_ERROR);
        }

        Self::from_year_month_day(year, date.month() as u8, date.day() as u8)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (self.year, self.month, self.day) {
            (Some(y), Some(m), Some(d)) => write!(f, "{y:04}{m:02}{d:02}"),
            (Some(y), Some(m), None) => write!(f, "{y:04}-{m:02}"),
            (Some(y), None, None) => write!(f, "{y:04}"),
            (None, Some(m), Some(d)) => write!(f, "--{m:02}{d:02}"),
            (None, Some(m), None) => write!(f, "--{m:02}"),
            (None, None, Some(d)) => write!(f, "---{d:02}"),
            // The constructors never build other combinations.
            _ => unreachable!(),
        }
    }
}

impl FromStr for Date {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if let Some(rest) = s.strip_prefix("---") {
            let bytes = rest.as_bytes();

            if bytes.len() != 2 {
                return Err(DATE_ERROR);
            }

            return Self::from_day(parse_digits(bytes).ok_or(DATE_ERROR)? as u8);
        }

        if let Some(rest) = s.strip_prefix("--") {
            let bytes = rest.as_bytes();

            return match bytes.len() {
                2 => Self::from_month(parse_digits(bytes).ok_or(DATE_ERROR)? as u8),
                4 => Self::from_month_day(
                    parse_digits(&bytes[..2]).ok_or(DATE_ERROR)? as u8,
                    parse_digits(&bytes[2..]).ok_or(DATE_ERROR)? as u8,
                ),
                _ => Err(DATE_ERROR),
            };
        }

        match bytes.len() {
            4 => Ok(Self::from_year(parse_digits(bytes).ok_or(DATE_ERROR)?)),
            7 if bytes[4] == b'-' => Self::from_year_month(
                parse_digits(&bytes[..4]).ok_or(DATE_ERROR)?,
                parse_digits(&bytes[5..]).ok_or(DATE_ERROR)? as u8,
            ),
            8 => Self::from_year_month_day(
                parse_digits(&bytes[..4]).ok_or(DATE_ERROR)?,
                parse_digits(&bytes[4..6]).ok_or(DATE_ERROR)? as u8,
                parse_digits(&bytes[6..]).ok_or(DATE_ERROR)? as u8,
            ),
            _ => Err(DATE_ERROR),
        }
    }
}

const TIME_ERROR: InvalidValueError = InvalidValueError::new("time");

/// A time value which allows the truncated forms of RFC 6350, e.g. `102200`, `1022`, `10`, `-2200`, `-22` and `--00`, with an optional zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    hour:   Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    zone:   Option<Zone>,
}

impl Time {
    /// Creates a complete time.
    pub const fn from_hour_minute_second(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, InvalidValueError> {
        if hour > 23 || minute > 59 || second > 60 {
            return Err(TIME_ERROR);
        }

        Ok(Self {
            hour: Some(hour), minute: Some(minute), second: Some(second), zone: None
        })
    }

    /// Creates a time with only an hour and a minute, e.g. `1022`.
    pub const fn from_hour_minute(hour: u8, minute: u8) -> Result<Self, InvalidValueError> {
        if hour > 23 || minute > 59 {
            return Err(TIME_ERROR);
        }

        Ok(Self {
            hour: Some(hour), minute: Some(minute), second: None, zone: None
        })
    }

    /// Creates a time with only an hour, e.g. `10`.
    pub const fn from_hour(hour: u8) -> Result<Self, InvalidValueError> {
        if hour > 23 {
            return Err(TIME_ERROR);
        }

        Ok(Self {
            hour: Some(hour), minute: None, second: None, zone: None
        })
    }

    /// Creates a truncated time with only a minute and a second, e.g. `-2200`.
    pub const fn from_minute_second(minute: u8, second: u8) -> Result<Self, InvalidValueError> {
        if minute > 59 || second > 60 {
            return Err(TIME_ERROR);
        }

        Ok(Self {
            hour: None, minute: Some(minute), second: Some(second), zone: None
        })
    }

    /// Creates a truncated time with only a minute, e.g. `-22`.
    pub const fn from_minute(minute: u8) -> Result<Self, InvalidValueError> {
        if minute > 59 {
            return Err(TIME_ERROR);
        }

        Ok(Self {
            hour: None, minute: Some(minute), second: None, zone: None
        })
    }

    /// Creates a truncated time with only a second, e.g. `--00`.
    pub const fn from_second(second: u8) -> Result<Self, InvalidValueError> {
        if second > 60 {
            return Err(TIME_ERROR);
        }

        Ok(Self {
            hour: None, minute: None, second: Some(second), zone: None
        })
    }

    /// Attaches time zone information to this time.
    #[inline]
    pub const fn with_zone(mut self, zone: Zone) -> Self {
        self.zone = Some(zone);

        self
    }

    /// Returns the hour part if it exists.
    #[inline]
    pub const fn hour(&self) -> Option<u8> {
        self.hour
    }

    /// Returns the minute part if it exists.
    #[inline]
    pub const fn minute(&self) -> Option<u8> {
        self.minute
    }

    /// Returns the second part if it exists.
    #[inline]
    pub const fn second(&self) -> Option<u8> {
        self.second
    }

    /// Returns the time zone information if it exists.
    #[inline]
    pub const fn zone(&self) -> Option<Zone> {
        self.zone
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (self.hour, self.minute, self.second) {
            (Some(h), Some(m), Some(s)) => write!(f, "{h:02}{m:02}{s:02}")?,
            (Some(h), Some(m), None) => write!(f, "{h:02}{m:02}")?,
            (Some(h), None, None) => write!(f, "{h:02}")?,
            (None, Some(m), Some(s)) => write!(f, "-{m:02}{s:02}")?,
            (None, Some(m), None) => write!(f, "-{m:02}")?,
            (None, None, Some(s)) => write!(f, "--{s:02}")?,
            // The constructors never build other combinations.
            _ => unreachable!(),
        }

        if let Some(zone) = self.zone {
            Display::fmt(&zone, f)?;
        }

        Ok(())
    }
}

impl FromStr for Time {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Count the leading hyphens to know which time parts are truncated.
        let truncated = if s.starts_with("--") {
            2
        } else if s.starts_with('-') {
            1
        } else {
            0
        };

        let bytes = &s.as_bytes()[truncated..];

        // Consume up to (3 - truncated) pairs of digits.
        let mut numbers = [None::<u8>; 3];
        let mut count = 0;
        let mut position = 0;

        while count < 3 - truncated && bytes.len() >= position + 2 {
            match parse_digits(&bytes[position..position + 2]) {
                Some(n) => numbers[count] = Some(n as u8),
                None => break,
            }

            count += 1;
            position += 2;
        }

        if count == 0 {
            return Err(TIME_ERROR);
        }

        let mut time = match truncated {
            0 => match (numbers[0], numbers[1], numbers[2]) {
                (Some(h), Some(m), Some(s)) => Self::from_hour_minute_second(h, m, s)?,
                (Some(h), Some(m), None) => Self::from_hour_minute(h, m)?,
                (Some(h), None, None) => Self::from_hour(h)?,
                _ => unreachable!(),
            },
            1 => match (numbers[0], numbers[1]) {
                (Some(m), Some(s)) => Self::from_minute_second(m, s)?,
                (Some(m), None) => Self::from_minute(m)?,
                _ => unreachable!(),
            },
            _ => Self::from_second(numbers[0].unwrap())?,
        };

        let rest = &s[truncated + position..];

        if !rest.is_empty() {
            time = time.with_zone(Zone::from_str(rest).map_err(|_| TIME_ERROR)?);
        }

        Ok(time)
    }
}

const DATE_TIME_ERROR: InvalidValueError = InvalidValueError::new("date-time");

/// A date-time value as defined by RFC 6350, e.g. `19961022T140000` or `--1022T1400Z`.
///
/// The date part must contain a day and the time part must contain an hour, as the `date-time` rule requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    /// Combines a date and a time, where the date must not be reduced and the time must not be truncated.
    pub const fn new(date: Date, time: Time) -> Result<Self, InvalidValueError> {
        if date.day().is_none() || time.hour().is_none() {
            return Err(DATE_TIME_ERROR);
        }

        Ok(Self {
            date,
            time,
        })
    }

    /// Returns the date part.
    #[inline]
    pub const fn date(&self) -> Date {
        self.date
    }

    /// Returns the time part.
    #[inline]
    pub const fn time(&self) -> Time {
        self.time
    }
}

impl Display for DateTime {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl FromStr for DateTime {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, time) = s.split_once(['T', 't']).ok_or(DATE_TIME_ERROR)?;

        Self::new(
            Date::from_str(date).map_err(|_| DATE_TIME_ERROR)?,
            Time::from_str(time).map_err(|_| DATE_TIME_ERROR)?,
        )
    }
}

/// A value of the `date-and-or-time` type, which is a date-time, a date, or a standalone time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateAndOrTime {
    /// A date with a time, e.g. `19961022T140000`.
    DateTime(DateTime),
    /// A date only, e.g. `19961022` or `--1022`.
    Date(Date),
    /// A time only, written with a leading `T`, e.g. `T1400`.
    Time(Time),
}

impl From<DateTime> for DateAndOrTime {
    #[inline]
    fn from(value: DateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<Date> for DateAndOrTime {
    #[inline]
    fn from(value: Date) -> Self {
        Self::Date(value)
    }
}

impl From<Time> for DateAndOrTime {
    #[inline]
    fn from(value: Time) -> Self {
        Self::Time(value)
    }
}

impl Display for DateAndOrTime {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::DateTime(date_time) => Display::fmt(date_time, f),
            Self::Date(date) => Display::fmt(date, f),
            Self::Time(time) => write!(f, "T{time}"),
        }
    }
}

impl FromStr for DateAndOrTime {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A leading T means a standalone time, an inner T means a date-time, otherwise it is a date.
        if let Some(rest) = s.strip_prefix(['T', 't']) {
            Time::from_str(rest).map(Self::Time)
        } else if s.contains(['T', 't']) {
            DateTime::from_str(s).map(Self::DateTime)
        } else {
            Date::from_str(s).map(Self::Date)
        }
        .map_err(|_| InvalidValueError::new("date-and-or-time"))
    }
}

const TIMESTAMP_ERROR: InvalidValueError = InvalidValueError::new("timestamp");

/// A complete date and time value defined by RFC 6350 section 4.3.5, e.g. `20080124T195509Z`.
///
/// This is the value type of the REV and CREATED properties and of the CREATED parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Timestamp {
    date_time: NaiveDateTime,
    zone:      Option<Zone>,
}

impl Timestamp {
    /// Creates a timestamp from a chrono `NaiveDateTime` and optional time zone information.
    ///
    /// The year must be within `0..=9999` and the sub-second part is dropped.
    pub fn new(date_time: NaiveDateTime, zone: Option<Zone>) -> Result<Self, InvalidValueError> {
        if !(0..=9999).contains(&date_time.year()) {
            return Err(TIMESTAMP_ERROR);
        }

        Ok(Self {
            date_time: date_time.with_nanosecond(0).unwrap(),
            zone,
        })
    }

    /// Creates a timestamp for the current time in UTC.
    #[inline]
    pub fn now() -> Self {
        Self::new(Utc::now().naive_utc(), Some(Zone::Utc)).unwrap()
    }

    /// Returns the date and time part without time zone information.
    #[inline]
    pub const fn date_time(&self) -> NaiveDateTime {
        self.date_time
    }

    /// Returns the time zone information if it exists.
    #[inline]
    pub const fn zone(&self) -> Option<Zone> {
        self.zone
    }

    /// Converts this timestamp into a chrono `DateTime`, returning `None` when it has no time zone information.
    #[inline]
    pub fn to_fixed_offset(&self) -> Option<chrono::DateTime<FixedOffset>> {
        self.zone
            .and_then(|zone| zone.to_fixed_offset().from_local_datetime(&self.date_time).single())
    }
}

impl From<chrono::DateTime<Utc>> for Timestamp {
    #[inline]
    fn from(date_time: chrono::DateTime<Utc>) -> Self {
        Self::new(date_time.naive_utc(), Some(Zone::Utc)).unwrap()
    }
}

impl From<chrono::DateTime<FixedOffset>> for Timestamp {
    #[inline]
    fn from(date_time: chrono::DateTime<FixedOffset>) -> Self {
        Self::new(date_time.naive_local(), Some(Zone::Offset((*date_time.offset()).into())))
            .unwrap()
    }
}

impl Display for Timestamp {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.date_time.format("%Y%m%dT%H%M%S"))?;

        if let Some(zone) = self.zone {
            Display::fmt(&zone, f)?;
        }

        Ok(())
    }
}

impl FromStr for Timestamp {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() < 15 || bytes[8] != b'T' && bytes[8] != b't' {
            return Err(TIMESTAMP_ERROR);
        }

        let date = NaiveDate::from_ymd_opt(
            i32::from(parse_digits(&bytes[0..4]).ok_or(TIMESTAMP_ERROR)?),
            u32::from(parse_digits(&bytes[4..6]).ok_or(TIMESTAMP_ERROR)?),
            u32::from(parse_digits(&bytes[6..8]).ok_or(TIMESTAMP_ERROR)?),
        )
        .ok_or(TIMESTAMP_ERROR)?;

        let time = chrono::NaiveTime::from_hms_opt(
            u32::from(parse_digits(&bytes[9..11]).ok_or(TIMESTAMP_ERROR)?),
            u32::from(parse_digits(&bytes[11..13]).ok_or(TIMESTAMP_ERROR)?),
            u32::from(parse_digits(&bytes[13..15]).ok_or(TIMESTAMP_ERROR)?),
        )
        .ok_or(TIMESTAMP_ERROR)?;

        let zone = if s.len() > 15 {
            Some(Zone::from_str(&s[15..]).map_err(|_| TIMESTAMP_ERROR)?)
        } else {
            None
        };

        Self::new(NaiveDateTime::new(date, time), zone)
    }
}
