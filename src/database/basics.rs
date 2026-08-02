use std::fs::DirBuilder;
use std::{env, path::PathBuf};

use rusqlite::Connection;

use crate::error;
use crate::error::Error;
use crate::util::choice_yesorno;

// Get the filepath to the database location
// This will create a directory for mlog if it doesn't exist
pub fn solve_database_path() -> Result<PathBuf, Error> {
    // Set a specific path, for testing or whatever
    if let Ok(path) = env::var("MLOG_DB_PATH") {
        return Ok(PathBuf::from(path));
    }

    // Try to follow XDG spec: https://specifications.freedesktop.org/basedir/latest/
    let mut base_dir = if let Ok(path) = env::var("XDG_DATA_HOME") {
        PathBuf::from(path)
    } else {
        let mut home = match env::home_dir() {
            Some(dir) => dir,
            None => error!("Can't get home directory"),
        };
        home.push(".local/share");
        home
    };

    if !base_dir.exists() {
        error!(format!(
            "The base data directory is expected to be {} but it doesn't exist. That seems weird!",
            base_dir.to_str().unwrap()
        ))
    }

    // Ask nicely to create the mlog directory
    base_dir.push("mlog");
    if !base_dir.exists() {
        let msg = format!(
            "The directory {} doesn't exist, can mlog create it?",
            base_dir.to_str().unwrap()
        );
        if choice_yesorno(&msg)? {
            DirBuilder::new().create(&base_dir)?;
            println!("Thanks, created.");
        } else {
            error!(
                "Directory not created but is required, create it yourself if that suits you better."
            )
        }
    }

    // SQLite needs a filename
    base_dir.push("mlog.db");
    Ok(base_dir)
}

pub fn upsert_tables(connection: &Connection) -> Result<(), Error> {
    match connection.execute(
        "CREATE TABLE IF NOT EXISTS artist(name TEXT PRIMARY KEY, WITHOUR ROWID)",
        [],
    ) {
        Ok(_) => {}
        Err(err) => error!(err.to_string()),
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
