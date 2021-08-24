/*
 * Copyright 2021 Julian Schmidhuber <github@schmiddi.anonaddy.com>
 *
 * This file is part of Tubefeeder-extractor.
 *
 * Tubefeeder-extractor is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Tubefeeder-extractor is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Tubefeeder-extractor.  If not, see <https://www.gnu.org/licenses/>.
 */

//! The Errors used in this crate.
//!
//! Errors can occur when something can not be parsed (see [`quick_xml::de::DeError`]) or
//! a url on the web cannot be reached (see [`reqwest::Error`]).

use std::fmt;

/// The collection of all errors that can occur.
#[derive(Debug, Clone)]
pub enum Error {
    /// A error parsing something.
    ParseError(ParseError),
    /// A error accessing the internet.
    NetworkError(NetworkError),
}

/// A error parsing something.
#[derive(Debug, Clone)]
pub struct ParseError(pub String);

/// A error accessing the internet.
#[derive(Debug, Clone)]
pub struct NetworkError(pub String);

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
        write!(f, "Error getting {}", self.0)
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

impl From<NetworkError> for Error {
    fn from(e: NetworkError) -> Self {
        Error::NetworkError(e)
    }
}

pub struct ErrorStore {
    errors: Vec<Error>,
}

impl ErrorStore {
    pub fn new() -> Self {
        ErrorStore { errors: vec![] }
    }

    pub fn add(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn iter(&self) -> impl Iterator<Item = Error> {
        self.errors.clone().into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn add_error(store: &mut ErrorStore) {
        store.add(NetworkError("Url".to_owned()).into())
    }

    #[test]
    fn errorstore() {
        let mut store = ErrorStore::new();
        add_error(&mut store);
        add_error(&mut store);

        assert_eq!(store.iter().count(), 2);
    }
}
