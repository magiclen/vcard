use super::*;

use std::cmp::Ordering;
use std::fmt::{Display, Write};

use chrono::prelude::*;
use validators::{Validated, ValidatedWrapper};

fn is_leap(year: u16) -> bool {
    (year % 4 == 0) && (year % 100 != 0) || year % 400 == 0
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum DateInner {
    YearMonthDay(u16, u8, u8),
    YearMonth(u16, u8),
    Year(u16),
    MonthDay(u8, u8),
    Day(u8),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Date {
    inner: DateInner,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DateRangeError {
    Year,
    Month,
    Day,
}

impl Date {
    pub fn from_year_month_day(year: u16, month: u8, day: u8) -> Result<Date, DateRangeError> {
        if year > 9999 {
            return Err(DateRangeError::Year);
        }

        if month == 1 {
            if day > 31 {
                return Err(DateRangeError::Day);
            }
        } else if month == 2 {
            if is_leap(year) {
                if day > 29 {
                    return Err(DateRangeError::Day);
                }
            } else if day > 28 {
                return Err(DateRangeError::Day);
            }
        } else if month <= 7 {
            if day % 2 == 1 {
                if day > 31 {
                    return Err(DateRangeError::Day);
                }
            } else if day > 30 {
                return Err(DateRangeError::Day);
            }
        } else if month <= 12 {
            if day % 2 == 1 {
                if day > 30 {
                    return Err(DateRangeError::Day);
                }
            } else if day > 31 {
                return Err(DateRangeError::Day);
            }
        } else {
            return Err(DateRangeError::Month);
        }

        Ok(Date {
            inner: DateInner::YearMonthDay(year, month, day),
        })
    }

    pub fn from_year_month(year: u16, month: u8) -> Result<Date, DateRangeError> {
        if year > 9999 {
            return Err(DateRangeError::Year);
        }

        if month == 0 || month > 12 {
            return Err(DateRangeError::Month);
        }

        Ok(Date {
            inner: DateInner::YearMonth(year, month),
        })
    }

    pub fn from_year(year: u16) -> Result<Date, DateRangeError> {
        if year > 9999 {
            return Err(DateRangeError::Year);
        }

        Ok(Date {
            inner: DateInner::Year(year),
        })
    }

    pub fn from_month_day(month: u8, day: u8) -> Result<Date, DateRangeError> {
        if month == 1 {
            if day > 31 {
                return Err(DateRangeError::Day);
            }
        } else if month == 2 {
            if day > 29 {
                return Err(DateRangeError::Day);
            }
        } else if month <= 7 {
            if day % 2 == 1 {
                if day > 31 {
                    return Err(DateRangeError::Day);
                }
            } else if day > 30 {
                return Err(DateRangeError::Day);
            }
        } else if month <= 12 {
            if day % 2 == 1 {
                if day > 30 {
                    return Err(DateRangeError::Day);
                }
            } else if day > 31 {
                return Err(DateRangeError::Day);
            }
        } else {
            return Err(DateRangeError::Month);
        }

        Ok(Date {
            inner: DateInner::MonthDay(month, day),
        })
    }

    pub fn from_day(day: u8) -> Result<Date, DateRangeError> {
        if day == 0 || day > 31 {
            return Err(DateRangeError::Day);
        }

        Ok(Date {
            inner: DateInner::Day(day),
        })
    }

    pub fn from_date_time<T: chrono::TimeZone>(
        date_time: chrono::DateTime<T>,
    ) -> Result<Date, DateRangeError> {
        let year = date_time.year();

        if !(0..=9999).contains(&year) {
            return Err(DateRangeError::Year);
        }

        let year = year as u16;

        let month = date_time.month() as u8;

        let day = date_time.day() as u8;

        Ok(Date {
            inner: DateInner::YearMonthDay(year, month, day),
        })
    }
}

impl Date {
    pub fn get_year(&self) -> Option<u16> {
        match self.inner {
            DateInner::YearMonthDay(year, _, _) => Some(year),
            DateInner::YearMonth(year, _) => Some(year),
            DateInner::Year(year) => Some(year),
            _ => None,
        }
    }

    pub fn get_month(&self) -> Option<u8> {
        match self.inner {
            DateInner::YearMonthDay(_, month, _) => Some(month),
            DateInner::YearMonth(_, month) => Some(month),
            DateInner::MonthDay(month, _) => Some(month),
            _ => None,
        }
    }

    pub fn get_day(&self) -> Option<u8> {
        match self.inner {
            DateInner::YearMonthDay(_, _, day) => Some(day),
            DateInner::MonthDay(_, day) => Some(day),
            DateInner::Day(day) => Some(day),
            _ => None,
        }
    }
}

impl Value for Date {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.inner {
            DateInner::YearMonthDay(year, month, day) => {
                f.write_fmt(format_args!("{:04}", year))?;
                f.write_fmt(format_args!("{:02}", month))?;
                f.write_fmt(format_args!("{:02}", day))?;
            }
            DateInner::YearMonth(year, month) => {
                f.write_fmt(format_args!("{:04}", year))?;
                f.write_char('-')?;
                f.write_fmt(format_args!("{:02}", month))?;
            }
            DateInner::Year(year) => {
                f.write_fmt(format_args!("{:04}", year))?;
            }
            DateInner::MonthDay(month, day) => {
                f.write_str("--")?;
                f.write_fmt(format_args!("{:02}", month))?;
                f.write_fmt(format_args!("{:02}", day))?;
            }
            DateInner::Day(day) => {
                f.write_str("---")?;
                f.write_fmt(format_args!("{:02}", day))?;
            }
        }

        Ok(())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for Date {}

impl ValidatedWrapper for Date {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum TimeInner {
    HourMinuteSecond(u8, u8, u8),
    HourMinute(u8, u8),
    Hour(u8),
    MinuteSecond(u8, u8),
    Second(u8),
    HourMinuteSecondUtc(u8, u8, u8),
    HourMinuteSecondZone(u8, u8, u8, i16),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Time {
    inner: TimeInner,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeRangeError {
    Hour,
    Minute,
    Second,
    Zone,
}

impl Time {
    pub fn from_hour_minute_second(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }
        if second >= 60 {
            return Err(TimeRangeError::Second);
        }

        Ok(Time {
            inner: TimeInner::HourMinuteSecond(hour, minute, second),
        })
    }

    pub fn from_hour_minute(hour: u8, minute: u8) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }

        Ok(Time {
            inner: TimeInner::HourMinute(hour, minute),
        })
    }

    pub fn from_hour(hour: u8) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }

        Ok(Time {
            inner: TimeInner::Hour(hour),
        })
    }

    pub fn from_minute_second(minute: u8, second: u8) -> Result<Time, TimeRangeError> {
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }
        if second >= 60 {
            return Err(TimeRangeError::Second);
        }

        Ok(Time {
            inner: TimeInner::MinuteSecond(minute, second),
        })
    }

    pub fn from_second(second: u8) -> Result<Time, TimeRangeError> {
        if second >= 60 {
            return Err(TimeRangeError::Minute);
        }

        Ok(Time {
            inner: TimeInner::Second(second),
        })
    }

    pub fn from_hour_minute_second_utc(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }
        if second >= 60 {
            return Err(TimeRangeError::Second);
        }

        Ok(Time {
            inner: TimeInner::HourMinuteSecondUtc(hour, minute, second),
        })
    }

    pub fn from_hour_minute_second_zone(
        hour: u8,
        minute: u8,
        second: u8,
        offset_minutes: i16,
    ) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }
        if second >= 60 {
            return Err(TimeRangeError::Second);
        }
        if offset_minutes >= 24 * 60 || offset_minutes <= -24 * 60 {
            return Err(TimeRangeError::Zone);
        }

        Ok(Time {
            inner: TimeInner::HourMinuteSecondZone(hour, minute, second, offset_minutes),
        })
    }

    pub fn from_date_time<T: chrono::TimeZone>(date_time: chrono::DateTime<T>) -> Time {
        let hour = date_time.hour() as u8;

        let minute = date_time.minute() as u8;

        let second = date_time.second() as u8;

        let offset_minutes =
            ((date_time.naive_local().timestamp() - date_time.naive_utc().timestamp()) / 60) as i16;

        if offset_minutes == 0 {
            Time {
                inner: TimeInner::HourMinuteSecondUtc(hour, minute, second),
            }
        } else {
            Time {
                inner: TimeInner::HourMinuteSecondZone(hour, minute, second, offset_minutes),
            }
        }
    }
}

