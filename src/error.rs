use std::fmt::Display;

#[macro_export]
macro_rules! error {
    ($msg:literal) => {
        return Err(Error::new($msg.to_string()))
    };
    ($msg:expr) => {
        return Err(Error::new($msg))
    };
    ($msg:stmt) => {
        return Err(Error::new($msg()))
    };
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
