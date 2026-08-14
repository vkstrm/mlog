use rusqlite::{Connection, OptionalExtension, params};

use crate::{
    error::Error,
    model::{Artist, Log, Release},
};

#[macro_export]
macro_rules! release {
    ($val:expr) => {
        Ok(Release {
            id: $val.get(0)?,
            name: $val.get(1)?,
            artist: $val.get(2)?,
            release_year: $val.get(3)?,
            logs: $val.get(4)?,
        })
    };
}

#[macro_export]
macro_rules! log {
    ($val:expr) => {
        Ok(Log {
            id: $val.get(0)?,
            date: $val.get(1)?,
            release: $val.get(2)?,
            artist: $val.get(3)?,
        })
    };
}

// This is just for the database interface
pub struct NewRelease {
    pub name: String,
    pub artist: String,
    pub release_year: u32,
}

pub fn add_release(connection: &Connection, release: NewRelease) -> Result<(), Error> {
    let mut stmt =
        connection.prepare("INSERT INTO release (name, artistname, year) VALUES (?1, ?2, ?3)")?;
    stmt.execute(params![release.name, release.artist, release.release_year])?;
    Ok(())
}

pub fn get_release(connection: &Connection, release_name: String) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release WHERE name = (?1)")?;
    let rows = stmt.query_map([release_name], |row| {
        // The macro isn't used here because the logs count isn't used where this release is used.
        Ok(Release {
            id: row.get(0)?,
            name: row.get(1)?,
            artist: row.get(2)?,
            release_year: row.get(3)?,
            logs: 0,
        })
    })?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn add_artist(connection: &Connection, artist: Artist) -> Result<(), Error> {
    let mut stmt = connection.prepare("INSERT INTO artist (name) VALUES (?1)")?;
    stmt.execute([artist.name])?;
    Ok(())
}

pub fn add_log(connection: &Connection, release_id: i32, date: String) -> Result<(), Error> {
    let mut stmt = connection.prepare("INSERT INTO log (release_id, date) VALUES (?1, ?2)")?;
    stmt.execute(params![release_id, date])?;
    Ok(())
}

pub fn list_log(connection: &Connection) -> Result<Vec<Log>, Error> {
    let mut stmt = connection.prepare(
        "SELECT log.id, log.date, release.name, artist.name FROM log
        JOIN release ON log.release_id = release.id
        JOIN artist ON release.artistname = artist.name
        ORDER BY log.date",
    )?;
    let rows = stmt.query_map([], |row| log!(row))?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn list_log_month(connection: &Connection, month: i32) -> Result<Vec<Log>, Error> {
    let mut stmt = connection.prepare(
        "SELECT log.id, log.date, release.name, artist.name FROM log
        JOIN release ON log.release_id = release.id
        JOIN artist ON release.artistname = artist.name
        WHERE log.date BETWEEN (SELECT date('now','start of year',(?1))) AND (SELECT date('now','start of year',(?2)))",
    )?;
    let rows = stmt.query_map(
        // sqlite jan starts at 0
        [format!("{} month", month - 1), format!("{} month", month)],
        |row| log!(row),
    )?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn get_log(connection: &Connection, id: i32) -> Result<Option<Log>, Error> {
    let mut stmt = connection.prepare(
        "SELECT log.id, log.date, release.name, artist.name FROM log
        JOIN release ON log.release_id = release.id
        JOIN artist ON release.artistname = artist.name
        WHERE log.id = (?1)",
    )?;
    Ok(stmt.query_one([id], |row| log!(row)).optional()?)
}

pub fn delete_log(connection: &Connection, id: i32) -> Result<(), Error> {
    let mut stmt = connection.prepare("DELETE FROM log WHERE id = (?1)")?;
    stmt.execute(params![id])?;
    Ok(())
}

pub fn releases_for_artist(connection: &Connection, artist: String) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare(
        "SELECT release.id, release.name, release.artistname, release.year, COUNT(log.release_id) as count
        FROM release
        JOIN log ON log.release_id = release.id
        WHERE release.artistname = (?1)
        GROUP BY release.id"
    )?;
    let rows = stmt.query_map([artist], |row| release!(row))?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn all_releases(connection: &Connection) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare(
        "SELECT release.id, release.name, release.artistname, release.year, COUNT(log.release_id) as count
        FROM release
        JOIN log ON log.release_id = release.id
        GROUP BY release.id"
    )?;
    let rows = stmt.query_map([], |row| release!(row))?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn artists(connection: &Connection) -> Result<Vec<Artist>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM artist")?;
    let rows = stmt.query_map([], |row| Ok(Artist { name: row.get(0)? }))?;
    Ok(rows.into_iter().flatten().collect())
}

// Get the logs that are older than the input month.
// How many months of logs before this becomes a little slow?
// Cant think of anything smarter right now
pub fn logs_before_month(connection: &Connection, month: i32) -> Result<Vec<Log>, Error> {
    let mut stmt = connection.prepare(
        "SELECT log.id, log.date, release.name, artist.name FROM log
        JOIN release ON log.release_id = release.id
        JOIN artist ON release.artistname = artist.name
        WHERE log.date < (SELECT date('now','start of year',(?1))) GROUP BY release.name",
    )?;
    let rows = stmt.query_map(
        // sqlite jan starts at 0
        [format!("{} month", month - 1)],
        |row| log!(row),
    )?;
    Ok(rows.into_iter().flatten().collect())
}