impl Time {
    pub fn get_hour(&self) -> Option<u8> {
        match self.inner {
            TimeInner::HourMinuteSecond(hour, _, _) => Some(hour),
            TimeInner::HourMinute(hour, _) => Some(hour),
            TimeInner::Hour(hour) => Some(hour),
            TimeInner::HourMinuteSecondUtc(hour, _, _) => Some(hour),
            TimeInner::HourMinuteSecondZone(hour, _, _, _) => Some(hour),
            _ => None,
        }
    }

    pub fn get_minute(&self) -> Option<u8> {
        match self.inner {
            TimeInner::HourMinuteSecond(_, minute, _) => Some(minute),
            TimeInner::HourMinute(_, minute) => Some(minute),
            TimeInner::MinuteSecond(minute, _) => Some(minute),
            TimeInner::HourMinuteSecondUtc(_, minute, _) => Some(minute),
            TimeInner::HourMinuteSecondZone(_, minute, _, _) => Some(minute),
            _ => None,
        }
    }

    pub fn get_second(&self) -> Option<u8> {
        match self.inner {
            TimeInner::HourMinuteSecond(_, _, second) => Some(second),
            TimeInner::MinuteSecond(_, second) => Some(second),
            TimeInner::Second(second) => Some(second),
            TimeInner::HourMinuteSecondUtc(_, _, second) => Some(second),
            TimeInner::HourMinuteSecondZone(_, _, second, _) => Some(second),
            _ => None,
        }
    }

