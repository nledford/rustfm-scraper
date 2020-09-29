use std::ops::Sub;

use chrono::Duration;
use num_format::{Locale, SystemLocale};

use crate::models::saved_scrobbles::SavedScrobble;

pub fn get_locale() -> Locale {
    let system_locale = SystemLocale::default().expect("Error retrieving system locale");

    let system_locale_name = system_locale
        .name()
        .split('.')
        .map(String::from)
        .collect::<Vec<String>>()
        .first()
        .expect("Error getting system locale first half")
        .split('_')
        .collect::<Vec<&str>>()
        .first()
        .expect("Error getting system local short name")
        .to_string();

    Locale::from_name(system_locale_name).expect("Error building locale from system locale")
}

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
