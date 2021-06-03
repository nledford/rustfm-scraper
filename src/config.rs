use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    let storage_format = set_storage_format();

    let config = Config::new(
        api_key,
        username,
        storage_format,
    );

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

fn set_storage_format() -> StorageFormat {
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

        storage_format = if selection.as_str() == "1" {
            valid_selection = true;
            Some(StorageFormat::Csv)
        } else if selection.as_str() == "2" {
            valid_selection = true;
            Some(StorageFormat::Json)
        } else if selection.as_str() == "3" {
            valid_selection = true;
            Some(StorageFormat::Sqlite)
        } else {
            println!("Invalid format selected. Valid values are 1, 2, or 3. Please try again.");
            None
        };
    }

    // We can safely unwrap here because `storage_format` should have a value
    storage_format.unwrap()
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
    pub fn new(api_key: String,
               default_username: String,
               storage_format: StorageFormat,
    ) -> Self {
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
}

fn build_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();

    let path = Path::new(config_dir.as_path());
    let path = path.join(CRATE_NAME);

    fs::create_dir_all(&path).expect("Path could not be created");

    path.join("config.json")
}
