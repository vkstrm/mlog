use musiklog::error::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    // This will set the proper exit code via the Termination trait
    // which is cool but the error isn't printed very nicely, can that be improved?
    musiklog::glue::handle_args(args)
}
