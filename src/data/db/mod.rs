use std::env;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::SqlitePool;

use crate::config::Config;
use crate::models::saved_scrobbles::{SavedScrobble, SavedScrobbles};

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
        return Ok(());
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

pub async fn insert_scrobbles(scrobbles: SavedScrobbles, pool: &SqlitePool) -> Result<i32> {
    let mut count = 0;

    for scrobble in scrobbles.get_saved_scrobbles() {
        insert_scrobble(scrobble, pool).await?;
        count = count + 1;
    }

    Ok(count)
}

async fn insert_scrobble(scrobble: SavedScrobble, pool: &SqlitePool) -> Result<i64> {
    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
        INSERT INTO scrobbles (track, artist, album, loved, timestamp_utc)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        scrobble.title,
        scrobble.artist,
        scrobble.album,
        scrobble.loved,
        scrobble.timestamp_utc,
    )
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

    Ok(id)
}

pub async fn get_most_recent_scrobble(pool: &SqlitePool) -> Result<i64> {
    let most_recent_scrobble: (i64, ) = sqlx::query_as(
        r#"
        SELECT timestamp_utc
        FROM scrobbles
        ORDER BY timestamp_utc
        DESC LIMIT 1
        "#)
        .fetch_one(pool)
        .await?;

    Ok(most_recent_scrobble.0)
}

/*pub async fn get_scrobbles(pool: &SqlitePool) -> Result<Vec<SavedScrobble>> {
    let mut recs = sqlx::query!(
        r#"
        SELECT track, artist, album, loved, timestamp_utc, timestamp_local
        FROM scrobbles_local
        ORDER BY timestamp_local DESC
        "#
    )
        .fetch_all(pool)
        .await?;


    let mut scrobbles = Vec::new();
    for rec in recs {
        let scrobble = SavedScrobble {
            title: rec.track,
            artist: rec.artist,
            album: rec.album,
            loved: rec.loved,
            timestamp_utc: rec.timestamp_utc,
            datetime_local: rec.timestamp_local,
        };
        scrobbles.push(scrobble)
    }

    Ok(scrobbles)
}*/