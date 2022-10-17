use crate::time::{Time, Period, Date, now};

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
            time: Time {hours: hours, minutes: minutes},
            period: None,
            breaks: breaks.iter()   // I think I recall there being a better way to do this?
                          .map(|&min| Time{hours: 0, minutes: min})
                          .collect(),
            date: date,
            description: Some(description),
        }
    }
}


