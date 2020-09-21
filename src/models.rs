use chrono::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Registered {
    pub unixtime: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    playlists: String,
    playcount: String,
    pub gender: String,
    pub name: String,
    pub subscriber: String,
    pub url: String,
    pub country: String,
    pub registered: Registered,
    #[serde(rename = "type")]
    pub user_type: String,
    pub age: String,
    pub bootstrap: String,
    pub realname: String,
}

impl User {
    pub fn playlists(&self) -> i32 {
        self.playlists.parse().unwrap()
    }

    pub fn play_count(&self) -> i32 {
        self.playcount.parse().unwrap()
    }
}

// ############################################################################

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RecentTracksResponse {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: RecentTracks,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RecentTracks {
    #[serde(rename = "@attr")]
    pub attr: Attr,
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

    pub fn total_pages(&self) -> i32 {
        self.total_pages.parse().unwrap_or(0)
    }

    pub fn last_page(&self) -> bool {
        self.page() == self.total_pages()
    }
}

// TODO add field when listening to track
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub artist: Artist,
    pub album: Album,
    // Skip `image`
    pub streamable: String,
    pub date: Date,
    pub name: String,
    pub mbid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    pub url: String,
    pub mbid: String,
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Album {
    pub mbid: String,
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Date {
    uts: String,
    #[serde(rename = "#text")]
    text: String,
}

impl Date {
    pub fn time_stamp(&self) -> NaiveDateTime {
        let timestamp: i64 = self.uts.parse().unwrap();
        let dt = NaiveDateTime::from_timestamp(timestamp, 0);

        dt
    }
}
