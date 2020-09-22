use std::{env, io, process};

use anyhow::Result;
use clap::Clap;

use rustfm::{files, lastfm, utils};
use rustfm::app::{Opts, SubCommand};
use rustfm::config::Config;

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

            let tracks =
                lastfm::fetch_tracks(&user, &config.api_key, page, limit, from, to).await?;

            println!("\nTotal Tracks: {}", &tracks.len());

            println!("Test writing to CSV file...");
            files::save_to_csv(tracks, &user.name);

            println!("\nDone!");
        }
    }

    Ok(())
}
