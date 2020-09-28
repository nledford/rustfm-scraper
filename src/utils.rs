use std::ops::Sub;

use chrono::Duration;

use crate::types::AllSavedTracks;

/// Retrieves the current UTC date and time as a unix timestamp in seconds
pub fn get_current_unix_timestamp() -> i64 {
    chrono::offset::Utc::now().timestamp()
}

fn get_duration(tracks: &AllSavedTracks) -> Duration {
    let first_date = tracks.last().expect("Could not get first track").date();
    let last_date = tracks.first().expect("Could not get last track").date();

    last_date.sub(first_date)
}

pub fn get_total_days(tracks: &AllSavedTracks) -> i64 {
    get_duration(tracks).num_days()
}

pub fn get_total_weeks(tracks: &AllSavedTracks) -> f64 {
    get_total_days(tracks) as f64 / (365 / 7) as f64
}

pub fn get_total_months(tracks: &AllSavedTracks) -> f64 {
    get_total_days(tracks) as f64 / (365 / 12) as f64
}

pub fn get_total_years(tracks: &AllSavedTracks) -> f64 {
    get_total_days(&tracks) as f64 / 365 as f64
}
