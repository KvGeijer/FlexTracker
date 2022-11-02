// Would probably be MUCH better, but less fun to use the chrono crate
use chrono::{self, Datelike};
use lazy_static::lazy_static;
use regex;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result};
use std::iter::Sum;
use std::ops::{Add, Sub};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Ord)]
pub struct Time {
    hours: usize,
    minutes: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Ord, Clone)]
pub struct Duration {
    minutes: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Period {
    from: Time,
    to: Time,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: usize,
    month: usize,
    day: usize,
}

// Was a hassle to implement, so I did it with chrono, but this means it should stay private :/
struct DateIterator {
    current: chrono::NaiveDate,
    end: chrono::NaiveDate,
}

impl Date {
    pub fn new(year: usize, month: usize, day: usize) -> Date {
        Self { year, month, day }
    }

    pub fn into_ymd(&self) -> (usize, usize, usize) {
        (self.year, self.month, self.day)
    }

    pub fn start_of_month(&self) -> Date {
        let mut trunkated = self.clone();
        trunkated.day = 1;
        trunkated
    }

    pub fn weekdays_since(&self, to: &Date) -> usize {
        // Returns a vector over all dates from self until to.
        DateIterator::weekdays_since(self, to)
    }

    fn to_naive_chrono(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd(self.year as i32, self.month as u32, self.day as u32)
    }
}

impl DateIterator {
    fn weekdays_since(from: &Date, to: &Date) -> usize {
        DateIterator {
            current: from.to_naive_chrono(),
            end: to.to_naive_chrono(),
        }
        .into_iter()
        .filter(|date| {
            date.weekday() != chrono::Weekday::Sat && date.weekday() != chrono::Weekday::Sun
        })
        .count()
    }
}

// NOTE: Could just use chronos iterators over naive dates...
impl Iterator for DateIterator {
    type Item = chrono::NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let current = self.current.clone();
            self.current = self.current.succ();
            Some(current)
        } else {
            None
        }
    }
}

impl Time {
    pub fn new(hours: usize, minutes: usize) -> Time {
        Self { hours, minutes }
    }

    // TODO: Change to just implementing Sub?
    // TODO: can work not just within one day!
    pub fn time_since(&self, earlier: &Self) -> Duration {
        let hours_diff = self.hours as i32 - earlier.hours as i32;
        let minutes_diff = self.minutes as i32 - earlier.minutes as i32;
        Duration::from_m(hours_diff * 60 + minutes_diff)
    }
}

//NOTE: Could just derive as it is lexical
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self
            .hours
            .cmp(&other.hours)
            .then(self.minutes.cmp(&other.minutes));
        Some(ord)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl Display for Period {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (hrs, min) = self.to_hm();
        let write_time = |buf: &mut Formatter, time: i32, time_type| {
            if time != 0 {
                write!(buf, "{} {}", time, time_type)?;
                if time.abs() != 1 {
                    write!(buf, "s")?;
                }
            }
            Ok(())
        };
        if hrs != 0 && min != 0 {
            write_time(f, hrs, "hour")?;
            write!(f, " and ")?;
            write_time(f, min, "minute")?;
        } else {
            write_time(f, hrs, "hour")?;
            write_time(f, min, "minute")?;
        }
        Ok(())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.minutes.cmp(&other.minutes))
    }
}

// NOTE: Maybe should do this for the borrowed type instead?
impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration {
            minutes: self.minutes - other.minutes,
        }
    }
}

// TODO: Write some test module for this
impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, other: Self) -> Self::Output {
        Duration {
            minutes: self.minutes + other.minutes,
        }
    }
}

impl Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Duration::from_m(0), |acc, dur| acc + dur)
    }
}

impl Period {
    pub fn new(from: Time, to: Time) -> Self {
        Self { from, to }
    }

    pub fn duration(&self) -> Duration {
        self.to.time_since(&self.from)
    }
}

impl Duration {
    pub fn from_m(minutes: i32) -> Self {
        Self { minutes }
    }

    pub fn from_hm(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: hours * 60 + minutes,
        }
    }

    pub fn to_hm(&self) -> (i32, i32) {
        (self.minutes / 60, self.minutes % 60)
    }
}

pub fn now() -> (Date, Time) {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(concat!(
            r"^(?P<year>\d{4})-",
            r"(?P<month>\d{2})-",
            r"(?P<day>\d{2})T",
            r"(?P<hour>\d{2}):",
            r"(?P<min>\d{2})"
        ))
        .unwrap();
    }

    let chrono_now = chrono::offset::Local::now().to_rfc3339();

    let caps = RE.captures(&chrono_now).expect("Wrong date regex!\n");

    let date = Date::new(
        cap_to_usize(caps.name("year")),
        cap_to_usize(caps.name("month")),
        cap_to_usize(caps.name("day")),
    );
    let time = Time::new(
        cap_to_usize(caps.name("hour")),
        cap_to_usize(caps.name("min")),
    );

    (date, time)
}

fn cap_to_usize(cap: Option<regex::Match>) -> usize {
    cap.unwrap()
        .as_str()
        .parse::<usize>()
        .expect("Time/Date should be usize")
}