    pub fn get_time_zone_offset(&self) -> Option<i16> {
        match self.inner {
            TimeInner::HourMinuteSecondUtc(..) => Some(0),
            TimeInner::HourMinuteSecondZone(_, _, _, offset_minutes) => Some(offset_minutes),
            _ => None,
        }
    }
}

impl Value for Time {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.inner {
            TimeInner::HourMinuteSecond(hour, minute, second) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;
            }
            TimeInner::HourMinute(hour, minute) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
            }
            TimeInner::Hour(hour) => {
                f.write_fmt(format_args!("{:02}", hour))?;
            }
            TimeInner::MinuteSecond(minute, second) => {
                f.write_char('-')?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;
            }
            TimeInner::Second(second) => {
                f.write_str("--")?;
                f.write_fmt(format_args!("{:02}", second))?;
            }
            TimeInner::HourMinuteSecondUtc(hour, minute, second) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;
                f.write_char('Z')?;
            }
            TimeInner::HourMinuteSecondZone(hour, minute, second, mut offset_minutes) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;

                if offset_minutes >= 0 {
                    f.write_char('+')?;
                } else {
                    f.write_char('-')?;
                    offset_minutes = -offset_minutes;
                }

                f.write_fmt(format_args!("{:02}", offset_minutes / 60))?;
                f.write_fmt(format_args!("{:02}", offset_minutes % 60))?;
            }
        }

        Ok(())
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for Time {}

impl ValidatedWrapper for Time {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub fn from_date_and_time(date: Date, time: Time) -> DateTime {
        DateTime {
            date,
            time,
        }
    }

