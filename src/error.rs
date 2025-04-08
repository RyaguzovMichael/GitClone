use std::io;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn message(&self) -> &String {
        return &self.message;
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            message: String::from(value),
        }
    }
}
