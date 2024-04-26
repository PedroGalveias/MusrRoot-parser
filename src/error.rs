use std::error::Error;
use std::fmt;

use nom::error::ErrorKind;

// Define a custom error type that implements Error trait
#[derive(Debug)]
pub struct ParsingError(ErrorKind);

// Implement the Error trait for ParsingError
impl Error for ParsingError {}

// Implement Display trait to provide human-readable error messages
impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing error: {:?}", self.0)
    }
}

// Public constructor function to create instances of ParsingError
pub fn parse_error(kind: ErrorKind) -> ParsingError {
    ParsingError(kind)
}
