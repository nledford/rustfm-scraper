use std::time;

use anyhow::Result;
use async_recursion::async_recursion;
use futures::prelude::*;
use indicatif::ProgressBar;

use crate::lastfm;
use crate::models::recent_tracks::{Attr, RecentTracksResponse, Track};
use crate::models::user::User;

// REFERENCE: https://stackoverflow.com/a/51047786
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
        println!(
            "Fetching {} tracks from one page...",
            metadata.total_tracks()
        );
    } else {
        println!(
            "Fetching {} tracks from {} pages...",
            metadata.total_tracks(),
            metadata.total_pages()
        );
    }

    println!("\nFetching tracks...");

    let urls: Vec<String> = (1..=metadata.total_pages())
        .map(|p| lastfm::build_request_url(user, api_key, p, limit, from, to))
        .collect();

    let bar = ProgressBar::new(metadata.total_pages() as u64);
    let mut tracks = stream::iter(urls)
        .map(|url| {
            let client = reqwest::Client::builder()
                .timeout(time::Duration::from_secs(60))
                .build()
                .unwrap();

            let task = tokio::spawn(async move { fetch_page(&url, &client).await });

            bar.inc(1);

            task
        })
        .buffer_unordered(lastfm::PARALLEL_REQUESTS)
        .map(|t| t.unwrap())
        .collect::<Vec<Vec<Track>>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Track>>();

    bar.finish();

    println!("Removing `Now Playing` track, if one exists...");
    if tracks.iter().any(|t| t.now_playing()) {
        tracks.retain(|t| !t.now_playing())
    }

    Ok(tracks)
}

#[async_recursion]
pub async fn fetch_page(url: &str, client: &reqwest::Client) -> Vec<Track> {
    let resp = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(_) => return fetch_page(url, client).await,
    };

    let recent_tracks = match resp.json::<RecentTracksResponse>().await {
        Ok(rtr) => rtr.recent_tracks.tracks,
        Err(_) => return fetch_page(url, client).await,
    };

    recent_tracks
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