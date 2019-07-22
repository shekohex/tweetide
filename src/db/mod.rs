use diesel::{Connection, SqliteConnection};

use crate::{config::AppConfig, AppResult};

pub mod user_model;

pub fn establish_connection(config: &AppConfig) -> AppResult<SqliteConnection> {
    let database_url = &config.database.url;
    let conn = SqliteConnection::establish(database_url)?;
    Ok(conn)
}
