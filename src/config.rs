use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::data::db;

static CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

#[derive(Deserialize, Serialize)]
pub enum StorageFormat {
    Csv,
    Json,
    Sqlite,
}

pub fn initialize_config() -> Result<()> {
    println!("Config file does not exist. Creating one now...");

    let api_key = set_api_key();
    let username = set_username();
    let storage_format = set_storage_format()?;

    let config = Config::new(api_key, username, storage_format);

    config.save_config()
}

/// Allows the user to change settings in the configuration file
pub fn update_config() -> Result<()> {
    let mut config = Config::load_config()?;

    let mut api_key = config.api_key;
    let mut username = config.default_username;
    let mut storage_format = config.storage_format;

    let mut choice = String::new();

    println!("Update API key? (y/n)");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read user selection");

    if choice.trim() == "y" {
        api_key = set_api_key();
    }

    println!("Update default username? (y/n)");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read user selection");

    if choice.trim() == "y" {
        username = set_username();
    }

    println!("Update storage format? (y/n)");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read user selection");

    if choice.trim() == "y" {
        storage_format = set_storage_format()?;
    }

    config = Config::new(api_key, username, storage_format);
    config.save_config()
}

fn set_api_key() -> String {
    println!("Enter your Last.fm API key: ");
    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .expect("Failed to read api key");

    api_key.trim().to_string()
}

fn set_username() -> String {
    println!("Enter your Last.fm username:");
    println!(
        "(You can retrieve the listening history for a different Last.fm user with the `-u` flag.)"
    );
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    username.trim().to_string()
}

fn set_storage_format() -> Result<StorageFormat> {
    let mut valid_selection = false;
    let mut selection = String::new();
    let mut storage_format = None;

    while !valid_selection {
        println!("Select how you would like to save your data: ");
        println!("1. CSV file");
        println!("2. JSON file");
        println!("3. Sqlite database");

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read selection");

        let trimmed = selection.trim();

        storage_format = if trimmed == "1" {
            valid_selection = true;
            Some(StorageFormat::Csv)
        } else if trimmed == "2" {
            valid_selection = true;
            Some(StorageFormat::Json)
        } else if trimmed == "3" {
            valid_selection = true;
            Some(StorageFormat::Sqlite)
        } else {
            println!("Invalid format selected. Valid values are 1, 2, or 3. Please try again.");
            None
        };
    }

    // We can safely unwrap here because `storage_format` should have a value
    let storage_format = storage_format.unwrap();

    if let StorageFormat::Sqlite = storage_format {
        println!("Sqlite storage format was selected. Building database now...");
        db::build_sqlite_database()?;
    }

    Ok(storage_format)
}

pub fn check_if_config_exists() -> bool {
    Path::exists(build_config_path().as_path())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub api_key: String,
    pub default_username: String,
    pub storage_format: StorageFormat,
}

impl Config {
    pub fn new(api_key: String, default_username: String, storage_format: StorageFormat) -> Self {
        Self {
            api_key,
            default_username,
            storage_format,
        }
    }

    pub fn load_config() -> Result<Self> {
        let file = File::open(build_config_path())?;
        let reader = BufReader::new(file);

        let config = match serde_json::from_reader(reader) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("An error occurred with your config file. Please create a new one.");
                initialize_config()?;
                Config::load_config()?
            }
        };

        Ok(config)
    }

    pub fn save_config(&self) -> Result<()> {
        let file = File::create(build_config_path())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;

        Ok(())
    }

    /// Deletes the configuration file
    pub fn delete_config(&self) -> Result<()> {
        let config_path = build_config_path();
        fs::remove_file(config_path)?;
        Ok(())
    }

    /// Prints the contents of the configuration file to the console.
    pub fn print_config(&self, full_config: bool) {
        let storage_format = match self.storage_format {
            StorageFormat::Csv => "CSV",
            StorageFormat::Json => "JSON",
            StorageFormat::Sqlite => "Sqlite",
        };

        println!("Current Configuration:");
        println!("Default Last.fm username: {}", self.default_username);
        println!("Default storage format: {}", storage_format);

        if full_config {
            println!("Current Last.fm API key: {}", self.api_key);
        }
    }
}

fn build_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();

    let path = Path::new(config_dir.as_path());
    let path = path.join(CRATE_NAME);

    fs::create_dir_all(&path).expect("Path could not be created");

    path.join("config.json")
}
