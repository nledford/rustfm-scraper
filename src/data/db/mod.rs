use std::env;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::SqlitePool;

use crate::config::Config;
use std::process::Command;

fn build_connection_string() -> Result<String> {
    let config = Config::load_config()?;
    let connection_string = format!("sqlite:{}.db", config.default_username);

    env::set_var("DATABASE_URL", &connection_string);

    Ok(connection_string)
}

pub fn build_sqlite_database() -> Result<()> {
    // Check if sqlite database already exists
    if Path::new("nateledford.db").exists() {
        // No need to re-create, return early
        return Ok(())
    }

    // We just want to set the environment variable
    let _ = build_connection_string();

    // Create the database
    let _create = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .output()
        .expect("Failed to create database");

    let _migrations = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .output()
        .expect("Failed to run database migrations");

    Ok(())
}

pub async fn get_sqlite_pool() -> Result<SqlitePool> {
    let connection_string = build_connection_string()?;

    let options = SqliteConnectOptions::from_str(&connection_string)?
        .busy_timeout(Duration::from_secs(30))
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}