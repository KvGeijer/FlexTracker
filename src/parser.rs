use std::fmt::Debug;
use clap::{Parser, Subcommand};
use regex;
use core::str::FromStr;
use crate::project_log::{WorkLog, PeriodLog, TimeLog};
use crate::time::{now, Date};
use lazy_static::lazy_static;


pub enum CliResult {
    Log(WorkLog),       // TODO: The parse should probably not have acces to this
    Init(Option<Date>),
}

#[derive(Subcommand)]
enum SubCli {
    Log(CliLog),
    Init(CliDate),
}

// TODO: Make one and only one of time and period be required. Use ArgGroup::multiple(true)
// TODO: Maybe break out times into short subcommand? Would that work? Or maybe just take timestamps as strings?
// TODO: Make time a struct and parse it with clap. Builder maybe?
/// Log a time or period at work
#[derive(Parser)]
pub struct CliLog {
    /// What did you do today?
    description: String,

    /// Time spent at work (hours[:minutes])
    #[arg(short, long)]
    time: Option<String>,

    /// Period spent at work (hours[:minutes]-hours[:minutes])
    #[arg(short, long)]
    period: Option<String>,

    /// Breaks not counted in work (hours[:minutes])*
    #[arg(short, long)]
    breaks: Vec<String>,
}

#[derive(Parser)]
pub struct CliDate {
    date: Option<String>,
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
        SubCli::Init(cli_date) => parse_init(cli_date.date),
    }
}

fn parse_init(date: Option<String>) -> CliResult {
    CliResult::Init(date.map(parse_date))
}

fn parse_date(date_str: String) -> Date {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(
            r"(\d{4})?-(1?\d)?([123]\d)").unwrap();
    }

    let caps = RE.captures(&date_str)
        .expect("Submitted date does not match date regex!");

    let (y, m, d) = now().0.into_ymd();

    let year = parse_cap(caps.get(1), y);    // TODO: Automatically find year
    let month = parse_cap(caps.get(2), m); // TODO: Get month
    let day = parse_cap(caps.get(3), d);   // TODO, better error handling?

    // TODO: better chech for valid date
    assert!(day <= 31);

    Date::new(year, month, day)
}

fn parse_log(log: CliLog) -> CliResult{
    let worklog =
        if log.time.is_some() {
            let (hours, minutes) = parse_time(&log.time.unwrap());
            WorkLog::Time(TimeLog::new(hours, minutes, log.description))
        } else  {
            let ((from_hrs, from_min), (to_hrs, to_min)) = parse_period(&log.period.unwrap());
            let breaks = log.breaks.iter()
                .map(|break_str| parse_time(break_str))
                .collect();
            WorkLog::Period(
                PeriodLog::new(from_hrs, from_min, to_hrs, to_min, breaks, log.description))
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

    let hours = parse_cap(caps.get(1), 0);
    let minutes = parse_cap(caps.get(2), 0);

    (hours, minutes)
}

fn parse_cap<T: FromStr>(cap: Option<regex::Match>, default: T) -> T
        where <T as FromStr>::Err: Debug {
    match cap {
        None => default,
        Some(string) => string
            .as_str()
            .parse::<T>()
            .expect("Was unable to parse a regex capture!"),    // TODO: Too broad! Return Result
    }
}
