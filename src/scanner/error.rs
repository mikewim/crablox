use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScanError {
    /// Unterminated string!
    UnterminatedString,
    /// Char not recognized
    UnknownToken,
    /// For now only number that ends with decimal
    NotValidNumber,
}

impl Error for ScanError {}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
