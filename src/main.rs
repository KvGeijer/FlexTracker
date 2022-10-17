pub mod parser;
mod logger;
pub mod time;

fn main() {
    match parser::parse() {
        parser::CliResult::Log(worklog) => {
            logger::Logger::log(worklog);
        },
    }
}
