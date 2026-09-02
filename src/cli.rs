use clap::{Parser, Subcommand, ValueEnum, arg, command};

use crate::dateinput::DateInput;

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
        #[arg(help = "Filter by release year", long = "year")]
        release_year: Option<u32>,
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
        #[arg(
            long = "count",
            help = "The number of entries to display for the top rankings."
        )]
        count: Option<usize>,
    },
}
