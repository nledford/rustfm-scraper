use std::collections::hash_map::DefaultHasher;
use std::fs::File;

use chrono::prelude::*;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

use crate::models::recent_tracks::Track;
use crate::stats::Stats;

pub struct SavedScrobbles {
    saved_scrobbles: Vec<SavedScrobble>,
}

impl Default for SavedScrobbles {
    fn default() -> Self {
        Self {
            saved_scrobbles: Vec::default(),
        }
    }
}

impl SavedScrobbles {
    pub fn new(saved_scrobbles: Vec<SavedScrobble>) -> Self {
        let mut saved_scrobbles = Self { saved_scrobbles };
        saved_scrobbles.sort();
        saved_scrobbles
    }

    pub fn from_scrobbles(scrobbles: &[Track]) -> Self {
        let mut saved_scrobbles = Self {
            saved_scrobbles: SavedScrobbles::convert_scrobbles(scrobbles),
        };
        saved_scrobbles.sort();
        saved_scrobbles
    }

    pub fn from_csv_reader(rdr: &mut Reader<File>) -> Self {
        let saved_scrobbles = rdr
            .deserialize::<SavedScrobble>()
            .map(|scrobble| scrobble.expect("Error deserializing scrobble"))
            .collect::<Vec<SavedScrobble>>();
        let mut saved_scrobbles = SavedScrobbles::new(saved_scrobbles);
        saved_scrobbles.sort();
        saved_scrobbles
    }

    pub fn to_csv_writer(&self, wtr: &mut Writer<File>) {
        for scrobble in &self.saved_scrobbles {
            wtr.serialize(scrobble).expect("Error serializing scrobble")
        }
    }

    pub fn append_new_scrobbles(&mut self, new_scrobbles: &[Track]) {
        let mut new_saved_scrobbles = SavedScrobbles::convert_scrobbles(new_scrobbles);
        self.saved_scrobbles.append(&mut new_saved_scrobbles);
        self.sort()
    }

    pub fn generate_stats(&self) -> Stats {
        Stats::new(&self.saved_scrobbles)
    }

    pub fn most_recent_scrobble(&self) -> Option<&SavedScrobble> {
        self.saved_scrobbles.first()
    }

    pub fn total_saved_scrobbles(&self) -> i32 {
        self.saved_scrobbles.len() as i32
    }

    fn sort(&mut self) {
        self.saved_scrobbles
            .sort_unstable_by_key(|s| s.timestamp_utc);
        self.saved_scrobbles.dedup_by_key(|s| s.calculate_hash());
        self.saved_scrobbles.reverse();
    }

    fn convert_scrobbles(scrobbles: &[Track]) -> Vec<SavedScrobble> {
        scrobbles
            .iter()
            .map(|scrobble| SavedScrobble::from_scrobble(scrobble))
            .collect::<Vec<SavedScrobble>>()
    }
}

/// Represents the data that is saved to a file from a given [Track](struct.Track.html)
#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct SavedScrobble {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub loved: bool,
    pub datetime_local: DateTime<Local>,
    pub timestamp_utc: i64,
}

impl SavedScrobble {
    pub fn from_scrobble(scrobble: &Track) -> Self {
        Self {
            title: scrobble.name.to_string(),
            artist: scrobble.artist.name.to_string(),
            album: scrobble.album.text.to_string(),
            loved: scrobble.loved(),
            datetime_local: scrobble.date().datetime_local(),
            timestamp_utc: scrobble.date().time_stamp(),
        }
    }

    pub fn from_scrobbles(scrobbles: &[Track]) -> Vec<SavedScrobble> {
        scrobbles
            .iter()
            .map(|scrobble| SavedScrobble::from_scrobble(scrobble))
            .collect::<Vec<SavedScrobble>>()
    }

    pub fn date(&self) -> NaiveDate {
        self.datetime_local.naive_local().date()
    }

    pub fn time(&self) -> NaiveTime {
        self.datetime_local.naive_local().time()
    }

    pub fn month_year(&self) -> String {
        self.date().format("%B-%Y").to_string()
    }

    pub fn song_artist(&self) -> String {
        format!("{} - {}", self.title, self.artist)
    }

    pub fn artist_album(&self) -> String {
        format!("{} - {}", self.artist, self.album)
    }

    pub fn combined_title(&self) -> String {
        format!("{} - {} - {}", self.title, self.artist, self.album)
    }

    pub fn calculate_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};

        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}
