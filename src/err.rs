extern crate url;

use std::fmt;
use std::error::Error;
use self::url::ParseError;


#[derive(Debug)]
pub enum LorisError {
    UrlParseError(ParseError),
}

impl From<ParseError> for LorisError {
    fn from(err: ParseError) -> Self {
        LorisError::UrlParseError(err)
    }
}

impl fmt::Display for LorisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for LorisError {
    fn description(&self) -> &str {
        match *self {
            LorisError::UrlParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            LorisError::UrlParseError(ref err) => Some(err),
        }
    }
}
