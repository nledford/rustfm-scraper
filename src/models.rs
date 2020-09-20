
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserResponse {
    pub user: User
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Registered {
    pub unixtime: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    playlists: String,
    playcount: String,
    pub gender: String,
    pub name: String,
    pub subscriber: String,
    pub url: String,
    pub country: String,
    pub registered: Registered,
    #[serde(rename = "type")]
    pub user_type: String,
    pub age: String,
    pub bootstrap: String,
    pub realname: String,
}

impl User {
    pub fn playlists(&self) -> i32 {
        self.playlists.parse().unwrap()
    }

    pub fn play_count(&self) -> i32 {
        self.playcount.parse().unwrap()
    }
}
