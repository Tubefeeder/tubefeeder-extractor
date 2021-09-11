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

use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::{Observable, ObserverList};

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

#[derive(Clone)]
pub struct ErrorStore {
    observers: ObserverList<ErrorEvent>,

    errors: Arc<Mutex<Vec<Error>>>,
}

impl ErrorStore {
    pub fn new() -> Self {
        ErrorStore {
            errors: Arc::new(Mutex::new(vec![])),
            observers: ObserverList::new(),
        }
    }

    pub fn add(&self, error: Error) {
        self.errors.lock().unwrap().push(error.clone());
        self.observers.notify(ErrorEvent::Add(error))
    }

    pub fn clear(&self) {
        self.errors.lock().unwrap().clear();
        self.observers.notify(ErrorEvent::Clear)
    }

    pub fn iter(&self) -> impl Iterator<Item = Error> {
        self.errors.lock().unwrap().clone().into_iter()
    }

    pub fn summary(&self) -> ErrorSummary {
        let parse = self
            .iter()
            .filter(|e| {
                if let Error::ParseError(_) = e {
                    return true;
                } else {
                    return false;
                }
            })
            .count();
        let network = self
            .iter()
            .filter(|e| {
                if let Error::NetworkError(_) = e {
                    return true;
                } else {
                    return false;
                }
            })
            .count();

        ErrorSummary { parse, network }
    }
}

impl Default for ErrorStore {
    fn default() -> Self {
        ErrorStore::new()
    }
}

pub struct ErrorSummary {
    parse: usize,
    network: usize,
}

impl ErrorSummary {
    pub fn parse(&self) -> usize {
        self.parse
    }

    pub fn network(&self) -> usize {
        self.network
    }
}

#[derive(Clone)]
pub enum ErrorEvent {
    Add(Error),
    Clear,
}

impl Observable<ErrorEvent> for ErrorStore {
    fn attach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn crate::Observer<ErrorEvent> + Send>>>,
    ) {
        self.observers.attach(observer);
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<dyn crate::Observer<ErrorEvent> + Send>>>,
    ) {
        self.observers.detach(observer);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn add_network_error(store: &ErrorStore) {
        store.add(NetworkError("Url".to_owned()).into())
    }

    fn add_parse_error(store: &ErrorStore) {
        store.add(ParseError("Parse".to_owned()).into())
    }

    #[test]
    fn errorstore_add() {
        let store = ErrorStore::new();
        add_network_error(&store);
        add_network_error(&store);

        assert_eq!(store.iter().count(), 2);
    }

    #[test]
    fn errorstore_clear() {
        let store = ErrorStore::new();
        add_network_error(&store);
        add_parse_error(&store);
        store.clear();

        assert_eq!(store.iter().count(), 0);
    }

    #[test]
    fn errorstore_summary() {
        let store = ErrorStore::new();
        add_network_error(&store);
        add_network_error(&store);
        add_parse_error(&store);

        let summary = store.summary();

        assert_eq!(summary.parse(), 1);
        assert_eq!(summary.network(), 2);
    }
}
