pub mod parser;
mod project_log;
pub mod time;

use parser::CliResult;
use project_log::{ProjectLog, WorkLog};

fn main() {
    match parser::parse() {
        CliResult::PeriodLog {
            project,
            period,
            date,
            desc,
            breaks,
        } => {
            let work_log = WorkLog::new_period(period, date, desc, breaks);
            ProjectLog::log(&project, work_log);
        }
        CliResult::SimpleLog {
            project,
            duration,
            date,
            desc,
        } => {
            let work_log = WorkLog::new_duration(duration, date, desc);
            ProjectLog::log(&project, work_log);
        }
        CliResult::Init {
            project,
            start_date,
        } => ProjectLog::init(project, start_date),
    }
}
