use std::str::FromStr;

use clap::{Parser, Subcommand, ValueEnum, arg, command};

use crate::error::Error;

#[derive(Clone)]
pub struct DateInput {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl FromStr for DateInput {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 3 {
            return Err(Error::new(
                "Invalid date format. Should be like 2025-01-12".to_string(),
            ));
        }
        let year: i32 = parts[0].parse().unwrap();
        if !(1970..=3000).contains(&year) {
            return Err(Error::new("The year is an unrealistic value".to_string()));
        }
        let month: u32 = parts[1].parse().unwrap();
        if !(1..=12).contains(&month) {
            return Err(Error::new("The month doesn't exist".to_string()));
        }
        let day: u32 = parts[2].parse().unwrap();
        if !(1..=31).contains(&day) {
            return Err(Error::new("The day doesn't exist".to_string()));
        }
        Ok(DateInput { year, month, day })
    }
}

#[cfg(test)]
mod tests_dateinput {
    use super::*;

    #[test]
    fn test_valid() {
        let valid = vec!["2025-11-21", "2026-1-31", "2025-12-1"];
        for v in valid {
            let x = DateInput::from_str(v);
            assert!(x.is_ok())
        }
    }

    #[test]
    fn test_invalid() {
        let invalid = vec!["2025-0-1", "2026-1-32", "1900-12-1"];
        for v in invalid {
            let x = DateInput::from_str(v);
            assert!(x.is_err())
        }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Commands for managing artists")]
    Artist {
        #[command(subcommand)]
        command: ArtistCommands,
    },
    #[command(about = "Commands for managing releases")]
    Release {
        #[command(subcommand)]
        command: ReleaseCommands,
    },
    #[command(about = "Commands for managing logs")]
    Log {
        #[command(subcommand)]
        command: LogCommands,
    },
    #[command(about = "Commands for seeing log summaries")]
    Summary {
        #[command(subcommand)]
        command: SummaryCommands,
    },
}

#[derive(Subcommand)]
pub enum ReleaseCommands {
    #[command()]
    Add {
        #[arg(help = "The artist name, which already needs to be registered")]
        artist: String,
        #[arg(help = "Name of the release")]
        name: String,
        #[arg(help = "The release year of the release")]
        year: u32,
    },
    #[command(about = "List releases")]
    List {
        #[arg(help = "List releases for this artist", long = "artist")]
        artist: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ArtistCommands {
    #[command(about = "Register a new artist")]
    Add {
        #[arg(help = "The name of the artist. Must be unique.")]
        name: String,
    },
    #[command(about = "List all artists")]
    List,
}

#[derive(Subcommand)]
pub enum LogCommands {
    #[command(about = "Log a listen of a release")]
    Add {
        #[arg(help = "Name of the release that is being logged.")]
        release: String,
        #[arg(long = "date", help = "When the log is for.")]
        date: Option<DateInput>,
    },
    #[command(about = "List logs")]
    List,
    #[command(about = "Delete a log")]
    Delete {
        #[arg(help = "Id of the log to delete")]
        id: i32,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Months {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Months {
    pub fn value(&self) -> i32 {
        match self {
            Self::Jan => 1,
            Self::Feb => 2,
            Self::Mar => 3,
            Self::Apr => 4,
            Self::May => 5,
            Self::Jun => 6,
            Self::Jul => 7,
            Self::Aug => 8,
            Self::Sep => 9,
            Self::Oct => 10,
            Self::Nov => 11,
            Self::Dec => 12,
        }
    }
}

#[derive(Subcommand)]
pub enum SummaryCommands {
    #[command(about = "Summary per month")]
    Month {
        #[arg(value_enum, help = "The month to get a summary for")]
        month: Months,
    },
}
