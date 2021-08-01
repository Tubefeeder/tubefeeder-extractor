//! The Errors used in this crate.
//!
//! Errors can occur when something can not be parsed (see [`quick_xml::de::DeError`]) or
//! a url on the web cannot be reached (see [`reqwest::Error`]).

use std::fmt;

/// The collection of all errors that can occur.
#[derive(Debug)]
pub enum Error {
    /// A error parsing something.
    ParseError(ParseError),
    /// A error accessing the internet.
    NetworkError(NetworkError),
}

/// A error parsing something.
#[derive(Debug)]
pub struct ParseError(pub String);

/// A error accessing the internet.
#[derive(Debug)]
pub struct NetworkError {
    url: Option<String>,
    error: reqwest::Error,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "{}", e),
            Error::NetworkError(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing {}", self.0)
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(url) = &self.url {
            write!(f, "Error getting {}: {}", url, self.error)
        } else {
            write!(f, "Error accessing web: {}", self.error)
        }
    }
}

impl std::error::Error for Error {}
impl std::error::Error for ParseError {}
impl std::error::Error for NetworkError {}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        let network_error = NetworkError {
            url: e.url().map(|u| u.to_string()),
            error: e,
        };

        return Error::NetworkError(network_error);
    }
}
