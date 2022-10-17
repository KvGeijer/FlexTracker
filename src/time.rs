// Would probably be better, but less frun to use the chrono crate
use chrono;
use regex;

pub struct Time {
    pub hours: usize,
    pub minutes: usize
}

pub struct Period {
    pub from: Time,
    pub to: Time
}

pub struct Date {
    pub day: usize,
    pub month: usize,
    pub year: usize
}

pub fn now() -> (Date, Time) {
    let re = regex::Regex::new(r"^
                        (?P<year>\d{4})-
                        (?P<month>\d{2})-
                        (?P<day>\d{2})T
                        (?P<hour>\d{2}):
                        (?P<min>\d{2})").unwrap();

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

impl Date {
    fn new(year: usize, month: usize, day: usize) -> Date {
        Self {
            year,
            month,
            day,
        }
    }
}

impl Time {
    fn new(hours: usize, minutes: usize) -> Time {
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
