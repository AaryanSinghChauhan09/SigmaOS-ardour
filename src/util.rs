//! ardour model — digital audio workstation.

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub tracks: Vec<String>,
    pub buses: Vec<String>,
    pub sample_rate: u32,
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

    pub fn create(&mut self, name: &str, sample_rate: u32) -> usize {
        let id = self.sessions.len();
        self.sessions.push(Session { name: name.into(), tracks: vec![], buses: vec![], sample_rate });
        id
    }

    pub fn add_track(&mut self, session_id: usize, track_name: &str) -> bool {
        if let Some(s) = self.sessions.get_mut(session_id) {
            s.tracks.push(track_name.into());
            true
        } else { false }
    }

    pub fn add_bus(&mut self, session_id: usize, bus_name: &str) -> bool {
        if let Some(s) = self.sessions.get_mut(session_id) {
            s.buses.push(bus_name.into());
            true
        } else { false }
    }

    pub fn list(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_tracks_buses() {
        let mut a = Model::new();
        let id = a.create("mix1", 48000);
        assert!(a.add_track(id, "drums"));
        assert!(a.add_bus(id, "master"));
        assert_eq!(a.sessions[id].tracks.len(), 1);
    }
}
