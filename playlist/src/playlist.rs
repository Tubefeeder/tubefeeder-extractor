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

use std::sync::Mutex;

use tf_observer::{Observable, Observer, ObserverList};

pub struct Playlist<T> {
    observers: ObserverList<PlaylistEvent<T>>,
    playlist: Vec<T>,
}

impl<T> Playlist<T>
where
    T: Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            observers: ObserverList::new(),
            playlist: Vec::new(),
        }
    }

    pub fn toggle(&mut self, item: &T) {
        if let Some(_i) = self.playlist.iter().find(|&i| i == item) {
            log::debug!("Removing item from playlist");
            self.playlist.retain(|i| i != item);
            self.observers.notify(PlaylistEvent::Remove(item.clone()))
        } else {
            log::debug!("Adding item to playlist");
            self.playlist.push(item.clone());
            self.observers.notify(PlaylistEvent::Add(item.clone()))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.playlist.iter()
    }

    pub fn len(&self) -> usize {
        self.playlist.len()
    }

    pub fn get(&self, item: &T) -> Option<&T> {
        self.playlist.iter().find(|&i| i == item)
    }
}

impl<T> Default for Playlist<T>
where
    T: Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub enum PlaylistEvent<T> {
    Add(T),
    Remove(T),
}

impl<T> Observable<PlaylistEvent<T>> for Playlist<T>
where
    T: Clone,
{
    fn attach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<(dyn Observer<PlaylistEvent<T>> + Send + 'static)>>>,
    ) {
        self.observers.attach(observer);
    }

    fn detach(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<(dyn Observer<PlaylistEvent<T>> + Send + 'static)>>>,
    ) {
        self.observers.detach(observer);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn playlist_toggle() {
        let mut playlist: Playlist<&str> = Playlist::new();
        assert_eq!(playlist.len(), 0);

        playlist.toggle(&"Item1");
        assert_eq!(playlist.len(), 1);

        playlist.toggle(&"Item2");
        assert_eq!(playlist.len(), 2);

        playlist.toggle(&"Item2");
        assert_eq!(playlist.len(), 1);
    }
}
