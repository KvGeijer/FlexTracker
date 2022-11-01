pub mod parser;
mod project_log;
pub mod time;

use parser::CliResult;
use project_log::ProjectLog;

fn main() {
    match parser::parse() {
        CliResult::Log(worklog) => {
            ProjectLog::log(worklog);
        },
        CliResult::Init(opt_date) => {
            let (today, _) = time::now();
            let date = opt_date.unwrap_or_else(|| today);
            ProjectLog::init(date)
        }
    }
}
