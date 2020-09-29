use crate::models::User;

pub mod profile;
pub mod recently_played;

const PARALLEL_REQUESTS: usize = 50;

fn build_request_url(
    user: &User,
    api_key: &str,
    page: i32,
    limit: i32,
    from: i64,
    to: i64,
) -> String {
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



