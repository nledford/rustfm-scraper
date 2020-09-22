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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    #[serde(rename = "@attr")]
    pub attr: Option<TrackAttr>,
    pub artist: Artist,
    pub album: Album,
    // Skip `image`
    pub streamable: String,
    pub date: Date,
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
        if self.loved == "0" {
            false
        } else {
            true
        }
    }

    pub fn now_playing(&self) -> bool {
        match &self.attr {
            Some(attr) => {
                if attr.now_playing == "true" {
                    true
                } else {
                    false
                }
            },
            None => false
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackAttr {
    #[serde(rename = "nowplaying")]
    now_playing: String
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
    pub fn time_stamp(&self) -> i64 {
        let timestamp: i64 = self.uts.parse().unwrap();
        let dt = NaiveDateTime::from_timestamp(timestamp, 0);

        dt.timestamp()
    }

    pub fn datetime_utc(&self) -> DateTime<Utc> {
        let dt =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.time_stamp(), 0), Utc);

        dt
    }

    pub fn datetime_local(&self) -> DateTime<Local> {
        let dt = self.datetime_utc().naive_utc();

        Local::from_utc_datetime(&Local, &dt)
    }
}

// ############################################################################

#[derive(serde::Serialize, serde::Deserialize)]
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
            datetime_local: track.date.datetime_local(),
            timestamp_utc: track.date.time_stamp(),
        }
    }

    pub fn combined_title(&self) -> String {
        format!("{} - {} - {}", self.title, self.artist, self.album)
    }
}
