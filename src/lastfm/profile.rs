use anyhow::Result;

use crate::models::{User, UserResponse};

pub async fn fetch_profile(username: &str, api_key: &str) -> Result<User> {
    let url = format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json",
                      method = "user.getInfo",
                      user = username,
                      api_key = api_key);
    let user_response = reqwest::get(&url).await?.json::<UserResponse>().await?;
    Ok(user_response.user)
}
