use std::env;

use crate::models::SavedTrack;
use crate::types::{AllSavedTracks, AllTracks};

fn sort_saved_tracks(saved_tracks: &mut AllSavedTracks) {
    saved_tracks.sort_unstable_by_key(|t| t.timestamp_utc);
    saved_tracks.dedup_by_key(|t| t.calculate_hash());
    saved_tracks.reverse();
}

pub fn save_to_csv(tracks: AllTracks, username: &str) {
    let current_dir = env::current_dir().unwrap();
    let file = current_dir.join(format!("{}.csv", username));

    let mut tracks: AllSavedTracks = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();
    sort_saved_tracks(&mut tracks);

    let mut wtr = csv::Writer::from_path(file).unwrap();

    for track in tracks {
        wtr.serialize(track).unwrap();
    }
    wtr.flush().unwrap();
}

pub fn append_to_csv(tracks: AllTracks, saved_tracks: &mut AllSavedTracks, username: &str) {
    let current_dir =
        env::current_dir().expect("Error fetching current directory from environment");
    let file = current_dir.join(format!("{}.csv", username));

    let mut new_tracks: AllSavedTracks = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();
    saved_tracks.append(&mut new_tracks);
    sort_saved_tracks(saved_tracks);

    let mut wtr = csv::Writer::from_path(file).expect("Error creating csv writer");
    for track in saved_tracks {
        wtr.serialize(track).expect("Error serializing track");
    }
    wtr.flush().expect("Error flushing csv writer");
}

pub fn load_from_csv(username: &str) -> AllSavedTracks {
    let current_dir =
        env::current_dir().expect("Error fetching current directory from environment");
    let file = current_dir.join(format!("{}.csv", username));

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
