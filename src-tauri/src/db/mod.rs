use rusqlite::{Connection, Result};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct DbState {
    pub connection: Mutex<Option<Connection>>,
}

pub fn init_db(app: &AppHandle) -> Result<Connection> {
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    let db_path = app_dir.join("settings.db");

    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS api_keys (
            provider TEXT PRIMARY KEY,
            key_value TEXT NOT NULL
        )",
        (),
    )?;

    Ok(conn)
}
