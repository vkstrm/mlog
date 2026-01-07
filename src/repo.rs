use rusqlite::{Connection, params};

use crate::{
    error::Error,
    model::{Artist, Log, Release},
};

pub fn add_release(connection: &Connection, release: Release) -> Result<(), Error> {
    let mut stmt =
        connection.prepare("INSERT INTO release (name, artistname, year) VALUES (?1, ?2, ?3)")?;
    stmt.execute(params![release.name, release.artist, release.release_year])?;
    Ok(())
}

pub fn get_release(connection: &Connection, release: String) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release where name = (?1)")?;
    let rows = stmt.query_map([release], |row| {
        Ok(Release {
            id: row.get(0)?,
            name: row.get(1)?,
            artist: row.get(2)?,
            release_year: row.get(3)?,
        })
    })?;
    let mut releases: Vec<Release> = vec![];
    for r in rows {
        releases.push(r?);
    }
    Ok(releases)
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
        "SELECT log.date, release.name, artist.name FROM log JOIN release ON log.release_id = release.id JOIN artist ON release.artistname = artist.name ORDER BY log.date",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Log {
            date: row.get(0)?,
            release: row.get(1)?,
            artist: row.get(2)?,
        })
    })?;
    let mut log: Vec<Log> = vec![];
    for r in rows {
        log.push(r?);
    }
    Ok(log)
}

pub fn releases_for_artist(connection: &Connection, artist: String) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release WHERE artistname = (?1)")?;
    let rows = stmt.query_map([artist], |row| {
        Ok(Release {
            id: row.get(0)?,
            name: row.get(1)?,
            artist: row.get(2)?,
            release_year: row.get(3)?,
        })
    })?;
    let mut releases: Vec<Release> = vec![];
    for r in rows {
        releases.push(r?);
    }
    Ok(releases)
}

pub fn all_releases(connection: &Connection) -> Result<Vec<Release>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM release")?;
    let rows = stmt.query_map([], |row| {
        Ok(Release {
            id: row.get(0)?,
            name: row.get(1)?,
            artist: row.get(2)?,
            release_year: row.get(3)?,
        })
    })?;
    let mut releases: Vec<Release> = vec![];
    for r in rows {
        releases.push(r?);
    }
    Ok(releases)
}

pub fn artists(connection: &Connection) -> Result<Vec<Artist>, Error> {
    let mut stmt = connection.prepare("SELECT * FROM artist")?;
    let rows = stmt.query_map([], |row| Ok(Artist { name: row.get(0)? }))?;
    let mut artists: Vec<Artist> = vec![];
    for r in rows {
        artists.push(r?);
    }
    Ok(artists)
}
