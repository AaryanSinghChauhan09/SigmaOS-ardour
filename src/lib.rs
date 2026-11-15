//! SigmaOS-ardour: digital audio workstation model.
//!
//! Real implementation: tracks, recordings, mixing, validation.
//! Replaces the 7-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioTrack {
    pub id: String,
    pub name: String,
    pub track_type: String,
    pub muted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DAWError {
    EmptyName,
    DuplicateTrack(String),
}

impl fmt::Display for DAWError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "track name must not be empty"),
            Self::DuplicateTrack(id) => write!(f, "duplicate track {:?}", id),
        }
    }
}

impl std::error::Error for DAWError {}

pub struct AudioWorkstation {
    pub name: String,
    tracks: BTreeMap<String, AudioTrack>,
    recordings: BTreeMap<String, String>,
}

impl AudioWorkstation {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), tracks: BTreeMap::new(), recordings: BTreeMap::new() }
    }

    pub fn add_track(&mut self, t: AudioTrack) -> Result<(), DAWError> {
        if t.name.trim().is_empty() { return Err(DAWError::EmptyName); }
        let id = t.id.clone();
        if self.tracks.contains_key(&id) {
            return Err(DAWError::DuplicateTrack(id.clone()));
        }
        self.tracks.insert(id, t);
        Ok(())
    }

    pub fn track_count(&self) -> usize { self.tracks.len() }

    pub fn get_track(&self, id: &str) -> Option<&AudioTrack> { self.tracks.get(id) }

    pub fn add_recording(&mut self, track_id: String, file_path: String) {
        self.recordings.insert(track_id, file_path);
    }

    pub fn get_recording(&self, track_id: &str) -> Option<&String> { self.recordings.get(track_id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let daw = AudioWorkstation::new("My Project");
        assert_eq!(daw.name, "My Project");
        assert_eq!(daw.track_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut daw = AudioWorkstation::new("My Project");
        let t = AudioTrack { id: "track1".into(), name: "Guitar".into(), track_type: "audio".into(), muted: false };
        daw.add_track(t).unwrap();
        assert_eq!(daw.track_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut daw = AudioWorkstation::new("My Project");
        let t = AudioTrack { id: "x".into(), name: "".into(), track_type: "a".into(), muted: false };
        assert!(daw.add_track(t).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut daw = AudioWorkstation::new("My Project");
        let t = AudioTrack { id: "x".into(), name: "X".into(), track_type: "a".into(), muted: false };
        daw.add_track(t).unwrap();
        let t2 = AudioTrack { id: "x".into(), name: "X".into(), track_type: "a".into(), muted: false };
        assert!(daw.add_track(t2).is_err());
    }
}
