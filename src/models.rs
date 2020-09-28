//! Represents data retrieved from the Last.fm API and stored locally in files

use std::collections::hash_map::DefaultHasher;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

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

/// Represents the data that is saved to a file from a given [Track](struct.Track.html)
#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct SavedTrack {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub loved: bool,
    pub datetime_local: DateTime<Local>,
    pub timestamp_utc: i64,
}

impl SavedTrack {
    pub fn from_track(track: &Track) -> Self {
        Self {
            title: track.name.to_string(),
            artist: track.artist.name.to_string(),
            album: track.album.text.to_string(),
            loved: track.loved(),
            datetime_local: track.date().datetime_local(),
            timestamp_utc: track.date().time_stamp(),
        }
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
