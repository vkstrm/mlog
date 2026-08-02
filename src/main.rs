use clap::Parser;
use musiklog::error;
use musiklog::files::solve_database_path;
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

// fn get_db_path() -> Result<PathBuf, Error> {
//     if let Ok(path) = env::var("MLOG_DB_PATH") {
//         return Ok(PathBuf::from(path));
//     }

//     let base_dir = if let Ok(path) = env::var("XDG_DATA_HOME") {
//         PathBuf::from(path)
//     } else {
//         let mut home = match env::home_dir() {
//             Some(dir) => dir,
//             None => error!("Can't get home directory"),
//         };
//         home.push(".local/share");
//         home
//     };

//     if !base_dir.exists() {
//         error!(format!(
//             "The base data directory is expected to be {} but it doesn't exist. That seems weird",
//             base_dir.to_str().unwrap()
//         ))
//     }

//     base_dir.push("mlog");
//     if !base_dir.exists() {
//         // let choice =
//     }

//     if let Some(parent) = base_dir.parent()
//         && !parent.exists()
//     {
//         DirBuilder::new().create(parent)?;
//     }
//     Ok(dir)
// }

fn upsert_tables(connection: &Connection) -> Result<(), Error> {
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
