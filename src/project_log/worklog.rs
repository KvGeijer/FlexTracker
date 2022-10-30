use serde::{Serialize, Deserialize};

use crate::time::{Time, Period, Date, now};

// TODO: Change to trait and have PeriodWorkLog and TimeWorkLog maybe?
#[derive(Serialize, Deserialize, Debug)]
pub struct PeriodLog {
    time: Time,     // Just derived from period
    period: Period,
    breaks: Vec<Time>,
    date: Date,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeLog {
    time: Time,
    date: Date,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkLog {
    Period(PeriodLog),
    Time(TimeLog),
}

impl PeriodLog {
    pub fn new(from_hours: usize, from_minutes: usize, to_hours: usize, to_minutes: usize,
               breaks: Vec<(usize, usize)>, description: String) -> Self {
        let (date, _time) = now();
        let from = Time::new(from_hours, from_minutes);
        let to = Time::new(to_hours, to_minutes);
        let time = to.time_since(&from);

        Self
        {
            time: time,
            period: Period::new(from, to),
            breaks: breaks.into_iter()
                          .map(|(hrs, min)| Time::new(hrs, min))
                          .collect(),
            date: date,
            description: Some(description),
        }
    }

    #[allow(dead_code)]
    fn date<'a>(&'a self) -> &'a Date {
        &self.date
    }
}

impl TimeLog {
    pub fn new(hours: usize, minutes: usize, description: String) -> Self {
        let (date, _time) = now();

        Self
        {
            time: Time::new(hours, minutes),
            date: date,
            description: Some(description),
        }
    }

}


