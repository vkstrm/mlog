use clap::Parser;
use musiklog::database::basics::{solve_database_path, upsert_tables};
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
    let db_path = solve_database_path()?;
    match Connection::open(db_path) {
        Ok(conn) => Ok(conn),
        Err(err) => error!(err.to_string()),
    }
}
