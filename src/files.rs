use std::env;

use crate::models::{SavedTrack, Track};

pub fn save_to_csv(tracks: Vec<Track>) {
    let current_dir = env::current_dir().unwrap();
    let file = current_dir.join("tracks.csv");

    let tracks: Vec<SavedTrack> = tracks.iter().map(|t| SavedTrack::from_track(t)).collect();

    let mut wtr = csv::Writer::from_path(file).unwrap();

    for track in tracks {
        wtr.serialize(track).unwrap();
    }
    wtr.flush().unwrap();
}
