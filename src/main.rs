use std::env;
use std::fs::DirBuilder;
use std::path::PathBuf;

use clap::Parser;
use musiklog::error;
use musiklog::{cli::Cli, error::Error, handlers::handle_input};
use rusqlite::Connection;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match handle(args) {
        Ok(()) => {}
        Err(err) => eprintln!("{}", err),
    }
}

fn handle(args: Vec<String>) -> Result<(), Error> {
    let parsed = Cli::parse_from(args);
    let conn = open_db()?;
    upsert_tables(&conn)?;
    handle_input(parsed, conn)
}

fn open_db() -> Result<Connection, Error> {
    let db_path = get_db_path()?;
    match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(err) => error!(err.to_string()),
    }
}

fn get_db_path() -> Result<PathBuf, Error> {
    match env::var("MLOG_DB_PATH") {
        Ok(path) => {
            return Ok(PathBuf::from(path))
        },
        Err(_) => {}
    };
    let mut dir = match env::home_dir() {
        Some(dir) => dir,
        None => error!("Can't get home directory"),
    };
    dir.push(".config/mlog/mlog.db");
    if let Some(parent) = dir.parent() && !parent.exists() {
            DirBuilder::new().create(parent)?;
        }
    Ok(dir)
}

fn upsert_tables(connection: &Connection) -> Result<(), Error> {
    match connection.execute("CREATE TABLE IF NOT EXISTS artist(name TEXT PRIMARY KEY, WITHOUR ROWID)", []) {
       Ok(_) => {},
       Err(err) => error!(err.to_string()) 
    };
    match connection.execute("CREATE TABLE IF NOT EXISTS release(id INTEGER PRIMARY KEY, name TEXT NOT NULL, artistname STRING NOT NULL, year INTEGER NOT NULL, FOREIGN KEY(artistname) REFERENCES artist(name))", []) {
       Ok(_) => {},
       Err(err) => error!(err.to_string()) 
    };
    match connection.execute("CREATE TABLE IF NOT EXISTS log(id INTEGER PRIMARY KEY, release_id INTEGER NOT NULL, date TEXT, FOREIGN KEY(release_id) REFERENCES release(id));", []) {
       Ok(_) => Ok(()),
       Err(err) => error!(err.to_string()) 
    }
}
