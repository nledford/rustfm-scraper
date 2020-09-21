use std::sync::Mutex;

use anyhow::Result;
use futures::prelude::*;
use indicatif::ProgressBar;

use crate::models::{Attr, RecentTracksResponse, Track, User, UserResponse};

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

    let tracks: Mutex<Vec<Track>> = Mutex::new(Vec::new());
    let urls: Vec<String> = (1..=metadata.total_pages()).map(|p| {
        build_request_url(user, api_key, p, limit, from, to)
    }).collect();

    let bar = ProgressBar::new(metadata.total_pages() as u64);
    let responses = stream::iter(urls)
        .map(|url| {
            let client = reqwest::Client::new();
            bar.inc(1);
            tokio::spawn(async move {
                let rtr: RecentTracksResponse = client.get(&url).send().await.unwrap().json().await.unwrap();
                rtr.recent_tracks.tracks
            })
        }).buffer_unordered(12);

    responses.for_each(|t| async {
        let mut recent_tracks = t.unwrap();
        let mut db = tracks.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();
        db.append(&mut recent_tracks);
    }).await;

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
