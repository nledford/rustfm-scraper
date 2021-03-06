use chrono::prelude::*;
use serde::Deserialize;

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
