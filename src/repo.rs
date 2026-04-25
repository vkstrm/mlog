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

pub fn add_release(connection: &Connection, release: Release) -> Result<(), Error> {
    let mut stmt =
        connection.prepare("INSERT INTO release (name, artistname, year) VALUES (?1, ?2, ?3)")?;
    stmt.execute(params![release.name, release.artist, release.release_year])?;
    Ok(())
}

pub fn get_release(connection: &Connection, release: String) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release where name = (?1)")?;
    let rows = stmt.query_map([release], |row| release!(row))?;
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
        WHERE log.date BETWEEN (SELECT date('now','start of year',(?1))) AND (SELECT date('now','start of year',(?2),'-1 days'))",
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
        ORDER BY log.date AND log.id = (?1)",
    )?;
    Ok(stmt.query_one([id], |row| log!(row)).optional()?)
}

pub fn delete_log(connection: &Connection, id: i32) -> Result<(), Error> {
    let mut stmt = connection.prepare("DELETE FROM log WHERE id = (?1)")?;
    stmt.execute(params![id])?;
    Ok(())
}

pub fn releases_for_artist(connection: &Connection, artist: String) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release WHERE artistname = (?1)")?;
    let rows = stmt.query_map([artist], |row| release!(row))?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn all_releases(connection: &Connection) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release")?;
    let rows = stmt.query_map([], |row| release!(row))?;
    Ok(rows.into_iter().flatten().collect())
}

pub fn artists(connection: &Connection) -> Result<Vec<Artist>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM artist")?;
    let rows = stmt.query_map([], |row| Ok(Artist { name: row.get(0)? }))?;
    Ok(rows.into_iter().flatten().collect())
}
