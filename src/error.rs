use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParsingError {
    IoError(io::Error),
    ParseError(String),
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::IoError(err) => write!(f, "IO Error: {}", err),
            ParsingError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

impl From<io::Error> for ParsingError {
    fn from(error: io::Error) -> Self {
        ParsingError::IoError(error)
    }
}
