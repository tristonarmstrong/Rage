use std::fs;

const MINUTE_SECONDS: u64 = 60;
const HOUR_SECONDS: u64 = 3_600;
const DAY_SECONDS: u64 = 86_400;
const WEEK_SECONDS: u64 = 604_800;
const MONTH_SECONDS: u64 = 2_628_003;
const YEAR_SECONDS: u64 = 31_536_000;

pub(crate) fn calculate_modified_time(time: u64) -> String {
    match time {
        t if t < MINUTE_SECONDS => format!("{} second(s) ago", time), // max 1 minute
        t if t < HOUR_SECONDS => format!("{} minute(s) ago", time / MINUTE_SECONDS), // max 1 hour
        t if t < DAY_SECONDS => format!("{} hour(s) ago", time / HOUR_SECONDS), // max 1 day
        t if t < WEEK_SECONDS => format!("{} day(s) ago", time / DAY_SECONDS), // max 1 week
        t if t < MONTH_SECONDS => format!("{} week(s) ago", time / WEEK_SECONDS), // max 1 month
        t if t < YEAR_SECONDS => format!("{} month(s) ago", time / MONTH_SECONDS), // max 1 year
        _ => format!("{} year(s) ago", (time / YEAR_SECONDS)),
    }
}

enum TermColors {
    INFO,
    ERR,
    WARN,
    CLS,
}

impl TermColors {
    const fn as_str(self) -> &'static str {
        match self {
            Self::INFO => "\x1b[34m",
            Self::CLS => "\x1b[0m",
            Self::ERR => "\x1b[31m",
            Self::WARN => "\x1b[33m",
        }
    }
}

pub(crate) struct Logger;
impl Logger {
    pub fn info(msg: &str) -> () {
        let cls = TermColors::CLS.as_str();
        let color = TermColors::INFO.as_str();
        println!("{color}[ INFO ]{cls} - {msg}")
    }

    pub fn err(msg: &str) -> () {
        let cls = TermColors::CLS.as_str();
        let color = TermColors::ERR.as_str();
        println!("{color}[ INFO ]{cls} - {msg}")
    }

    pub fn warn(msg: &str) -> () {
        let cls = TermColors::CLS.as_str();
        let color = TermColors::WARN.as_str();
        println!("{color}[ INFO ]{cls} - {msg}")
    }
}
