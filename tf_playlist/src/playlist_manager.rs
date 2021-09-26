use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

use tf_observer::{Observable, Observer};

use crate::{Playlist, PlaylistEvent};

#[derive(Clone)]
pub struct PlaylistManager<I, T> {
    playlists: Arc<Mutex<PlaylistManagerInternal<I, T>>>,
}

struct PlaylistManagerInternal<I, T> {
    playlists: HashMap<I, Playlist<T>>,
}

impl<I, T> PlaylistManagerInternal<I, T>
where
    I: Hash + Eq + Clone,
    T: Hash + Eq + Clone,
{
    fn new() -> Self {
        PlaylistManagerInternal {
            playlists: HashMap::new(),
        }
    }

    fn toggle(&mut self, ident: &I, item: &T) {
        if let Some(playlist) = self.playlists.get_mut(ident.borrow()) {
            log::debug!("Adding item to existing playlist");
            playlist.toggle(item.borrow());
        } else {
            log::debug!("Creating new playlist for item");
            let mut playlist = Playlist::new();
            playlist.toggle(item.borrow());
            self.playlists.insert(ident.borrow().clone(), playlist);
        }
    }

    fn items(&self, ident: &I) -> Vec<&T> {
        if let Some(playlist) = self.playlists.get(ident.borrow()) {
            playlist.iter().collect()
        } else {
            vec![]
        }
    }

    fn attach_at(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<(dyn Observer<PlaylistEvent<T>> + Send + 'static)>>>,
        ident: &I,
    ) {
        if let Some(playlist) = self.playlists.get_mut(ident.borrow()) {
            log::debug!("Adding observer to existing playlist");
            playlist.attach(observer);
        } else {
            log::debug!("Creating new playlist for observer");
            let mut playlist = Playlist::new();
            playlist.attach(observer);
            self.playlists.insert(ident.borrow().clone(), playlist);
        }
    }

    fn detach_at(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<(dyn Observer<PlaylistEvent<T>> + Send + 'static)>>>,
        ident: &I,
    ) {
        if let Some(playlist) = self.playlists.get_mut(ident.borrow()) {
            playlist.detach(observer);
        }
    }
}

impl<I, T> PlaylistManager<I, T>
where
    I: Hash + Eq + Clone,
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        PlaylistManager {
            playlists: Arc::new(Mutex::new(PlaylistManagerInternal::new())),
        }
    }

    pub fn toggle(&mut self, ident: &I, item: &T) {
        self.playlists.lock().unwrap().toggle(ident, item);
    }

    pub fn items(&self, ident: &I) -> Vec<T> {
        self.playlists
            .lock()
            .unwrap()
            .items(ident)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn attach_at(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<(dyn Observer<PlaylistEvent<T>> + Send + 'static)>>>,
        ident: &I,
    ) {
        self.playlists.lock().unwrap().attach_at(observer, ident);
    }

    pub fn detach_at(
        &mut self,
        observer: std::sync::Weak<Mutex<Box<(dyn Observer<PlaylistEvent<T>> + Send + 'static)>>>,
        ident: &I,
    ) {
        self.playlists.lock().unwrap().detach_at(observer, ident);
    }
}

impl<I, T> Default for PlaylistManager<I, T>
where
    I: Hash + Eq + Clone,
    T: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn playlistmanagerinternal_toggle_empty() {
        let mut manager: PlaylistManagerInternal<&str, &str> = PlaylistManagerInternal::new();
        assert!(manager.playlists.is_empty());

        manager.toggle(&"Playlist1", &"Item1_1");

        assert_eq!(manager.playlists.len(), 1);

        assert!(manager.playlists.get(&"Playlist1").is_some());
        assert_eq!(manager.playlists.get(&"Playlist1").unwrap().len(), 1);
        assert!(manager
            .playlists
            .get(&"Playlist1")
            .unwrap()
            .get(&"Item1_1")
            .is_some());
    }

    #[test]
    fn playlistmanagerinternal_toggle_complex() {
        let mut manager: PlaylistManagerInternal<&str, &str> = PlaylistManagerInternal::new();
        assert!(manager.playlists.is_empty());

        manager.toggle(&"Playlist1", &"Item1_1");
        manager.toggle(&"Playlist1", &"Item1_2");
        manager.toggle(&"Playlist2", &"Item1_2");

        assert_eq!(manager.playlists.len(), 2);

        assert!(manager.playlists.get(&"Playlist1").is_some());
        assert!(manager.playlists.get(&"Playlist2").is_some());
        assert_eq!(manager.playlists.get(&"Playlist1").unwrap().len(), 2);
        assert_eq!(manager.playlists.get(&"Playlist2").unwrap().len(), 1);
    }
}
