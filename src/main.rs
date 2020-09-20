use clap::Clap;

use rustfm::app::Opts;

fn main() {
    let opts = Opts::parse();

    println!("Your last.fm username is `{}`", &opts.username)
}
