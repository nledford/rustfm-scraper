use anyhow::Result;

use crate::files;
use crate::models::recent_tracks::Track;
use crate::models::saved_scrobbles::SavedScrobbles;

pub fn check_if_csv_exists(username: &str) -> bool {
    files::check_if_file_exists(username, "csv")
}

pub fn save_to_csv(scrobbles: &[Track], username: &str) -> Result<i32> {
    let file = files::build_file_path(username, "csv");

    let scrobbles = SavedScrobbles::from_scrobbles(scrobbles);

    let mut wtr = csv::Writer::from_path(file).unwrap();
    scrobbles.to_csv_writer(&mut wtr);
    wtr.flush().unwrap();

    Ok(scrobbles.total_saved_scrobbles())
}

pub fn append_to_csv(
    scrobbles: &[Track],
    saved_scrobbles: &mut SavedScrobbles,
    username: &str,
) -> Result<i32> {
    let file = files::build_file_path(username, "csv");

    saved_scrobbles.append_new_scrobbles(scrobbles);

    let mut wtr = csv::Writer::from_path(file).expect("Error creating csv writer");
    saved_scrobbles.to_csv_writer(&mut wtr);
    wtr.flush().expect("Error flushing csv writer");

    Ok(saved_scrobbles.total_saved_scrobbles())
}

pub fn load_from_csv(username: &str) -> Result<SavedScrobbles> {
    println!("Loading saved scrobbles from `{}.csv`...", username);

    let file = files::build_file_path(username, "csv");

    let mut rdr = csv::Reader::from_path(file)?;
    let saved_scrobbles = SavedScrobbles::from_csv_reader(&mut rdr);

    println!(
        "{} saved scrobbles retrieved from file\n",
        &saved_scrobbles.total_saved_scrobbles_formatted()
    );

    Ok(saved_scrobbles)
}
