use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match musiklog::glue::handle_args(args) {
        Ok(_) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{}", err.message);
            ExitCode::from(1)
        }
    }
}
