use anyhow::Result;

use crate::files;
use crate::models::recent_tracks::Track;
use crate::models::saved_scrobbles::SavedScrobbles;

pub fn save_to_json(scrobbles: &[Track], username: &str) -> Result<i32> {
    let file = files::build_file_path(username, "json");
    let scrobbles = SavedScrobbles::from_scrobbles(scrobbles);
    scrobbles.save_as_json(&file)?;
    Ok(scrobbles.total_saved_scrobbles())
}

pub fn append_to_json(
    scrobbles: &[Track],
    saved_scrobbles: &mut SavedScrobbles,
    username: &str,
) -> Result<i32> {
    let file = files::build_file_path(username, "json");
    saved_scrobbles.append_new_scrobbles(scrobbles);
    saved_scrobbles.save_as_json(&file)?;
    Ok(saved_scrobbles.total_saved_scrobbles())
}

pub fn load_from_json(username: &str) -> Result<SavedScrobbles> {
    let file = files::build_file_path(username, "json");
    Ok(SavedScrobbles::load_from_json(&file)?)
}
