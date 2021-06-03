use std::str::FromStr;
use std::time::Duration;
use std::env;

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::SqlitePool;

use crate::config::Config;

fn build_connection_string() -> Result<String> {
    let config = Config::load_config()?;
    let connection_string = format!("sqlite:{}.db", config.default_username);

    env::set_var("DATABASE_URL", &connection_string);

    Ok(connection_string)
}

/*pub fn build_sqlite_database() -> Result<()> {

}*/

pub async fn get_sqlite_pool() -> Result<SqlitePool> {
    let connection_string = build_connection_string()?;

    let options = SqliteConnectOptions::from_str(&connection_string)?
        .busy_timeout(Duration::from_secs(30))
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}