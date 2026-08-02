use std::str::FromStr;

use chrono::{DateTime, Local, TimeZone, Timelike};

use crate::error;
use crate::error::Error;

#[derive(Clone)]
pub struct Year(pub i32);

impl Year {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let parsed = s.parse::<i32>()?;
        if !(2025..=2100).contains(&parsed) {
            error!("The year is an unrealistic value!")
        }

        Ok(Year(parsed))
    }
}

#[derive(Clone)]
pub struct Month(pub u32);

impl Month {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let parsed = s.parse::<u32>()?;
        if !(1..=12).contains(&parsed) {
            error!("That month doesn't exist!")
        }

        Ok(Month(parsed))
    }
}

#[derive(Clone)]
pub struct Day(pub u32);

impl Day {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let parsed = s.parse::<u32>()?;
        if !(1..=31).contains(&parsed) {
            error!("That day doesn't exist!")
        }

        Ok(Day(parsed))
    }
}

#[derive(Clone)]
pub struct DateInput {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}

impl FromStr for DateInput {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 3 {
            error!("Invalid date format. Should be like 2025-01-12")
        }
        let year = Year::from_str(parts[0])?;
        let month = Month::from_str(parts[1])?;
        let day = Day::from_str(parts[2])?;
        Ok(DateInput { year, month, day })
    }
}

pub fn parse_dateinput(di: Option<DateInput>) -> Result<DateTime<Local>, Error> {
    match di {
        Some(date_input) => {
            let now = Local::now();
            let Year(year) = date_input.year;
            let Month(month) = date_input.month;
            let Day(day) = date_input.day;
            match Local.with_ymd_and_hms(year, month, day, now.hour(), now.minute(), now.second()) {
                chrono::offset::LocalResult::Single(v) => Ok(v),
                chrono::offset::LocalResult::Ambiguous(earliest, _) => Ok(earliest),
                chrono::offset::LocalResult::None => error!("Can't create date from input"),
            }
        }
        None => Ok(Local::now()),
    }
}

#[cfg(test)]
mod tests_dateinput {
    use chrono::Datelike;

    use super::*;

    #[test]
    fn test_year_valid() {
        let valid = vec!["2025", "2026", "02026"];
        for v in valid {
            let x = Year::from_str(v);
            assert!(x.is_ok())
        }
    }

    #[test]
    fn test_year_invalid() {
        let valid = vec!["1900", "26", "2123", "262626"];
        for v in valid {
            let x = Year::from_str(v);
            assert!(x.is_err())
        }
    }

    #[test]
    fn test_month_valid() {
        let valid = vec!["01", "1", "12", "012"];
        for v in valid {
            let x = Month::from_str(v);
            assert!(x.is_ok())
        }
    }

    #[test]
    fn test_month_invalid() {
        let valid = vec!["0", "00", "13", "013", "123"];
        for v in valid {
            let x = Month::from_str(v);
            assert!(x.is_err())
        }
    }

    #[test]
    fn test_day_valid() {
        let valid = vec!["01", "1", "30", "31", "031"];
        for v in valid {
            let x = Day::from_str(v);
            assert!(x.is_ok())
        }
    }

    #[test]
    fn test_day_invalid() {
        let valid = vec!["0", "00", "32", "032", "100"];
        for v in valid {
            let x = Day::from_str(v);
            assert!(x.is_err())
        }
    }

    #[test]
    fn test_valid() {
        let valid = vec!["2025-11-21", "2026-1-31", "2025-12-1"];
        for v in valid {
            let x = DateInput::from_str(v);
            assert!(x.is_ok())
        }
    }

    #[test]
    fn test_invalid() {
        let invalid = vec!["2025-0-1", "2026-1-32", "1900-12-1", "2026", "2026-01"];
        for v in invalid {
            let x = DateInput::from_str(v);
            assert!(x.is_err())
        }
    }

    #[test]
    fn test_parse_dateinput_none() {
        let now = Local::now();
        let parsed = parse_dateinput(None).unwrap();
        assert!(now.minute() == parsed.minute());
        assert!(now.hour() == parsed.hour());
        assert!(now.day() == parsed.day());
        assert!(now.month() == parsed.month());
        assert!(now.year() == parsed.year());
    }

    #[test]
    fn test_parse_dateinput() {
        let expected = DateInput {
            year: Year(2026),
            month: Month(8),
            day: Day(2),
        };

        let actual = parse_dateinput(Some(expected)).unwrap();
        assert!(actual.year() == 2026);
        assert!(actual.month() == 8);
        assert!(actual.day() == 2);
    }
}
