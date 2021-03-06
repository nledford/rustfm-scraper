use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::recent_tracks::Track;
use crate::models::saved_scrobbles::SavedScrobbles;

mod csv;
pub mod db;
mod json;

fn validate_extension(extension: &str) {
    let valid_extensions = vec!["csv", "json"];

    if !valid_extensions.contains(&extension) {
        panic!("A valid extension was not provided.")
    }
}

fn build_file_path(username: &str, extension: &str) -> Result<PathBuf> {
    validate_extension(extension);

    let current_dir =
        env::current_dir().context("Error fetching current directory from environment")?;
    Ok(current_dir.join(format!("{}.{}", username, extension)))
}

pub fn check_if_file_exists(username: &str, extension: &str) -> Result<bool> {
    validate_extension(extension);

    let file = build_file_path(username, extension)?;

    Ok(file.exists())
}

pub fn find_which_file_exists(username: &str) -> Result<Option<&str>> {
    let exists = if check_if_file_exists(username, "csv")? {
        Some("csv")
    } else if check_if_file_exists(username, "json")? {
        Some("json")
    } else {
        None
    };

    Ok(exists)
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
    let file_format = find_which_file_exists(username)?.context("No valid file was found")?;
    load_from_file(username, file_format)
}

pub fn load_from_file(username: &str, file_format: &str) -> Result<SavedScrobbles> {
    println!(
        "Loading saved scrobbles from `{}.{}`...",
        username, file_format
    );

    if file_format == "csv" {
        csv::load_from_csv(username)
    } else {
        json::load_from_json(username)
    }
}
