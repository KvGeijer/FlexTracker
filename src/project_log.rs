mod worklog;

pub use self::worklog::{WorkLog, TimeLog, PeriodLog};

use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::time::Date;

const ROOT_FOLDER: &str = "logs";   // TODO: Make it absolute when compiling maybe?
const FILE_NAME: &str = "cisco";
const FILE_TYPE: &str = "json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectLog {
    logs: Vec<WorkLog>,
    start_date: Date,
    // TODO: Add pre-computed/cached flex here?
    // TODO: Add different projects
    // TODO: Be able to end projects
    // TODO: Rename projects
}

impl ProjectLog {
    pub fn init(date: Date) {
        let project = Self {
            logs: vec![],
            start_date: date,
        };

        assert!(!Path::new(&Self::get_path()).exists(), "Project already exists!");

        project.save();
    }

    pub fn log(worklog: WorkLog) {
        let mut project = Self::open();
        project.logs.push(worklog);
        project.save();
    }

    fn open() -> Self {
        let path = Self::get_path();

        // TODO: Make some nice resut mapping to avoid match
        match std::fs::read_to_string(path) {
            Ok(file_str) =>
                serde_json::from_str(&file_str)
                    .expect("ProjectLog can't be deserialized!"),
            Err(_) =>
                panic!("Initialize the project before logging to it.")
        }
    }

    // TODO: return result? For better error handling
    fn save(self) {
        assert_eq!(FILE_TYPE, "json");
        std::fs::create_dir_all(ROOT_FOLDER)
            .expect("Failed to create parent dirs to log");

        let path = Self::get_path();
        let serialized_str = serde_json::to_string(&self)
            .expect("Failed to serialize project log.");

        std::fs::write(path, &serialized_str)
            .expect("Could not save file.");
    }

    fn get_path() -> String {
        format!("{}/{}.{}", ROOT_FOLDER, FILE_NAME, FILE_TYPE)
            .to_string()
    }
}
