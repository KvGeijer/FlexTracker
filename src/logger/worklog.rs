use serde::{Serialize, Deserialize};

use crate::time::{Time, Period, Date, now};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkLog {
    time: Time,
    period: Option<Period>,
    breaks: Vec<Time>,
    date: Date,
    description: Option<String>,
}

impl WorkLog {
    pub fn new(hours: usize, minutes: usize, description: String, breaks: Vec<usize>) -> WorkLog {
        let (date, _time) = now();

        WorkLog
        {
            time: Time::new(hours, minutes),
            period: None,
            breaks: breaks.iter()   // I think I recall there being a better way to do this?
                          .map(|&min| Time::new(0, min))
                          .collect(),
            date: date,
            description: Some(description),
        }
    }

    pub fn date<'a>(&'a self) -> &'a Date {
        &self.date
    }
}


