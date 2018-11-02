use super::*;

use std::fmt::Display;

use chrono::prelude::*;
use validators::{Validated, ValidatedWrapper};

fn is_leap(year: u16) -> bool {
    (year % 4 == 0) && (year % 100 != 0) || year % 400 == 0
}

#[derive(Clone, Debug, PartialEq)]
pub enum Date {
    YearMonthDay(u16, u8, u8),
    YearMonth(u16, u8),
    Year(u16),
    MonthDay(u8, u8),
    Day(u8),
}

#[derive(Clone, Debug)]
pub enum DateRangeError {
    Year,
    Month,
    Day,
}

impl Date {
    pub fn with_year_month_day(year: u16, month: u8, day: u8) -> Result<Date, DateRangeError> {
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
            } else {
                if day > 30 {
                    return Err(DateRangeError::Day);
                }
            }
        } else if month <= 12 {
            if day % 2 == 1 {
                if day > 30 {
                    return Err(DateRangeError::Day);
                }
            } else {
                if day > 31 {
                    return Err(DateRangeError::Day);
                }
            }
        } else {
            return Err(DateRangeError::Month);
        }

        Ok(Date::YearMonthDay(year, month, day))
    }

    pub fn with_year_month(year: u16, month: u8) -> Result<Date, DateRangeError> {
        if year > 9999 {
            return Err(DateRangeError::Year);
        }

        if month == 0 || month > 12 {
            return Err(DateRangeError::Month);
        }

        Ok(Date::YearMonth(year, month))
    }

    pub fn with_year(year: u16) -> Result<Date, DateRangeError> {
        if year > 9999 {
            return Err(DateRangeError::Year);
        }

        Ok(Date::Year(year))
    }

    pub fn with_month_day(month: u8, day: u8) -> Result<Date, DateRangeError> {
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
            } else {
                if day > 30 {
                    return Err(DateRangeError::Day);
                }
            }
        } else if month <= 12 {
            if day % 2 == 1 {
                if day > 30 {
                    return Err(DateRangeError::Day);
                }
            } else {
                if day > 31 {
                    return Err(DateRangeError::Day);
                }
            }
        } else {
            return Err(DateRangeError::Month);
        }

        Ok(Date::MonthDay(month, day))
    }

    pub fn with_day(day: u8) -> Result<Date, DateRangeError> {
        if day == 0 || day > 31 {
            return Err(DateRangeError::Day);
        }

        Ok(Date::Day(day))
    }
}

impl Date {
    pub fn get_year(&self) -> Option<u16> {
        if let Date::YearMonthDay(year, _, _) = self {
            Some(*year)
        } else if let Date::YearMonth(year, _) = self {
            Some(*year)
        } else if let Date::Year(year) = self {
            Some(*year)
        } else {
            None
        }
    }

    pub fn get_month(&self) -> Option<u8> {
        if let Date::YearMonthDay(_, month, _) = self {
            Some(*month)
        } else if let Date::YearMonth(_, month) = self {
            Some(*month)
        } else if let Date::MonthDay(month, _) = self {
            Some(*month)
        } else {
            None
        }
    }

    pub fn get_day(&self) -> Option<u8> {
        if let Date::YearMonthDay(_, _, day) = self {
            Some(*day)
        } else if let Date::MonthDay(_, day) = self {
            Some(*day)
        } else if let Date::Day(day) = self {
            Some(*day)
        } else {
            None
        }
    }
}