    pub fn from_date_time<T: chrono::TimeZone>(
        date_time: chrono::DateTime<T>,
    ) -> Result<DateTime, DateRangeError> {
        let date = Date::from_date_time(date_time.clone())?;
        let time = Time::from_date_time(date_time);

        Ok(Self::from_date_and_time(date, time))
    }
}

impl DateTime {
    pub fn get_date(&self) -> &Date {
        &self.date
    }

    pub fn get_time(&self) -> &Time {
        &self.time
    }
}

impl Value for DateTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(&self.date, f)?;
        f.write_char('T')?;
        Value::fmt(&self.time, f)?;

        Ok(())
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for DateTime {}

impl ValidatedWrapper for DateTime {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

validated_customized_ranged_number!(pub UtcOffset, i16, -1439, 1439);

impl Value for UtcOffset {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut offset_minutes = self.get_number();

        if offset_minutes >= 0 {
            f.write_char('+')?;
        } else {
            f.write_char('-')?;
            offset_minutes = -offset_minutes;
        }

        f.write_fmt(format_args!("{:02}", offset_minutes / 60))?;
        f.write_fmt(format_args!("{:02}", offset_minutes % 60))?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DateAndOrTime {
    Date(Date),
    Time(Time),
    DateTime(DateTime),
}

impl DateAndOrTime {
    pub fn get_date(&self) -> Option<&Date> {
        if let DateAndOrTime::DateTime(dt) = self {
            Some(&dt.date)
        } else if let DateAndOrTime::Date(d) = self {
            Some(&d)
        } else {
            None
        }
    }

    pub fn get_time(&self) -> Option<&Time> {
        if let DateAndOrTime::DateTime(dt) = self {
            Some(&dt.time)
        } else if let DateAndOrTime::Time(t) = self {
            Some(&t)
        } else {
            None
        }
    }

    pub fn get_date_time(&self) -> Option<&DateTime> {
        if let DateAndOrTime::DateTime(dt) = self {
            Some(&dt)
        } else {
            None
        }
    }
}

impl Value for DateAndOrTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            DateAndOrTime::Date(d) => {
                Value::fmt(d, f)?;
            }
            DateAndOrTime::Time(t) => {
                Value::fmt(t, f)?;
            }
            DateAndOrTime::DateTime(dt) => {
                Value::fmt(dt, f)?;
            }
        }

        Ok(())
    }
}

impl Display for DateAndOrTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for DateAndOrTime {}

impl ValidatedWrapper for DateAndOrTime {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DateOrDateTime {
    Date(Date),
    DateTime(DateTime),
}

impl DateOrDateTime {
    pub fn get_date(&self) -> &Date {
        match self {
            DateOrDateTime::Date(d) => &d,
            DateOrDateTime::DateTime(dt) => dt.get_date(),
        }
    }

    pub fn get_time(&self) -> Option<&Time> {
        if let DateOrDateTime::DateTime(dt) = self {
            Some(&dt.time)
        } else {
            None
        }
    }

    pub fn get_date_time(&self) -> Option<&DateTime> {
        if let DateOrDateTime::DateTime(dt) = self {
            Some(&dt)
        } else {
            None
        }
    }
}

impl Value for DateOrDateTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            DateOrDateTime::Date(d) => {
                Value::fmt(d, f)?;
            }
            DateOrDateTime::DateTime(dt) => {
                Value::fmt(dt, f)?;
            }
        }

        Ok(())
    }
}

impl Display for DateOrDateTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for DateOrDateTime {}

