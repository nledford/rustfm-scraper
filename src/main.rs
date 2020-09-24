use std::{env, io, process};

use anyhow::Result;
use clap::Clap;

use rustfm_scraper::{files, lastfm, utils};
use rustfm_scraper::app::Opts;
use rustfm_scraper::app::SubCommand;
use rustfm_scraper::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let opts = Opts::parse();

    // First check if a config file has been created
    // If not, prompt the user to create one
    if !Config::check_if_config_exists() {
        println!("Config file does not exist. Creating one now...");

        println!("Enter your Last.fm API key: ");
        let mut api_key = String::new();
        io::stdin()
            .read_line(&mut api_key)
            .expect("Failed to read api key");

        let config = Config::new(api_key);
        config.save_config()?;

        process::exit(0);
    }

    let config = Config::load_config()?;

    match opts.subcmd {
        SubCommand::Fetch(f) => {
            let user = lastfm::fetch_profile(&f.username, &config.api_key).await?;

            println!("Username: {}", user.name);
            println!("Number of scrobbles: {}", user.play_count());

            let mut append_tracks = false;
            let mut saved_tracks = if f.append {
                append_tracks = true;
                println!("Loading existing tracks from the local hard drive...");
                files::load_from_csv(&user.name)
            } else {
                Vec::new()
            };

            let min_timestamp = match saved_tracks.get(0) {
                Some(track) => track.timestamp_utc,
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
                    lastfm::fetch_tracks(&user, &config.api_key, page, limit, min_timestamp, to)
                        .await?;

                println!(
                    "Saving {} new tracks to existing file...",
                    &new_tracks.len()
                );
                files::append_to_csv(new_tracks, &mut saved_tracks, &user.name);
            } else {
                let tracks =
                    lastfm::fetch_tracks(&user, &config.api_key, page, limit, from, to).await?;

                println!("Saving {} tracks to file...", &tracks.len());
                files::save_to_csv(tracks, &user.name);
            }

            println!("\nDone!");
        }
    }

    Ok(())
}
