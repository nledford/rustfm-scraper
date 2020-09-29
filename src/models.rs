//! Represents data retrieved from the Last.fm API and stored locally in files

use std::collections::hash_map::DefaultHasher;
use std::fs::File;

use chrono::prelude::*;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

use crate::stats::Stats;
use crate::types::{AllSavedScrobbles, AllTracks};

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct Registered {
    pub unixtime: String,
}

/// Response to a `user.getInfo` request
///
/// Returns information about a user's profile. Certain fields, such as `bootstrap` and `images`,
/// have been omitted from deserialization since they are not used in this application
#[derive(Debug, Deserialize)]
pub struct User {
    /// The total number of playlists the user has created
    playlists: String,

    /// The total number of tracks scrobbled by the user
    #[serde(rename = "playcount")]
    play_count: String,

    /// The user's gender
    pub gender: String,

    /// The user's username
    pub name: String,

    /// Indicates if the user is a subscriber to Last.fm
    pub subscriber: String,

    /// The user's profile URL
    pub url: String,

    /// The user's country
    pub country: String,

    /// The date and time the user registered their profile, represented as a unix timestamp
    ///
    /// See [Registered](struct.Registered.html)
    pub registered: Registered,

    /// The user's profile type. Could be a normal user or a staff user.
    #[serde(rename = "type")]
    pub user_type: String,

    /// The user's age
    pub age: String,
    // pub bootstrap: String,
    /// The user's real name, if provided
    #[serde(rename = "realname")]
    pub real_name: String,
}

impl User {
    /// Get the number of playlists created by the user
    pub fn playlists(&self) -> i32 {
        self.playlists.parse().unwrap()
    }

    /// Get the total number of scrobbles by the user
    pub fn play_count(&self) -> i32 {
        self.play_count.parse().unwrap()
    }
}

// ############################################################################

#[derive(Debug, Deserialize)]
pub struct RecentTracksResponse {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: RecentTracks,
}

#[derive(Debug, Deserialize)]
pub struct RecentTracks {
    #[serde(rename = "@attr")]
    pub attr: Attr,
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Attr {
    page: String,
    #[serde(rename = "perPage")]
    per_page: String,
    user: String,
    total: String,
    #[serde(rename = "totalPages")]
    total_pages: String,
}

impl Attr {
    pub fn page(&self) -> i32 {
        self.page.parse().unwrap_or(0)
    }

    pub fn per_page(&self) -> i32 {
        self.per_page.parse().unwrap_or(0)
    }

    pub fn total_tracks(&self) -> i32 {
        self.total.parse().unwrap_or(0)
    }

    pub fn total_pages(&self) -> i32 {
        self.total_pages.parse().unwrap_or(0)
    }

    pub fn last_page(&self) -> bool {
        self.page() == self.total_pages()
    }

    pub fn single_page(&self) -> bool {
        self.total_pages() == 1
    }

    pub fn single_track(&self) -> bool {
        self.total_tracks() == 1
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "@attr")]
    pub attr: Option<TrackAttr>,
    pub artist: Artist,
    pub album: Album,
    // Skip `image`
    pub streamable: String,
    pub date: Option<Date>,
    pub name: String,
    pub mbid: String,
    loved: String,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.combined_title().eq(&other.combined_title())
    }
}

impl Track {
    pub fn combined_title(&self) -> String {
        format!("{} - {} - {}", self.name, self.artist.name, self.album.text)
    }

    pub fn loved(&self) -> bool {
        self.loved == "1"
    }

    pub fn now_playing(&self) -> bool {
        match &self.attr {
            Some(attr) => attr.now_playing == "true",
            None => false,
        }
    }

    pub fn date(&self) -> Date {
        self.date.clone().unwrap()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TrackAttr {
    #[serde(rename = "nowplaying")]
    now_playing: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Artist {
    pub url: String,
    pub mbid: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Album {
    pub mbid: String,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Date {
    uts: String,
    #[serde(rename = "#text")]
    text: String,
}

impl Date {
    pub fn time_stamp(&self) -> i64 {
        let timestamp: i64 = self.uts.parse().unwrap();
        let dt = NaiveDateTime::from_timestamp(timestamp, 0);

        dt.timestamp()
    }

    pub fn datetime_utc(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.time_stamp(), 0), Utc)
    }

    pub fn datetime_local(&self) -> DateTime<Local> {
        let dt = self.datetime_utc().naive_utc();

        Local::from_utc_datetime(&Local, &dt)
    }
}

// ############################################################################

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
    pub fn new(saved_scrobbles: AllSavedScrobbles) -> Self {
        let mut saved_scrobbles = Self { saved_scrobbles };
        saved_scrobbles.sort();
        saved_scrobbles
    }

    pub fn from_scrobbles(scrobbles: AllTracks) -> Self {
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
            .collect::<AllSavedScrobbles>();
        let mut saved_scrobbles = SavedScrobbles::new(saved_scrobbles);
        saved_scrobbles.sort();
        saved_scrobbles
    }

    pub fn to_csv_writer(&self, wtr: &mut Writer<File>) {
        for scrobble in &self.saved_scrobbles {
            wtr.serialize(scrobble).expect("Error serializing scrobble")
        }
    }

    pub fn append_new_scrobbles(&mut self, new_scrobbles: AllTracks) {
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

    fn convert_scrobbles(scrobbles: AllTracks) -> Vec<SavedScrobble> {
        scrobbles
            .iter()
            .map(|scrobble| SavedScrobble::from_scrobble(scrobble))
            .collect::<AllSavedScrobbles>()
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

    pub fn from_scrobbles(scrobbles: &[Track]) -> AllSavedScrobbles {
        scrobbles
            .iter()
            .map(|scrobble| SavedScrobble::from_scrobble(scrobble))
            .collect::<AllSavedScrobbles>()
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
