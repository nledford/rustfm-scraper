use std::env;

use anyhow::Result;
use clap::Clap;

use rustfm_scraper::app::SubCommand;
use rustfm_scraper::app::{Fetch, Opts};
use rustfm_scraper::config::Config;
use rustfm_scraper::{config, files, lastfm, utils};

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let opts = Opts::parse();

    // First check if a config file has been created
    // If not, prompt the user to create one
    if !config::check_if_config_exists() {
        config::initialize_config()?;
    }

    let config = Config::load_config()?;

    match opts.subcmd {
        SubCommand::Fetch(f) => fetch(f, config).await?,
    }

    println!("\nDone!");

    Ok(())
}

async fn fetch(f: Fetch, config: Config) -> Result<()> {
    let username = match f.username {
        Some(username) => username,
        None => config.default_username,
    };

    println!("Fetching user profile `{}`...", &username);
    let user = lastfm::fetch_profile(&username, &config.api_key).await?;

    println!("\nUsername: {}", user.name);
    println!("Number of scrobbles: {}", user.play_count());

    let (append_tracks, mut saved_tracks) = if files::check_if_csv_exists(&user.name) && !f.new_file
    {
        println!("Loading existing files from hard drive...");
        (true, files::load_from_csv(&user.name))
    } else {
        println!("Creating new file...");
        (false, Vec::new())
    };

    let min_timestamp = match saved_tracks.get(0) {
        Some(track) => track.timestamp_utc + 10,
        None => 0,
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
        None => 50,
    };

    let from = match f.from {
        Some(from) => from,
        None => 0,
    };

    let to = match f.to {
        Some(to) => to,
        None => utils::get_current_unix_timestamp(),
    };

    if append_tracks {
        let new_tracks =
            lastfm::fetch_tracks(&user, &config.api_key, page, limit, min_timestamp, to).await?;

        println!(
            "Saving {} new tracks to existing file...",
            &new_tracks.len()
        );
        files::append_to_csv(new_tracks, &mut saved_tracks, &user.name);
    } else {
        let tracks = lastfm::fetch_tracks(&user, &config.api_key, page, limit, from, to).await?;

        println!("Saving {} tracks to file...", &tracks.len());
        files::save_to_csv(tracks, &user.name);
    }

    Ok(())
}
