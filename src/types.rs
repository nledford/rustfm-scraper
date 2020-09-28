//! Custom types used throughout the application

use crate::models::{SavedScrobble, Track};

// /// A collection of [Track](../models/struct.Track.html)s.
// /// Typically a single page or the flattened collection of all pages from the Last.fm API.
// pub type Tracks = Vec<Track>;
//
// /// A unflattened collection of track pages from the Last.fm API.
// pub type CollectedTracks = Vec<Tracks>;

pub type Page = Vec<Track>;
pub type AllPages = Vec<Page>;
pub type AllTracks = Vec<Track>;

pub type AllSavedScrobbles = Vec<SavedScrobble>;