impl ValidatedWrapper for DateOrDateTime {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    offset_minutes: Option<i16>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimestampRangeError {
    Date(DateRangeError),
    Time(TimeRangeError),
}

impl Timestamp {
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset_minutes: Option<i16>,
    ) -> Result<Timestamp, TimestampRangeError> {
        if year > 9999 {
            return Err(TimestampRangeError::Date(DateRangeError::Year));
        }

        if month == 1 {
            if day > 31 {
                return Err(TimestampRangeError::Date(DateRangeError::Day));
            }
        } else if month == 2 {
            if is_leap(year) {
                if day > 29 {
                    return Err(TimestampRangeError::Date(DateRangeError::Day));
                }
            } else if day > 28 {
                return Err(TimestampRangeError::Date(DateRangeError::Day));
            }
        } else if month <= 7 {
            if day % 2 == 1 {
                if day > 31 {
                    return Err(TimestampRangeError::Date(DateRangeError::Day));
                }
            } else if day > 30 {
                return Err(TimestampRangeError::Date(DateRangeError::Day));
            }
        } else if month <= 12 {
            if day % 2 == 1 {
                if day > 30 {
                    return Err(TimestampRangeError::Date(DateRangeError::Day));
                }
            } else if day > 31 {
                return Err(TimestampRangeError::Date(DateRangeError::Day));
            }
        } else {
            return Err(TimestampRangeError::Date(DateRangeError::Month));
        }

        if hour >= 24 {
            return Err(TimestampRangeError::Time(TimeRangeError::Hour));
        }
        if minute >= 60 {
            return Err(TimestampRangeError::Time(TimeRangeError::Minute));
        }
        if second >= 60 {
            return Err(TimestampRangeError::Time(TimeRangeError::Second));
        }

        if let Some(offset_minutes) = offset_minutes {
            if offset_minutes >= 24 * 60 || offset_minutes <= -24 * 60 {
                return Err(TimestampRangeError::Time(TimeRangeError::Zone));
            }
        }

        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset_minutes,
        })
    }

    pub fn from_date_time<T: chrono::TimeZone>(
        date_time: chrono::DateTime<T>,
    ) -> Result<Timestamp, TimestampRangeError> {
        let year = date_time.year();

        if !(0..=9999).contains(&year) {
            return Err(TimestampRangeError::Date(DateRangeError::Year));
        }

        let year = year as u16;

        let month = date_time.month() as u8;

        let day = date_time.day() as u8;

        let hour = date_time.hour() as u8;

        let minute = date_time.minute() as u8;

        let second = date_time.second() as u8;

        let offset_minutes =
            ((date_time.naive_local().timestamp() - date_time.naive_utc().timestamp()) / 60) as i16;

        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset_minutes: Some(offset_minutes),
        })
    }

    pub fn now() -> Timestamp {
        Self::from_date_time(Utc::now()).unwrap()
    }
}

impl Value for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{:04}", self.year))?;
        f.write_fmt(format_args!("{:02}", self.month))?;
        f.write_fmt(format_args!("{:02}", self.day))?;

        f.write_char('T')?;

        f.write_fmt(format_args!("{:02}", self.hour))?;
        f.write_fmt(format_args!("{:02}", self.minute))?;
        f.write_fmt(format_args!("{:02}", self.second))?;

        if let Some(mut offset_minutes) = self.offset_minutes {
            match offset_minutes.cmp(&0) {
                Ordering::Greater => f.write_char('+')?,
                Ordering::Less => {
                    f.write_char('-')?;
                    offset_minutes = -offset_minutes;
                }
                Ordering::Equal => {
                    f.write_char('Z')?;
                    return Ok(());
                }
            }

            let m = offset_minutes / 60;
            let s = offset_minutes % 60;

            f.write_fmt(format_args!("{:02}", m))?;

            if s != 0 {
                f.write_fmt(format_args!("{:02}", s))?;
            }
        }

        Ok(())
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Value::fmt(self, f)
    }
}

impl Validated for Timestamp {}

impl ValidatedWrapper for Timestamp {
    type Error = &'static str;

    fn from_string(_from_string_input: String) -> Result<Self, Self::Error> {
        unimplemented!();
    }

    fn from_str(_from_str_input: &str) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
