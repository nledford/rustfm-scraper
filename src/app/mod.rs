pub mod fetch;
pub mod stats;

use clap::Clap;

/// Provides commands to download your listening history from Last.fm and export it to several formats
#[derive(Clap)]
#[clap(version = "1.0", author = "Nathaniel Ledford <nate@nateledford.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Fetch(Fetch),
    Stats(Stats),
}

// TODO add config subcommand and allow a default file location to be set

/// A subcommand for fetching your listening history from Last.fm
#[derive(Clap)]
pub struct Fetch {
    /// A Last.fm username
    #[clap(short)]
    pub username: Option<String>,
    /// The page number to fetch. Defaults to first page.
    #[clap(short)]
    pub page: Option<i32>,
    /// The number of results to fetch per page. Defaults to 50. Maximum is 200.
    #[clap(short)]
    pub limit: Option<i32>,
    /// Beginning timestamp of a range - only display scrobbles after this time, in UNIX timestamp format
    #[clap(short)]
    pub from: Option<i64>,
    /// End timestamp of a range - only display scrobbles before this time, in UNIX timestamp format
    #[clap(short)]
    pub to: Option<i64>,
    /// Create new file, rather than append tracks to an existing file
    #[clap(short = 'n', takes_value = false)]
    pub new_file: bool,
    /// Fetches all new tracks from beginning of current day, rather than since last saved track
    #[clap(long, takes_value = false)]
    pub current_day: bool,
}

/// A subcommand for calculating stats from a saved file
#[derive(Clap)]
pub struct Stats {
    /// A Last.fm username
    #[clap(short)]
    pub username: Option<String>,
}
