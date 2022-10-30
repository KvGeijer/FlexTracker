pub mod parser;
mod project_log;
pub mod time;

fn main() {
    match parser::parse() {
        parser::CliResult::Log(worklog) => {
            project_log::ProjectLog::log(worklog);
        },
    }
}
