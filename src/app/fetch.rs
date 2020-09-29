use anyhow::Result;
use num_format::ToFormattedString;

use crate::{files, lastfm, utils};
use crate::app::Fetch;
use crate::config::Config;
use crate::models::saved_scrobbles::SavedScrobbles;

pub async fn fetch(f: Fetch, config: Config) -> Result<()> {
    let username = match f.username {
        Some(username) => username,
        None => config.default_username,
    };

    println!("Fetching user profile `{}`...", &username);
    let user = lastfm::profile::fetch_profile(&username, &config.api_key).await?;

    println!("Username: {}", user.name);
    println!("Number of scrobbles: {}", user.play_count_formatted());

    let (append_tracks, mut saved_tracks) = if files::check_if_csv_exists(&user.name) && !f.new_file
    {
        (true, files::load_from_csv(&user.name))
    } else {
        println!("Creating new file...");
        (false, SavedScrobbles::default())
    };

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
            } else if limit > 200 {
                200
            } else {
                limit
            }
        }
        None => 200,
    };

    let from = match f.from {
        Some(from) => from,
        None => 0,
    };

    let to = match f.to {
        Some(to) => to,
        None => utils::get_current_unix_timestamp(),
    };

    let min_timestamp = if f.current_day {
        use chrono::prelude::*;

        Utc::now().date().and_hms(0, 0, 0).timestamp()
    } else if append_tracks {
        match saved_tracks.most_recent_scrobble() {
            Some(track) => track.timestamp_utc + 10,
            None => 0,
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

    let new_total = if append_tracks {
        println!(
            "Saving {} new tracks to existing file...",
            &new_tracks.len().to_formatted_string(&utils::get_locale())
        );
        files::append_to_csv(&new_tracks, &mut saved_tracks, &user.name)?
    } else {
        println!("Saving {} tracks to file...", &new_tracks.len().to_formatted_string(&utils::get_locale()));
        files::save_to_csv(&new_tracks, &user.name)?
    };

    if new_total != user.play_count() {
        println!(
            "{} scrobbles were saved to the file, when {} scrobbles were expected.",
            new_total.to_formatted_string(&utils::get_locale()),
            user.play_count().to_formatted_string(&utils::get_locale())
        );
        println!("Please consider creating a new file with the new file flag. `-n`");
    } else {
        println!("{} scrobbles saved.", new_total.to_formatted_string(&utils::get_locale()));
    }

    Ok(())
}
