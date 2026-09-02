use std::collections::HashMap;

use crate::cli::SummaryCommands;
use crate::database::operations::{
    NewRelease, add_artist, add_log, add_release, all_releases, artists, delete_log, get_log,
    get_release, list_log, list_log_month, logs_before_month, releases_for_artist,
};
use crate::dateinput::parse_dateinput;
use crate::error;
use crate::util::{choice, output_pretty};
use crate::{
    cli::{ArtistCommands, Cli, Commands, LogCommands, ReleaseCommands},
    error::Error,
    model::{Artist, Release},
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub fn handle_input(cli: Cli, connection: Connection) -> Result<(), Error> {
    match cli.command {
        Some(Commands::Artist { command }) => handle_artist(command, connection)?,
        Some(Commands::Release { command }) => handle_release(command, connection)?,
        Some(Commands::Log { command }) => handle_log(command, connection)?,
        Some(Commands::Summary { command }) => handle_summary(command, connection)?,
        None => {}
    };
    Ok(())
}

pub fn handle_log(command: LogCommands, connection: Connection) -> Result<(), Error> {
    match command {
        LogCommands::Add { release, date } => {
            let date = parse_dateinput(date)?;
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
            output_pretty(&logs)?
        }
        LogCommands::Delete { id } => {
            if let Some(log) = get_log(&connection, id)? {
                let msg = format!(
                    "Really delete log?\n {}\n[y/n]: ",
                    serde_json::to_string_pretty(&log).unwrap()
                );
                let choice: String = choice(&msg)?;
                if choice.trim().to_lowercase() != "y" {
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

#[derive(Serialize, Deserialize)]
struct TopEntry {
    name: String,
    count: usize,
}

#[derive(Serialize, Deserialize)]
struct Summary {
    #[serde(rename = "totalLogs")]
    total_logs: usize,
    #[serde(rename = "totalArtists")]
    total_artists: usize,
    #[serde(rename = "uniqueReleases")]
    unique_releases: usize,
    #[serde(rename = "newReleaseCount")]
    new_release_count: usize,
    #[serde(rename = "topArtists")]
    artist_top: Vec<TopEntry>,
    #[serde(rename = "topReleases")]
    release_top: Vec<TopEntry>,
}

pub fn handle_summary(command: SummaryCommands, connection: Connection) -> Result<(), Error> {
    match command {
        SummaryCommands::Month { month, count } => {
            let display_top_count = count.unwrap_or(5);
            let logs = list_log_month(&connection, month.value())?;
            let total_logs_count = logs.len();

            let old_logs = logs_before_month(&connection, month.value())?;

            let mut artists: HashMap<String, Vec<String>> = HashMap::new();
            let mut releases: HashMap<String, usize> = HashMap::new();
            logs.into_iter().for_each(|log| {
                match releases.get(&log.release) {
                    Some(cur) => releases.insert(log.release.to_string(), cur + 1),
                    None => releases.insert(log.release.to_string(), 1),
                };

                match artists.get_mut(&log.artist) {
                    Some(v) => v.push(log.release),
                    None => drop(artists.insert(log.artist.to_string(), vec![log.release])),
                }
            });

            let total_artist_count = artists.keys().len();
            let unique_releases_count = releases.len();

            // filter older logs against this month
            let old_releases: Vec<String> = old_logs
                .iter()
                .map(|log| log.release.to_string())
                .filter(|r| releases.contains_key(r))
                .collect();
            let new_release_count = releases.len() - old_releases.len();

            // prepare the artists for printing
            let mut artist_entries: Vec<TopEntry> = artists
                .iter()
                .map(|(k, v)| TopEntry {
                    name: k.to_string(),
                    count: v.len(),
                })
                .collect();
            artist_entries.sort_by(|a, b| a.count.cmp(&b.count).reverse());

            // prepare the releases for printing
            let mut release_entries: Vec<TopEntry> = releases
                .iter()
                .map(|(k, v)| TopEntry {
                    name: if !old_releases.contains(k) {
                        format!("{} (New)", k)
                    } else {
                        k.to_string()
                    },
                    count: v.to_owned(),
                })
                .collect();
            release_entries.sort_by(|a, b| a.count.cmp(&b.count).reverse());

            let summary = Summary {
                total_logs: total_logs_count,
                total_artists: total_artist_count,
                artist_top: artist_entries.into_iter().take(display_top_count).collect(),
                release_top: release_entries
                    .into_iter()
                    .take(display_top_count)
                    .collect(),
                unique_releases: unique_releases_count,
                new_release_count,
            };
            output_pretty(&summary)?;
        }
    }
    Ok(())
}

pub fn handle_release(command: ReleaseCommands, connection: Connection) -> Result<(), Error> {
    match command {
        ReleaseCommands::Add { artist, name, year } => add_release(
            &connection,
            NewRelease {
                name,
                artist,
                release_year: year,
            },
        )?,
        ReleaseCommands::List {
            artist,
            release_year,
        } => {
            let releases = if let Some(artist) = artist {
                let mut r = releases_for_artist(&connection, artist)?;
                r.sort_by(|a, b| a.release_year.cmp(&b.release_year));
                r
            } else {
                all_releases(&connection)?
            };
            let releases = if let Some(year) = release_year {
                let mut r: Vec<Release> = releases
                    .into_iter()
                    .filter(|x| x.release_year == year)
                    .collect();
                r.sort_by(|a, b| (b.logs).cmp(&a.logs));
                r
            } else {
                releases
            };
            output_pretty(&releases)?;
        }
    }
    Ok(())
}

pub fn handle_artist(command: ArtistCommands, connection: Connection) -> Result<(), Error> {
    match command {
        ArtistCommands::Add { name } => add_artist(&connection, Artist { name })?,
        ArtistCommands::List => {
            let artists = artists(&connection)?;
            output_pretty(&artists)?;
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
    let choice: usize = choice("Pick a release by the number:")?;
    if choice < 1 || choice > releases.len() {
        error!("Invalid choice")
    }
    match releases.get(choice - 1) {
        Some(release) => Ok(release),
        None => error!("Invalid choice"),
    }
}
