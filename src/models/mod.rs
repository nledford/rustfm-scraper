//! Represents data retrieved from the Last.fm API and stored locally in files

use serde::Deserialize;

pub mod recent_tracks;
pub mod saved_scrobbles;
pub mod user;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success(T),
    Failure(ErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: i32,
    pub message: String,
}
