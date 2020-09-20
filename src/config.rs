use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use anyhow::Result;

fn build_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();

    let path = Path::new(config_dir.as_path());
    let path = path.join("rustfm");

    fs::create_dir_all(&path).expect("Path could not be created");

    path.join("config.json")
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn check_if_config_exists() -> bool {
        Path::exists(build_config_path().as_path())
    }

    pub fn load_config() -> Result<Self> {
        let file = File::open(build_config_path())?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;

        Ok(config)
    }

    pub fn save_config(&self) -> Result<()> {
        let file = File::create(build_config_path())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;

        Ok(())
    }
}
