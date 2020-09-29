use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Deserialize)]
pub struct Registered {
    pub unixtime: String,
}

/// Response to a `user.getInfo` request
///
/// Returns information about a user's profile. Certain fields, such as `bootstrap` and `images`,
/// have been omitted from deserialization since they are not used in this application
#[derive(Deserialize)]
pub struct User {
    /// The total number of playlists the user has created
    playlists: String,

    /// The total number of tracks scrobbled by the user
    #[serde(rename = "playcount")]
    play_count: String,

    /// The user's gender
    pub gender: String,

    /// The user's username
    pub name: String,

    /// Indicates if the user is a subscriber to Last.fm
    pub subscriber: String,

    /// The user's profile URL
    pub url: String,

    /// The user's country
    pub country: String,

    /// The date and time the user registered their profile, represented as a unix timestamp
    ///
    /// See [Registered](struct.Registered.html)
    pub registered: Registered,

    /// The user's profile type. Could be a normal user or a staff user.
    #[serde(rename = "type")]
    pub user_type: String,

    /// The user's age
    pub age: String,
    // pub bootstrap: String,
    /// The user's real name, if provided
    #[serde(rename = "realname")]
    pub real_name: String,
}

impl User {
    /// Get the number of playlists created by the user
    pub fn playlists(&self) -> i32 {
        self.playlists.parse().unwrap()
    }

    /// Get the total number of scrobbles by the user
    pub fn play_count(&self) -> i32 {
        self.play_count.parse().unwrap()
    }
}
