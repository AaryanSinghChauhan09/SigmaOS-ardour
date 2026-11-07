//! ardour model — digital audio workstation.

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub type_: String,
    pub channels: u32,
    pub muted: bool,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub sample_rate: u32,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_session(&mut self, name: &str, sample_rate: u32) -> bool {
        self.sessions.push(Session { name: name.into(), sample_rate, tracks: vec![] });
        true
    }

    pub fn add_track(&mut self, session: &str, track: Track) -> bool {
        if let Some(s) = self.sessions.iter_mut().find(|s| s.name == session) {
            s.tracks.push(track); true
        } else { false }
    }

    pub fn list_sessions(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_track() {
        let mut a = Model::new();
        assert!(a.create_session("mix1", 48000));
        assert!(a.add_track("mix1", Track { name: "vocals".into(), type_: "audio".into(), channels: 2, muted: false }));
        assert_eq!(a.list_sessions()[0].tracks.len(), 1);
    }
}
