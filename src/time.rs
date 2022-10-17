// Would probably be better, but less frun to use the chrono crate
use chrono;
use regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    hours: usize,
    minutes: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    from: Time,
    to: Time
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Date {
    day: usize,
    month: usize,
    year: usize
}

impl Date {
    pub fn new(year: usize, month: usize, day: usize) -> Date {
        Self {
            year,
            month,
            day,
        }
    }

    pub fn into_ymd(&self) -> (usize, usize, usize) {
        (self.year, self.month, self.day)
    }

    pub fn start_of_month(&self) -> Date {
        let mut trunkated = self.clone();
        trunkated.day = 1;
        trunkated
    }
}

impl Time {
    pub fn new(hours: usize, minutes: usize) -> Time {
        Self {
            hours,
            minutes,
        }
    }
}

fn cap_to_usize(cap: Option<regex::Match>) -> usize {
    cap.unwrap()
        .as_str()
        .parse::<usize>()
        .expect("Time/Date should be usize")
}

pub fn now() -> (Date, Time) {
    let re = regex::Regex::new(concat!(
        r"^(?P<year>\d{4})-",
        r"(?P<month>\d{2})-",
        r"(?P<day>\d{2})T",
        r"(?P<hour>\d{2}):",
        r"(?P<min>\d{2})")).unwrap();

    let chrono_now = chrono::offset::Local::now()
        .to_rfc3339();

    let caps = re.captures(&chrono_now)
        .expect("Wrong date regex!\n");

    let date = Date::new(
        cap_to_usize(caps.name("year")),
        cap_to_usize(caps.name("month")),
        cap_to_usize(caps.name("day")));
    let time = Time::new(
        cap_to_usize(caps.name("hour")),
        cap_to_usize(caps.name("min")),
    );

    (date, time)

}
