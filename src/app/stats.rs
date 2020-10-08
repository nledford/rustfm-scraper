use anyhow::Result;

use crate::config::Config;
use crate::{app, files};

pub fn stats(s: app::Stats, config: Config) -> Result<()> {
    let username = match s.username {
        Some(username) => username,
        None => config.default_username,
    };

    match files::find_which_file_exists(&username)? {
        Some(_) => true,
        None => {
            println!(
                "No file for `{}` exists. Stats cannot be calculated.",
                &username
            );
            return Ok(());
        }
    };

    let saved_scrobbles = files::load_from_any_file(&username)?;

    println!("Crunching stats for {}...\n", &username);
    let stats = saved_scrobbles.generate_stats();
    stats.print();

    Ok(())
}
