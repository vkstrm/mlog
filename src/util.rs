use serde::Serialize;

use crate::error;
use crate::error::Error;
use std::error::Error as StdError;
use std::io::stdin;
use std::str::FromStr;

pub fn choice<T>(message: &str) -> Result<T, Error>
where
    T: FromStr,
    T::Err: StdError,
{
    eprintln!("{}", message);
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    match buffer.trim().parse::<T>() {
        Ok(parsed) => Ok(parsed),
        Err(err) => error!(err.to_string()),
    }
}

pub fn output_pretty<T>(value: T) -> Result<(), Error>
where
    T: Serialize,
{
    match serde_json::to_string_pretty(&value) {
        Ok(pretty) => {
            println!("{}", pretty);
            Ok(())
        }
        Err(err) => error!(err.to_string()),
    }
}
