use std::sync::Mutex;
use std::thread;

use anyhow::Result;
use indicatif::ProgressBar;
use reqwest::blocking::Client;

use crate::models::{Attr, RecentTracksResponse, Track, User, UserResponse};
use crate::utils;

fn build_request_url(user: &User, api_key: &str, page: i32, limit: i32, from: i64, to: i64) -> String {
    format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json&extended=1&page={page}&limit={limit}&from={from}&to={to}",
            method = "user.getRecentTracks",
            user = user.name,
            api_key = api_key,
            page = page,
            limit = limit,
            from = from,
            to = to,
    )
}

pub async fn fetch_profile(username: &str, api_key: &str) -> Result<User> {
    let url = format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json",
                      method = "user.getInfo",
                      user = username,
                      api_key = api_key);
    let user_response = reqwest::get(&url).await?.json::<UserResponse>().await?;
    Ok(user_response.user)
}

pub async fn fetch_tracks(
    user: &User,
    api_key: &str,
    page: i32,
    limit: i32,
    from: i64,
    to: i64,
) -> Result<Vec<Track>> {
    println!("\nFetching metadata...");
    let metadata = fetch_tracks_metadata(user, api_key, page, limit, from, to).await?;

    if metadata.single_page() && metadata.single_track() {
        println!("Fetching one new track...");
    } else if metadata.single_page() {
        println!("Fetching {} tracks from one page...", metadata.total_tracks());
    } else {
        println!("Fetching {} tracks from {} pages...", metadata.total_tracks(), metadata.total_pages());
    }

    println!("\nFetching tracks...");
    if metadata.single_page() {
        let url = build_request_url(user, api_key, page, limit, from, to);
        let recent_tracks_response: RecentTracksResponse = reqwest::get(&url).await.unwrap().json().await.unwrap();

        return Ok(recent_tracks_response.recent_tracks.tracks);
    }

    let tracks: Mutex<Vec<Track>> = Mutex::new(Vec::new());

    let pages = (page..=metadata.total_pages()).collect::<Vec<i32>>();
    let bar = ProgressBar::new(pages.len() as u64);
    let client = Client::new();
    pages.iter().for_each(|page| {
        bar.inc(1);

        let url = build_request_url(user, api_key, *page, limit, from, to);
        let recent_tracks_response: RecentTracksResponse = client.get(&url).send().unwrap().json().unwrap();
        let mut recent_tracks = recent_tracks_response.recent_tracks.tracks;

        let mut db = tracks.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();
        db.append(&mut recent_tracks);

        thread::sleep(utils::gen_random_duration());
    });
    bar.finish();

    Ok(tracks.into_inner().unwrap())
}

pub async fn fetch_tracks_metadata(
    user: &User,
    api_key: &str,
    page: i32,
    limit: i32,
    from: i64,
    to: i64,
) -> Result<Attr> {
    let url = format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json&extended=1&page={page}&limit={limit}&from={from}&to={to}",
                      method = "user.getRecentTracks",
                      user = user.name,
                      api_key = api_key,
                      page = page,
                      limit = limit,
                      from = from,
                      to = to,
    );

    let recent_tracks_response: RecentTracksResponse =
        reqwest::get(&url).await.unwrap().json().await.unwrap();

    Ok(recent_tracks_response.recent_tracks.attr)
}
