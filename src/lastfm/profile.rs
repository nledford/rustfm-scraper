use std::process;

use anyhow::Result;

use crate::models::ApiResponse;
use crate::models::user::{User, UserResponse};

pub async fn fetch_profile(username: &str, api_key: &str) -> Result<User> {
    let url = format!("http://ws.audioscrobbler.com/2.0/?method={method}&user={user}&api_key={api_key}&format=json",
                      method = "user.getInfo",
                      user = username,
                      api_key = api_key);
    let user_response = reqwest::get(&url).await?.json::<ApiResponse<UserResponse>>().await?;
    let user = match user_response {
        ApiResponse::Success(user_response) => user_response.user,
        ApiResponse::Failure(error) => {
            println!("An error occurred at Last.fm's API:");
            println!("{}", &error.message);
            process::exit(0);
        }
    };

    Ok(user)
}
