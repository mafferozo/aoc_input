use std::{error::Error as stdError, fmt::Display};
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    ConnectionError(reqwest::Error),
    SessionRequired
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionError(ref e) => Display::fmt(e, f),
            Error::SessionRequired => write!(f,"Session token required"),
        }
    }
}

impl stdError for Error {
    fn source(&self) -> Option<&(dyn stdError + 'static)> {
        match *self {
            Error::ConnectionError(ref e) => Some(e),
            Error::SessionRequired => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ConnectionError(e)
    }
}