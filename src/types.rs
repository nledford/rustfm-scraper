//! Custom types used throughout the application

use crate::models::Track;

/// A collection of [Track](../models/struct.Track.html)s.
/// Typically a single page or the flattened collection of all pages from the Last.fm API.
pub type Tracks = Vec<Track>;

/// A unflattened collection of track pages from the Last.fm API.
pub type CollectedTracks = Vec<Tracks>;