impl Value for Date {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Date::YearMonthDay(year, month, day) => {
                f.write_fmt(format_args!("{:04}", year))?;
                f.write_fmt(format_args!("{:02}", month))?;
                f.write_fmt(format_args!("{:02}", day))?;
            }
            Date::YearMonth(year, month) => {
                f.write_fmt(format_args!("{:04}", year))?;
                f.write_str("-")?;
                f.write_fmt(format_args!("{:02}", month))?;
            }
            Date::Year(year) => {
                f.write_fmt(format_args!("{:04}", year))?;
            }
            Date::MonthDay(month, day) => {
                f.write_str("--")?;
                f.write_fmt(format_args!("{:02}", month))?;
                f.write_fmt(format_args!("{:02}", day))?;
            }
            Date::Day(day) => {
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

impl Value for List<Date> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<Date> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Time {
    HourMinuteSecond(u8, u8, u8),
    HourMinute(u8, u8),
    Hour(u8),
    MinuteSecond(u8, u8),
    Second(u8),
    HourMinuteSecondUtc(u8, u8, u8),
    HourMinuteSecondZone(u8, u8, u8, i16),
}

#[derive(Clone, Debug)]
pub enum TimeRangeError {
    Hour,
    Minute,
    Second,
    Zone,
}

impl Time {
    pub fn with_hour_minute_second(
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

        Ok(Time::HourMinuteSecond(hour, minute, second))
    }

    pub fn with_hour_minute(hour: u8, minute: u8) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }

        Ok(Time::HourMinute(hour, minute))
    }

    pub fn with_hour(hour: u8) -> Result<Time, TimeRangeError> {
        if hour >= 24 {
            return Err(TimeRangeError::Hour);
        }

        Ok(Time::Hour(hour))
    }

    pub fn with_minute_second(minute: u8, second: u8) -> Result<Time, TimeRangeError> {
        if minute >= 60 {
            return Err(TimeRangeError::Minute);
        }
        if second >= 60 {
            return Err(TimeRangeError::Second);
        }

        Ok(Time::MinuteSecond(minute, second))
    }

    pub fn with_second(second: u8) -> Result<Time, TimeRangeError> {
        if second >= 60 {
            return Err(TimeRangeError::Minute);
        }

        Ok(Time::Second(second))
    }

    pub fn with_hour_minute_second_utc(
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

        Ok(Time::HourMinuteSecondUtc(hour, minute, second))
    }

    pub fn with_hour_minute_second_zone(
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

        Ok(Time::HourMinuteSecondZone(
            hour,
            minute,
            second,
            offset_minutes,
        ))
    }
}

impl Time {
    pub fn get_hour(&self) -> Option<u8> {
        if let Time::HourMinuteSecond(hour, _, _) = self {
            Some(*hour)
        } else if let Time::HourMinute(hour, _) = self {
            Some(*hour)
        } else if let Time::Hour(hour) = self {
            Some(*hour)
        } else if let Time::HourMinuteSecondUtc(hour, _, _) = self {
            Some(*hour)
        } else if let Time::HourMinuteSecondZone(hour, _, _, _) = self {
            Some(*hour)
        } else {
            None
        }
    }

    pub fn get_minute(&self) -> Option<u8> {
        if let Time::HourMinuteSecond(_, minute, _) = self {
            Some(*minute)
        } else if let Time::HourMinute(_, minute) = self {
            Some(*minute)
        } else if let Time::MinuteSecond(minute, _) = self {
            Some(*minute)
        } else if let Time::HourMinuteSecondUtc(_, minute, _) = self {
            Some(*minute)
        } else if let Time::HourMinuteSecondZone(_, minute, _, _) = self {
            Some(*minute)
        } else {
            None
        }
    }

    pub fn get_second(&self) -> Option<u8> {
        if let Time::HourMinuteSecond(_, _, second) = self {
            Some(*second)
        } else if let Time::MinuteSecond(_, second) = self {
            Some(*second)
        } else if let Time::Second(second) = self {
            Some(*second)
        } else if let Time::HourMinuteSecondUtc(_, _, second) = self {
            Some(*second)
        } else if let Time::HourMinuteSecondZone(_, _, second, _) = self {
            Some(*second)
        } else {
            None
        }
    }

    pub fn get_time_zone_offset(&self) -> Option<i16> {
        if let Time::HourMinuteSecondUtc(..) = self {
            Some(0)
        } else if let Time::HourMinuteSecondZone(_, _, _, offset_minutes) = self {
            Some(*offset_minutes)
        } else {
            None
        }
    }
}

