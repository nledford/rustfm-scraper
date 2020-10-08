#![doc(html_root_url = "https://docs.rs/rustfm-scraper/0.1.0")]
//! # rustfm-scraper
//!
//! Command line application that uses the Last.fm API (specifically the
//! [`user.getRecentTracks`](https://www.last.fm/api/show/user.getRecentTracks) endpoint)
//! to download the listening history for a given user and saves the data locally to a file.
//!
//! In order to run this application, you will need to generate your own [API Key](https://www.last.fm/api).
//!
//! # Example usage
//!
//! To download the entire listening history of the [LAST.HQ](https://www.last.fm/user/LAST.HQ) profile,
//! use the `fetch` command. This will call the `user.getRecentTracks` endpoint using the default values
//! for each parameter.
//!
//! ```shell
//! rustfm-scraper fetch LAST.HQ
//! ```
//!

pub mod app;
pub mod config;
pub mod errors;
pub mod files;
pub mod lastfm;
pub mod models;
pub mod stats;
pub mod utils;
