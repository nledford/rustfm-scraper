use std::time;

use rand::Rng;

pub fn gen_random_duration() -> time::Duration {
    let mut rng = rand::thread_rng();

    let duration = rng.gen_range(100, 500);
    time::Duration::from_millis(duration)
}

pub fn get_current_unix_timestamp() -> i64 {
    chrono::offset::Utc::now().timestamp()
}
