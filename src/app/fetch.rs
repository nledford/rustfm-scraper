use anyhow::Result;
use num_format::ToFormattedString;

use crate::app::Fetch;
use crate::config::{Config, StorageFormat};
use crate::data::db;
use crate::models::saved_scrobbles::SavedScrobbles;
use crate::{data, lastfm, utils};

pub async fn fetch(f: Fetch, config: Config) -> Result<()> {
    let username = match f.username {
        Some(username) => username,
        None => config.default_username,
    };

    println!("Fetching user profile `{}`...", &username);
    let user = lastfm::profile::fetch_profile(&username, &config.api_key).await?;

    println!("Username: {}", user.name);
    println!("Number of scrobbles: {}", user.play_count_formatted());

    let file_format = match config.storage_format {
        StorageFormat::Csv => "csv",
        StorageFormat::Json => "json",
        StorageFormat::Sqlite => "db",
    };

    // TODO handle sqlite files

    // Timestamp from sqlite database, if used
    let mut most_recent_timestamp: i64 = 0;

    // Data from CSV/JSON files
    let mut saved_tracks: SavedScrobbles = SavedScrobbles::default();

    if !f.new_file {
        if file_format == "db" {
            let pool = db::get_sqlite_pool().await?;
            // most_recent_timestamp = db::get_most_recent_scrobble(&pool).await?;

            most_recent_timestamp = match db::get_most_recent_scrobble(&pool).await {
                Ok(timestamp) => timestamp,
                Err(_) => {
                    println!("Existing Sqlite database not found. Creating one now...");
                    db::build_sqlite_database()?;
                    0
                }
            }
        } else {
            saved_tracks = match data::load_from_file(&user.name, &file_format) {
                Ok(saved_scrobbles) => saved_scrobbles,
                Err(_) => {
                    println!(
                        "Existing file for `{}` not found. Creating new file...",
                        &user.name
                    );
                    SavedScrobbles::default()
                }
            }
        }
    }

    /*let mut saved_tracks = if !f.new_file {
        match data::load_from_file(&user.name, &file_format) {
            Ok(saved_scrobbles) => saved_scrobbles,
            Err(_) => {
                println!(
                    "Existing file for `{}` not found. Creating new file...",
                    &user.name
                );
                SavedScrobbles::default()
            }
        }
    } else {
        println!("Creating new file...");
        SavedScrobbles::default()
    };*/

    let page = match f.page {
        Some(page) => {
            if page <= 0 {
                1
            } else {
                page
            }
        }
        None => 1,
    };

    let limit = match f.limit {
        Some(limit) => {
            if limit <= 0 {
                50
            } else if limit > 1000 {
                1000
            } else {
                limit
            }
        }
        None => 1000,
    };

    let from = f.from.unwrap_or(0);

    let to = match f.to {
        Some(to) => to,
        None => utils::get_current_unix_timestamp(),
    };

    let min_timestamp = if f.current_day {
        use chrono::prelude::*;

        Utc::now().date().and_hms(0, 0, 0).timestamp()
    } else if !saved_tracks.is_empty() || most_recent_timestamp != 0 {
        if !saved_tracks.is_empty() {
            match saved_tracks.most_recent_scrobble() {
                Some(track) => track.timestamp_utc + 9,
                None => -1,
            }
        } else {
            most_recent_timestamp
        }
    } else {
        from
    };

    let new_tracks = lastfm::recently_played::fetch_tracks(
        &user,
        &config.api_key,
        page,
        limit,
        min_timestamp,
        to,
    )
    .await?;

    if new_tracks.is_empty() {
        println!("No new tracks were retrieved from Last.fm");
        return Ok(());
    }

    let new_tracks_len = &new_tracks.len();
    /*let new_total = if !saved_tracks.is_empty() {
        match new_tracks_len {
            1 => println!("Saving one new track to existing file..."),
            _ => println!(
                "Saving {} new tracks to existing file...",
                new_tracks_len.to_formatted_string(&utils::get_locale())
            ),
        }
        data::append_to_file(&new_tracks, &mut saved_tracks, &user.name, &file_format)?
    } else {
        println!(
            "Saving {} tracks to file...",
            &new_tracks.len().to_formatted_string(&utils::get_locale())
        );
        data::save_to_file(&new_tracks, &user.name, &file_format)?
    };*/

    let new_total;
    if file_format != "db" {
        new_total = if !saved_tracks.is_empty() {
            match new_tracks_len {
                1 => println!("Saving one new track to existing file..."),
                _ => println!(
                    "Saving {} new tracks to existing file...",
                    new_tracks_len.to_formatted_string(&utils::get_locale())
                ),
            }
            data::append_to_file(&new_tracks, &mut saved_tracks, &user.name, &file_format)?
        } else {
            println!(
                "Saving {} tracks to file...",
                &new_tracks.len().to_formatted_string(&utils::get_locale())
            );
            data::save_to_file(&new_tracks, &user.name, &file_format)?
        };
    } else {
        if most_recent_timestamp != 0 {
            match new_tracks_len {
                1 => println!("Saving one new track to existing file..."),
                _ => println!(
                    "Saving {} new tracks to existing file...",
                    new_tracks_len.to_formatted_string(&utils::get_locale())
                ),
            }
        } else {
            println!(
                "Saving {} tracks to file...",
                &new_tracks.len().to_formatted_string(&utils::get_locale())
            );
        }
        let new_scrobbles = SavedScrobbles::from_scrobbles(&new_tracks);
        let pool = db::get_sqlite_pool().await?;
        new_total = db::insert_scrobbles(new_scrobbles, &pool).await?;
    }

    if new_total != user.play_count() && !f.current_day {
        println!(
            "{} scrobbles were saved to the file, when {} scrobbles were expected.",
            new_total.to_formatted_string(&utils::get_locale()),
            user.play_count().to_formatted_string(&utils::get_locale())
        );
        println!("Please consider creating a new file with the new file flag. `-n`");
    } else {
        match new_total {
            1 => println!("One scrobble saved"),
            _ => println!(
                "{} scrobbles saved.",
                new_total.to_formatted_string(&utils::get_locale())
            ),
        }
    }

    Ok(())
}
