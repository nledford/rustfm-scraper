use std::env;
use std::path::PathBuf;

pub mod csv;
pub mod json;

fn validate_extension(extension: &str) {
    let valid_extensions = vec!["csv", "json"];

    if !valid_extensions.contains(&extension) {
        panic!(format!("{} is not a valid extension", extension))
    }
}

fn build_file_path(username: &str, extension: &str) -> PathBuf {
    validate_extension(extension);

    let current_dir =
        env::current_dir().expect("Error fetching current directory from environment");
    current_dir.join(format!("{}.{}", username, extension))
}

fn check_if_file_exists(username: &str, extension: &str) -> bool {
    validate_extension(extension);

    let file = build_file_path(username, extension);

    file.exists()
}
