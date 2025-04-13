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

pub(crate) fn read_file(file_path: &str, pub_path: &str) -> Option<String> {
    let path = format!("{}/{}", pub_path, file_path);
    match fs::canonicalize(path) {
        Ok(path) => {
            if path.starts_with(pub_path) {
                return fs::read_to_string(path).ok();
            }

            None
        }

        Err(_) => None,
    }
}
