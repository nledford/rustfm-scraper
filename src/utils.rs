use std::ops::Sub;

use chrono::Duration;

use crate::models::saved_scrobbles::SavedScrobble;

/// Retrieves the current UTC date and time as a unix timestamp in seconds
pub fn get_current_unix_timestamp() -> i64 {
    chrono::offset::Utc::now().timestamp()
}

fn get_duration(scrobbles: &[SavedScrobble]) -> Duration {
    let first_date = scrobbles.last().expect("Could not get first track").date();
    let last_date = scrobbles.first().expect("Could not get last track").date();

    last_date.sub(first_date)
}

pub fn get_total_days(scrobbles: &[SavedScrobble]) -> i64 {
    get_duration(scrobbles).num_days()
}

pub fn get_total_weeks(scrobbles: &[SavedScrobble]) -> f64 {
    get_total_days(scrobbles) as f64 / (365 / 52) as f64
}

pub fn get_total_months(scrobbles: &[SavedScrobble]) -> f64 {
    get_total_days(scrobbles) as f64 / (365 / 12) as f64
}

pub fn get_total_years(scrobbles: &[SavedScrobble]) -> f64 {
    get_total_days(scrobbles) as f64 / 365_f64
}
