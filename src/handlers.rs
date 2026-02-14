use std::io::stdin;

use crate::error;
use crate::repo::{
    add_artist, add_release, all_releases, artists, delete_log, get_log, releases_for_artist,
};
use crate::{
    cli::{ArtistCommands, Cli, Commands, DateInput, LogCommands, ReleaseCommands},
    error::Error,
    model::{Artist, Release},
    repo::{add_log, get_release, list_log},
};
use chrono::{DateTime, Local, TimeZone, Timelike};
use rusqlite::Connection;
use serde::Serialize;

pub fn handle_input(cli: Cli, connection: Connection) -> Result<(), Error> {
    match cli.command {
        Some(Commands::Artist { command }) => handle_artist(command, connection)?,
        Some(Commands::Release { command }) => handle_release(command, connection)?,
        Some(Commands::Log { command }) => log(command, connection)?,
        None => {}
    };
    Ok(())
}

pub fn log(command: LogCommands, connection: Connection) -> Result<(), Error> {
    match command {
        LogCommands::Add { release, date } => {
            let date = get_date(date);
            let releases = get_release(&connection, release)?;
            if releases.is_empty() {
                error!("No such release")
            }
            let release = if releases.len() > 1 {
                pick_release(&releases)?
            } else {
                match releases.first() {
                    Some(release) => release,
                    None => error!("Invalid choice"),
                }
            };
            add_log(&connection, release.id, date.to_rfc3339())?;
        }
        LogCommands::List => {
            let logs = list_log(&connection)?;
            print(&logs)?
        }
        LogCommands::Delete { id } => {
            if let Some(log) = get_log(&connection, id)? {
                eprint!(
                    "Really delete log?\n {}\n[y/n]: ",
                    serde_json::to_string_pretty(&log).unwrap()
                );
                let mut buffer = String::new();
                stdin().read_line(&mut buffer)?;
                if buffer.trim().to_lowercase() != "y" {
                    println!("OK, aborting delete");
                    return Ok(());
                }
                delete_log(&connection, id)?;
                println!("Deleted log");
            } else {
                println!("No log found");
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn handle_release(command: ReleaseCommands, connection: Connection) -> Result<(), Error> {
    match command {
        ReleaseCommands::Add { artist, name, year } => add_release(
            &connection,
            Release {
                id: 0,
                name,
                artist,
                release_year: year,
            },
        )?,
        ReleaseCommands::List { artist } => {
            let releases = if let Some(artist) = artist {
                releases_for_artist(&connection, artist)?
            } else {
                all_releases(&connection)?
            };
            print(&releases)?;
        }
    }
    Ok(())
}

pub fn handle_artist(command: ArtistCommands, connection: Connection) -> Result<(), Error> {
    match command {
        ArtistCommands::Add { name } => add_artist(&connection, Artist { name })?,
        ArtistCommands::List => {
            let artists = artists(&connection)?;
            print(&artists)?;
        }
    }
    Ok(())
}

fn pick_release(releases: &[Release]) -> Result<&Release, Error> {
    let mut index = 1;
    for release in releases {
        eprintln!("{}. {}", index, release.artist);
        index += 1;
    }
    eprintln!("Pick a release by the number:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let choice = match buffer.trim().parse::<usize>() {
        Ok(choice) => choice,
        Err(err) => error!(err.to_string()),
    };
    if choice < 1 || choice > releases.len() {
        error!("Invalid choice")
    }
    match releases.get(choice - 1) {
        Some(release) => Ok(release),
        None => error!("Invalid choice"),
    }
}

fn print<T>(value: T) -> Result<(), Error>
where
    T: Serialize,
{
    match serde_json::to_string_pretty(&value) {
        Ok(pretty) => {
            println!("{}", pretty)
        }
        Err(err) => return Err(Error::new(err.to_string())),
    }
    Ok(())
}

fn get_date(date: Option<DateInput>) -> DateTime<Local> {
    match date {
        Some(date_input) => {
            let now = Local::now();
            Local
                .with_ymd_and_hms(
                    date_input.year,
                    date_input.month,
                    date_input.day,
                    now.hour(),
                    now.minute(),
                    now.second(),
                )
                .unwrap() // TODO
        }
        None => Local::now(),
    }
}
