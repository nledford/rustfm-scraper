use std::process;

use clap::Clap;

use rustfm::app::Opts;
use rustfm::config::Config;
use std::io;

use anyhow::Result;

fn main() -> Result<()> {
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

    println!("Your last.fm username is `{}`", &opts.username);

    Ok(())
}
