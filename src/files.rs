use std::env;

use crate::models::SavedTrack;
use crate::types::{AllSavedTracks, AllTracks};

pub fn save_to_csv(tracks: AllTracks, username: &str) {
    let current_dir = env::current_dir().unwrap();
    let file = current_dir.join(format!("{}.csv", username));

    let tracks: AllSavedTracks = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();

    let mut wtr = csv::Writer::from_path(file).unwrap();

    for track in tracks {
        wtr.serialize(track).unwrap();
    }
    wtr.flush().unwrap();
}

pub fn append_to_csv(tracks: AllTracks, saved_tracks: &mut AllSavedTracks, username: &str) {
    let current_dir = env::current_dir().expect("Error fetching current directory from environment");
    let file = current_dir.join(format!("{}.csv", username));

    let mut new_tracks: AllSavedTracks = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();
    saved_tracks.append(&mut new_tracks);
    saved_tracks.sort_unstable_by_key(|t| t.timestamp_utc);
    saved_tracks.reverse();

    let mut wtr = csv::Writer::from_path(file).expect("Error creating csv writer");
    for track in saved_tracks {
        wtr.serialize(track).expect("Error serializing track");
    }
    wtr.flush().expect("Error flushing csv writer");
}

pub fn load_from_csv(username: &str) -> AllSavedTracks {
    let current_dir = env::current_dir().expect("Error fetching current directory from environment");
    let file = current_dir.join(format!("{}.csv", username));

    let mut rdr = csv::Reader::from_path(file).expect("Error creating csv reader");

    rdr.deserialize().map(|result| {
        let saved_track: SavedTrack = result.expect("Error deserializing csv record");
        saved_track
    }).collect::<AllSavedTracks>()
}
