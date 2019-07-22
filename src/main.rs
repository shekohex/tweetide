#![feature(async_await)]
#[macro_use]
extern crate diesel;

use diesel::SqliteConnection;
use parking_lot::{Mutex, MutexGuard};

mod config;
mod db;
mod schema;
mod users;

pub type AppResult<T> = std::result::Result<T, failure::Error>;

pub struct AppData {
    conn: Mutex<SqliteConnection>,
}

impl AppData {
    pub fn db(&self) -> MutexGuard<SqliteConnection> { self.conn.lock() }
}

fn main() -> AppResult<()> {
    let config = config::AppConfig::new()?;
    let conn = db::establish_connection(&config)?;
    let app_data = AppData {
        conn: Mutex::new(conn),
    };
    let mut app = tide::App::new(app_data);
    let mut base = app.at("/api/v1");
    let mut users_endpoint = base.at("/users");
    users_endpoint.at("create").post(users::create);
    app.serve(&config.server_addr)?;
    Ok(())
}
