use clap::Clap;

/// Downloads all of your scrobbles from last.fm
#[derive(Clap)]
#[clap(version = "1.0", author = "Nathaniel Ledford <nate@nateledford.com>")]
pub struct Opts {
    /// Your last.fm username
    pub username: String,
}
