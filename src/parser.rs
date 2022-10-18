use clap::{Parser, Subcommand};
use regex;
use crate::logger::WorkLog;
use lazy_static::lazy_static;


pub enum CliResult {
    Log(WorkLog),
}

#[derive(Subcommand)]
enum SubCli {
    Log(CliLog),
    NotImplemented,
}

// TODO: Make one and only one of time and period be required. Use ArgGroup::multiple(true)
// TODO: Maybe break out times into short subcommand? Would that work? Or maybe just take timestamps as strings?
// TODO: Make time a struct and parse it with clap. Builder maybe?
/// Log a time or period at work
#[derive(Parser)]
pub struct CliLog {
    /// Time spent at work (hours[:minutes])
    #[arg(short, long)]
    time: Option<String>,

    /// Period spent at work (hours[:minutes]-hours[:minutes])
    #[arg(short, long)]
    period: Option<String>,

    /// Breaks not counted in work (hours[:minutes])*
    #[arg(short, long)]
    breaks: Vec<String>,

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
    let worklog =
        if log.time.is_some() {
            let (hours, minutes) = parse_time(&log.time.unwrap());
            WorkLog::new_time(hours, minutes, log.description)
        } else  {
            let ((from_hrs, from_min), (to_hrs, to_min)) = parse_period(&log.period.unwrap());
            let breaks = log.breaks.iter()
                .map(|break_str| parse_time(break_str))
                .collect();
            WorkLog::new_period(from_hrs, from_min, to_hrs, to_min, breaks, log.description)
        };

    CliResult::Log(worklog)
}

// TODO: return option
fn parse_period(period_str: &str) -> ((usize, usize), (usize, usize)) {
    let mut split = period_str.split('-');
    let from = split.next()
        .expect("Inavlid period which dould not be split!");
    let to = split.next()
        .expect("Input a valid period!");

    (parse_time(from), parse_time(to))
}

// TODO: Return Option for better error report
fn parse_time(time_str: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(
            r"(\d\d?)(?::(\d\d?))?").unwrap();
    }

    let caps = RE.captures(time_str)
        .expect("Invalid time parsing regex!");

    let hours = parse_time_cap(caps.get(1));
    let minutes = parse_time_cap(caps.get(2));

    (hours, minutes)
}

fn parse_time_cap(cap: Option<regex::Match>) -> usize {
    match cap {
        None => 0,
        Some(string) => string
            .as_str()
            .parse::<usize>()
            .expect("Time should be an integer!"),
    }
}