impl Value for Time {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Time::HourMinuteSecond(hour, minute, second) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;
            }
            Time::HourMinute(hour, minute) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
            }
            Time::Hour(hour) => {
                f.write_fmt(format_args!("{:02}", hour))?;
            }
            Time::MinuteSecond(minute, second) => {
                f.write_str("-")?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;
            }
            Time::Second(second) => {
                f.write_str("--")?;
                f.write_fmt(format_args!("{:02}", second))?;
            }
            Time::HourMinuteSecondUtc(hour, minute, second) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;
                f.write_str("Z")?;
            }
            Time::HourMinuteSecondZone(hour, minute, second, mut offset_minutes) => {
                f.write_fmt(format_args!("{:02}", hour))?;
                f.write_fmt(format_args!("{:02}", minute))?;
                f.write_fmt(format_args!("{:02}", second))?;

                if offset_minutes >= 0 {
                    f.write_str("+")?;
                } else {
                    f.write_str("-")?;
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

impl Value for List<Time> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<Time> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub fn with_date_time(date: Date, time: Time) -> DateTime {
        DateTime { date, time }
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
        f.write_str("T")?;
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

impl Value for List<DateTime> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<DateTime> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}

validated_customized_ranged_number!(pub UtcOffset, i16, -1439, 1439);

impl Value for UtcOffset {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut offset_minutes = self.get_number();

        if offset_minutes >= 0 {
            f.write_str("+")?;
        } else {
            f.write_str("-")?;
            offset_minutes = -offset_minutes;
        }

        f.write_fmt(format_args!("{:02}", offset_minutes / 60))?;
        f.write_fmt(format_args!("{:02}", offset_minutes % 60))?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DateAndOrTime {
    Date(Date),
    Time(Time),
    DateTime(DateTime),
}

impl DateAndOrTime {
    pub fn with_date(date: Date) -> DateAndOrTime {
        DateAndOrTime::Date(date)
    }

    pub fn with_time(time: Time) -> DateAndOrTime {
        DateAndOrTime::Time(time)
    }

    pub fn with_date_time(date: Date, time: Time) -> DateAndOrTime {
        DateAndOrTime::DateTime(DateTime::with_date_time(date, time))
    }
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

impl Value for List<DateAndOrTime> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<DateAndOrTime> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    offset_minutes: Option<i16>,
}

#[derive(Clone, Debug)]
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
            } else {
                if day > 30 {
                    return Err(TimestampRangeError::Date(DateRangeError::Day));
                }
            }
        } else if month <= 12 {
            if day % 2 == 1 {
                if day > 30 {
                    return Err(TimestampRangeError::Date(DateRangeError::Day));
                }
            } else {
                if day > 31 {
                    return Err(TimestampRangeError::Date(DateRangeError::Day));
                }
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

    pub fn with_date_time<T: chrono::TimeZone>(
        date_time: chrono::DateTime<T>,
    ) -> Result<Timestamp, TimestampRangeError> {
        let year = date_time.year();

        if year < 0 || year > 9999 {
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
}

impl Value for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{:04}", self.year))?;
        f.write_fmt(format_args!("{:02}", self.month))?;
        f.write_fmt(format_args!("{:02}", self.day))?;

        f.write_str("T")?;

        f.write_fmt(format_args!("{:02}", self.hour))?;
        f.write_fmt(format_args!("{:02}", self.minute))?;
        f.write_fmt(format_args!("{:02}", self.second))?;

        if let Some(mut offset_minutes) = self.offset_minutes {
            if offset_minutes > 0 {
                f.write_str("+")?;
            } else if offset_minutes < 0 {
                f.write_str("-")?;
                offset_minutes = -offset_minutes;
            } else {
                f.write_str("Z")?;
                return Ok(());
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

impl Value for List<Timestamp> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v: &Vec<Timestamp> = self.as_vec();

        Value::fmt(&v[0], f)?;

        for e in v.iter().skip(1) {
            f.write_str(",")?;
            Value::fmt(e, f)?;
        }

        Ok(())
    }
}
