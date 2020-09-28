use std::env;
use std::path::PathBuf;

use anyhow::Result;

use crate::models::{SavedTrack, Track};
use crate::types::AllSavedTracks;

fn sort_saved_tracks(saved_tracks: &mut AllSavedTracks) {
    saved_tracks.sort_unstable_by_key(|t| t.timestamp_utc);
    saved_tracks.dedup_by_key(|t| t.calculate_hash());
    saved_tracks.reverse();
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

pub fn save_to_csv(tracks: &[Track], username: &str) -> Result<i32> {
    let file = build_csv_path(username);

    let mut tracks: AllSavedTracks = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();
    sort_saved_tracks(&mut tracks);

    let mut wtr = csv::Writer::from_path(file).unwrap();

    for track in &tracks {
        wtr.serialize(track).unwrap();
    }
    wtr.flush().unwrap();

    Ok(tracks.len() as i32)
}

pub fn append_to_csv(tracks: &[Track], saved_tracks: &mut AllSavedTracks, username: &str) -> Result<i32> {
    let file = build_csv_path(username);

    let mut new_tracks: AllSavedTracks = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();
    saved_tracks.append(&mut new_tracks);
    sort_saved_tracks(saved_tracks);

    let new_total_scrobbles = saved_tracks.len() as i32;

    let mut wtr = csv::Writer::from_path(file).expect("Error creating csv writer");
    for track in saved_tracks {
        wtr.serialize(track).expect("Error serializing track");
    }
    wtr.flush().expect("Error flushing csv writer");

    Ok(new_total_scrobbles)
}

pub fn load_from_csv(username: &str) -> AllSavedTracks {
    println!("Loading saved tracks from `{}.csv`...", username);

    let file = build_csv_path(username);

    let mut rdr = csv::Reader::from_path(file).expect("Error creating csv reader");

    let mut saved_tracks = rdr
        .deserialize()
        .map(|result| {
            let saved_track: SavedTrack = result.expect("Error deserializing csv record");
            saved_track
        })
        .collect::<AllSavedTracks>();
    sort_saved_tracks(&mut saved_tracks);

    saved_tracks
}
