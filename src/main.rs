use std::env;

use anyhow::Result;
use clap::Clap;

use rustfm_scraper::{app, config};
use rustfm_scraper::app::{Opts, SubCommand};
use rustfm_scraper::config::Config;

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
        SubCommand::Fetch(f) => app::fetch::fetch(f, config).await?,
        SubCommand::Stats(s) => app::stats::stats(s, config)?,
    }

    println!("\nDone!");

    Ok(())
}
