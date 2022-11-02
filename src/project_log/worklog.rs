use serde::{Deserialize, Serialize};

use crate::time::{Date, Duration, Period};
use std::fmt::{Display, Formatter, Result};

// TODO: Change to trait and have PeriodWorkLog and TimeWorkLog maybe?
#[derive(Serialize, Deserialize, Debug)]
pub struct PeriodLog {
    duration: Duration, // Just derived from period, can be removed
    period: Period,
    breaks: Vec<Duration>,
    date: Date,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DurationLog {
    duration: Duration,
    date: Date,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkLog {
    Period(PeriodLog),
    Duration(DurationLog),
}

impl WorkLog {
    pub fn get_duration(&self) -> Duration {
        match self {
            WorkLog::Period(period_log) => period_log.duration.clone(),
            WorkLog::Duration(duration_log) => duration_log.duration.clone(),
        }
    }

    pub fn get_date(&self) -> Date {
        match self {
            WorkLog::Period(period_log) => period_log.date.clone(),
            WorkLog::Duration(duration_log) => duration_log.date.clone(),
        }
    }

    pub fn new_period(period: Period, date: Date, desc: String, breaks: Vec<Duration>) -> WorkLog {
        WorkLog::Period(PeriodLog::new(period, date, desc, breaks))
    }

    pub fn new_duration(duration: Duration, date: Date, desc: String) -> WorkLog {
        WorkLog::Duration(DurationLog::new(duration, date, desc))
    }
}

impl PeriodLog {
    pub fn new(period: Period, date: Date, description: String, breaks: Vec<Duration>) -> Self {
        let duration = period.duration() - breaks.clone().into_iter().sum(); // TODO: Don't clone

        Self {
            duration,
            period,
            breaks,
            date,
            description: Some(description),
        }
    }

    #[allow(dead_code)]
    fn date<'a>(&'a self) -> &'a Date {
        &self.date
    }
}

impl DurationLog {
    pub fn new(duration: Duration, date: Date, description: String) -> Self {
        Self {
            duration,
            date,
            description: Some(description),
        }
    }
}

impl Display for WorkLog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            WorkLog::Period(period_log) => period_log.fmt(f),
            WorkLog::Duration(duration_log) => duration_log.fmt(f),
        }
    }
}

impl Display for PeriodLog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}: {}, {}, {}",
            self.date,
            self.duration,
            self.period,
            self.description.clone().unwrap_or("Work".to_owned())
        )?;

        if self.breaks.len() > 0 {
            write!(f, " | Breaks: {}", self.breaks[0])?;
            for dur in &self.breaks[1..] {
                write!(f, ", {}", dur)?;
            }
        }
        Ok(())
    }
}

impl Display for DurationLog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}: {}, {}\n",
            self.date,
            self.duration,
            &self.description.clone().unwrap_or("Work".to_owned())
        )
    }
}
