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
//! Errors can currently only occur when something can not be parsed or a url on the web cannot be reached.

use std::{
    fmt,
    sync::{Arc, Mutex},
};

use tf_observer::{Observable, Observer, ObserverList};

/// The collection of all errors that can occur.
#[derive(Debug, Clone)]
pub enum Error {
    /// A error parsing something.
    ParseError(ParseError),
    /// A error accessing the internet.
    NetworkError(NetworkError),
}

/// A error parsing something.
/// The variable should hold information about what cannot be parsed, e.g. the channel name or id.
#[derive(Debug, Clone)]
pub struct ParseError(pub String);

/// A error accessing the internet.
/// The variable should hold information about what url failed to request.
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

/// A [Observable] holding a list of [Error]s.
#[derive(Clone)]
pub struct ErrorStore {
    /// The observers.
    observers: ObserverList<ErrorEvent>,

    /// The errors.
    errors: Arc<Mutex<Vec<Error>>>,
}

impl ErrorStore {
    /// Create a new [ErrorStore] without any observers and without any errors.
    pub fn new() -> Self {
        ErrorStore {
            errors: Arc::new(Mutex::new(vec![])),
            observers: ObserverList::new(),
        }
    }

    /// Add the given [Error] to the store.
    /// Will notify the observers using [ErrorEvent::Add].
    pub fn add(&self, error: Error) {
        self.errors.lock().unwrap().push(error.clone());
        self.observers.notify(ErrorEvent::Add(error))
    }

    /// Clear the store of all errors.
    /// Will notify the observers using [ErrorEvent::Clear].
    pub fn clear(&self) {
        self.errors.lock().unwrap().clear();
        self.observers.notify(ErrorEvent::Clear)
    }

    /// Iterate over a copy of all errors.
    pub fn iter(&self) -> impl Iterator<Item = Error> {
        self.errors.lock().unwrap().clone().into_iter()
    }

    /// Give a [ErrorSummary] of the [ErrorStore], e.g. how many of each [Error]-type are inside the store.
    pub fn summary(&self) -> ErrorSummary {
        let parse = self
            .iter()
            .filter(|e| matches!(e, Error::ParseError(_)))
            .count();
        let network = self
            .iter()
            .filter(|e| matches!(e, Error::NetworkError(_)))
            .count();

        ErrorSummary { parse, network }
    }
}

impl Default for ErrorStore {
    fn default() -> Self {
        ErrorStore::new()
    }
}

/// Summarize the [Error]s held in a [ErrorStore] by counting the number of different [Error]-types.
pub struct ErrorSummary {
    /// [ParseError]s
    parse: usize,
    /// [NetworkError]s
    network: usize,
}

impl ErrorSummary {
    /// Give the amount of [ParseError]s int the [ErrorStore].
    pub fn parse(&self) -> usize {
        self.parse
    }

    /// Give the amount of [NetworkError]s int the [ErrorStore].
    pub fn network(&self) -> usize {
        self.network
    }
}

/// A event from the [ErrorStore].
#[derive(Clone)]
pub enum ErrorEvent {
    /// A new [Error] was added into the [ErrorStore]. See [ErrorStore::add].
    Add(Error),
    /// The [ErrorStore] was cleared. See [ErrorStore::clear].
    Clear,
}

impl Observable<ErrorEvent> for ErrorStore {
    fn attach(&mut self, observer: std::sync::Weak<Mutex<Box<dyn Observer<ErrorEvent> + Send>>>) {
        self.observers.attach(observer);
    }

    fn detach(&mut self, observer: std::sync::Weak<Mutex<Box<dyn Observer<ErrorEvent> + Send>>>) {
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
