use std::sync::Mutex;
use std::thread;

use anyhow::Result;

use crate::models::{Attr, RecentTracksResponse, Track, User, UserResponse};
use crate::utils;
use indicatif::ProgressBar;

fn build_request_url(username: &str, api_key: &str, method: &str) -> String {
    format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json",
            method = method,
            user = username,
            api_key = api_key)
}

pub async fn fetch_profile(username: &str, api_key: &str) -> Result<User> {
    let url = build_request_url(username, api_key, "user.getinfo");
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
    use rayon::prelude::*;

    let tracks: Mutex<Vec<Track>> = Mutex::new(Vec::new());

    println!("\nFetching metadata...");
    let metadata = fetch_tracks_metadata(user, api_key, page, limit, from, to).await?;
    let pages = (page..=metadata.total_pages()).collect::<Vec<i32>>();
    let bar = ProgressBar::new(pages.len() as u64);
    println!(
        "Fetching {} tracks from {} pages",
        &metadata.total_tracks(),
        &metadata.total_pages()
    );

    println!("\nFetching tracks...");
    pages.par_iter().for_each(|page| {
        bar.inc(1);

        let url = format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json&extended=1&page={page}&limit={limit}&from={from}&to={to}",
                          method = "user.getRecentTracks",
                          user = user.name,
                          api_key = api_key,
                          page = page,
                          limit = limit,
                          from = from,
                          to = to,
        );

        let recent_tracks_response: RecentTracksResponse = reqwest::blocking::get(&url).unwrap().json().unwrap();
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
