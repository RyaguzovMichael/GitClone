use std::{error, fmt, io};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn message(&self) -> &String {
        return &self.message;
    }

    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Application exception because: {}", self.message)
    }
}

impl error::Error for Error {}
