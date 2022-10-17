mod worklog;
mod monthlog;

pub use self::{worklog::WorkLog, monthlog::MonthLog};

pub struct Logger {

}

impl Logger {
    pub fn log(worklog: WorkLog) {
        MonthLog::log(worklog);
    }
}
