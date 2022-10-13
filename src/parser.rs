use clap::{Parser, Subcommand};
use crate::worklog::WorkLog;

pub enum CliResult {
    Log(WorkLog),
}

#[derive(Subcommand)]
enum SubCli {
    Log(CliLog),
    NotImplemented,
}

// TODO: Make one of the times to be required! Use ArgGroup::multiple(true)
// TODO: Maybe break out times into short subcommand? Would that work? Or maybe just take timestamps as strings?
#[derive(Parser)]
pub struct CliLog {
    /// Logged hours
    #[arg(long, default_value_t = 0)]
    hours: usize,

    /// Logged minutes
    #[arg(long, default_value_t = 0)]
    minutes: usize,

    /// Breaks not counted in work (in minutes)
    #[arg(short, long)]
    breaks: Vec<usize>,

    /// What did you do today?
    #[arg(short, long)]
    description: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub: SubCli
}

pub fn parse() -> CliResult {
    let cli = Cli::parse();
    match cli.sub {
        SubCli::Log(log) => parse_log(log),
        _ => panic!("What?"),
    }
}

pub fn parse_log(log: CliLog) -> CliResult{
    let worklog = WorkLog::new(log.hours, log.minutes, log.description, log.breaks);
    CliResult::Log(worklog)
}

