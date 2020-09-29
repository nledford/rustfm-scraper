use std::env;
use std::path::PathBuf;

use anyhow::Result;

use crate::models::{SavedScrobble, SavedScrobbles, Track};
use crate::types::{AllSavedScrobbles, AllTracks};

fn sort_saved_scrobbles(saved_scrobbles: &mut AllSavedScrobbles) {
    saved_scrobbles.sort_unstable_by_key(|s| s.timestamp_utc);
    saved_scrobbles.dedup_by_key(|s| s.calculate_hash());
    saved_scrobbles.reverse();
}

fn build_csv_path(username: &str) -> PathBuf {
    let current_dir =
        env::current_dir().expect("Error fetching current directory from environment");
    current_dir.join(format!("{}.csv", username))
}

pub fn check_if_csv_exists(username: &str) -> bool {
    let file = build_csv_path(username);

    file.exists()
}

pub fn save_to_csv(scrobbles: &[Track], username: &str) -> Result<i32> {
    let file = build_csv_path(username);

    let mut scrobbles = SavedScrobble::from_scrobbles(scrobbles);
    sort_saved_scrobbles(&mut scrobbles);

    let mut wtr = csv::Writer::from_path(file).unwrap();

    for scrobble in &scrobbles {
        wtr.serialize(scrobble).unwrap();
    }
    wtr.flush().unwrap();

    Ok(scrobbles.len() as i32)
}

pub fn append_to_csv(
    scrobbles: AllTracks,
    saved_scrobbles: &mut SavedScrobbles,
    username: &str,
) -> Result<i32> {
    let file = build_csv_path(username);

    saved_scrobbles.append_new_scrobbles(scrobbles);

    let new_total_scrobbles = saved_scrobbles.total_saved_scrobbles();

    let mut wtr = csv::Writer::from_path(file).expect("Error creating csv writer");
    saved_scrobbles.to_csv_writer(&mut wtr);
    wtr.flush().expect("Error flushing csv writer");

    Ok(new_total_scrobbles)
}

pub fn load_from_csv(username: &str) -> SavedScrobbles {
    println!("Loading saved scrobbles from `{}.csv`...", username);

    let file = build_csv_path(username);

    let mut rdr = csv::Reader::from_path(file).expect("Error creating csv reader");
    let saved_scrobbles = SavedScrobbles::from_csv_reader(&mut rdr);

    println!(
        "{} saved scrobbles retrieved from file\n",
        &saved_scrobbles.total_saved_scrobbles()
    );

    saved_scrobbles
}
