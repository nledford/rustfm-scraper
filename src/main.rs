use std::io;
use std::process;
use std::sync::Mutex;

use anyhow::Result;
use clap::Clap;
use math::round;

use rustfm::app::Opts;
use rustfm::config::Config;
use rustfm::models::{RecentTracksResponse, Track, User, UserResponse};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    // First check if a config file has been created
    // If not, prompt the user to create one
    if !Config::check_if_config_exists() {
        println!("Config file does not exist. Creating one now...");

        println!("Enter your Last.fm API key: ");
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).expect("Failed to read api key");

        let config = Config::new(api_key);
        config.save_config()?;

        process::exit(0);
    }

    let config = Config::load_config()?;

    let user = fetch_profile(&opts.username, &config.api_key).await?;

    println!("Username: {}", user.name);
    println!("Number of scrobbles: {}", user.play_count());

    println!("\nFetching tracks...");
    let tracks = fetch_tracks(&user, &config.api_key, 1, 200, 0).await?;

    println!("\nTotal Tracks: {} (Expected {})", &tracks.len(), &user.play_count());

    println!("\nDone!");

    Ok(())
}

fn build_request_url(username: &str, api_key: &str, method: &str) -> String {
    format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json",
            method = method,
            user = username,
            api_key = api_key)
}

async fn fetch_profile(username: &str, api_key: &str) -> Result<User> {
    let url = build_request_url(username, api_key, "user.getinfo");
    let user_response = reqwest::get(&url).await?.json::<UserResponse>().await?;
    Ok(user_response.user)
}


async fn fetch_tracks(user: &User, api_key: &str, page: i32, limit: i32, from: i64) -> Result<Vec<Track>> {
    use rayon::prelude::*;

    let tracks: Mutex<Vec<Track>> = Mutex::new(Vec::new());

    let number_of_pages = round::ceil(user.play_count() as f64 / limit as f64, 0) as i32;

    (page..=number_of_pages).into_par_iter()
        .for_each(|page| {
            print!("\rFetching page {} of {}...", page, number_of_pages);

            let url = format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json&page={page}&limit={limit}&from={from}",
                              method = "user.getRecentTracks",
                              user = user.name,
                              api_key = api_key,
                              page = page,
                              limit = limit,
                              from = from);

            let recent_tracks_response: RecentTracksResponse = reqwest::blocking::get(&url).unwrap().json().unwrap();
            let mut recent_tracks = recent_tracks_response.recent_tracks.tracks;

            let mut db = tracks.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();
            db.append(&mut recent_tracks);
        });

    Ok(tracks.into_inner().unwrap())
}
