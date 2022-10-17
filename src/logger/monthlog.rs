use serde::{Serialize, Deserialize};
use std::fs;

use super::WorkLog;
use crate::time::{Time, Date};

macro_rules! file_type {
    () => { "json" };
}

macro_rules! root_folder {
    () => { "/Users/kvongeij/dev/flex_cli/logs" }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonthLog {
    start_date: Date,
    worklogs: Vec<WorkLog>,
    starting_flex: Option<Time>,    // TODO implement actual option functionality
}

impl MonthLog {
    pub fn log(worklog: WorkLog) {
        // register the new worklog
        // TODO: More operations
        let mut monthlog = Self::read(worklog.date());
        monthlog.add_worklog(worklog);
        monthlog.write();
    }

    fn add_worklog(&mut self, worklog: WorkLog) {
        self.worklogs.push(worklog);
    }

    fn new(date: &Date) -> MonthLog {
        // Just starts at the beginning of the month
        Self {
            start_date: date.start_of_month(),
            worklogs: vec![],
            starting_flex: None,      // TODO
        }
    }

    fn read(date: &Date) -> MonthLog {
        // Opens the MonthLog for the given date. If none exist, create new empty log
        let name = get_file_name(date);
        match fs::read_to_string(name) {
            Ok(string) => serde_json::from_str(&string)
                .expect("Not correct JSON format"),
            Err(_) => {
                MonthLog::new(date)
            }
        }
    }

    fn write(&self) {
        let name = get_file_name(&self.start_date);
        fs::write(
            name,
            serde_json::to_string_pretty(self).unwrap(),
        ).expect("Problems with writing file");
    }

    // TODO: Open month halfway through month?
}

fn get_file_name(date: &Date) -> String {
    let (year, month, _) = date.into_ymd();
    format!("{}/monthlog_{}-{}.{}", root_folder!(), year, month, file_type!())
}
