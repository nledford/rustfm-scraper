use std::io;
use std::process;

use anyhow::Result;
use clap::Clap;

use rustfm::app::Opts;
use rustfm::config::Config;
use rustfm::models::{User, UserResponse};

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

    let user = fetch_profile(opts.username, config.api_key).await?;

    println!("Username: {}", user.name);
    println!("Number of scrobbles: {}", user.play_count());

    Ok(())
}

async fn fetch_profile(username: String, api_key: String) -> Result<User> {
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getinfo&user={user}&api_key={api_key}&format=json",
                      user = username,
                      api_key = api_key);
    let resp = reqwest::get(&url).await?.text().await?;
    println!("{:#?}", &resp);

    let user = reqwest::get(&url).await?.json::<UserResponse>().await?;
    Ok(user.user)
}
