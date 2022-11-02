mod worklog;

pub use self::worklog::{WorkLog, TimeLog, PeriodLog};

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::time::{Date, Duration, now};

const FILE_NAME: &str = "cisco";
const FILE_TYPE: &str = "json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectLog {
    logs: Vec<WorkLog>,
    start_date: Date,
    // TODO: Add different projects
    // TODO: Be able to end projects
    // TODO: Rename projects
    // TODO: Delete a log (Probably not edit, as it would be a bit of a hassle)
}

impl ProjectLog {
    pub fn init(date: Date) {
        let project = Self {
            logs: vec![],
            start_date: date,
        };

        assert!(!&Self::get_path().as_path().exists(), "Project already exists!");

        project.save();
    }

    pub fn log(worklog: WorkLog) {
        let mut project = Self::open();
        let log_msg = worklog.to_string();
        project.logs.push(worklog);
        project.save();
        println!("Work logged:\n{}\n\nRemaining flex time: {}", log_msg, project.get_flex_time());
    }

    fn open() -> Self {
        let path = Self::get_path();

        match std::fs::read_to_string(path) {
            Ok(file_str) =>
                serde_json::from_str(&file_str)
                    .expect("ProjectLog can't be deserialized!"),
            Err(_) =>
                panic!("Initialize the project before logging to it.")
        }
    }

    // TODO: return result? For better error handling
    fn save(&self) {
        folders::create_folders();

        let path = Self::get_path();
        let serialized_str = serde_json::to_string(self)
            .expect("Failed to serialize project log.");

        std::fs::write(path, &serialized_str)
            .expect("Could not save file.");
    }

    // NOTE: Could memoise to save time if logs become lengthy
    fn get_flex_time(&self) -> Duration {
        let worked_hours: Duration = self.logs
            .iter()
            .map(|log| log.get_time())
            .sum();
        let expected_hours = now().0
            .weekdays_since(&self.start_date) * 8;

        worked_hours - Duration::from_hm(expected_hours as i32, 0)
    }

    fn get_path() -> PathBuf {
        let mut path = folders::log_folder();
        path.set_file_name(FILE_NAME);
        path.set_extension(FILE_TYPE);
        path
    }
}

mod folders {
    use std::path::PathBuf;

    const BIN_NAME: &str = env!("CARGO_PKG_NAME");

    pub fn log_folder() -> PathBuf {
        let mut path = dirs::data_dir()
            .expect("Could not find shared data folder.");
        path.push(BIN_NAME);
        path.push("logs");
        path
    }

    pub fn create_folders() {
        std::fs::create_dir_all(log_folder())
            .expect("Failed to create the data folder.");
    }
}