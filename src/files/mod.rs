use std::env;
use std::path::PathBuf;

use anyhow::Result;

use crate::models::recent_tracks::Track;
use crate::models::saved_scrobbles::SavedScrobbles;

mod csv;
mod json;

fn validate_extension(extension: &str) {
    let valid_extensions = vec!["csv", "json"];

    if !valid_extensions.contains(&extension) {
        panic!(format!("{} is not a valid extension", extension))
    }
}

fn build_file_path(username: &str, extension: &str) -> PathBuf {
    validate_extension(extension);

    let current_dir =
        env::current_dir().expect("Error fetching current directory from environment");
    current_dir.join(format!("{}.{}", username, extension))
}

pub fn check_if_file_exists(username: &str, extension: &str) -> bool {
    validate_extension(extension);

    let file = build_file_path(username, extension);

    file.exists()
}

pub fn find_which_file_exists(username: &str) -> Option<&str> {
    if check_if_file_exists(username, "csv") {
        Some("csv")
    } else if check_if_file_exists(username, "json") {
        Some("json")
    } else {
        None
    }
}

pub fn save_to_file(scrobbles: &[Track], username: &str, file_format: &str) -> Result<i32> {
    if file_format == "csv" {
        csv::save_to_csv(scrobbles, username)
    } else {
        json::save_to_json(scrobbles, username)
    }
}

pub fn append_to_file(
    scrobbles: &[Track],
    saved_scrobbles: &mut SavedScrobbles,
    username: &str,
    file_format: &str,
) -> Result<i32> {
    if file_format == "csv" {
        csv::append_to_csv(scrobbles, saved_scrobbles, username)
    } else {
        json::append_to_json(scrobbles, saved_scrobbles, username)
    }
}

pub fn load_from_any_file(username: &str) -> Result<SavedScrobbles> {
    let file_format = find_which_file_exists(username).expect("No valid file was found");
    Ok(load_from_file(username, file_format)?)
}

pub fn load_from_file(username: &str, file_format: &str) -> Result<SavedScrobbles> {
    if file_format == "csv" {
        csv::load_from_csv(username)
    } else {
        json::load_from_json(username)
    }
}
