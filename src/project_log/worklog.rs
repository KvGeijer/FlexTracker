use serde::{Serialize, Deserialize};

use crate::time::{Duration, Time, Period, Date, now};
use std::fmt::{Display, Formatter, Result};

// TODO: Change to trait and have PeriodWorkLog and TimeWorkLog maybe?
#[derive(Serialize, Deserialize, Debug)]
pub struct PeriodLog {
    time: Duration,     // Just derived from period
    period: Period,
    breaks: Vec<Duration>,
    date: Date,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeLog {
    time: Duration,
    date: Date,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkLog {
    Period(PeriodLog),
    Time(TimeLog),
}

impl WorkLog {
    pub fn get_time(&self) -> Duration {
        match self {
            WorkLog::Period(period_log)  => period_log.time.clone(),
            WorkLog::Time(time_log)      => time_log.time.clone(),
        }
    }
}

impl PeriodLog {
    pub fn new(from_hours: usize, from_minutes: usize, to_hours: usize, to_minutes: usize,
               parsed_breaks: Vec<(usize, usize)>, description: String) -> Self {
        let breaks: Vec<Duration> = parsed_breaks.into_iter()
            .map(|(hrs, min)| Duration::from_hm(hrs as i32, min as i32))
            .collect();
        let (date, _time) = now();
        let from = Time::new(from_hours, from_minutes);
        let to = Time::new(to_hours, to_minutes);
        let time = to.time_since(&from) - breaks.clone().into_iter().sum();

        Self
        {
            time: time,
            period: Period::new(from, to),
            breaks: breaks,
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
            time: Duration::from_hm(hours as i32, minutes as i32),
            date: date,
            description: Some(description),
        }
    }
}

impl Display for WorkLog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            WorkLog::Period(period_log) => period_log.fmt(f),
            WorkLog::Time(time_log) => time_log.fmt(f),
        }
    }
}

impl Display for PeriodLog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}, {}, {}", self.date, self.time, self.period,
            self.description.clone().unwrap_or("Work".to_owned()))?;

        if self.breaks.len() > 0 {
            write!(f, " | Breaks: {}", self.breaks[0])?;
            for dur in &self.breaks[1..] {
                write!(f, ", {}", dur)?;
            }
        }
        Ok(())
    }
}

impl Display for TimeLog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}, {}\n", self.date, self.time,
            &self.description.clone().unwrap_or("Work".to_owned()))
    }
}
