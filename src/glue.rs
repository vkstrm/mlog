use clap::Parser;

use crate::database::basics::{get_connection, upsert_tables};
use crate::{cli::Cli, error::Error, handlers::handle_input};

pub fn handle_args(args: Vec<String>) -> Result<(), Error> {
    let parsed = Cli::parse_from(args);
    let conn = get_connection()?;
    upsert_tables(&conn)?;
    handle_input(parsed, conn)
}
