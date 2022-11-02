use crate::time::{now, Date, Duration, Period, Time};
use clap::{Parser, Subcommand};
use core::str::FromStr;
use lazy_static::lazy_static;
use regex;
use std::fmt::Debug;

pub enum CliResult {
    PeriodLog {
        project: String,
        period: Period,
        date: Date,
        desc: String,
        breaks: Vec<Duration>,
    },
    SimpleLog {
        project: String,
        duration: Duration,
        date: Date,
        desc: String,
    },
    Init {
        project: String,
        start_date: Date,
    },
}

#[derive(Subcommand)]
enum SubCli {
    Log(CliLog),
    Init(InitProject),
}

// TODO: Make one and only one of time and period be required. Use ArgGroup::multiple(true)
// TODO: Maybe break out times into short subcommand? Would that work? Or maybe just take timestamps as strings?
// TODO: Make time a struct and parse it with clap. Builder maybe?
/// Log a time or period at work
#[derive(Parser)]
pub struct CliLog {
    /// Which project shall we log the time towards?
    project: String, // TODO: Make default project accesable

    /// What did you do today?
    description: String,

    /// Time spent at work (hours[:minutes])
    #[arg(short, long)]
    time: Option<String>,

    /// Period spent at work (hours[:minutes]-hours[:minutes])
    #[arg(short, long)]
    period: Option<String>,

    /// Which date you want to log it for
    #[arg(short, long)]
    date: Option<String>,

    /// Breaks not counted in work (hours[:minutes])*
    #[arg(short, long)]
    breaks: Vec<String>,
}

#[derive(Parser)]
pub struct InitProject {
    /// Project name
    name: String,

    /// Starting date
    date: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub: SubCli,
}

pub fn parse() -> CliResult {
    let cli = Cli::parse();
    match cli.sub {
        SubCli::Log(log) => parse_log(log),
        SubCli::Init(init) => parse_init(init.name, init.date),
    }
}

fn parse_init(project: String, opt_date: Option<String>) -> CliResult {
    let start_date = opt_date.map_or(now().0, parse_date);
    CliResult::Init {
        project,
        start_date,
    }
}

fn parse_date(date_str: String) -> Date {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d{4})?-(1?\d)?([123]\d)").unwrap();
    }

    let caps = RE
        .captures(&date_str)
        .expect("Submitted date does not match date regex!");

    let (y, m, d) = now().0.into_ymd();

    let year = parse_cap(caps.get(1), y);
    let month = parse_cap(caps.get(2), m);
    let day = parse_cap(caps.get(3), d); // TODO, better error handling?

    // TODO: better check for valid date
    assert!(day <= 31);

    Date::new(year, month, day)
}

fn parse_log(log: CliLog) -> CliResult {
    let date = log.date.map_or(now().0, parse_date);

    match log.period {
        Some(str_period) => {
            let ((from_hrs, from_min), (to_hrs, to_min)) = parse_period(&str_period);
            let period = Period::new(Time::new(from_hrs, from_min), Time::new(to_hrs, to_min));
            let breaks: Vec<Duration> = log
                .breaks
                .iter()
                .map(|break_str| parse_time(break_str))
                .map(|(hrs, min)| Duration::from_hm(hrs as i32, min as i32))
                .collect();
            CliResult::PeriodLog {
                project: log.project,
                period,
                date,
                desc: log.description,
                breaks,
            }
        }
        None => {
            let (hours, minutes) =
                parse_time(&log.time.expect("Must supply either time or period!"));
            let duration = Duration::from_hm(hours as i32, minutes as i32);
            CliResult::SimpleLog {
                project: log.project,
                duration,
                date,
                desc: log.description,
            }
        }
    }
}

// TODO: return option
fn parse_period(period_str: &str) -> ((usize, usize), (usize, usize)) {
    let mut split = period_str.split('-');
    let from = split
        .next()
        .expect("Inavlid period which dould not be split!");
    let to = split.next().expect("Input a valid period!");

    (parse_time(from), parse_time(to))
}

// TODO: Return Option for better error report
fn parse_time(time_str: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d\d?)(?::(\d\d?))?").unwrap();
    }

    let caps = RE.captures(time_str).expect("Invalid time parsing regex!");

    let hours = parse_cap(caps.get(1), 0);
    let minutes = parse_cap(caps.get(2), 0);

    (hours, minutes)
}

fn parse_cap<T: FromStr>(cap: Option<regex::Match>, default: T) -> T
where
    <T as FromStr>::Err: Debug,
{
    match cap {
        None => default,
        Some(string) => string
            .as_str()
            .parse::<T>()
            .expect("Was unable to parse a regex capture!"), // TODO: Too broad! Return Result
    }
}
