/// Retrieves the current UTC date and time as a unix timestamp in seconds
pub fn get_current_unix_timestamp() -> i64 {
    chrono::offset::Utc::now().timestamp()
}
