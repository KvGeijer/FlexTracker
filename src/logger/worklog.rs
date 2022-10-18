use serde::{Serialize, Deserialize};

use crate::time::{Time, Period, Date, now};

// TODO: Change to trait and have PeriodWorkLog and TimeWorkLog maybe?
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLog {
    time: Time,
    period: Option<Period>,
    breaks: Vec<Time>,
    date: Date,
    description: Option<String>,
}

impl WorkLog {
    pub fn new_time(hours: usize, minutes: usize, description: String) -> WorkLog {
        let (date, _time) = now();

        WorkLog
        {
            time: Time::new(hours, minutes),
            period: None,
            breaks: vec![],
            date: date,
            description: Some(description),
        }
    }

    pub fn new_period(from_hours: usize, from_minutes: usize, to_hours: usize, to_minutes: usize,
                      breaks: Vec<(usize, usize)>, description: String) -> WorkLog {
        let (date, _time) = now();
        let from = Time::new(from_hours, from_minutes);
        let to = Time::new(to_hours, to_minutes);
        let time = to.time_since(&from);

        WorkLog
        {
            time: time,
            period: Some(Period::new(from, to)),
            breaks: breaks.iter()   // I think I recall there being a better way to do this?
                          .map(|&(hrs, min)| Time::new(hrs, min))
                          .collect(),
            date: date,
            description: Some(description),
        }
    }

    pub fn date<'a>(&'a self) -> &'a Date {
        &self.date
    }
}


