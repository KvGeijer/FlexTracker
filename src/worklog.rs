use crate::util::{Time, Period, Date};

pub struct WorkLog {
    time: Time,
    period: Option<Period>,
    breaks: Vec<Time>,
    date: Date,
    description: Option<String>,
}

impl WorkLog {
    pub fn new(hours: usize, minutes: usize, description: String, breaks: Vec<usize>) -> WorkLog {
        WorkLog
        {
            time: Time {hours: hours, minutes: minutes},
            period: None,
            breaks: breaks.iter()   // I think I recall there being a better way to do this?
                          .map(|&min| Time{hours: 0, minutes: min})
                          .collect(),
            date: Date {
                day: 13,
                month: 10,
                year: 2022,
            },
            description: Some(description),
        }
    }
}


